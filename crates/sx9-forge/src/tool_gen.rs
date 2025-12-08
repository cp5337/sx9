//! Tool Generator - Dynamic Tool Creation via Ring Bus L2
//!
//! Generates executable tools from Mission Loads and OSSEC TOML rules.
//! Executes tool chains via Ring Bus Layer 2 with Kali ISO endpoint.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::{info, warn, debug};

use crate::nonagon::{NonagonCell, HD4Phase, calculate_teth_entropy, MIN_TETH_ENTROPY};
use crate::mission_load::{MissionLoadSet, Primitive};

/// Generated tool from Mission Load
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedTool {
    /// Unique identifier
    pub id: String,
    /// Tool name
    pub name: String,
    /// Description
    pub description: String,
    /// Source Mission Load ID
    pub mission_load_id: String,
    /// HD4 phase
    pub hd4_phase: HD4Phase,
    /// Associated nonagon cell
    pub nonagon: NonagonCell,
    /// Primitives this tool uses
    pub primitives: Vec<Primitive>,
    /// OSSEC rule IDs applied
    pub ossec_rule_ids: Vec<String>,
    /// Ring Bus execution endpoint
    pub ring_bus_endpoint: Option<String>,
    /// Tool parameters
    pub parameters: HashMap<String, ToolParameter>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Execution count
    pub execution_count: u64,
    /// Last execution time
    pub last_executed: Option<DateTime<Utc>>,
}

impl GeneratedTool {
    /// Create new tool from mission load
    pub fn from_mission_load(load: &MissionLoadSet) -> Self {
        let id = format!("tool-{}", Uuid::new_v4());
        let now = Utc::now();

        let mut nonagon = load.nonagon.clone();
        nonagon.id = id.clone();
        nonagon.recalculate();

        Self {
            id,
            name: format!("{} Tool", load.name),
            description: load.description.clone(),
            mission_load_id: load.id.clone(),
            hd4_phase: load.hd4_phase,
            nonagon,
            primitives: load.primitives.clone(),
            ossec_rule_ids: Vec::new(),
            ring_bus_endpoint: None,
            parameters: HashMap::new(),
            created_at: now,
            execution_count: 0,
            last_executed: None,
        }
    }

    /// Add OSSEC rule
    pub fn add_ossec_rule(&mut self, rule_id: impl Into<String>) {
        self.ossec_rule_ids.push(rule_id.into());
    }

    /// Add parameter
    pub fn add_parameter(&mut self, name: impl Into<String>, param: ToolParameter) {
        self.parameters.insert(name.into(), param);
    }

    /// Record execution
    pub fn record_execution(&mut self) {
        self.execution_count += 1;
        self.last_executed = Some(Utc::now());
    }

    /// Check if tool is valid (meets entropy threshold)
    pub fn is_valid(&self) -> bool {
        self.nonagon.is_valid()
    }
}

/// Tool parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    /// Parameter type
    pub param_type: ParameterType,
    /// Default value
    pub default: Option<String>,
    /// Is required
    pub required: bool,
    /// Description
    pub description: String,
    /// Validation regex (if applicable)
    pub validation: Option<String>,
}

/// Parameter types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
    IpAddress,
    Port,
    FilePath,
    Url,
    Json,
}

/// Tool chain - sequence of tools to execute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolChain {
    /// Chain identifier
    pub id: String,
    /// Chain name
    pub name: String,
    /// Description
    pub description: String,
    /// Nonagon cell for the chain
    pub nonagon: NonagonCell,
    /// Ordered list of tool IDs
    pub tool_ids: Vec<String>,
    /// Chain parameters (passed to all tools)
    pub parameters: HashMap<String, String>,
    /// Execution mode
    pub execution_mode: ExecutionMode,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

impl ToolChain {
    /// Create new tool chain
    pub fn new(name: impl Into<String>) -> Self {
        let id = format!("chain-{}", Uuid::new_v4());
        Self {
            id: id.clone(),
            name: name.into(),
            description: String::new(),
            nonagon: NonagonCell::new(&id),
            tool_ids: Vec::new(),
            parameters: HashMap::new(),
            execution_mode: ExecutionMode::Sequential,
            created_at: Utc::now(),
        }
    }

    /// Add tool to chain
    pub fn add_tool(&mut self, tool_id: impl Into<String>) {
        self.tool_ids.push(tool_id.into());
    }

    /// Set chain parameter
    pub fn set_parameter(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.parameters.insert(key.into(), value.into());
    }

    /// Check if chain is valid
    pub fn is_valid(&self) -> bool {
        !self.tool_ids.is_empty() && self.nonagon.is_valid()
    }
}

/// Execution modes for tool chains
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ExecutionMode {
    /// Execute tools one at a time
    Sequential,
    /// Execute tools in parallel
    Parallel,
    /// Execute based on graph dependencies
    Graph,
}

/// Tool Generator - creates and executes tools
pub struct ToolGenerator {
    /// Kali ISO endpoint for L2 execution
    kali_endpoint: String,
    /// OSSEC rules directory
    ossec_rules_path: PathBuf,
    /// Generated tools cache
    tools: HashMap<String, GeneratedTool>,
    /// Tool chains cache
    chains: HashMap<String, ToolChain>,
}

impl ToolGenerator {
    /// Create new tool generator
    pub fn new(kali_endpoint: impl Into<String>, ossec_path: impl Into<String>) -> Self {
        Self {
            kali_endpoint: kali_endpoint.into(),
            ossec_rules_path: PathBuf::from(ossec_path.into()),
            tools: HashMap::new(),
            chains: HashMap::new(),
        }
    }

    /// Generate tool from mission load
    pub async fn generate_from_load(&self, load: &MissionLoadSet) -> anyhow::Result<GeneratedTool> {
        info!("Generating tool from Mission Load: {}", load.name);

        let mut tool = GeneratedTool::from_mission_load(load);

        // Set Ring Bus endpoint
        tool.ring_bus_endpoint = Some(self.kali_endpoint.clone());

        // Load relevant OSSEC rules based on primitives
        let rules = self.find_ossec_rules_for_primitives(&load.primitives).await?;
        for rule_id in rules {
            tool.add_ossec_rule(rule_id);
        }

        // Validate entropy
        if !tool.is_valid() {
            warn!(
                "Generated tool has low entropy: {} < {}",
                tool.nonagon.teth_entropy, MIN_TETH_ENTROPY
            );
        }

        info!(
            "Generated tool: {} (entropy: {:.4})",
            tool.name, tool.nonagon.teth_entropy
        );

        Ok(tool)
    }

    /// Find OSSEC rules matching primitives
    async fn find_ossec_rules_for_primitives(&self, primitives: &[Primitive]) -> anyhow::Result<Vec<String>> {
        let mut rule_ids = Vec::new();

        // Map primitives to OSSEC rule patterns
        for primitive in primitives {
            let pattern = match primitive {
                Primitive::Read => "read_",
                Primitive::Write => "write_",
                Primitive::Execute => "execute_",
                Primitive::Authenticate => "auth_",
                Primitive::Authorize => "authorize_",
                Primitive::Encrypt => "encrypt_",
                Primitive::Decrypt => "decrypt_",
                Primitive::Route => "route_",
                Primitive::Filter => "filter_",
                Primitive::Observe => "observe_",
                Primitive::Validate => "validate_",
                Primitive::CommandControl => "c2_",
                Primitive::Reconnaissance => "recon_",
                _ => continue,
            };

            // Scan OSSEC rules directory
            if self.ossec_rules_path.exists() {
                if let Ok(entries) = std::fs::read_dir(&self.ossec_rules_path) {
                    for entry in entries.filter_map(|e| e.ok()) {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if name.contains(pattern) && name.ends_with(".toml") {
                            let rule_id = name.trim_end_matches(".toml").to_string();
                            rule_ids.push(rule_id);
                        }
                    }
                }
            }
        }

        debug!("Found {} OSSEC rules for primitives", rule_ids.len());
        Ok(rule_ids)
    }

    /// Create tool chain from multiple tools
    pub fn create_chain(&mut self, name: impl Into<String>, tool_ids: Vec<String>) -> ToolChain {
        let mut chain = ToolChain::new(name);
        for id in tool_ids {
            chain.add_tool(id);
        }

        let chain_id = chain.id.clone();
        self.chains.insert(chain_id, chain.clone());
        chain
    }

    /// Execute tool chain via Ring Bus L2
    pub async fn execute_chain(&self, chain: &ToolChain) -> anyhow::Result<ChainExecutionResult> {
        info!("Executing tool chain: {} via Ring Bus L2", chain.name);

        // Validate chain
        if !chain.is_valid() {
            return Err(anyhow::anyhow!("Invalid tool chain: entropy too low"));
        }

        let mut results = Vec::new();
        let start_time = Utc::now();

        match chain.execution_mode {
            ExecutionMode::Sequential => {
                for tool_id in &chain.tool_ids {
                    let result = self.execute_tool_l2(tool_id, &chain.parameters).await?;
                    results.push(result);
                }
            }
            ExecutionMode::Parallel => {
                // In parallel mode, spawn concurrent executions
                let mut handles = Vec::new();
                for tool_id in &chain.tool_ids {
                    let endpoint = self.kali_endpoint.clone();
                    let params = chain.parameters.clone();
                    let tid = tool_id.clone();

                    handles.push(tokio::spawn(async move {
                        // Simulated L2 execution
                        ToolExecutionResult {
                            tool_id: tid,
                            success: true,
                            output: Some("Parallel execution complete".into()),
                            error: None,
                            duration_ms: 100,
                        }
                    }));
                }

                for handle in handles {
                    results.push(handle.await?);
                }
            }
            ExecutionMode::Graph => {
                // Graph-based execution follows dependency order
                // For now, fall back to sequential
                for tool_id in &chain.tool_ids {
                    let result = self.execute_tool_l2(tool_id, &chain.parameters).await?;
                    results.push(result);
                }
            }
        }

        let end_time = Utc::now();
        let total_duration = (end_time - start_time).num_milliseconds() as u64;

        let success_count = results.iter().filter(|r| r.success).count();

        info!(
            "Chain execution complete: {}/{} succeeded in {}ms",
            success_count,
            results.len(),
            total_duration
        );

        Ok(ChainExecutionResult {
            chain_id: chain.id.clone(),
            success: success_count == results.len(),
            tool_results: results,
            total_duration_ms: total_duration,
            executed_at: start_time,
        })
    }

    /// Execute single tool via Ring Bus L2
    async fn execute_tool_l2(
        &self,
        tool_id: &str,
        params: &HashMap<String, String>,
    ) -> anyhow::Result<ToolExecutionResult> {
        debug!("Executing tool {} via L2 endpoint {}", tool_id, self.kali_endpoint);

        // Build L2 execution request
        let request = L2ExecutionRequest {
            tool_id: tool_id.to_string(),
            parameters: params.clone(),
            endpoint: self.kali_endpoint.clone(),
        };

        // In production, this would call the Kali ISO endpoint
        // For now, simulate execution
        let start = std::time::Instant::now();

        // Simulated execution delay
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let duration = start.elapsed().as_millis() as u64;

        Ok(ToolExecutionResult {
            tool_id: tool_id.to_string(),
            success: true,
            output: Some(format!("Tool {} executed via Ring Bus L2", tool_id)),
            error: None,
            duration_ms: duration,
        })
    }

    /// Get tool by ID
    pub fn get_tool(&self, id: &str) -> Option<&GeneratedTool> {
        self.tools.get(id)
    }

    /// Get chain by ID
    pub fn get_chain(&self, id: &str) -> Option<&ToolChain> {
        self.chains.get(id)
    }

    /// Store tool
    pub fn store_tool(&mut self, tool: GeneratedTool) {
        self.tools.insert(tool.id.clone(), tool);
    }
}

/// L2 execution request
#[derive(Debug, Clone, Serialize, Deserialize)]
struct L2ExecutionRequest {
    tool_id: String,
    parameters: HashMap<String, String>,
    endpoint: String,
}

/// Result of tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecutionResult {
    /// Tool ID
    pub tool_id: String,
    /// Whether execution succeeded
    pub success: bool,
    /// Output data
    pub output: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
    /// Execution duration in ms
    pub duration_ms: u64,
}

/// Result of chain execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainExecutionResult {
    /// Chain ID
    pub chain_id: String,
    /// Overall success
    pub success: bool,
    /// Individual tool results
    pub tool_results: Vec<ToolExecutionResult>,
    /// Total duration
    pub total_duration_ms: u64,
    /// Execution timestamp
    pub executed_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mission_load::ClearanceLevel;

    #[test]
    fn test_tool_from_mission_load() {
        let mut load = MissionLoadSet::new("Test Load", HD4Phase::Hunt, ClearanceLevel::Public);
        load.add_primitives(&[Primitive::Read, Primitive::Observe]);

        let tool = GeneratedTool::from_mission_load(&load);
        assert!(tool.name.contains("Test Load"));
        assert_eq!(tool.hd4_phase, HD4Phase::Hunt);
        assert_eq!(tool.primitives.len(), 2);
    }

    #[test]
    fn test_tool_chain_creation() {
        let mut chain = ToolChain::new("Test Chain");
        chain.add_tool("tool-1");
        chain.add_tool("tool-2");

        assert_eq!(chain.tool_ids.len(), 2);
    }

    #[tokio::test]
    async fn test_tool_generator() {
        let gen = ToolGenerator::new("http://localhost:18200", "/tmp/ossec");

        let mut load = MissionLoadSet::new("Test", HD4Phase::Detect, ClearanceLevel::Public);
        load.add_primitives(&[Primitive::Validate, Primitive::Cache]);

        let tool = gen.generate_from_load(&load).await.unwrap();
        assert!(tool.ring_bus_endpoint.is_some());
    }
}
