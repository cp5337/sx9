//! CTAS-7 Security Provenance Module
//! Security audit and vulnerability tracking
//! Follows CTAS-7 standards: ≤200 LOC

use serde::{Deserialize, Serialize};

/// Security audit provenance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityProvenance {
    /// Security audit results
    pub audit_results: SecurityAuditResults,
    /// Vulnerability scan data
    pub vulnerability_scan: VulnerabilityScan,
    /// Cryptographic verification
    pub crypto_verification: CryptoVerification,
    /// Security compliance status
    pub compliance_status: ComplianceStatus,
    /// Tesla security approval
    pub tesla_security_approved: bool,
}

/// Security audit results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditResults {
    /// Audit timestamp
    pub audit_timestamp: u64,
    /// Auditor information
    pub auditor: String,
    /// Audit score (0-100)
    pub audit_score: u8,
    /// Critical findings count
    pub critical_findings: u32,
    /// High severity findings
    pub high_findings: u32,
    /// Medium severity findings
    pub medium_findings: u32,
    /// Low severity findings
    pub low_findings: u32,
    /// Audit report hash
    pub report_hash: String,
    /// Tesla auditor verified
    pub tesla_auditor: bool,
}

/// Vulnerability scanning results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityScan {
    /// Scan timestamp
    pub scan_timestamp: u64,
    /// Scanner used
    pub scanner: String,
    /// Scanner version
    pub scanner_version: String,
    /// Known vulnerabilities found
    pub vulnerabilities: Vec<Vulnerability>,
    /// Dependency vulnerabilities
    pub dependency_vulns: Vec<DependencyVulnerability>,
    /// Overall risk score
    pub risk_score: u8,
}

/// Individual vulnerability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub id: String, // CVE, etc.
    pub severity: String,
    pub description: String,
    pub affected_component: String,
    pub fix_available: bool,
    pub fix_version: Option<String>,
    pub cvss_score: f32,
}

/// Dependency vulnerability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyVulnerability {
    pub dependency_name: String,
    pub dependency_version: String,
    pub vulnerability: Vulnerability,
    pub patched: bool,
}

/// Cryptographic verification data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoVerification {
    /// Cryptographic libraries used
    pub crypto_libs: Vec<String>,
    /// Encryption algorithms validated
    pub validated_algorithms: Vec<String>,
    /// Key management validation
    pub key_management_secure: bool,
    /// Random number generation secure
    pub rng_secure: bool,
    /// Tesla crypto standards compliance
    pub tesla_crypto_compliant: bool,
}

/// Security compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    /// NIST compliance
    pub nist_compliant: bool,
    /// ISO 27001 compliance
    pub iso27001_compliant: bool,
    /// SOC 2 compliance
    pub soc2_compliant: bool,
    /// Tesla security standards
    pub tesla_standards_met: bool,
    /// Compliance verification date
    pub verification_date: u64,
}

impl Default for SecurityProvenance {
    fn default() -> Self {
        Self {
            audit_results: SecurityAuditResults::default(),
            vulnerability_scan: VulnerabilityScan::default(),
            crypto_verification: CryptoVerification::default(),
            compliance_status: ComplianceStatus::default(),
            tesla_security_approved: false,
        }
    }
}

impl Default for SecurityAuditResults {
    fn default() -> Self {
        Self {
            audit_timestamp: 0,
            auditor: String::new(),
            audit_score: 0,
            critical_findings: 0,
            high_findings: 0,
            medium_findings: 0,
            low_findings: 0,
            report_hash: String::new(),
            tesla_auditor: false,
        }
    }
}

impl Default for VulnerabilityScan {
    fn default() -> Self {
        Self {
            scan_timestamp: 0,
            scanner: String::new(),
            scanner_version: String::new(),
            vulnerabilities: Vec::new(),
            dependency_vulns: Vec::new(),
            risk_score: 0,
        }
    }
}

impl Default for CryptoVerification {
    fn default() -> Self {
        Self {
            crypto_libs: Vec::new(),
            validated_algorithms: Vec::new(),
            key_management_secure: false,
            rng_secure: false,
            tesla_crypto_compliant: false,
        }
    }
}

impl Default for ComplianceStatus {
    fn default() -> Self {
        Self {
            nist_compliant: false,
            iso27001_compliant: false,
            soc2_compliant: false,
            tesla_standards_met: false,
            verification_date: 0,
        }
    }
}

impl SecurityProvenance {
    /// Validate security provenance
    pub fn is_valid(&self) -> bool {
        self.audit_results.audit_timestamp > 0
            && self.vulnerability_scan.scan_timestamp > 0
            && self.audit_results.audit_score > 0
    }

    /// Tesla-grade security validation
    pub fn tesla_security_validation(&self) -> bool {
        self.tesla_security_approved
            && self.audit_results.tesla_auditor
            && self.audit_results.critical_findings == 0
            && self.audit_results.audit_score >= 95
            && self.compliance_status.tesla_standards_met
            && self.crypto_verification.tesla_crypto_compliant
    }

    /// Calculate security risk score
    pub fn calculate_risk_score(&mut self) -> u8 {
        let audit_weight = 0.4;
        let vuln_weight = 0.3;
        let compliance_weight = 0.3;

        let audit_score = self.audit_results.audit_score as f32;
        let vuln_score = (100 - self.vulnerability_scan.risk_score) as f32;
        let compliance_score = if self.compliance_status.tesla_standards_met { 100.0 } else { 50.0 };

        let weighted_score = (audit_score * audit_weight)
            + (vuln_score * vuln_weight)
            + (compliance_score * compliance_weight);

        self.vulnerability_scan.risk_score = (100 - weighted_score as u8);
        self.vulnerability_scan.risk_score
    }

    /// Get critical vulnerabilities count
    pub fn critical_vulnerabilities_count(&self) -> usize {
        self.vulnerability_scan.vulnerabilities
            .iter()
            .filter(|v| v.severity == "Critical")
            .count()
    }

    /// Check if all vulnerabilities are patched
    pub fn all_vulnerabilities_patched(&self) -> bool {
        self.vulnerability_scan.dependency_vulns
            .iter()
            .all(|dv| dv.patched)
            && self.vulnerability_scan.vulnerabilities
                .iter()
                .all(|v| v.fix_available)
    }
}

impl SecurityAuditResults {
    /// Create new audit results
    pub fn new(auditor: String, tesla_auditor: bool) -> Self {
        Self {
            audit_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            auditor,
            audit_score: 0,
            critical_findings: 0,
            high_findings: 0,
            medium_findings: 0,
            low_findings: 0,
            report_hash: String::new(),
            tesla_auditor,
        }
    }

    /// Calculate total findings
    pub fn total_findings(&self) -> u32 {
        self.critical_findings + self.high_findings + self.medium_findings + self.low_findings
    }
}

impl VulnerabilityScan {
    /// Create new vulnerability scan
    pub fn new(scanner: String, scanner_version: String) -> Self {
        Self {
            scan_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            scanner,
            scanner_version,
            vulnerabilities: Vec::new(),
            dependency_vulns: Vec::new(),
            risk_score: 0,
        }
    }

    /// Add vulnerability
    pub fn add_vulnerability(&mut self, vulnerability: Vulnerability) {
        self.vulnerabilities.push(vulnerability);
    }

    /// Add dependency vulnerability
    pub fn add_dependency_vulnerability(&mut self, dep_vuln: DependencyVulnerability) {
        self.dependency_vulns.push(dep_vuln);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_provenance_validation() {
        let mut prov = SecurityProvenance::default();
        assert!(!prov.is_valid());

        prov.audit_results.audit_timestamp = 1234567890;
        prov.vulnerability_scan.scan_timestamp = 1234567890;
        prov.audit_results.audit_score = 95;

        assert!(prov.is_valid());
    }

    #[test]
    fn test_tesla_security_validation() {
        let mut prov = SecurityProvenance::default();
        prov.tesla_security_approved = true;
        prov.audit_results.tesla_auditor = true;
        prov.audit_results.critical_findings = 0;
        prov.audit_results.audit_score = 95;
        prov.compliance_status.tesla_standards_met = true;
        prov.crypto_verification.tesla_crypto_compliant = true;

        assert!(prov.tesla_security_validation());
    }
}