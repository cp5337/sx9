//! Orchestration ontology for meta-structure control and oversight

use super::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Orchestration ontology structure for meta-control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationOntology {
    /// Control hierarchy mapping
    pub control_hierarchy: ControlHierarchy,
    /// Dependency relationships between orchestrators
    pub dependency_graph: DependencyGraph,
    /// Oversight mechanisms and monitoring
    pub oversight: OversightMechanisms,
    /// Core CTAS-7 dependency crates
    pub foundation_crates: FoundationCrates,
}

/// Four core dependency crates for the CTAS-7 foundation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundationCrates {
    /// Foundation Core: shared traits, types, orchestration
    pub foundation_core: CrateSpec,
    /// Data Foundation: USIM, SCH vectors, persistence
    pub data_foundation: CrateSpec,
    /// Interface Foundation: API, CDN, networking
    pub interface_foundation: CrateSpec,
    /// Tactical Foundation: threat hunting, OODA, autonomy
    pub tactical_foundation: CrateSpec,
}

/// Crate specification with dependency info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateSpec {
    pub name: String,
    pub version: String,
    pub responsibilities: Vec<String>,
    pub exports: Vec<String>,
    pub dependencies: Vec<String>,
}

/// Control hierarchy for orchestrator relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlHierarchy {
    /// Root orchestrator (typically Shipyard)
    pub root: Uuid,
    /// Parent-child relationships
    pub hierarchy: HashMap<Uuid, Vec<Uuid>>,
    /// Control permissions
    pub permissions: HashMap<Uuid, Vec<ControlPermission>>,
}

/// Dependency graph between orchestrators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    /// Direct dependencies
    pub dependencies: HashMap<Uuid, Vec<Uuid>>,
    /// Dependency types
    pub dependency_types: HashMap<(Uuid, Uuid), DependencyType>,
}

/// Oversight mechanisms for monitoring and control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OversightMechanisms {
    /// Monitoring policies
    pub monitoring: MonitoringPolicy,
    /// Alert conditions
    pub alerting: AlertingPolicy,
    /// Intervention thresholds
    pub intervention: InterventionPolicy,
}

/// Control permissions for orchestrator hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlPermission {
    Start,
    Stop,
    Configure,
    Monitor,
    Override,
}

/// Dependency relationship types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Hard,       // Must be running
    Soft,       // Preferred but not required
    Circular,   // Circular dependency (needs careful handling)
    Conditional, // Depends on configuration
}

/// Monitoring policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringPolicy {
    pub health_check_interval_secs: u64,
    pub metrics_collection_interval_secs: u64,
    pub status_reporting_interval_secs: u64,
}

/// Alerting policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingPolicy {
    pub error_threshold: u32,
    pub cpu_threshold_percent: f32,
    pub memory_threshold_mb: u32,
    pub response_time_threshold_ms: u64,
}

/// Intervention policy for automatic responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionPolicy {
    pub auto_restart_on_failure: bool,
    pub cascade_stop_on_critical_failure: bool,
    pub max_restart_attempts: u32,
}

impl Default for FoundationCrates {
    fn default() -> Self {
        Self {
            foundation_core: CrateSpec {
                name: "ctas7-foundation-core".to_string(),
                version: "0.1.0".to_string(),
                responsibilities: vec![
                    "Orchestration traits and types".to_string(),
                    "Status reporting and TOML serialization".to_string(),
                    "Dependency management".to_string(),
                    "Ontology definitions".to_string(),
                ],
                exports: vec![
                    "Orchestrator trait".to_string(),
                    "OrchestrationStatus".to_string(),
                    "TomlStatusReporter".to_string(),
                    "StandardDependencies".to_string(),
                ],
                dependencies: vec!["serde".to_string(), "uuid".to_string(), "chrono".to_string()],
            },
            data_foundation: CrateSpec {
                name: "ctas7-data-foundation".to_string(),
                version: "0.1.0".to_string(),
                responsibilities: vec![
                    "USIM trivariate hashing".to_string(),
                    "SCH vector generation".to_string(),
                    "Data persistence and retrieval".to_string(),
                    "Trivariate cryptographic operations".to_string(),
                ],
                exports: vec![
                    "USIMTrivariate".to_string(),
                    "SCHVector".to_string(),
                    "USIMProcessor".to_string(),
                ],
                dependencies: vec!["sx9-foundation-core".to_string(), "serde".to_string()],
            },
            interface_foundation: CrateSpec {
                name: "ctas7-interface-foundation".to_string(),
                version: "0.1.0".to_string(),
                responsibilities: vec![
                    "API definitions and routing".to_string(),
                    "CDN gateway functionality".to_string(),
                    "Network communication protocols".to_string(),
                    "Port management".to_string(),
                ],
                exports: vec![
                    "ApiRouter".to_string(),
                    "CdnGateway".to_string(),
                    "PortManager".to_string(),
                ],
                dependencies: vec!["tokio".to_string(), "reqwest".to_string(), "axum".to_string()],
            },
            tactical_foundation: CrateSpec {
                name: "ctas7-tactical-foundation".to_string(),
                version: "0.1.0".to_string(),
                responsibilities: vec![
                    "Threat hunting algorithms".to_string(),
                    "OODA loop implementation".to_string(),
                    "Neural Mux decision making".to_string(),
                    "Autonomous response systems".to_string(),
                ],
                exports: vec![
                    "ThreatHunter".to_string(),
                    "NeuralMux".to_string(),
                    "OodaProcessor".to_string(),
                ],
                dependencies: vec!["ctas7-data-foundation".to_string()],
            },
        }
    }
}