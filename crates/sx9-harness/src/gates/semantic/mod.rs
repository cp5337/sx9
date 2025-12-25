//! Semantic QA Gate - Configurable Drift Detection
//!
//! Analyzes behavioral drift from declared N-V-N-N patterns.
//! Provides optionality for enforcement levels based on:
//! - Project configuration
//! - License tier
//! - Governance policy
//!
//! Emits a QualityCertification after compile for end-to-end observability.
//!
//! Per CLSGS Annex A.3.2, RFC-9141, and RFC-9142.
//!
//! # Module Structure
//!
//! | Module        | Contents                                    | LOC  |
//! |---------------|---------------------------------------------|------|
//! | config        | EnforcementMode, SemanticConfig, thresholds | ~280 |
//! | certification | QualityCertification, levels, gate summary  | ~290 |
//! | cognitix      | Lifetime metrics, badges, marketing         | ~170 |
//! | degradation   | CertificationDelta, trend tracking          | ~280 |
//! | mod (this)    | SemanticGate, SemanticReport                | ~290 |
//!
//! Total: ~1300 lines split across 5 files (~260 avg per file)

pub mod config;
pub mod certification;
pub mod cognitix;
pub mod degradation;

// Re-exports for convenience
pub use config::{
    EnforcementMode, DriftThresholds, SemanticConfig, SemanticConfigBuilder, TierEnforcement,
};
pub use certification::{
    CertificationLevel, QualityCertification, GateSummary, DriftMetrics,
    certification_subjects,
};
pub use cognitix::{
    CognitixLifetimeMetrics, CognitixBadge, CertificationCounts,
};
pub use degradation::{
    CertificationDelta, DeltaDirection, DegradationCause, UpdateTrigger,
    CertificationTrend, TrendHealth, TrendDirection,
};

use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::agents::{BehavioralScope, DriftSignal, DriftVector, GovernanceGate};
use sx9_foundation_core::data::{DateTime, Utc};

// ============================================================================
// SEMANTIC REPORT
// ============================================================================

/// Semantic QA report
///
/// Contains results of semantic drift analysis including signals,
/// gate level, and pass/fail determination based on enforcement mode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticReport {
    /// Schema version
    pub schema_version: String,
    /// Unique loadset identifier
    pub loadset_id: String,

    /// Overall pass/fail based on enforcement mode
    pub passed: bool,

    /// Enforcement mode used for this run
    pub enforcement_mode: EnforcementMode,

    /// Detected drift signals
    pub drift_signals: Vec<DriftSignal>,

    /// Governance gate level determined
    pub gate_level: GovernanceGate,

    /// Files analyzed
    pub files_analyzed: u32,

    /// Annotations found
    pub annotations_found: u32,

    /// Missing annotations (if require_annotations = true)
    pub missing_annotations: Vec<MissingAnnotation>,

    /// Summary message
    pub summary: String,

    /// Timestamp
    pub analyzed_at: DateTime<Utc>,
}

/// Missing N-V-N-N annotation location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingAnnotation {
    /// File path
    pub file: String,
    /// Line number
    pub line: u32,
    /// Function name (if applicable)
    pub function_name: Option<String>,
    /// Suggested behavioral scope
    pub suggested_scope: Option<BehavioralScope>,
}

// ============================================================================
// SEMANTIC GATE
// ============================================================================

/// Semantic QA Gate with configurable drift detection
///
/// Per RFC-9142: Semantic drift is treated as a first-class operational signal.
/// The gate supports multiple enforcement modes from advisory to strict.
pub struct SemanticGate {
    config: SemanticConfig,
    license_tier: Option<String>,
}

impl SemanticGate {
    /// Create with default configuration (advisory mode)
    pub fn new() -> Self {
        Self {
            config: SemanticConfig::default(),
            license_tier: None,
        }
    }

    /// Create with specific configuration
    pub fn with_config(config: SemanticConfig) -> Self {
        Self {
            config,
            license_tier: None,
        }
    }

    /// Set license tier for enforcement level capping
    pub fn with_tier(mut self, tier: &str) -> Self {
        self.license_tier = Some(tier.to_lowercase());
        self
    }

    /// Load configuration from file
    pub fn from_config_file(path: &Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config: {}", e))?;

        let config: SemanticConfig = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))?;

        Ok(Self::with_config(config))
    }

    /// Get effective enforcement mode (capped by license tier)
    pub fn effective_enforcement(&self) -> EnforcementMode {
        let base_mode = self.config.enforcement_mode;

        // Cap enforcement based on license tier
        if let Some(ref tier) = self.license_tier {
            let tier_max = match tier.as_str() {
                "free" => self.config.tier_enforcement.free,
                "pro" => self.config.tier_enforcement.pro,
                "enterprise" => self.config.tier_enforcement.enterprise,
                "government" => self.config.tier_enforcement.government,
                _ => EnforcementMode::Advisory,
            };

            // Return the lesser of base_mode and tier_max
            return self.min_enforcement(base_mode, tier_max);
        }

        base_mode
    }

    /// Get enforcement mode for specific drift vector
    pub fn vector_enforcement(&self, vector: DriftVector) -> EnforcementMode {
        let vector_key = format!("{:?}", vector).to_lowercase();

        // Check if vector is ignored
        if self.config.ignored_vectors.contains(&vector) {
            return EnforcementMode::Advisory;
        }

        // Check for vector-specific override
        if let Some(mode) = self.config.vector_modes.get(&vector_key) {
            return *mode;
        }

        self.effective_enforcement()
    }

    /// Get thresholds for specific drift vector
    pub fn vector_thresholds(&self, vector: DriftVector) -> &DriftThresholds {
        let vector_key = format!("{:?}", vector).to_lowercase();

        self.config.thresholds
            .get(&vector_key)
            .unwrap_or(&self.config.default_thresholds)
    }

    /// Check if path is exempt from enforcement
    pub fn is_exempt(&self, path: &str) -> bool {
        self.config.exempt_paths.iter().any(|p| path.contains(p))
    }

    /// Run semantic analysis on a crate
    pub async fn run(&self, crate_path: &Path) -> Result<SemanticReport, String> {
        let now = Utc::now();
        let enforcement = self.effective_enforcement();

        // Scan for N-V-N-N annotations
        let (annotations_found, missing) = self.scan_annotations(crate_path).await?;

        // Analyze behavioral drift
        let drift_signals = self.analyze_drift(crate_path).await?;

        // Determine gate level based on drift signals and enforcement
        let gate_level = self.determine_gate_level(&drift_signals);

        // Determine pass/fail based on enforcement mode
        let passed = self.evaluate_pass(&drift_signals, &missing, enforcement);

        let summary = self.generate_summary(&drift_signals, &missing, enforcement, passed);

        Ok(SemanticReport {
            schema_version: "1.0".to_string(),
            loadset_id: format!("semantic-{}", now.format("%Y%m%d-%H%M%S")),
            passed,
            enforcement_mode: enforcement,
            drift_signals,
            gate_level,
            files_analyzed: 0, // TODO: count actual files
            annotations_found,
            missing_annotations: missing,
            summary,
            analyzed_at: now,
        })
    }

    /// Scan for N-V-N-N annotations in source files
    async fn scan_annotations(&self, _crate_path: &Path) -> Result<(u32, Vec<MissingAnnotation>), String> {
        // TODO: Implement actual annotation scanning
        // - Parse Rust files for // ROLE_ACTION_CONSTRAINT_OBJECT patterns
        // - Track functions/structs without annotations
        // - Suggest scopes based on function names
        Ok((0, vec![]))
    }

    /// Analyze behavioral drift from declared patterns
    async fn analyze_drift(&self, _crate_path: &Path) -> Result<Vec<DriftSignal>, String> {
        // TODO: Implement actual drift analysis
        // - Compare current behavior to declared N-V-N-N scope
        // - Use AI/heuristics to detect drift vectors:
        //   - Role: undeclared responsibilities
        //   - Constraint: boundary erosion
        //   - Coupling: unexpected dependencies
        //   - Authority: decision-making migration
        //   - Pattern: structural vs behavioral mismatch
        Ok(vec![])
    }

    /// Determine governance gate level based on drift signals
    ///
    /// Per RFC-9142 Section 7: Gate levels determine response.
    fn determine_gate_level(&self, drift_signals: &[DriftSignal]) -> GovernanceGate {
        if drift_signals.is_empty() {
            return GovernanceGate::Observe;
        }

        let max_score = drift_signals
            .iter()
            .map(|d| d.score)
            .fold(0.0_f32, f32::max);

        let max_delta = drift_signals
            .iter()
            .map(|d| d.delta_angle)
            .fold(0.0_f32, f32::max);

        // Check against default thresholds
        let thresholds = &self.config.default_thresholds;

        if max_score >= thresholds.gate_threshold || max_delta >= thresholds.delta_angle_gate {
            // Check if any signal is Pattern type and auto-escalate is on
            let has_pattern_drift = drift_signals.iter().any(|d| d.vector == DriftVector::Pattern);
            if has_pattern_drift && self.config.auto_escalate_pattern {
                return GovernanceGate::Escalate;
            }
            return GovernanceGate::Gate;
        }

        if max_score >= thresholds.warn_threshold || max_delta >= thresholds.delta_angle_warn {
            return GovernanceGate::Warn;
        }

        GovernanceGate::Observe
    }

    /// Evaluate pass/fail based on enforcement mode
    fn evaluate_pass(
        &self,
        drift_signals: &[DriftSignal],
        missing: &[MissingAnnotation],
        enforcement: EnforcementMode,
    ) -> bool {
        match enforcement {
            EnforcementMode::Advisory => true, // Always pass in advisory
            EnforcementMode::Warn => true,     // Warnings don't block
            EnforcementMode::Gate => {
                // Fail if any signal exceeds gate threshold
                let thresholds = &self.config.default_thresholds;
                !drift_signals.iter().any(|d| d.score >= thresholds.gate_threshold)
            }
            EnforcementMode::Enforce => {
                // Fail if any signal exceeds warn threshold
                let thresholds = &self.config.default_thresholds;
                !drift_signals.iter().any(|d| d.score >= thresholds.warn_threshold)
                    && (missing.is_empty() || !self.config.require_annotations)
            }
            EnforcementMode::Strict => {
                // Fail if any drift or missing annotations
                drift_signals.is_empty()
                    && (missing.is_empty() || !self.config.require_annotations)
            }
        }
    }

    /// Generate human-readable summary
    fn generate_summary(
        &self,
        drift_signals: &[DriftSignal],
        missing: &[MissingAnnotation],
        enforcement: EnforcementMode,
        passed: bool,
    ) -> String {
        let drift_count = drift_signals.len();
        let missing_count = missing.len();

        let status = if passed { "PASSED" } else { "FAILED" };
        let mode = format!("{:?}", enforcement).to_lowercase();

        if drift_count == 0 && missing_count == 0 {
            return format!("[{}] Semantic QA {} - No drift detected", mode, status);
        }

        let mut parts = vec![];

        if drift_count > 0 {
            let max_score = drift_signals.iter().map(|d| d.score).fold(0.0_f32, f32::max);
            parts.push(format!("{} drift signals (max score: {:.2})", drift_count, max_score));
        }

        if missing_count > 0 && self.config.require_annotations {
            parts.push(format!("{} missing annotations", missing_count));
        }

        format!("[{}] Semantic QA {} - {}", mode, status, parts.join(", "))
    }

    /// Return the lesser enforcement mode
    fn min_enforcement(&self, a: EnforcementMode, b: EnforcementMode) -> EnforcementMode {
        if a <= b { a } else { b }
    }
}

impl Default for SemanticGate {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_advisory() {
        let gate = SemanticGate::new();
        assert_eq!(gate.effective_enforcement(), EnforcementMode::Advisory);
    }

    #[test]
    fn test_tier_caps_enforcement() {
        let config = SemanticConfigBuilder::new()
            .enforcement(EnforcementMode::Strict)
            .build();

        let gate = SemanticGate::with_config(config).with_tier("free");

        // Free tier should cap at Advisory
        assert_eq!(gate.effective_enforcement(), EnforcementMode::Advisory);
    }

    #[test]
    fn test_vector_specific_enforcement() {
        let config = SemanticConfigBuilder::new()
            .enforcement(EnforcementMode::Warn)
            .vector_enforcement(DriftVector::Pattern, EnforcementMode::Enforce)
            .build();

        let gate = SemanticGate::with_config(config);

        assert_eq!(gate.vector_enforcement(DriftVector::Role), EnforcementMode::Warn);
        assert_eq!(gate.vector_enforcement(DriftVector::Pattern), EnforcementMode::Enforce);
    }

    #[test]
    fn test_ignored_vectors() {
        let config = SemanticConfigBuilder::new()
            .enforcement(EnforcementMode::Enforce)
            .ignore_vectors(vec![DriftVector::Coupling])
            .build();

        let gate = SemanticGate::with_config(config);

        // Ignored vector should always be advisory
        assert_eq!(gate.vector_enforcement(DriftVector::Coupling), EnforcementMode::Advisory);
        assert_eq!(gate.vector_enforcement(DriftVector::Role), EnforcementMode::Enforce);
    }

    #[test]
    fn test_exempt_paths() {
        let gate = SemanticGate::new();

        assert!(gate.is_exempt("tests/unit_test.rs"));
        assert!(gate.is_exempt("examples/demo.rs"));
        assert!(!gate.is_exempt("src/lib.rs"));
    }

    #[test]
    fn test_preset_configs() {
        let dev = SemanticConfig::development();
        assert_eq!(dev.enforcement_mode, EnforcementMode::Advisory);

        let prod = SemanticConfig::production();
        assert_eq!(prod.enforcement_mode, EnforcementMode::Gate);
        assert!(prod.auto_escalate_pattern);

        let gov = SemanticConfig::government();
        assert_eq!(gov.enforcement_mode, EnforcementMode::Strict);
        assert!(gov.require_annotations);
    }

    #[test]
    fn test_gate_level_determination() {
        let gate = SemanticGate::new();

        // No signals = Observe
        assert_eq!(gate.determine_gate_level(&[]), GovernanceGate::Observe);

        // Low score signal = Observe
        let low_signal = DriftSignal {
            vector: DriftVector::Role,
            score: 0.3,
            delta_angle: 10.0,
            explanation: "Minor drift".to_string(),
            detected_at: Utc::now(),
        };
        assert_eq!(gate.determine_gate_level(&[low_signal.clone()]), GovernanceGate::Observe);

        // Medium score = Warn
        let med_signal = DriftSignal {
            score: 0.6,
            ..low_signal.clone()
        };
        assert_eq!(gate.determine_gate_level(&[med_signal]), GovernanceGate::Warn);

        // High score = Gate
        let high_signal = DriftSignal {
            score: 0.9,
            ..low_signal
        };
        assert_eq!(gate.determine_gate_level(&[high_signal]), GovernanceGate::Gate);
    }
}
