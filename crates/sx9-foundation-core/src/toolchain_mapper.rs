//! Toolchain Mapper - Tool Orchestration and Mapping
//! 
//! This module provides toolchain mapping and orchestration
//! capabilities for operational tool management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// Toolchain mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolchainMapping {
    pub mapping_id: String,
    pub mission_type: String,
    pub tools: Vec<ToolMapping>,
    pub dependencies: Vec<String>,
    pub execution_order: Vec<String>,
}

/// Tool mapping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMapping {
    pub tool_name: String,
    pub tool_version: String,
    pub category: ToolCategory,
    pub dependencies: Vec<String>,
    pub parameters: HashMap<String, String>,
}

/// Tool categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolCategory {
    Reconnaissance,
    Exploitation,
    PostExploitation,
    Persistence,
    Defense,
    Analysis,
}

/// Toolchain Mapper
pub struct ToolchainMapper {
    mappings: HashMap<String, ToolchainMapping>,
}

impl ToolchainMapper {
    /// Create a new toolchain mapper
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    /// Add toolchain mapping
    pub fn add_mapping(&mut self, mapping: ToolchainMapping) {
        info!("🗺️ Adding toolchain mapping: {}", mapping.mapping_id);
        self.mappings.insert(mapping.mapping_id.clone(), mapping);
    }

    /// Get mapping for mission type
    pub fn get_mapping(&self, mission_type: &str) -> Option<&ToolchainMapping> {
        self.mappings.values().find(|m| m.mission_type == mission_type)
    }

    /// Get all mappings
    pub fn get_all_mappings(&self) -> Vec<&ToolchainMapping> {
        self.mappings.values().collect()
    }
}

impl Default for ToolchainMapper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapper_creation() {
        let mapper = ToolchainMapper::new();
        assert_eq!(mapper.mappings.len(), 0);
    }
}
