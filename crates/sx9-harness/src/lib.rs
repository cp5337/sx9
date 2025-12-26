//! SX9 Harness - Unified Forge Execution \u0026 QA System
//!
//! This crate provides:
//! - Forge execution engine (from TypeScript harness)
//! - QA gates (static, architecture, pattern)
//! - State management and validation
//! - NATS integration for distributed execution
//! - Linear Gateway Service (RFC-9030)

pub mod types;
pub mod executor;
pub mod actions;
pub mod middleware;
pub mod validators;
pub mod reducer;
pub mod selectors;
pub mod gates;
pub mod nats;
pub mod agents;
pub mod tasks;
pub mod linear;
pub mod lineage;
pub mod security;

// Re-exports
pub use types::*;
pub use executor::Executor;
pub use gates::{
    HeartbeatGate, StaticGate, SemanticGate, ArchGate, PatternGate,
    SemanticConfig, SemanticConfigBuilder, EnforcementMode, DriftThresholds, TierEnforcement,
    // Quality Certification
    QualityCertification, CertificationLevel, GateSummary, DriftMetrics,
    // Cognitix Metrics
    CognitixLifetimeMetrics, CognitixBadge, CertificationCounts,
    // Degradation Tracking
    CertificationDelta, DeltaDirection, DegradationCause, UpdateTrigger,
    CertificationTrend, TrendHealth, TrendDirection,
};
pub use agents::{
    AgentRegistry, Agent, AiProvider,
    // Skill system
    Skill, SkillRegistry, SkillCategory, SkillSlo, SkillInput, SkillOutput,
    SkillContext, SkillArtifact, SkillExecutor, SkillExecutionError,
};
pub use tasks::{TaskId, Task, TaskGraph, TASKS};
pub use linear::{LinearGateway, LinearIssue, LinearWebhook};
pub use lineage::{LineageTracker, LineageMarker, LineageAnalysis};
pub use security::{
    SbomGenerator, SbomFormat, SoftwareBom,
    SecretsScan, SecretFinding, SecretType,
    DependencyAudit, AuditFinding,
    ContainerScan, ContainerFinding,
    ComplianceChecker, ComplianceReport, CatoStatus,
    Severity, SecurityPipelineResult, run_security_pipeline,
};
