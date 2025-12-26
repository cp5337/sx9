//! DevSecOps Security Module
//!
//! Implements DoD DevSecOps Reference Design requirements for cATO compliance.
//!
//! ## DoD cATO Requirements (2024)
//!
//! Three pillars for Continuous Authorization to Operate:
//! 1. Continuous monitoring of RMF controls
//! 2. Active cyber defense
//! 3. Approved DevSecOps reference design with secure software supply chain
//!
//! ## Pipeline Security Gates
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                    DOD DEVSECOPS PIPELINE                                │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │                                                                          │
//! │  CODE ──► SECRETS ──► SAST ──► BUILD ──► SBOM ──► SCAN ──► SIGN ──► DEPLOY
//! │            SCAN       SCAN              GEN       IMAGE    ARTIFACT      │
//! │                                                                          │
//! │  ┌─────────────────────────────────────────────────────────────────────┐│
//! │  │                     POLICY ENGINE                                   ││
//! │  │  • No critical CVEs      • SBOM complete                           ││
//! │  │  • No secrets in code    • Dependencies audited                    ││
//! │  │  • Signed artifacts      • Container hardened                      ││
//! │  └─────────────────────────────────────────────────────────────────────┘│
//! │                                                                          │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## References
//!
//! - DoD DevSecOps Continuous Authorization Implementation Guide (April 2024)
//! - DoD Container Hardening Guide
//! - DoD Container Image Creation and Deployment Guide
//! - Iron Bank: https://ironbank.dsop.io

use serde::{Deserialize, Serialize};

/// Severity levels for security findings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

pub mod sbom;
pub mod secrets_scan;
pub mod dependency_audit;
pub mod container_scan;
pub mod compliance;

pub use sbom::{SbomGenerator, SbomFormat, SoftwareBom};
pub use secrets_scan::{SecretsScan, SecretFinding, SecretType};
pub use dependency_audit::{DependencyAudit, AuditFinding};
pub use container_scan::{ContainerScan, ContainerFinding};
pub use compliance::{ComplianceChecker, ComplianceReport, CatoStatus};

/// DevSecOps pipeline security result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPipelineResult {
    /// SBOM generated
    pub sbom: Option<SoftwareBom>,

    /// Secrets scan results
    pub secrets: Vec<SecretFinding>,

    /// Dependency audit results
    pub audit: Vec<AuditFinding>,

    /// Container scan results
    pub container: Vec<ContainerFinding>,

    /// Overall compliance status
    pub compliance: ComplianceReport,

    /// Pipeline passed all gates
    pub passed: bool,

    /// Blocking issues
    pub blockers: Vec<String>,
}

impl SecurityPipelineResult {
    /// Check if pipeline can proceed
    pub fn can_proceed(&self) -> bool {
        self.passed && self.blockers.is_empty()
    }

    /// Get summary for reporting
    pub fn summary(&self) -> String {
        format!(
            "Security Pipeline: {} | Secrets: {} found | CVEs: {} | cATO: {:?}",
            if self.passed { "PASSED" } else { "FAILED" },
            self.secrets.len(),
            self.audit.len(),
            self.compliance.cato_status
        )
    }
}

/// Run full security pipeline
pub async fn run_security_pipeline(
    crate_path: &std::path::Path,
    container_image: Option<&str>,
) -> Result<SecurityPipelineResult, String> {
    let mut blockers = Vec::new();

    // 1. Generate SBOM
    let sbom_gen = SbomGenerator::new();
    let sbom = sbom_gen.generate(crate_path).ok();

    // 2. Scan for secrets
    let secrets_scanner = SecretsScan::new();
    let secrets = secrets_scanner.scan(crate_path).await;

    if secrets.iter().any(|s| s.severity == Severity::Critical) {
        blockers.push("Critical secrets found in codebase".to_string());
    }

    // 3. Audit dependencies
    let auditor = DependencyAudit::new();
    let audit = auditor.audit(crate_path).await;

    let critical_vulns = audit.iter()
        .filter(|v| v.severity == Severity::Critical)
        .count();
    if critical_vulns > 0 {
        blockers.push(format!("{} critical vulnerabilities found", critical_vulns));
    }

    // 4. Scan container (if provided)
    let container = if let Some(image) = container_image {
        let scanner = ContainerScan::new();
        scanner.scan(image).await
    } else {
        Vec::new()
    };

    if container.iter().any(|f| f.severity == Severity::Critical) {
        blockers.push("Critical container vulnerabilities found".to_string());
    }

    // 5. Check compliance
    let checker = ComplianceChecker::new();
    let compliance = checker.check(&sbom, &audit, &secrets, &container);

    let passed = blockers.is_empty() && compliance.cato_status != CatoStatus::NotCompliant;

    Ok(SecurityPipelineResult {
        sbom,
        secrets,
        audit,
        container,
        compliance,
        passed,
        blockers,
    })
}
