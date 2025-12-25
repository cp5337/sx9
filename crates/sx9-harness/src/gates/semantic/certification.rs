//! Quality Certification System
//!
//! Provides end-to-end observability through certification stamps
//! issued after all QA gates complete.
//!
//! Per RFC-9141 (Assembly Line QA Doctrine) and RFC-9050 (Two-Heartbeat).
//!
//! # Certification Levels
//!
//! | Level    | Requirements                                    |
//! |----------|-------------------------------------------------|
//! | None     | Gates failed or not run                         |
//! | Bronze   | Static QA passed                                |
//! | Silver   | Static + Semantic passed (warn level)           |
//! | Gold     | All gates passed (gate level), <50% drift       |
//! | Platinum | Enforce mode, <30% drift                        |
//! | Diamond  | Strict mode, zero drift, 90%+ annotations       |

use serde::{Deserialize, Serialize};
use sx9_foundation_core::data::{DateTime, Duration, Utc};

use super::config::EnforcementMode;

// ============================================================================
// CERTIFICATION LEVEL
// ============================================================================

/// Quality certification level - emitted after all gates complete
///
/// Levels are ordered and comparable. Higher levels require stricter
/// QA enforcement and lower drift scores.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationLevel {
    /// No certification - gates failed or not run
    None = 0,
    /// Bronze - Static QA passed, semantic advisory
    Bronze = 1,
    /// Silver - Static + Semantic passed at warn level
    Silver = 2,
    /// Gold - All gates passed at gate level
    Gold = 3,
    /// Platinum - All gates passed at enforce level, no drift
    Platinum = 4,
    /// Diamond - Strict mode, zero drift, full annotations
    Diamond = 5,
}

impl CertificationLevel {
    /// Display badge text with emoji
    pub fn badge(&self) -> &'static str {
        match self {
            CertificationLevel::None => "⚪ UNCERTIFIED",
            CertificationLevel::Bronze => "🥉 BRONZE",
            CertificationLevel::Silver => "🥈 SILVER",
            CertificationLevel::Gold => "🥇 GOLD",
            CertificationLevel::Platinum => "💎 PLATINUM",
            CertificationLevel::Diamond => "💠 DIAMOND",
        }
    }

    /// Color for UI display (hex)
    pub fn color(&self) -> &'static str {
        match self {
            CertificationLevel::None => "#6b7280",     // gray
            CertificationLevel::Bronze => "#cd7f32",   // bronze
            CertificationLevel::Silver => "#c0c0c0",   // silver
            CertificationLevel::Gold => "#ffd700",     // gold
            CertificationLevel::Platinum => "#e5e4e2", // platinum
            CertificationLevel::Diamond => "#b9f2ff",  // diamond blue
        }
    }

    /// Minimum tier required to achieve this certification
    pub fn min_tier(&self) -> &'static str {
        match self {
            CertificationLevel::None => "free",
            CertificationLevel::Bronze => "free",
            CertificationLevel::Silver => "free",
            CertificationLevel::Gold => "pro",
            CertificationLevel::Platinum => "enterprise",
            CertificationLevel::Diamond => "government",
        }
    }
}

// ============================================================================
// GATE SUMMARY
// ============================================================================

/// Summary of all gate results
///
/// Captures pass/fail status and scores for each QA gate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateSummary {
    /// Dual heartbeat verification passed
    pub heartbeat_passed: bool,
    /// Static QA gate passed
    pub static_passed: bool,
    /// Static QA score (0-100)
    pub static_score: u32,
    /// Semantic QA gate passed
    pub semantic_passed: bool,
    /// Semantic QA score (0-100)
    pub semantic_score: u32,
    /// Architecture gate passed
    pub arch_passed: bool,
    /// Pattern gate passed
    pub pattern_passed: bool,
}

// ============================================================================
// DRIFT METRICS
// ============================================================================

/// Drift detection metrics
///
/// Captures aggregate drift statistics for certification evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftMetrics {
    /// Total drift signals detected
    pub total_signals: u32,
    /// Maximum drift score observed (0.0-1.0)
    pub max_score: f32,
    /// Maximum delta angle observed (degrees)
    pub max_delta_angle: f32,
    /// Drift vectors detected (e.g., "Role", "Coupling")
    pub vectors_detected: Vec<String>,
    /// Number of files with drift
    pub files_affected: u32,
    /// N-V-N-N annotation coverage (0-100%)
    pub annotation_coverage: u8,
}

impl Default for DriftMetrics {
    fn default() -> Self {
        Self {
            total_signals: 0,
            max_score: 0.0,
            max_delta_angle: 0.0,
            vectors_detected: vec![],
            files_affected: 0,
            annotation_coverage: 0,
        }
    }
}

// ============================================================================
// QUALITY CERTIFICATION
// ============================================================================

/// Quality certification emitted after compile
///
/// Represents the complete QA status of a crate at a point in time.
/// Certifications have an expiration and can be signed for tamper detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityCertification {
    /// Unique certification ID (cert-{crate}-{timestamp})
    pub id: String,

    /// Crate/package name
    pub crate_name: String,

    /// Version certified
    pub version: String,

    /// Git commit SHA (if available)
    pub commit_sha: Option<String>,

    /// Certification level achieved
    pub level: CertificationLevel,

    /// Numeric score (0-100)
    pub score: u8,

    /// Gate results summary
    pub gates: GateSummary,

    /// Drift metrics
    pub drift_metrics: DriftMetrics,

    /// Enforcement mode used
    pub enforcement_mode: EnforcementMode,

    /// License tier of certifier
    pub certifier_tier: String,

    /// Certification timestamp
    pub certified_at: DateTime<Utc>,

    /// Expiration (certifications expire after 30 days by default)
    pub expires_at: Option<DateTime<Utc>>,

    /// Digital signature (for tamper detection)
    pub signature: Option<String>,
}

impl QualityCertification {
    /// Create certification from gate results
    ///
    /// Calculates certification level and score based on gate outcomes.
    pub fn from_gates(
        crate_name: &str,
        version: &str,
        gates: GateSummary,
        drift_metrics: DriftMetrics,
        enforcement: EnforcementMode,
        tier: &str,
    ) -> Self {
        let now = Utc::now();

        // Calculate certification level based on gates and enforcement
        let level = Self::calculate_level(&gates, &drift_metrics, enforcement);

        // Calculate numeric score
        let score = Self::calculate_score(&gates, &drift_metrics);

        Self {
            id: format!("cert-{}-{}", crate_name, now.format("%Y%m%d%H%M%S")),
            crate_name: crate_name.to_string(),
            version: version.to_string(),
            commit_sha: None,
            level,
            score,
            gates,
            drift_metrics,
            enforcement_mode: enforcement,
            certifier_tier: tier.to_string(),
            certified_at: now,
            expires_at: Some(now + Duration::days(30)), // 30-day validity
            signature: None,
        }
    }

    /// Calculate certification level from results
    ///
    /// Per RFC-9141: Certification requires passing both heartbeats.
    fn calculate_level(
        gates: &GateSummary,
        drift: &DriftMetrics,
        enforcement: EnforcementMode,
    ) -> CertificationLevel {
        // Must pass heartbeat and static for any certification
        if !gates.heartbeat_passed || !gates.static_passed {
            return CertificationLevel::None;
        }

        // Diamond: Strict mode, zero drift, full annotations
        if enforcement == EnforcementMode::Strict
            && drift.total_signals == 0
            && drift.annotation_coverage >= 90
            && gates.semantic_passed
            && gates.arch_passed
            && gates.pattern_passed
        {
            return CertificationLevel::Diamond;
        }

        // Platinum: Enforce mode, minimal drift
        if enforcement >= EnforcementMode::Enforce
            && drift.max_score < 0.3
            && gates.semantic_passed
            && gates.arch_passed
        {
            return CertificationLevel::Platinum;
        }

        // Gold: Gate mode passed
        if enforcement >= EnforcementMode::Gate
            && gates.semantic_passed
            && drift.max_score < 0.5
        {
            return CertificationLevel::Gold;
        }

        // Silver: Warn mode, semantic passed
        if gates.semantic_passed && drift.max_score < 0.7 {
            return CertificationLevel::Silver;
        }

        // Bronze: Static passed (minimum)
        CertificationLevel::Bronze
    }

    /// Calculate numeric score (0-100)
    ///
    /// Points allocated:
    /// - Heartbeat: 10
    /// - Static: 20
    /// - Semantic: 25
    /// - Arch: 15
    /// - Pattern: 10
    /// - Low drift bonus: up to 10
    /// - Annotation bonus: up to 5
    fn calculate_score(gates: &GateSummary, drift: &DriftMetrics) -> u8 {
        let mut score = 0u8;

        // Base points for passing gates
        if gates.heartbeat_passed { score += 10; }
        if gates.static_passed { score += 20; }
        if gates.semantic_passed { score += 25; }
        if gates.arch_passed { score += 15; }
        if gates.pattern_passed { score += 10; }

        // Bonus for low drift
        if drift.total_signals == 0 {
            score += 10;
        } else if drift.max_score < 0.3 {
            score += 5;
        }

        // Bonus for annotation coverage
        score += (drift.annotation_coverage / 10).min(5);

        score.min(100)
    }

    /// Generate human-readable summary
    pub fn summary(&self) -> String {
        format!(
            "{} | Score: {}/100 | {} | Drift: {:.0}% | Coverage: {}%",
            self.level.badge(),
            self.score,
            self.crate_name,
            self.drift_metrics.max_score * 100.0,
            self.drift_metrics.annotation_coverage
        )
    }

    /// Check if certification is still valid
    pub fn is_valid(&self) -> bool {
        if let Some(expires) = self.expires_at {
            Utc::now() < expires
        } else {
            true
        }
    }

    /// Set commit SHA
    pub fn with_commit(mut self, sha: &str) -> Self {
        self.commit_sha = Some(sha.to_string());
        self
    }

    /// Sign the certification (placeholder for actual crypto)
    pub fn sign(mut self, _private_key: &str) -> Self {
        // TODO: Implement actual signing with ed25519 or similar
        let payload = format!(
            "{}:{}:{}:{:?}:{}",
            self.id, self.crate_name, self.version, self.level, self.score
        );
        self.signature = Some(format!("sha256:{:x}", md5_hash(&payload)));
        self
    }
}

/// Simple hash for certification (placeholder)
fn md5_hash(input: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

// ============================================================================
// NATS SUBJECTS
// ============================================================================

/// NATS subjects for certification events
///
/// Enables distributed observability of certification lifecycle.
pub mod certification_subjects {
    /// Certification issued
    pub const ISSUED: &str = "sx9.qa.certification.issued";
    /// Certification expired
    pub const EXPIRED: &str = "sx9.qa.certification.expired";
    /// Certification revoked
    pub const REVOKED: &str = "sx9.qa.certification.revoked";

    /// Subject for specific crate
    pub fn for_crate(crate_name: &str) -> String {
        format!("sx9.qa.certification.{}", crate_name)
    }

    /// Subject for certification level
    pub fn for_level(level: &str) -> String {
        format!("sx9.qa.certification.level.{}", level.to_lowercase())
    }
}
