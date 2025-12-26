//! Container Scanning
//!
//! Integrates with Trivy/Grype for container image vulnerability scanning.
//! Aligned with DoD Container Hardening Guide and Iron Bank requirements.

use serde::{Deserialize, Serialize};
use std::process::Command;

use super::Severity;

/// Container scan finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerFinding {
    /// CVE or advisory ID
    pub id: String,

    /// Package name
    pub package: String,

    /// Installed version
    pub version: String,

    /// Fixed version (if available)
    pub fixed_version: Option<String>,

    /// Severity
    pub severity: Severity,

    /// Description
    pub description: String,

    /// Layer in which vulnerability was found
    pub layer: Option<String>,
}

/// Container scanner integrating with Trivy or Grype
pub struct ContainerScan {
    /// Scanner to use (trivy or grype)
    scanner: ScannerType,
}

/// Available container scanners
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScannerType {
    Trivy,
    Grype,
}

impl Default for ContainerScan {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerScan {
    /// Create new container scanner (auto-detects available scanner)
    pub fn new() -> Self {
        let scanner = if Self::is_trivy_available() {
            ScannerType::Trivy
        } else {
            ScannerType::Grype
        };

        Self { scanner }
    }

    /// Use specific scanner
    pub fn with_scanner(mut self, scanner: ScannerType) -> Self {
        self.scanner = scanner;
        self
    }

    /// Scan container image for vulnerabilities
    pub async fn scan(&self, image: &str) -> Vec<ContainerFinding> {
        match self.scanner {
            ScannerType::Trivy => self.scan_with_trivy(image),
            ScannerType::Grype => self.scan_with_grype(image),
        }
    }

    /// Scan using Trivy
    fn scan_with_trivy(&self, image: &str) -> Vec<ContainerFinding> {
        let output = Command::new("trivy")
            .args(["image", "--format", "json", "--quiet", image])
            .output();

        match output {
            Ok(output) => self.parse_trivy_output(&output.stdout),
            Err(_) => Vec::new(),
        }
    }

    /// Parse Trivy JSON output
    fn parse_trivy_output(&self, output: &[u8]) -> Vec<ContainerFinding> {
        let mut findings = Vec::new();

        if let Ok(json) = serde_json::from_slice::<serde_json::Value>(output) {
            if let Some(results) = json.get("Results").and_then(|r| r.as_array()) {
                for result in results {
                    let target = result.get("Target")
                        .and_then(|t| t.as_str())
                        .map(|s| s.to_string());

                    if let Some(vulns) = result.get("Vulnerabilities").and_then(|v| v.as_array()) {
                        for vuln in vulns {
                            if let Some(finding) = self.parse_trivy_vuln(vuln, &target) {
                                findings.push(finding);
                            }
                        }
                    }
                }
            }
        }

        findings
    }

    /// Parse single Trivy vulnerability
    fn parse_trivy_vuln(&self, vuln: &serde_json::Value, layer: &Option<String>) -> Option<ContainerFinding> {
        let severity = match vuln.get("Severity").and_then(|s| s.as_str()) {
            Some("CRITICAL") => Severity::Critical,
            Some("HIGH") => Severity::High,
            Some("MEDIUM") => Severity::Medium,
            Some("LOW") => Severity::Low,
            _ => Severity::Info,
        };

        Some(ContainerFinding {
            id: vuln.get("VulnerabilityID")?.as_str()?.to_string(),
            package: vuln.get("PkgName")?.as_str()?.to_string(),
            version: vuln.get("InstalledVersion")?.as_str()?.to_string(),
            fixed_version: vuln.get("FixedVersion")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            severity,
            description: vuln.get("Title")
                .or(vuln.get("Description"))
                .and_then(|v| v.as_str())
                .unwrap_or("No description")
                .to_string(),
            layer: layer.clone(),
        })
    }

    /// Scan using Grype
    fn scan_with_grype(&self, image: &str) -> Vec<ContainerFinding> {
        let output = Command::new("grype")
            .args([image, "-o", "json", "-q"])
            .output();

        match output {
            Ok(output) => self.parse_grype_output(&output.stdout),
            Err(_) => Vec::new(),
        }
    }

    /// Parse Grype JSON output
    fn parse_grype_output(&self, output: &[u8]) -> Vec<ContainerFinding> {
        let mut findings = Vec::new();

        if let Ok(json) = serde_json::from_slice::<serde_json::Value>(output) {
            if let Some(matches) = json.get("matches").and_then(|m| m.as_array()) {
                for m in matches {
                    if let Some(finding) = self.parse_grype_match(m) {
                        findings.push(finding);
                    }
                }
            }
        }

        findings
    }

    /// Parse single Grype match
    fn parse_grype_match(&self, m: &serde_json::Value) -> Option<ContainerFinding> {
        let vuln = m.get("vulnerability")?;
        let artifact = m.get("artifact")?;

        let severity = match vuln.get("severity").and_then(|s| s.as_str()) {
            Some("Critical") => Severity::Critical,
            Some("High") => Severity::High,
            Some("Medium") => Severity::Medium,
            Some("Low") => Severity::Low,
            _ => Severity::Info,
        };

        Some(ContainerFinding {
            id: vuln.get("id")?.as_str()?.to_string(),
            package: artifact.get("name")?.as_str()?.to_string(),
            version: artifact.get("version")?.as_str()?.to_string(),
            fixed_version: vuln.get("fix")
                .and_then(|f| f.get("versions"))
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            severity,
            description: vuln.get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("No description")
                .to_string(),
            layer: artifact.get("locations")
                .and_then(|l| l.as_array())
                .and_then(|arr| arr.first())
                .and_then(|loc| loc.get("path"))
                .and_then(|p| p.as_str())
                .map(|s| s.to_string()),
        })
    }

    /// Check if Trivy is available
    fn is_trivy_available() -> bool {
        Command::new("trivy")
            .args(["--version"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Check if Grype is available
    pub fn is_grype_available() -> bool {
        Command::new("grype")
            .args(["version"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Check if any scanner is available
    pub fn is_available(&self) -> bool {
        match self.scanner {
            ScannerType::Trivy => Self::is_trivy_available(),
            ScannerType::Grype => Self::is_grype_available(),
        }
    }

    /// Get Iron Bank compliance check
    pub fn check_iron_bank_compliance(&self, findings: &[ContainerFinding]) -> IronBankCompliance {
        let critical_count = findings.iter()
            .filter(|f| f.severity == Severity::Critical)
            .count();
        let high_count = findings.iter()
            .filter(|f| f.severity == Severity::High)
            .count();

        let status = if critical_count > 0 {
            IronBankStatus::NotCompliant
        } else if high_count > 5 {
            IronBankStatus::NeedsRemediation
        } else {
            IronBankStatus::Compliant
        };

        IronBankCompliance {
            status,
            critical_count,
            high_count,
            total_findings: findings.len(),
            message: match status {
                IronBankStatus::Compliant => "Container meets Iron Bank standards".to_string(),
                IronBankStatus::NeedsRemediation => format!(
                    "Container has {} high severity vulnerabilities - remediation required",
                    high_count
                ),
                IronBankStatus::NotCompliant => format!(
                    "Container has {} critical vulnerabilities - not compliant",
                    critical_count
                ),
            },
        }
    }
}

/// Iron Bank compliance result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IronBankCompliance {
    pub status: IronBankStatus,
    pub critical_count: usize,
    pub high_count: usize,
    pub total_findings: usize,
    pub message: String,
}

/// Iron Bank compliance status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IronBankStatus {
    Compliant,
    NeedsRemediation,
    NotCompliant,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_scan_creation() {
        let scanner = ContainerScan::new();
        // Just test it doesn't panic
        assert!(true);
    }

    #[test]
    fn test_iron_bank_compliance_check() {
        let scanner = ContainerScan::new();

        // Test with no findings
        let compliance = scanner.check_iron_bank_compliance(&[]);
        assert_eq!(compliance.status, IronBankStatus::Compliant);

        // Test with critical finding
        let findings = vec![ContainerFinding {
            id: "CVE-2024-1234".to_string(),
            package: "openssl".to_string(),
            version: "1.0.0".to_string(),
            fixed_version: Some("1.0.1".to_string()),
            severity: Severity::Critical,
            description: "Critical vulnerability".to_string(),
            layer: None,
        }];
        let compliance = scanner.check_iron_bank_compliance(&findings);
        assert_eq!(compliance.status, IronBankStatus::NotCompliant);
    }

    #[test]
    fn test_parse_empty_outputs() {
        let scanner = ContainerScan::new();

        let trivy_findings = scanner.parse_trivy_output(b"{}");
        assert!(trivy_findings.is_empty());

        let grype_findings = scanner.parse_grype_output(b"{}");
        assert!(grype_findings.is_empty());
    }
}
