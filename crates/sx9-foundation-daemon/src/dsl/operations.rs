//! DSL Operation Types
//!
//! Defines the core operation types that can be expressed in the DSL.
//! Each operation type maps to specific Neural Mux routing and execution semantics.

use crate::dsl::hash_extractor::{HashComponents, SemanticHash};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// DSL Operation Type Enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DSLOperation {
    /// Hash-driven operation trigger
    HashTrigger(HashTriggerOp),
    /// Intelligence collection operation
    IntelCollection(IntelCollectionOp),
    /// Penetration testing cluster spawn
    PentestSpawn(PentestSpawnOp),
    /// Ephemeral asset lifecycle management
    EphemeralAsset(EphemeralAssetOp),
    /// Node interview execution
    NodeInterview(NodeInterviewOp),
    /// Kali tool orchestration
    KaliTool(KaliToolOp),
    /// Workflow composition
    Workflow(WorkflowOp),
    /// Parallel execution
    Parallel(ParallelOp),
    /// Conditional execution
    Conditional(ConditionalOp),
}

/// Hash Trigger Operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HashTriggerOp {
    pub sch: String,
    pub unicode: char,
    pub operation: String,
    pub metadata: HashMap<String, String>,
}

/// Intelligence Collection Operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IntelCollectionOp {
    pub hash: Option<String>,
    pub semantic_hash: Option<String>,
    pub unicode: char,
    pub tool: String,
    pub gpu_tier: String,
    pub isolation: String,
    pub max_cost: f64,
    pub timeout: String,
    pub depends_on: Vec<String>,
}

/// Penetration Testing Spawn Operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PentestSpawnOp {
    pub target_hash: Option<String>,
    pub cuid: Option<String>,
    pub unicode: char,
    pub tools: Vec<String>,
    pub spawn_type: String,
    pub bridge_type: String,
    pub geo_constraint: Option<String>,
}

/// Ephemeral Asset Management Operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EphemeralAssetOp {
    pub uuid: Option<String>,
    pub unicode: char,
    pub trigger: String,
    pub cleanup: String,
    pub preserve: Vec<String>,
    pub forward: String,
    pub persistence: Option<String>,
    pub world_id: Option<u32>,
}

/// Node Interview Operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeInterviewOp {
    pub operational_hash: String,
    pub semantic_hash: Option<SemanticHashData>,
    pub task_id: String,
    pub task_name: Option<String>,
    pub hd4_phase: String,
    pub mitre_technique: Option<String>,
    pub escalation_tier: Option<u32>,
    pub databases: Vec<String>,
    pub glaf_sync: bool,
    pub kali_tools: Vec<KaliToolRef>,
    pub wasm_sensor: Option<WASMSensorConfig>,
    pub escalation: Option<EscalationPath>,
}

/// Semantic hash data for serialization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SemanticHashData {
    pub block_id: String,
    pub semantic_hash: String,
    pub auth_sig: String,
}

impl From<SemanticHash> for SemanticHashData {
    fn from(h3: SemanticHash) -> Self {
        Self {
            block_id: h3.block_id,
            semantic_hash: h3.semantic_hash,
            auth_sig: h3.auth_sig,
        }
    }
}

impl From<SemanticHashData> for SemanticHash {
    fn from(data: SemanticHashData) -> Self {
        Self::new(data.block_id, data.semantic_hash, data.auth_sig)
    }
}

/// Kali Tool Reference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KaliToolRef {
    pub name: String,
    pub unicode: char,
}

/// WASM Sensor Configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WASMSensorConfig {
    pub sensor_id: String,
    pub unicode: char,
    pub telemetry_interval_ms: u64,
}

/// Escalation Path Configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EscalationPath {
    pub tier_1_shell: String,
    pub tier_2_wasm: String,
    pub tier_3_rust: String,
    pub tier_4_docker: String,
}

/// Kali Tool Orchestration Operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KaliToolOp {
    pub tool: String,
    pub unicode: char,
    pub target: String,
    pub container: String,
    pub l2_orchestration: bool,
    pub memory_mesh: bool,
}

/// Workflow Composition Operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowOp {
    pub name: String,
    pub operational_hash: Option<String>,
    pub steps: Vec<DSLOperation>,
    pub parallel_groups: Vec<Vec<String>>,
}

/// Parallel Execution Operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParallelOp {
    pub tasks: Vec<DSLOperation>,
    pub max_concurrency: usize,
}

/// Conditional Execution Operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConditionalOp {
    pub if_hash: String,
    pub then: Box<DSLOperation>,
    pub otherwise: Option<Box<DSLOperation>>,
}

/// Execution context for DSL operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSLExecutionContext {
    pub operation_id: String,
    pub hash_components: Option<HashComponents>,
    pub semantic_hash: Option<SemanticHash>,
    pub priority: Priority,
    pub timestamp: String,
    pub environmental_mask: HashMap<String, String>,
}

/// Priority levels for Neural Mux routing
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Critical = 4,
    High = 3,
    Medium = 2,
    Low = 1,
}

impl DSLOperation {
    /// Get the operation's priority for Neural Mux routing
    pub fn priority(&self) -> Priority {
        match self {
            DSLOperation::HashTrigger(_) => Priority::High,
            DSLOperation::IntelCollection(_) => Priority::Critical,
            DSLOperation::PentestSpawn(_) => Priority::High,
            DSLOperation::EphemeralAsset(_) => Priority::Medium,
            DSLOperation::NodeInterview(_) => Priority::Critical,
            DSLOperation::KaliTool(_) => Priority::High,
            DSLOperation::Workflow(_) => Priority::Medium,
            DSLOperation::Parallel(_) => Priority::High,
            DSLOperation::Conditional(_) => Priority::Medium,
        }
    }

    /// Get Unicode trigger for this operation
    pub fn unicode_trigger(&self) -> char {
        match self {
            DSLOperation::HashTrigger(op) => op.unicode,
            DSLOperation::IntelCollection(op) => op.unicode,
            DSLOperation::PentestSpawn(op) => op.unicode,
            DSLOperation::EphemeralAsset(op) => op.unicode,
            DSLOperation::NodeInterview(_) => '\u{E320}', // Semantic processor
            DSLOperation::KaliTool(op) => op.unicode,
            DSLOperation::Workflow(_) => '\u{E000}', // System controller
            DSLOperation::Parallel(_) => '\u{E100}', // Trivariate processor
            DSLOperation::Conditional(_) => '\u{E200}', // Context processor
        }
    }

    /// Get operation name for logging/debugging
    pub fn operation_name(&self) -> &str {
        match self {
            DSLOperation::HashTrigger(_) => "hash_trigger",
            DSLOperation::IntelCollection(_) => "intel_collection",
            DSLOperation::PentestSpawn(_) => "pentest_spawn",
            DSLOperation::EphemeralAsset(_) => "ephemeral_asset",
            DSLOperation::NodeInterview(_) => "node_interview",
            DSLOperation::KaliTool(_) => "kali_tool",
            DSLOperation::Workflow(_) => "workflow",
            DSLOperation::Parallel(_) => "parallel",
            DSLOperation::Conditional(_) => "conditional",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_priority() {
        let intel_op = DSLOperation::IntelCollection(IntelCollectionOp {
            hash: None,
            semantic_hash: None,
            unicode: '\u{E320}',
            tool: "kali_recon".to_string(),
            gpu_tier: "high".to_string(),
            isolation: "threat_intel".to_string(),
            max_cost: 50.0,
            timeout: "4h".to_string(),
            depends_on: vec![],
        });

        assert_eq!(intel_op.priority(), Priority::Critical);
    }

    #[test]
    fn test_unicode_trigger() {
        let hash_op = DSLOperation::HashTrigger(HashTriggerOp {
            sch: "3kJ9mP4xQ7R8sN2m".to_string(),
            unicode: '\u{E100}',
            operation: "test".to_string(),
            metadata: HashMap::new(),
        });

        assert_eq!(hash_op.unicode_trigger(), '\u{E100}');
    }

    #[test]
    fn test_operation_name() {
        let kali_op = DSLOperation::KaliTool(KaliToolOp {
            tool: "nmap".to_string(),
            unicode: '\u{E800}',
            target: "192.168.1.0/24".to_string(),
            container: "alpine_split".to_string(),
            l2_orchestration: true,
            memory_mesh: true,
        });

        assert_eq!(kali_op.operation_name(), "kali_tool");
    }
}
