//! QA Gates module
//!
//! Implements the 6-stage QA pipeline:
//! 0. Heartbeat Gate - Dual heartbeat zero-trust verification (MUST pass first)
//! 1. Static Gate - Code structure and complexity (BLOCKING)
//! 2. Semantic Gate - Behavioral drift detection (CONFIGURABLE)
//! 3. Architecture Gate - ECS compliance and patterns
//! 4. Pattern Gate - Canonical pattern matching
//!
//! Per RFC-9050 (Two-Heartbeat) and RFC-9141 (Assembly Line QA):
//! - Static QA is Cold Truth (deterministic, read-only, non-negotiable)
//! - Semantic QA is Warm Annotation (interpretive, context-aware, advisory)
//!
//! The Semantic Gate supports multiple enforcement modes:
//! - Advisory: Log only (development)
//! - Warn: Annotate PR/Issue
//! - Gate: Require acknowledgment
//! - Enforce: Block until resolved
//! - Strict: Full enforcement with escalation

pub mod heartbeat_gate;
pub mod static_gate;
pub mod semantic;
pub mod arch_gate;
pub mod pattern_gate;

pub use heartbeat_gate::HeartbeatGate;
pub use static_gate::StaticGate;
pub use semantic::{
    // Core gate
    SemanticGate, SemanticReport, MissingAnnotation,
    // Configuration (RFC-9142 Section 7)
    SemanticConfig, SemanticConfigBuilder, EnforcementMode, DriftThresholds, TierEnforcement,
    // Quality Certification (end-to-end observability)
    QualityCertification, CertificationLevel, GateSummary, DriftMetrics,
    certification_subjects,
    // Cognitix Lifetime Metrics (sales/marketing)
    CognitixLifetimeMetrics, CognitixBadge, CertificationCounts,
    // Certification Degradation Tracking (update accountability)
    CertificationDelta, DeltaDirection, DegradationCause, UpdateTrigger,
    CertificationTrend, TrendHealth, TrendDirection,
};
pub use arch_gate::ArchGate;
pub use pattern_gate::PatternGate;
