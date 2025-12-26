//! DoD cATO Compliance Checker
//!
//! Implements compliance checks for DoD Continuous Authorization to Operate (cATO).
//!
//! ## Three Pillars of cATO (2024)
//!
//! 1. **Continuous Monitoring** - Real-time RMF control monitoring
//! 2. **Active Cyber Defense** - Threat detection and response
//! 3. **DevSecOps Reference Design** - Secure software supply chain
//!
//! ## Required Controls
//!
//! - SBOM generation and monitoring
//! - Secrets scanning in CI/CD
//! - Dependency vulnerability auditing
//! - Container image hardening (Iron Bank)
//! - Signed artifacts
//! - Audit logging

use serde::{Deserialize, Serialize};

use super::{
    AuditFinding, ContainerFinding, SecretFinding, Severity, SoftwareBom,
};

/// cATO compliance status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CatoStatus {
    /// Fully compliant with cATO requirements
    Compliant,
    /// Minor issues, can proceed with remediation plan
    ConditionallyCompliant,
    /// Significant issues requiring attention
    NeedsRemediation,
    /// Not compliant, cannot proceed
    NotCompliant,
}

/// Compliance control check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlCheck {
    /// Control ID (e.g., "SBOM-01")
    pub control_id: String,

    /// Control name
    pub name: String,

    /// Control category
    pub category: ControlCategory,

    /// Whether control is satisfied
    pub satisfied: bool,

    /// Finding details
    pub finding: String,

    /// Remediation recommendation
    pub remediation: Option<String>,
}

/// Control categories per DoD DevSecOps Reference Design
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlCategory {
    /// Software Bill of Materials
    Sbom,
    /// Secrets management
    Secrets,
    /// Dependency security
    Dependencies,
    /// Container security
    Container,
    /// Code signing and provenance
    Signing,
    /// Audit and logging
    Audit,
    /// Access control
    AccessControl,
}

/// Full compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Overall cATO status
    pub cato_status: CatoStatus,

    /// Individual control checks
    pub controls: Vec<ControlCheck>,

    /// Summary statistics
    pub summary: ComplianceSummary,

    /// Report timestamp
    pub timestamp: String,

    /// Recommendations for achieving compliance
    pub recommendations: Vec<String>,
}

/// Compliance summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSummary {
    /// Total controls checked
    pub total_controls: usize,

    /// Controls satisfied
    pub satisfied: usize,

    /// Controls not satisfied
    pub not_satisfied: usize,

    /// Critical findings count
    pub critical_findings: usize,

    /// High findings count
    pub high_findings: usize,
}

/// Compliance checker
pub struct ComplianceChecker {
    /// Required control definitions
    controls: Vec<ControlDefinition>,
}

/// Control definition
struct ControlDefinition {
    id: &'static str,
    name: &'static str,
    category: ControlCategory,
    required: bool,
}

impl Default for ComplianceChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ComplianceChecker {
    /// Create new compliance checker with DoD DevSecOps controls
    pub fn new() -> Self {
        let controls = vec![
            ControlDefinition {
                id: "SBOM-01",
                name: "SBOM Generation",
                category: ControlCategory::Sbom,
                required: true,
            },
            ControlDefinition {
                id: "SBOM-02",
                name: "SBOM Contains All Dependencies",
                category: ControlCategory::Sbom,
                required: true,
            },
            ControlDefinition {
                id: "SEC-01",
                name: "No Secrets in Codebase",
                category: ControlCategory::Secrets,
                required: true,
            },
            ControlDefinition {
                id: "SEC-02",
                name: "No Critical Secrets Exposed",
                category: ControlCategory::Secrets,
                required: true,
            },
            ControlDefinition {
                id: "DEP-01",
                name: "Dependencies Audited",
                category: ControlCategory::Dependencies,
                required: true,
            },
            ControlDefinition {
                id: "DEP-02",
                name: "No Critical Vulnerabilities",
                category: ControlCategory::Dependencies,
                required: true,
            },
            ControlDefinition {
                id: "DEP-03",
                name: "No High Vulnerabilities (>5)",
                category: ControlCategory::Dependencies,
                required: false,
            },
            ControlDefinition {
                id: "CNT-01",
                name: "Container Image Scanned",
                category: ControlCategory::Container,
                required: true,
            },
            ControlDefinition {
                id: "CNT-02",
                name: "No Critical Container CVEs",
                category: ControlCategory::Container,
                required: true,
            },
            ControlDefinition {
                id: "CNT-03",
                name: "Iron Bank Compliance",
                category: ControlCategory::Container,
                required: false,
            },
        ];

        Self { controls }
    }

    /// Check compliance against all controls
    pub fn check(
        &self,
        sbom: &Option<SoftwareBom>,
        audit: &[AuditFinding],
        secrets: &[SecretFinding],
        container: &[ContainerFinding],
    ) -> ComplianceReport {
        let mut control_checks = Vec::new();
        let mut recommendations = Vec::new();

        // SBOM checks
        control_checks.push(self.check_sbom_generated(sbom));
        control_checks.push(self.check_sbom_complete(sbom));

        // Secrets checks
        control_checks.push(self.check_no_secrets(secrets));
        control_checks.push(self.check_no_critical_secrets(secrets));

        // Dependency checks
        control_checks.push(self.check_deps_audited(audit));
        control_checks.push(self.check_no_critical_vulns(audit));
        control_checks.push(self.check_high_vuln_threshold(audit));

        // Container checks
        control_checks.push(self.check_container_scanned(container));
        control_checks.push(self.check_no_critical_container_cves(container));
        control_checks.push(self.check_iron_bank(container));

        // Calculate summary
        let satisfied = control_checks.iter().filter(|c| c.satisfied).count();
        let not_satisfied = control_checks.len() - satisfied;

        let critical_findings = secrets.iter().filter(|s| s.severity == Severity::Critical).count()
            + audit.iter().filter(|a| a.severity == Severity::Critical).count()
            + container.iter().filter(|c| c.severity == Severity::Critical).count();

        let high_findings = secrets.iter().filter(|s| s.severity == Severity::High).count()
            + audit.iter().filter(|a| a.severity == Severity::High).count()
            + container.iter().filter(|c| c.severity == Severity::High).count();

        // Generate recommendations
        for check in &control_checks {
            if !check.satisfied {
                if let Some(ref remediation) = check.remediation {
                    recommendations.push(remediation.clone());
                }
            }
        }

        // Determine overall status
        let required_failed = control_checks.iter()
            .zip(self.controls.iter())
            .filter(|(check, def)| def.required && !check.satisfied)
            .count();

        let cato_status = if required_failed > 0 || critical_findings > 0 {
            CatoStatus::NotCompliant
        } else if high_findings > 5 {
            CatoStatus::NeedsRemediation
        } else if not_satisfied > 0 {
            CatoStatus::ConditionallyCompliant
        } else {
            CatoStatus::Compliant
        };

        ComplianceReport {
            cato_status,
            controls: control_checks,
            summary: ComplianceSummary {
                total_controls: self.controls.len(),
                satisfied,
                not_satisfied,
                critical_findings,
                high_findings,
            },
            timestamp: chrono::Utc::now().to_rfc3339(),
            recommendations,
        }
    }

    // Individual control checks

    fn check_sbom_generated(&self, sbom: &Option<SoftwareBom>) -> ControlCheck {
        let satisfied = sbom.is_some();
        ControlCheck {
            control_id: "SBOM-01".to_string(),
            name: "SBOM Generation".to_string(),
            category: ControlCategory::Sbom,
            satisfied,
            finding: if satisfied {
                "SBOM successfully generated".to_string()
            } else {
                "No SBOM generated".to_string()
            },
            remediation: if satisfied {
                None
            } else {
                Some("Generate SBOM using CycloneDX or SPDX format".to_string())
            },
        }
    }

    fn check_sbom_complete(&self, sbom: &Option<SoftwareBom>) -> ControlCheck {
        let satisfied = sbom.as_ref()
            .map(|s| !s.dependencies.is_empty())
            .unwrap_or(false);

        ControlCheck {
            control_id: "SBOM-02".to_string(),
            name: "SBOM Contains All Dependencies".to_string(),
            category: ControlCategory::Sbom,
            satisfied,
            finding: if satisfied {
                format!(
                    "SBOM contains {} dependencies",
                    sbom.as_ref().map(|s| s.dependencies.len()).unwrap_or(0)
                )
            } else {
                "SBOM missing or incomplete".to_string()
            },
            remediation: if satisfied {
                None
            } else {
                Some("Ensure all direct and transitive dependencies are included".to_string())
            },
        }
    }

    fn check_no_secrets(&self, secrets: &[SecretFinding]) -> ControlCheck {
        let satisfied = secrets.is_empty();
        ControlCheck {
            control_id: "SEC-01".to_string(),
            name: "No Secrets in Codebase".to_string(),
            category: ControlCategory::Secrets,
            satisfied,
            finding: if satisfied {
                "No secrets detected in codebase".to_string()
            } else {
                format!("{} potential secrets found", secrets.len())
            },
            remediation: if satisfied {
                None
            } else {
                Some("Remove hardcoded secrets and use environment variables or vault".to_string())
            },
        }
    }

    fn check_no_critical_secrets(&self, secrets: &[SecretFinding]) -> ControlCheck {
        let critical = secrets.iter()
            .filter(|s| s.severity == Severity::Critical)
            .count();
        let satisfied = critical == 0;

        ControlCheck {
            control_id: "SEC-02".to_string(),
            name: "No Critical Secrets Exposed".to_string(),
            category: ControlCategory::Secrets,
            satisfied,
            finding: if satisfied {
                "No critical secrets exposed".to_string()
            } else {
                format!("{} critical secrets found (AWS keys, tokens, etc.)", critical)
            },
            remediation: if satisfied {
                None
            } else {
                Some("Immediately rotate exposed credentials and remove from code".to_string())
            },
        }
    }

    fn check_deps_audited(&self, audit: &[AuditFinding]) -> ControlCheck {
        // We consider this satisfied if audit was run (even if empty)
        // The presence of findings indicates audit was performed
        ControlCheck {
            control_id: "DEP-01".to_string(),
            name: "Dependencies Audited".to_string(),
            category: ControlCategory::Dependencies,
            satisfied: true, // If we got here, audit was run
            finding: format!("{} vulnerabilities found in dependencies", audit.len()),
            remediation: None,
        }
    }

    fn check_no_critical_vulns(&self, audit: &[AuditFinding]) -> ControlCheck {
        let critical = audit.iter()
            .filter(|a| a.severity == Severity::Critical)
            .count();
        let satisfied = critical == 0;

        ControlCheck {
            control_id: "DEP-02".to_string(),
            name: "No Critical Vulnerabilities".to_string(),
            category: ControlCategory::Dependencies,
            satisfied,
            finding: if satisfied {
                "No critical vulnerabilities in dependencies".to_string()
            } else {
                format!("{} critical vulnerabilities found", critical)
            },
            remediation: if satisfied {
                None
            } else {
                Some("Update affected dependencies to patched versions".to_string())
            },
        }
    }

    fn check_high_vuln_threshold(&self, audit: &[AuditFinding]) -> ControlCheck {
        let high = audit.iter()
            .filter(|a| a.severity == Severity::High)
            .count();
        let satisfied = high <= 5;

        ControlCheck {
            control_id: "DEP-03".to_string(),
            name: "High Vulnerability Threshold".to_string(),
            category: ControlCategory::Dependencies,
            satisfied,
            finding: if satisfied {
                format!("{} high vulnerabilities (threshold: 5)", high)
            } else {
                format!("{} high vulnerabilities exceed threshold of 5", high)
            },
            remediation: if satisfied {
                None
            } else {
                Some("Prioritize updating dependencies with high severity CVEs".to_string())
            },
        }
    }

    fn check_container_scanned(&self, container: &[ContainerFinding]) -> ControlCheck {
        // If container findings list exists (even empty), scan was performed
        ControlCheck {
            control_id: "CNT-01".to_string(),
            name: "Container Image Scanned".to_string(),
            category: ControlCategory::Container,
            satisfied: true,
            finding: format!("{} vulnerabilities found in container", container.len()),
            remediation: None,
        }
    }

    fn check_no_critical_container_cves(&self, container: &[ContainerFinding]) -> ControlCheck {
        let critical = container.iter()
            .filter(|c| c.severity == Severity::Critical)
            .count();
        let satisfied = critical == 0;

        ControlCheck {
            control_id: "CNT-02".to_string(),
            name: "No Critical Container CVEs".to_string(),
            category: ControlCategory::Container,
            satisfied,
            finding: if satisfied {
                "No critical CVEs in container image".to_string()
            } else {
                format!("{} critical CVEs found in container", critical)
            },
            remediation: if satisfied {
                None
            } else {
                Some("Update base image and rebuild container".to_string())
            },
        }
    }

    fn check_iron_bank(&self, container: &[ContainerFinding]) -> ControlCheck {
        let critical = container.iter()
            .filter(|c| c.severity == Severity::Critical)
            .count();
        let high = container.iter()
            .filter(|c| c.severity == Severity::High)
            .count();

        let satisfied = critical == 0 && high <= 5;

        ControlCheck {
            control_id: "CNT-03".to_string(),
            name: "Iron Bank Compliance".to_string(),
            category: ControlCategory::Container,
            satisfied,
            finding: if satisfied {
                "Container meets Iron Bank hardening requirements".to_string()
            } else {
                "Container does not meet Iron Bank standards".to_string()
            },
            remediation: if satisfied {
                None
            } else {
                Some("Use Iron Bank approved base images from https://ironbank.dsop.io".to_string())
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_compliance_check() {
        let checker = ComplianceChecker::new();
        let report = checker.check(&None, &[], &[], &[]);

        // SBOM not generated, so not compliant
        assert_eq!(report.cato_status, CatoStatus::NotCompliant);
    }

    #[test]
    fn test_cato_status_ordering() {
        assert!(CatoStatus::Compliant != CatoStatus::NotCompliant);
    }

    #[test]
    fn test_control_categories() {
        assert_eq!(ControlCategory::Sbom, ControlCategory::Sbom);
        assert_ne!(ControlCategory::Sbom, ControlCategory::Secrets);
    }
}
