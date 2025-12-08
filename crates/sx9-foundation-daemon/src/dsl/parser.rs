//! DSL Parser and Builder Functions
//!
//! Provides builder-style functions for DSL operations.
//! These functions act like macros but are regular Rust functions for easier integration.
//!
//! NOTE: Full procedural macros would require a separate proc-macro crate.
//! For now, we use builder functions with similar ergonomics.

use crate::dsl::operations::*;
use crate::dsl::hash_extractor::*;
use crate::dsl::{DSLError, DSLResult};
use std::collections::HashMap;

/// Builder for hash_trigger! operation
pub struct HashTriggerBuilder {
    sch: Option<String>,
    unicode: Option<char>,
    operation: Option<String>,
    metadata: HashMap<String, String>,
}

impl HashTriggerBuilder {
    pub fn new() -> Self {
        Self {
            sch: None,
            unicode: Some('\u{E100}'), // Default to trivariate processor
            operation: None,
            metadata: HashMap::new(),
        }
    }

    pub fn sch(mut self, sch: impl Into<String>) -> Self {
        self.sch = Some(sch.into());
        self
    }

    pub fn unicode(mut self, unicode: char) -> Self {
        self.unicode = Some(unicode);
        self
    }

    pub fn operation(mut self, operation: impl Into<String>) -> Self {
        self.operation = Some(operation.into());
        self
    }

    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> DSLResult<DSLOperation> {
        Ok(DSLOperation::HashTrigger(HashTriggerOp {
            sch: self.sch.ok_or_else(|| DSLError::InvalidParameters("sch is required".to_string()))?,
            unicode: self.unicode.unwrap(),
            operation: self.operation.ok_or_else(|| DSLError::InvalidParameters("operation is required".to_string()))?,
            metadata: self.metadata,
        }))
    }
}

/// Builder for intel_collection! operation
pub struct IntelCollectionBuilder {
    hash: Option<String>,
    semantic_hash: Option<String>,
    unicode: char,
    tool: Option<String>,
    gpu_tier: String,
    isolation: String,
    max_cost: f64,
    timeout: String,
    depends_on: Vec<String>,
}

impl IntelCollectionBuilder {
    pub fn new() -> Self {
        Self {
            hash: None,
            semantic_hash: None,
            unicode: '\u{E320}', // Intelligence processor
            tool: None,
            gpu_tier: "standard".to_string(),
            isolation: "default".to_string(),
            max_cost: 100.0,
            timeout: "1h".to_string(),
            depends_on: Vec::new(),
        }
    }

    pub fn hash(mut self, hash: impl Into<String>) -> Self {
        self.hash = Some(hash.into());
        self
    }

    pub fn semantic_hash(mut self, semantic_hash: impl Into<String>) -> Self {
        self.semantic_hash = Some(semantic_hash.into());
        self
    }

    pub fn unicode(mut self, unicode: char) -> Self {
        self.unicode = unicode;
        self
    }

    pub fn tool(mut self, tool: impl Into<String>) -> Self {
        self.tool = Some(tool.into());
        self
    }

    pub fn gpu_tier(mut self, tier: impl Into<String>) -> Self {
        self.gpu_tier = tier.into();
        self
    }

    pub fn isolation(mut self, isolation: impl Into<String>) -> Self {
        self.isolation = isolation.into();
        self
    }

    pub fn max_cost(mut self, cost: f64) -> Self {
        self.max_cost = cost;
        self
    }

    pub fn timeout(mut self, timeout: impl Into<String>) -> Self {
        self.timeout = timeout.into();
        self
    }

    pub fn depends_on(mut self, deps: Vec<String>) -> Self {
        self.depends_on = deps;
        self
    }

    pub fn build(self) -> DSLResult<DSLOperation> {
        Ok(DSLOperation::IntelCollection(IntelCollectionOp {
            hash: self.hash,
            semantic_hash: self.semantic_hash,
            unicode: self.unicode,
            tool: self.tool.ok_or_else(|| DSLError::InvalidParameters("tool is required".to_string()))?,
            gpu_tier: self.gpu_tier,
            isolation: self.isolation,
            max_cost: self.max_cost,
            timeout: self.timeout,
            depends_on: self.depends_on,
        }))
    }
}

/// Builder for pentest_spawn! operation
pub struct PentestSpawnBuilder {
    target_hash: Option<String>,
    cuid: Option<String>,
    unicode: char,
    tools: Vec<String>,
    spawn_type: String,
    bridge_type: String,
    geo_constraint: Option<String>,
}

impl PentestSpawnBuilder {
    pub fn new() -> Self {
        Self {
            target_hash: None,
            cuid: None,
            unicode: '\u{E200}', // Context processor
            tools: Vec::new(),
            spawn_type: "ephemeral".to_string(),
            bridge_type: "default".to_string(),
            geo_constraint: None,
        }
    }

    pub fn target_hash(mut self, hash: impl Into<String>) -> Self {
        self.target_hash = Some(hash.into());
        self
    }

    pub fn cuid(mut self, cuid: impl Into<String>) -> Self {
        self.cuid = Some(cuid.into());
        self
    }

    pub fn unicode(mut self, unicode: char) -> Self {
        self.unicode = unicode;
        self
    }

    pub fn tools(mut self, tools: Vec<String>) -> Self {
        self.tools = tools;
        self
    }

    pub fn spawn_type(mut self, spawn_type: impl Into<String>) -> Self {
        self.spawn_type = spawn_type.into();
        self
    }

    pub fn bridge_type(mut self, bridge_type: impl Into<String>) -> Self {
        self.bridge_type = bridge_type.into();
        self
    }

    pub fn geo_constraint(mut self, constraint: impl Into<String>) -> Self {
        self.geo_constraint = Some(constraint.into());
        self
    }

    pub fn build(self) -> DSLResult<DSLOperation> {
        if self.tools.is_empty() {
            return Err(DSLError::InvalidParameters("tools list cannot be empty".to_string()));
        }

        Ok(DSLOperation::PentestSpawn(PentestSpawnOp {
            target_hash: self.target_hash,
            cuid: self.cuid,
            unicode: self.unicode,
            tools: self.tools,
            spawn_type: self.spawn_type,
            bridge_type: self.bridge_type,
            geo_constraint: self.geo_constraint,
        }))
    }
}

/// Builder for ephemeral_asset! operation
pub struct EphemeralAssetBuilder {
    uuid: Option<String>,
    unicode: char,
    trigger: String,
    cleanup: String,
    preserve: Vec<String>,
    forward: String,
    persistence: Option<String>,
    world_id: Option<u32>,
}

impl EphemeralAssetBuilder {
    pub fn new() -> Self {
        Self {
            uuid: None,
            unicode: '\u{E000}', // System controller
            trigger: "operation_complete".to_string(),
            cleanup: "immediate".to_string(),
            preserve: Vec::new(),
            forward: "main_ops".to_string(),
            persistence: None,
            world_id: None,
        }
    }

    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    pub fn unicode(mut self, unicode: char) -> Self {
        self.unicode = unicode;
        self
    }

    pub fn trigger(mut self, trigger: impl Into<String>) -> Self {
        self.trigger = trigger.into();
        self
    }

    pub fn cleanup(mut self, cleanup: impl Into<String>) -> Self {
        self.cleanup = cleanup.into();
        self
    }

    pub fn preserve(mut self, items: Vec<String>) -> Self {
        self.preserve = items;
        self
    }

    pub fn forward(mut self, forward: impl Into<String>) -> Self {
        self.forward = forward.into();
        self
    }

    pub fn persistence(mut self, persistence: impl Into<String>) -> Self {
        self.persistence = Some(persistence.into());
        self
    }

    pub fn world_id(mut self, world_id: u32) -> Self {
        self.world_id = Some(world_id);
        self
    }

    pub fn build(self) -> DSLResult<DSLOperation> {
        Ok(DSLOperation::EphemeralAsset(EphemeralAssetOp {
            uuid: self.uuid,
            unicode: self.unicode,
            trigger: self.trigger,
            cleanup: self.cleanup,
            preserve: self.preserve,
            forward: self.forward,
            persistence: self.persistence,
            world_id: self.world_id,
        }))
    }
}

/// Builder for node_interview! operation
pub struct NodeInterviewBuilder {
    operational_hash: Option<String>,
    semantic_hash: Option<SemanticHashData>,
    task_id: Option<String>,
    task_name: Option<String>,
    hd4_phase: String,
    mitre_technique: Option<String>,
    escalation_tier: Option<u32>,
    databases: Vec<String>,
    glaf_sync: bool,
    kali_tools: Vec<KaliToolRef>,
    wasm_sensor: Option<WASMSensorConfig>,
    escalation: Option<EscalationPath>,
}

impl NodeInterviewBuilder {
    pub fn new() -> Self {
        Self {
            operational_hash: None,
            semantic_hash: None,
            task_id: None,
            task_name: None,
            hd4_phase: "detect".to_string(),
            mitre_technique: None,
            escalation_tier: None,
            databases: vec!["surrealdb".to_string()],
            glaf_sync: true,
            kali_tools: Vec::new(),
            wasm_sensor: None,
            escalation: None,
        }
    }

    pub fn operational_hash(mut self, hash: impl Into<String>) -> Self {
        self.operational_hash = Some(hash.into());
        self
    }

    pub fn semantic_hash(mut self, h3: SemanticHashData) -> Self {
        self.semantic_hash = Some(h3);
        self
    }

    pub fn task_id(mut self, id: impl Into<String>) -> Self {
        self.task_id = Some(id.into());
        self
    }

    pub fn task_name(mut self, name: impl Into<String>) -> Self {
        self.task_name = Some(name.into());
        self
    }

    pub fn hd4_phase(mut self, phase: impl Into<String>) -> Self {
        self.hd4_phase = phase.into();
        self
    }

    pub fn mitre_technique(mut self, technique: impl Into<String>) -> Self {
        self.mitre_technique = Some(technique.into());
        self
    }

    pub fn escalation_tier(mut self, tier: u32) -> Self {
        self.escalation_tier = Some(tier);
        self
    }

    pub fn databases(mut self, databases: Vec<String>) -> Self {
        self.databases = databases;
        self
    }

    pub fn glaf_sync(mut self, sync: bool) -> Self {
        self.glaf_sync = sync;
        self
    }

    pub fn kali_tools(mut self, tools: Vec<KaliToolRef>) -> Self {
        self.kali_tools = tools;
        self
    }

    pub fn wasm_sensor(mut self, sensor: WASMSensorConfig) -> Self {
        self.wasm_sensor = Some(sensor);
        self
    }

    pub fn escalation(mut self, escalation: EscalationPath) -> Self {
        self.escalation = Some(escalation);
        self
    }

    pub fn build(self) -> DSLResult<DSLOperation> {
        Ok(DSLOperation::NodeInterview(NodeInterviewOp {
            operational_hash: self.operational_hash.ok_or_else(|| DSLError::InvalidParameters("operational_hash is required".to_string()))?,
            semantic_hash: self.semantic_hash,
            task_id: self.task_id.ok_or_else(|| DSLError::InvalidParameters("task_id is required".to_string()))?,
            task_name: self.task_name,
            hd4_phase: self.hd4_phase,
            mitre_technique: self.mitre_technique,
            escalation_tier: self.escalation_tier,
            databases: self.databases,
            glaf_sync: self.glaf_sync,
            kali_tools: self.kali_tools,
            wasm_sensor: self.wasm_sensor,
            escalation: self.escalation,
        }))
    }
}

/// Builder for kali_tool! operation
pub struct KaliToolBuilder {
    tool: Option<String>,
    unicode: char,
    target: Option<String>,
    container: String,
    l2_orchestration: bool,
    memory_mesh: bool,
}

impl KaliToolBuilder {
    pub fn new() -> Self {
        Self {
            tool: None,
            unicode: '\u{E800}', // Kali tools processor
            target: None,
            container: "alpine_split".to_string(),
            l2_orchestration: true,
            memory_mesh: true,
        }
    }

    pub fn tool(mut self, tool: impl Into<String>) -> Self {
        self.tool = Some(tool.into());
        self
    }

    pub fn unicode(mut self, unicode: char) -> Self {
        self.unicode = unicode;
        self
    }

    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    pub fn container(mut self, container: impl Into<String>) -> Self {
        self.container = container.into();
        self
    }

    pub fn l2_orchestration(mut self, enabled: bool) -> Self {
        self.l2_orchestration = enabled;
        self
    }

    pub fn memory_mesh(mut self, enabled: bool) -> Self {
        self.memory_mesh = enabled;
        self
    }

    pub fn build(self) -> DSLResult<DSLOperation> {
        Ok(DSLOperation::KaliTool(KaliToolOp {
            tool: self.tool.ok_or_else(|| DSLError::InvalidParameters("tool is required".to_string()))?,
            unicode: self.unicode,
            target: self.target.ok_or_else(|| DSLError::InvalidParameters("target is required".to_string()))?,
            container: self.container,
            l2_orchestration: self.l2_orchestration,
            memory_mesh: self.memory_mesh,
        }))
    }
}

/// Convenience functions that mimic macro syntax

pub fn hash_trigger() -> HashTriggerBuilder {
    HashTriggerBuilder::new()
}

pub fn intel_collection() -> IntelCollectionBuilder {
    IntelCollectionBuilder::new()
}

pub fn pentest_spawn() -> PentestSpawnBuilder {
    PentestSpawnBuilder::new()
}

pub fn ephemeral_asset() -> EphemeralAssetBuilder {
    EphemeralAssetBuilder::new()
}

pub fn node_interview() -> NodeInterviewBuilder {
    NodeInterviewBuilder::new()
}

pub fn kali_tool() -> KaliToolBuilder {
    KaliToolBuilder::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_trigger_builder() {
        let op = hash_trigger()
            .sch("3kJ9mP4xQ7R8sN2m")
            .operation("component_discovery")
            .metadata("test_key", "test_value")
            .build()
            .unwrap();

        assert_eq!(op.operation_name(), "hash_trigger");
    }

    #[test]
    fn test_intel_collection_builder() {
        let op = intel_collection()
            .tool("kali_recon")
            .gpu_tier("high")
            .max_cost(50.0)
            .timeout("4h")
            .build()
            .unwrap();

        assert_eq!(op.operation_name(), "intel_collection");
        assert_eq!(op.priority(), Priority::Critical);
    }

    #[test]
    fn test_pentest_spawn_builder() {
        let op = pentest_spawn()
            .tools(vec!["nmap".to_string(), "metasploit".to_string()])
            .spawn_type("ephemeral_cluster")
            .build()
            .unwrap();

        assert_eq!(op.operation_name(), "pentest_spawn");
    }

    #[test]
    fn test_kali_tool_builder() {
        let op = kali_tool()
            .tool("nmap")
            .target("192.168.1.0/24")
            .container("alpine_split")
            .build()
            .unwrap();

        assert_eq!(op.operation_name(), "kali_tool");
    }

    #[test]
    fn test_builder_validation() {
        // Should fail without required fields
        let result = hash_trigger().build();
        assert!(result.is_err());

        let result = intel_collection().build();
        assert!(result.is_err());
    }
}




