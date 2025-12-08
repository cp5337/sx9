//! EEI Processor - Essential Elements of Information processing
//! Processes EEIs and correlates them with toolchains and crates

use crate::eei_types::*;
use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct EEIProcessor {
    pub eei_registry: Arc<RwLock<HashMap<String, EEI>>>,
    pub toolchain_mappings: Arc<RwLock<HashMap<String, ToolChainMapping>>>,
    pub satisfaction_matrix: Arc<RwLock<HashMap<String, Vec<EEISatisfaction>>>>,
}

#[derive(Debug, Clone)]
pub struct ToolChainMapping {
    pub toolchain_id: String,
    pub tools: Vec<ToolSpec>,
    pub rust_wrappers: Vec<RustWrapper>,
    pub xsd_integration: XSDIntegration,
    pub eei_satisfaction: Vec<EEISatisfaction>,
}

#[derive(Debug, Clone)]
pub struct ToolSpec {
    pub tool_name: String,
    pub tool_version: String,
    pub wrapper_crate: Option<String>,
    pub layer2_interface: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RustWrapper {
    pub wrapper_name: String,
    pub target_tool: String,
    pub wrapper_completeness: f32,
    pub xsd_ready: bool,
    pub metaprogramming_hooks: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct XSDIntegration {
    pub schema_compliance: bool,
    pub metaprogramming_enabled: bool,
    pub layer2_touch_points: Vec<String>,
    pub code_generation_rules: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct EEISatisfaction {
    pub eei_id: String,
    pub satisfaction_method: String,
    pub required_tools: Vec<String>,
    pub confidence_level: f32,
}

impl EEIProcessor {
    pub fn new() -> Self {
        Self {
            eei_registry: Arc::new(RwLock::new(HashMap::new())),
            toolchain_mappings: Arc::new(RwLock::new(HashMap::new())),
            satisfaction_matrix: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register an EEI in the processor
    pub async fn register_eei(&self, eei: EEI) -> Result<()> {
        let mut registry = self.eei_registry.write().await;
        registry.insert(eei.eei_id.clone(), eei);
        Ok(())
    }

    /// Process EEI requirements and find matching toolchains
    pub async fn process_eei_requirements(&self, eei_id: &str) -> Result<EEIProcessingResult> {
        let registry = self.eei_registry.read().await;
        let eei = registry.get(eei_id)
            .ok_or_else(|| anyhow::anyhow!("EEI not found: {}", eei_id))?;

        let toolchain_mappings = self.toolchain_mappings.read().await;
        let mut matching_toolchains = Vec::new();
        let mut missing_tools = Vec::new();
        let mut wrapper_requirements = Vec::new();

        // Find toolchains that can satisfy this EEI
        for (toolchain_id, toolchain) in toolchain_mappings.iter() {
            let satisfaction_score = self.calculate_satisfaction_score(eei, toolchain);
            if satisfaction_score > 0.7 {
                matching_toolchains.push(ToolChainMatch {
                    toolchain_id: toolchain_id.clone(),
                    satisfaction_score,
                    coverage: self.calculate_coverage(eei, toolchain),
                });
            }
        }

        // Identify missing tools and wrapper requirements
        for tool_req in &eei.required_tools {
            let tool_available = toolchain_mappings.values()
                .any(|tc| tc.tools.iter().any(|t| t.tool_name == tool_req.tool_name));

            if !tool_available {
                missing_tools.push(tool_req.clone());
            }

            if tool_req.rust_wrapper_status == WrapperStatus::NotWrapped ||
               tool_req.rust_wrapper_status == WrapperStatus::PartialWrapper {
                wrapper_requirements.push(WrapperRequirement {
                    tool_name: tool_req.tool_name.clone(),
                    wrapper_priority: self.calculate_wrapper_priority(tool_req),
                    xsd_integration_needed: tool_req.xsd_integration_level == XSDIntegrationLevel::None,
                    layer2_touch_point: None,
                });
            }
        }

        // Sort toolchains by satisfaction score
        matching_toolchains.sort_by(|a, b| b.satisfaction_score.partial_cmp(&a.satisfaction_score).unwrap());

        Ok(EEIProcessingResult {
            eei_id: eei_id.to_string(),
            matching_toolchains,
            missing_tools,
            wrapper_requirements,
            overall_satisfaction: self.calculate_overall_satisfaction(&matching_toolchains),
        })
    }

    /// Calculate satisfaction score for a toolchain against an EEI
    fn calculate_satisfaction_score(&self, eei: &EEI, toolchain: &ToolChainMapping) -> f32 {
        let mut score = 0.0;
        let mut total_requirements = 0;

        for tool_req in &eei.required_tools {
            total_requirements += 1;
            
            // Check if tool is available in toolchain
            if toolchain.tools.iter().any(|t| t.tool_name == tool_req.tool_name) {
                score += 1.0;
            }

            // Check wrapper status
            if let Some(wrapper) = toolchain.rust_wrappers.iter()
                .find(|w| w.target_tool == tool_req.tool_name) {
                score += wrapper.wrapper_completeness * 0.5;
            }

            // Check XSD integration
            if toolchain.xsd_integration.schema_compliance {
                score += 0.3;
            }
        }

        if total_requirements == 0 {
            0.0
        } else {
            score / total_requirements as f32
        }
    }

    /// Calculate coverage percentage for a toolchain
    fn calculate_coverage(&self, eei: &EEI, toolchain: &ToolChainMapping) -> f32 {
        let covered_tools = eei.required_tools.iter()
            .filter(|req| toolchain.tools.iter().any(|t| t.tool_name == req.tool_name))
            .count();

        if eei.required_tools.is_empty() {
            0.0
        } else {
            covered_tools as f32 / eei.required_tools.len() as f32
        }
    }

    /// Calculate wrapper priority based on tool category
    fn calculate_wrapper_priority(&self, tool_req: &ToolRequirement) -> WrapperPriority {
        match tool_req.tool_category {
            ToolCategory::NetworkTool => WrapperPriority::High,
            ToolCategory::ForensicTool => WrapperPriority::Critical,
            ToolCategory::MalwareAnalysisTool => WrapperPriority::Critical,
            ToolCategory::VulnerabilityScanner => WrapperPriority::Medium,
            ToolCategory::ThreatIntelTool => WrapperPriority::High,
        }
    }

    /// Calculate overall satisfaction from matching toolchains
    fn calculate_overall_satisfaction(&self, toolchains: &[ToolChainMatch]) -> f32 {
        if toolchains.is_empty() {
            0.0
        } else {
            toolchains.iter().map(|tc| tc.satisfaction_score).sum::<f32>() / toolchains.len() as f32
        }
    }

    /// Get all registered EEIs
    pub async fn get_all_eeis(&self) -> Result<Vec<EEI>> {
        let registry = self.eei_registry.read().await;
        Ok(registry.values().cloned().collect())
    }

    /// Get EEI by ID
    pub async fn get_eei(&self, eei_id: &str) -> Result<Option<EEI>> {
        let registry = self.eei_registry.read().await;
        Ok(registry.get(eei_id).cloned())
    }
}

#[derive(Debug, Clone)]
pub struct EEIProcessingResult {
    pub eei_id: String,
    pub matching_toolchains: Vec<ToolChainMatch>,
    pub missing_tools: Vec<ToolRequirement>,
    pub wrapper_requirements: Vec<WrapperRequirement>,
    pub overall_satisfaction: f32,
}

#[derive(Debug, Clone)]
pub struct ToolChainMatch {
    pub toolchain_id: String,
    pub satisfaction_score: f32,
    pub coverage: f32,
}

#[derive(Debug, Clone)]
pub struct WrapperRequirement {
    pub tool_name: String,
    pub wrapper_priority: WrapperPriority,
    pub xsd_integration_needed: bool,
    pub layer2_touch_point: Option<String>,
}

#[derive(Debug, Clone)]
pub enum WrapperPriority {
    Critical,
    High,
    Medium,
    Low,
}
