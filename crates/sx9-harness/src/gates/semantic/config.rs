//! Semantic QA Configuration
//!
//! Defines enforcement modes, thresholds, and tier-based configuration
//! for the semantic drift detection system.
//!
//! Per RFC-9142 Section 7 (Governance Gates) and RFC-9050 (QA Two-Heartbeat).
//!
//! # Enforcement Modes
//!
//! | Mode     | Behavior                              |
//! |----------|---------------------------------------|
//! | Advisory | Log only, never block                 |
//! | Warn     | Annotate PR/Issue                     |
//! | Gate     | Require acknowledgment before merge   |
//! | Enforce  | Block until drift resolved            |
//! | Strict   | Full enforcement with escalation      |

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::agents::DriftVector;

// ============================================================================
// ENFORCEMENT MODE
// ============================================================================

/// Semantic QA enforcement mode
///
/// Per RFC-9142 Section 7.1: Gate levels determine response to drift.
/// Modes are ordered from least to most restrictive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnforcementMode {
    /// Log only, never block (development)
    Advisory = 0,
    /// Warn in PR comments, don't block merge
    Warn = 1,
    /// Require acknowledgment before merge
    Gate = 2,
    /// Block merge until drift resolved
    Enforce = 3,
    /// Full enforcement with escalation to security team
    Strict = 4,
}

impl Default for EnforcementMode {
    fn default() -> Self {
        EnforcementMode::Advisory
    }
}

// ============================================================================
// DRIFT THRESHOLDS
// ============================================================================

/// Per-vector drift thresholds
///
/// Per RFC-9142 Section 5 (Drift Scoring):
/// - 0.0 = No drift
/// - 0.3 = Minor deviation
/// - 0.6 = Significant misalignment
/// - 0.8+ = Governance concern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftThresholds {
    /// Score threshold to trigger warning (0.0-1.0)
    pub warn_threshold: f32,
    /// Score threshold to trigger gate/block (0.0-1.0)
    pub gate_threshold: f32,
    /// Delta angle threshold for warning (degrees, 0-180)
    /// Per RFC-9142 Section 6: 15-45° = Monitor
    pub delta_angle_warn: f32,
    /// Delta angle threshold for blocking (degrees, 0-180)
    /// Per RFC-9142 Section 6: 45-90° = Investigate, 90°+ = Escalate
    pub delta_angle_gate: f32,
}

impl Default for DriftThresholds {
    fn default() -> Self {
        Self {
            warn_threshold: 0.5,
            gate_threshold: 0.8,
            delta_angle_warn: 30.0,
            delta_angle_gate: 60.0,
        }
    }
}

// ============================================================================
// TIER ENFORCEMENT
// ============================================================================

/// License tier enforcement mapping
///
/// Each tier has a maximum enforcement level. Attempting to configure
/// a stricter mode than allowed is capped to the tier maximum.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierEnforcement {
    /// Free tier max enforcement
    pub free: EnforcementMode,
    /// Pro tier max enforcement
    pub pro: EnforcementMode,
    /// Enterprise tier max enforcement
    pub enterprise: EnforcementMode,
    /// Government tier max enforcement
    pub government: EnforcementMode,
}

impl Default for TierEnforcement {
    fn default() -> Self {
        Self {
            free: EnforcementMode::Advisory,      // Free: advisory only
            pro: EnforcementMode::Warn,           // Pro: warnings
            enterprise: EnforcementMode::Gate,    // Enterprise: gating
            government: EnforcementMode::Strict,  // Government: full enforcement
        }
    }
}

// ============================================================================
// SEMANTIC CONFIGURATION
// ============================================================================

/// Semantic QA configuration
///
/// Central configuration for the semantic drift detection system.
/// Supports per-vector overrides, exempt paths, and tier-based enforcement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticConfig {
    /// Global enforcement mode
    pub enforcement_mode: EnforcementMode,

    /// Per-vector enforcement overrides
    /// Key: lowercase vector name (e.g., "role", "coupling")
    pub vector_modes: HashMap<String, EnforcementMode>,

    /// Per-vector thresholds
    /// Key: lowercase vector name
    pub thresholds: HashMap<String, DriftThresholds>,

    /// Default thresholds for vectors without specific config
    pub default_thresholds: DriftThresholds,

    /// Vectors to completely ignore
    pub ignored_vectors: Vec<DriftVector>,

    /// Crates/paths exempt from enforcement
    pub exempt_paths: Vec<String>,

    /// Require N-V-N-N annotations in new code
    pub require_annotations: bool,

    /// Auto-escalate to governance gate on pattern violation
    /// Per RFC-9142: Pattern drift is behavioral, not aesthetic
    pub auto_escalate_pattern: bool,

    /// License tier determines max enforcement level
    pub tier_enforcement: TierEnforcement,
}

impl Default for SemanticConfig {
    fn default() -> Self {
        Self {
            enforcement_mode: EnforcementMode::Advisory,
            vector_modes: HashMap::new(),
            thresholds: HashMap::new(),
            default_thresholds: DriftThresholds::default(),
            ignored_vectors: vec![],
            exempt_paths: vec![
                "tests/".to_string(),
                "examples/".to_string(),
                "benches/".to_string(),
            ],
            require_annotations: false,
            auto_escalate_pattern: false,
            tier_enforcement: TierEnforcement::default(),
        }
    }
}

// ============================================================================
// BUILDER PATTERN
// ============================================================================

/// Builder for SemanticConfig
///
/// Provides fluent API for constructing semantic configurations.
///
/// # Example
///
/// ```ignore
/// let config = SemanticConfigBuilder::new()
///     .enforcement(EnforcementMode::Gate)
///     .require_annotations(true)
///     .auto_escalate_pattern(true)
///     .build();
/// ```
pub struct SemanticConfigBuilder {
    config: SemanticConfig,
}

impl SemanticConfigBuilder {
    /// Create new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: SemanticConfig::default(),
        }
    }

    /// Set global enforcement mode
    pub fn enforcement(mut self, mode: EnforcementMode) -> Self {
        self.config.enforcement_mode = mode;
        self
    }

    /// Set enforcement for specific vector
    pub fn vector_enforcement(mut self, vector: DriftVector, mode: EnforcementMode) -> Self {
        let key = format!("{:?}", vector).to_lowercase();
        self.config.vector_modes.insert(key, mode);
        self
    }

    /// Set thresholds for specific vector
    pub fn vector_thresholds(mut self, vector: DriftVector, thresholds: DriftThresholds) -> Self {
        let key = format!("{:?}", vector).to_lowercase();
        self.config.thresholds.insert(key, thresholds);
        self
    }

    /// Set default thresholds
    pub fn default_thresholds(mut self, thresholds: DriftThresholds) -> Self {
        self.config.default_thresholds = thresholds;
        self
    }

    /// Ignore specific drift vectors
    pub fn ignore_vectors(mut self, vectors: Vec<DriftVector>) -> Self {
        self.config.ignored_vectors = vectors;
        self
    }

    /// Add exempt paths
    pub fn exempt_paths(mut self, paths: Vec<String>) -> Self {
        self.config.exempt_paths.extend(paths);
        self
    }

    /// Require N-V-N-N annotations
    pub fn require_annotations(mut self, require: bool) -> Self {
        self.config.require_annotations = require;
        self
    }

    /// Auto-escalate on pattern violations
    pub fn auto_escalate_pattern(mut self, escalate: bool) -> Self {
        self.config.auto_escalate_pattern = escalate;
        self
    }

    /// Set tier enforcement levels
    pub fn tier_enforcement(mut self, tier: TierEnforcement) -> Self {
        self.config.tier_enforcement = tier;
        self
    }

    /// Build the configuration
    pub fn build(self) -> SemanticConfig {
        self.config
    }
}

impl Default for SemanticConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// PRESET CONFIGURATIONS
// ============================================================================

impl SemanticConfig {
    /// Development preset - advisory only
    ///
    /// Use during active development when drift detection
    /// should inform but never block.
    pub fn development() -> Self {
        SemanticConfigBuilder::new()
            .enforcement(EnforcementMode::Advisory)
            .build()
    }

    /// CI/CD preset - warnings for drift
    ///
    /// Use in continuous integration pipelines.
    /// Drift is reported but does not block builds.
    pub fn ci_cd() -> Self {
        SemanticConfigBuilder::new()
            .enforcement(EnforcementMode::Warn)
            .require_annotations(false)
            .build()
    }

    /// Production preset - gate on high drift
    ///
    /// Use for production releases.
    /// High drift requires explicit acknowledgment.
    pub fn production() -> Self {
        SemanticConfigBuilder::new()
            .enforcement(EnforcementMode::Gate)
            .default_thresholds(DriftThresholds {
                warn_threshold: 0.4,
                gate_threshold: 0.7,
                delta_angle_warn: 25.0,
                delta_angle_gate: 50.0,
            })
            .auto_escalate_pattern(true)
            .build()
    }

    /// Government/compliance preset - strict enforcement
    ///
    /// Use for regulated environments requiring full traceability.
    /// Any drift blocks release pending review.
    pub fn government() -> Self {
        SemanticConfigBuilder::new()
            .enforcement(EnforcementMode::Strict)
            .require_annotations(true)
            .auto_escalate_pattern(true)
            .default_thresholds(DriftThresholds {
                warn_threshold: 0.3,
                gate_threshold: 0.5,
                delta_angle_warn: 15.0,
                delta_angle_gate: 30.0,
            })
            .tier_enforcement(TierEnforcement {
                free: EnforcementMode::Advisory,
                pro: EnforcementMode::Gate,
                enterprise: EnforcementMode::Enforce,
                government: EnforcementMode::Strict,
            })
            .build()
    }
}
