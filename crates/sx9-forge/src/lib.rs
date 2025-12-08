//! SX9 Forge - Dynamic Tool Generation Engine
//!
//! Provides:
//! - GLAF graph storage (Neo4j-like embedded)
//! - RFC-9302 Nonagon Analytic Nodes
//! - Mission Load Sets (in-app purchases)
//! - Dynamic tool creation via Ring Bus Layer 2
//!
//! Architecture:
//! ```
//! GLAF Graph → Nonagon Cell → Tool Chain → Ring Bus L2 Execution
//!      ↓           ↓             ↓              ↓
//!   Neo4j-like   9-vertex    Mission Load   Kali ISO Tools
//! ```

pub mod nonagon;
pub mod mission_load;
pub mod graph;
pub mod tool_gen;
pub mod ossec_loader;

pub use nonagon::{NonagonCell, NonagonConfig, calculate_teth_entropy};
pub use mission_load::{MissionLoadSet, MissionLoadCatalog, HD4Phase, ClearanceLevel, Primitive};
pub use graph::{ForgeGraph, ForgeNode, ForgeEdge};
pub use tool_gen::{ToolGenerator, GeneratedTool, ToolChain};
pub use ossec_loader::{OssecLoader, OssecRule, LoaderStats};

use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// RFC-9302 precision constant (6 decimal places)
pub const DELTA_PRECISION: f64 = 1e-6;

/// Minimum TETH entropy threshold (RFC-9302 validated: 3.9232)
pub const MIN_TETH_ENTROPY: f64 = 2.5;

// Forge engine coordinates GLAF graph and tool generation
// ForgeEngine uses Nonagon cells for workflow analytics
// Mission Load Sets define purchasable tool chain combinations

/// SX9 Forge Engine
pub struct ForgeEngine {
    /// GLAF graph storage
    graph: Arc<RwLock<ForgeGraph>>,
    /// Mission Load catalog
    catalog: Arc<RwLock<MissionLoadCatalog>>,
    /// Tool generator
    tool_gen: Arc<ToolGenerator>,
    /// Configuration
    config: ForgeConfig,
}

/// Forge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeConfig {
    /// Sled database path
    pub db_path: String,
    /// Ring bus node ID
    pub ring_bus_node_id: u8,
    /// Kali ISO endpoint
    pub kali_iso_endpoint: String,
    /// OSSEC rules path
    pub ossec_rules_path: String,
    /// Enable Layer 2 execution
    pub l2_execution: bool,
}

impl Default for ForgeConfig {
    fn default() -> Self {
        Self {
            db_path: "/opt/sx9/forge/db".to_string(),
            ring_bus_node_id: 9,
            kali_iso_endpoint: "http://localhost:18200".to_string(),
            ossec_rules_path: "/opt/sx9/forge/ossec_toml_rules/".to_string(),
            l2_execution: true,
        }
    }
}

impl ForgeEngine {
    /// Create new Forge engine
    pub async fn new(config: ForgeConfig) -> anyhow::Result<Self> {
        info!("Initializing SX9 Forge Engine");

        // Initialize GLAF graph
        let graph = Arc::new(RwLock::new(ForgeGraph::new(&config.db_path).await?));

        // Initialize Mission Load catalog with defaults
        let catalog = Arc::new(RwLock::new(MissionLoadCatalog::new_with_defaults()));

        // Initialize tool generator
        let tool_gen = Arc::new(ToolGenerator::new(
            config.kali_iso_endpoint.clone(),
            config.ossec_rules_path.clone(),
        ));

        info!("SX9 Forge Engine initialized");
        info!("  - Ring Bus Node: {}", config.ring_bus_node_id);
        info!("  - L2 Execution: {}", config.l2_execution);

        Ok(Self {
            graph,
            catalog,
            tool_gen,
            config,
        })
    }

    /// Get graph reference
    pub fn graph(&self) -> Arc<RwLock<ForgeGraph>> {
        self.graph.clone()
    }

    /// Get catalog reference
    pub fn catalog(&self) -> Arc<RwLock<MissionLoadCatalog>> {
        self.catalog.clone()
    }

    /// Get tool generator
    pub fn tool_gen(&self) -> Arc<ToolGenerator> {
        self.tool_gen.clone()
    }

    /// Create dynamic tool from Mission Load
    pub async fn create_tool_from_load(&self, load_id: &str) -> anyhow::Result<GeneratedTool> {
        let catalog = self.catalog.read().await;
        let load = catalog.get(load_id)
            .ok_or_else(|| anyhow::anyhow!("Mission Load not found: {}", load_id))?;

        // Generate tool from mission load
        let tool = self.tool_gen.generate_from_load(load).await?;

        // Add to graph
        let mut graph = self.graph.write().await;
        graph.add_tool_node(&tool).await?;

        info!("Created tool from Mission Load: {} -> {}", load_id, tool.id);
        Ok(tool)
    }

    /// Execute tool chain via Ring Bus L2
    pub async fn execute_tool_chain(&self, chain: &ToolChain) -> anyhow::Result<()> {
        if !self.config.l2_execution {
            return Err(anyhow::anyhow!("L2 execution disabled"));
        }

        info!("Executing tool chain via Ring Bus L2: {}", chain.id);

        // Validate nonagon cell
        let entropy = calculate_teth_entropy(&chain.nonagon);
        if entropy < MIN_TETH_ENTROPY {
            return Err(anyhow::anyhow!(
                "TETH entropy too low: {} < {}",
                entropy,
                MIN_TETH_ENTROPY
            ));
        }

        // Execute via tool generator
        self.tool_gen.execute_chain(chain).await?;

        info!("Tool chain executed: {}", chain.id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_forge_engine_creation() {
        let config = ForgeConfig {
            db_path: "/tmp/sx9-forge-test".to_string(),
            ..Default::default()
        };

        // Note: This test requires the sx9-glaf-core crate
        // In production, would verify engine initialization
    }
}
