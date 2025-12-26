//! Tool Executor Integration for Scenario Engine
//!
//! Bridges the Scenario Engine with the Kali Plasma Tool Executor,
//! enabling PTCC personas to drive tool execution and capture outputs
//! through the unified crosswalk.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    EmulationError, HD4Phase, ValidatedPtccOperator, 
    PtccPersonaAssignment, PhaseOperation,
};

/// Tool executor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecutorConfig {
    /// Path to tool profiles TOML
    pub profiles_path: PathBuf,
    /// Path to unified crosswalk JSON
    pub crosswalk_path: PathBuf,
    /// NATS URL for event emission
    pub nats_url: String,
    /// Tool corpus output directory
    pub corpus_path: PathBuf,
    /// Default execution tier (0-3)
    pub default_tier: u8,
}

impl Default for ToolExecutorConfig {
    fn default() -> Self {
        Self {
            profiles_path: PathBuf::from("tools/kali-plasma/tool-exerciser/tool_profiles.toml"),
            crosswalk_path: PathBuf::from("tools/abe/iac/node-interview-generator/output/unified_task_tool_ptcc_crosswalk.json"),
            nats_url: "nats://localhost:4222".to_string(),
            corpus_path: PathBuf::from("data/tool-corpus"),
            default_tier: 0,
        }
    }
}

/// Unified crosswalk data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedCrosswalk {
    pub metadata: CrosswalkMetadata,
    pub tasks: HashMap<String, TaskToolMapping>,
    pub tools: HashMap<String, ToolTaskMapping>,
    pub ptcc_primitives: HashMap<u8, String>,
    pub skill_categories: HashMap<String, Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrosswalkMetadata {
    pub version: String,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskToolMapping {
    pub task_name: String,
    pub kali_tools: Vec<String>,
    pub hd4_phase: String,
    pub category: String,
    pub mitre_techniques: Vec<String>,
    pub inferred_skills: Vec<String>,
    pub inferred_ptcc: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolTaskMapping {
    pub tasks: Vec<String>,
    pub skill: String,
    pub ptcc: Vec<String>,
}

/// Tool execution context passed to the executor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecutionContext {
    /// Unique execution ID
    pub execution_id: String,
    /// Scenario being executed
    pub scenario_id: String,
    /// Assigned persona
    pub persona_id: String,
    /// Current HD4 phase
    pub hd4_phase: HD4Phase,
    /// CTAS task IDs this tool supports
    pub task_ids: Vec<String>,
    /// PTCC primitives for this tool
    pub ptcc_primitives: Vec<String>,
    /// Skill category
    pub skill_category: String,
    /// Execution tier (0-3)
    pub tier: u8,
    /// MITRE techniques
    pub mitre_techniques: Vec<String>,
}

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecutionResult {
    /// Execution context
    pub context: ToolExecutionContext,
    /// Tool name
    pub tool_name: String,
    /// Short code (e.g., NMP7X2D)
    pub short_code: String,
    /// Semantic hash (h1)
    pub h1_hash: String,
    /// Operational hash (h2)
    pub h2_hash: String,
    /// Heredity expression
    pub heredity: String,
    /// Exit code
    pub exit_code: i32,
    /// Execution duration in milliseconds
    pub duration_ms: u64,
    /// Raw output path
    pub raw_output_path: PathBuf,
    /// Parsed output path (if applicable)
    pub parsed_output_path: Option<PathBuf>,
    /// Execution timestamp
    pub executed_at: DateTime<Utc>,
}

/// Tool executor integration service
#[derive(Debug)]
pub struct ToolExecutorIntegration {
    /// Configuration
    config: ToolExecutorConfig,
    /// Loaded crosswalk
    crosswalk: Option<UnifiedCrosswalk>,
    /// Execution history
    execution_history: Vec<ToolExecutionResult>,
}

impl ToolExecutorIntegration {
    /// Create new tool executor integration
    pub fn new(config: ToolExecutorConfig) -> Self {
        Self {
            config,
            crosswalk: None,
            execution_history: Vec::new(),
        }
    }

    /// Load the unified crosswalk
    pub async fn load_crosswalk(&mut self) -> Result<(), EmulationError> {
        let crosswalk_content = tokio::fs::read_to_string(&self.config.crosswalk_path)
            .await
            .map_err(|e| EmulationError::ConfigError(format!("Failed to load crosswalk: {}", e)))?;
        
        let crosswalk: UnifiedCrosswalk = serde_json::from_str(&crosswalk_content)
            .map_err(|e| EmulationError::ConfigError(format!("Failed to parse crosswalk: {}", e)))?;
        
        self.crosswalk = Some(crosswalk);
        Ok(())
    }

    /// Get tools for a PTCC persona based on skill category
    pub fn get_tools_for_persona(&self, persona: &ValidatedPtccOperator) -> Vec<String> {
        let crosswalk = match &self.crosswalk {
            Some(c) => c,
            None => return vec![persona.tool.clone()],
        };

        // Map persona tool to skill category
        let tool_lower = persona.tool.to_lowercase();
        if let Some(tool_mapping) = crosswalk.tools.get(&tool_lower) {
            // Return tools in the same skill category
            crosswalk.tools
                .iter()
                .filter(|(_, mapping)| mapping.skill == tool_mapping.skill)
                .map(|(tool_name, _)| tool_name.clone())
                .collect()
        } else {
            vec![persona.tool.clone()]
        }
    }

    /// Get PTCC primitives for a tool
    pub fn get_ptcc_primitives(&self, tool_name: &str) -> Vec<String> {
        let crosswalk = match &self.crosswalk {
            Some(c) => c,
            None => return vec![],
        };

        crosswalk.tools
            .get(&tool_name.to_lowercase())
            .map(|m| m.ptcc.clone())
            .unwrap_or_default()
    }

    /// Get task IDs supported by a tool
    pub fn get_task_ids(&self, tool_name: &str) -> Vec<String> {
        let crosswalk = match &self.crosswalk {
            Some(c) => c,
            None => return vec![],
        };

        crosswalk.tools
            .get(&tool_name.to_lowercase())
            .map(|m| m.tasks.clone())
            .unwrap_or_default()
    }

    /// Create execution context for a tool within a scenario
    pub fn create_execution_context(
        &self,
        scenario_id: &str,
        persona: &PtccPersonaAssignment,
        hd4_phase: HD4Phase,
        tool_name: &str,
        tier: u8,
    ) -> ToolExecutionContext {
        ToolExecutionContext {
            execution_id: Uuid::new_v4().to_string(),
            scenario_id: scenario_id.to_string(),
            persona_id: persona.persona_id.clone(),
            hd4_phase,
            task_ids: self.get_task_ids(tool_name),
            ptcc_primitives: self.get_ptcc_primitives(tool_name),
            skill_category: self.crosswalk
                .as_ref()
                .and_then(|c| c.tools.get(&tool_name.to_lowercase()))
                .map(|m| m.skill.clone())
                .unwrap_or_else(|| "unknown".to_string()),
            tier,
            mitre_techniques: vec![], // Would be populated from crosswalk
        }
    }

    /// Execute a tool chain for a phase operation
    pub async fn execute_tool_chain(
        &mut self,
        scenario_id: &str,
        persona: &PtccPersonaAssignment,
        operation: &PhaseOperation,
    ) -> Result<Vec<ToolExecutionResult>, EmulationError> {
        let mut results = Vec::new();

        for tool in &operation.required_tools {
            let tool_name = format!("{:?}", tool).to_lowercase();
            let context = self.create_execution_context(
                scenario_id,
                persona,
                operation.assigned_phases().first().cloned().unwrap_or(HD4Phase::Hunt),
                &tool_name,
                self.config.default_tier,
            );

            // Execute tool (stub - would call Docker executor)
            let result = self.execute_tool(&tool_name, context).await?;
            results.push(result);
        }

        Ok(results)
    }

    /// Execute a single tool
    async fn execute_tool(
        &mut self,
        tool_name: &str,
        context: ToolExecutionContext,
    ) -> Result<ToolExecutionResult, EmulationError> {
        // This would invoke the Docker-based tool executor
        // For now, create a stub result
        
        let short_code = self.generate_short_code(tool_name);
        let h1_hash = format!("{:016x}", rand::random::<u64>());
        let h2_hash = format!("{:016x}", rand::random::<u64>());
        
        let result = ToolExecutionResult {
            context: context.clone(),
            tool_name: tool_name.to_string(),
            short_code: short_code.clone(),
            h1_hash: h1_hash.clone(),
            h2_hash: h2_hash.clone(),
            heredity: format!("(cons {} {})", h1_hash, h2_hash),
            exit_code: 0,
            duration_ms: 1000,
            raw_output_path: self.config.corpus_path.join("raw").join(tool_name).join(format!("{}.raw", short_code)),
            parsed_output_path: None,
            executed_at: Utc::now(),
        };

        self.execution_history.push(result.clone());
        
        // Emit NATS event (stub)
        self.emit_tool_output_event(&result).await?;
        
        Ok(result)
    }

    /// Generate a short code for a tool output
    fn generate_short_code(&self, tool_name: &str) -> String {
        let prefix = tool_name.chars().take(3).collect::<String>().to_uppercase();
        let suffix = format!("{:04X}", rand::random::<u16>());
        format!("{}{}", prefix, suffix)
    }

    /// Emit tool output event to NATS
    async fn emit_tool_output_event(&self, result: &ToolExecutionResult) -> Result<(), EmulationError> {
        // Stub - would connect to NATS and publish
        tracing::info!(
            "📤 Tool output: {} -> {} (scenario: {}, persona: {})",
            result.tool_name,
            result.short_code,
            result.context.scenario_id,
            result.context.persona_id
        );
        Ok(())
    }

    /// Get tools for an HD4 phase from crosswalk
    pub fn get_tools_for_phase(&self, phase: &HD4Phase) -> Vec<String> {
        let crosswalk = match &self.crosswalk {
            Some(c) => c,
            None => return vec![],
        };

        // Map HD4 phase to skill categories
        let skill_categories = match phase {
            HD4Phase::Hunt => vec!["reconnaissance"],
            HD4Phase::Detect => vec!["reconnaissance", "evasion"],
            HD4Phase::Disrupt => vec!["exploitation"],
            HD4Phase::Disable => vec!["exploitation", "execution"],
            HD4Phase::Dominate => vec!["execution", "cyber_physical"],
        };

        crosswalk.tools
            .iter()
            .filter(|(_, mapping)| skill_categories.contains(&mapping.skill.as_str()))
            .map(|(tool_name, _)| tool_name.clone())
            .collect()
    }

    /// Get execution history
    pub fn get_execution_history(&self) -> &[ToolExecutionResult] {
        &self.execution_history
    }
}

// Extension trait for PhaseOperation to get assigned phases
trait PhaseOperationExt {
    fn assigned_phases(&self) -> Vec<HD4Phase>;
}

impl PhaseOperationExt for PhaseOperation {
    fn assigned_phases(&self) -> Vec<HD4Phase> {
        // Would extract from operation - stub returns Hunt
        vec![HD4Phase::Hunt]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tool_executor_creation() {
        let config = ToolExecutorConfig::default();
        let executor = ToolExecutorIntegration::new(config);
        assert!(executor.crosswalk.is_none());
    }
}

