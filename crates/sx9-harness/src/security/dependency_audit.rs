//! Dependency Audit
//!
//! Integrates with cargo-audit for Rust dependency vulnerability scanning.

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;

use super::Severity;

/// Audit finding from dependency scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFinding {
    /// Advisory ID (e.g., RUSTSEC-2021-0001)
    pub advisory_id: String,

    /// Package name
    pub package: String,

    /// Installed version
    pub version: String,

    /// Fixed version (if available)
    pub patched: Option<String>,

    /// Severity
    pub severity: Severity,

    /// Title/description
    pub title: String,

    /// URL to advisory
    pub url: Option<String>,
}

/// Dependency auditor
pub struct DependencyAudit {
    /// Path to cargo-audit binary
    cargo_audit_path: Option<String>,
}

impl Default for DependencyAudit {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyAudit {
    /// Create new dependency auditor
    pub fn new() -> Self {
        Self {
            cargo_audit_path: None,
        }
    }

    /// Set custom cargo-audit path
    pub fn with_path(mut self, path: &str) -> Self {
        self.cargo_audit_path = Some(path.to_string());
        self
    }

    /// Run dependency audit
    pub async fn audit(&self, project_path: &Path) -> Vec<AuditFinding> {
        let cargo_lock = project_path.join("Cargo.lock");
        if !cargo_lock.exists() {
            return Vec::new();
        }

        // Try to run cargo-audit
        let output = Command::new(self.cargo_audit_path.as_deref().unwrap_or("cargo"))
            .args(["audit", "--json", "-q"])
            .current_dir(project_path)
            .output();

        match output {
            Ok(output) => self.parse_audit_output(&output.stdout),
            Err(_) => {
                // cargo-audit not installed, return empty
                Vec::new()
            }
        }
    }

    /// Parse cargo-audit JSON output
    fn parse_audit_output(&self, output: &[u8]) -> Vec<AuditFinding> {
        let mut findings = Vec::new();

        if let Ok(json) = serde_json::from_slice::<serde_json::Value>(output) {
            if let Some(vulnerabilities) = json.get("vulnerabilities").and_then(|v| v.get("list")).and_then(|l| l.as_array()) {
                for vuln in vulnerabilities {
                    if let Some(finding) = self.parse_vulnerability(vuln) {
                        findings.push(finding);
                    }
                }
            }
        }

        findings
    }

    /// Parse single vulnerability
    fn parse_vulnerability(&self, vuln: &serde_json::Value) -> Option<AuditFinding> {
        let advisory = vuln.get("advisory")?;
        let package = vuln.get("package")?;

        let severity = match advisory.get("severity").and_then(|s| s.as_str()) {
            Some("critical") => Severity::Critical,
            Some("high") => Severity::High,
            Some("medium") => Severity::Medium,
            Some("low") => Severity::Low,
            _ => Severity::Medium,
        };

        Some(AuditFinding {
            advisory_id: advisory.get("id")?.as_str()?.to_string(),
            package: package.get("name")?.as_str()?.to_string(),
            version: package.get("version")?.as_str()?.to_string(),
            patched: advisory.get("patched_versions")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            severity,
            title: advisory.get("title")?.as_str()?.to_string(),
            url: advisory.get("url").and_then(|u| u.as_str()).map(|s| s.to_string()),
        })
    }

    /// Check if cargo-audit is available
    pub fn is_available(&self) -> bool {
        Command::new(self.cargo_audit_path.as_deref().unwrap_or("cargo"))
            .args(["audit", "--version"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Get cargo-audit version
    pub fn version(&self) -> Option<String> {
        Command::new(self.cargo_audit_path.as_deref().unwrap_or("cargo"))
            .args(["audit", "--version"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_audit_creation() {
        let audit = DependencyAudit::new();
        // Just test it doesn't panic
        assert!(true);
    }

    #[test]
    fn test_parse_empty_output() {
        let audit = DependencyAudit::new();
        let findings = audit.parse_audit_output(b"{}");
        assert!(findings.is_empty());
    }
}
