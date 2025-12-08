//! GLAF Graph Storage - Neo4j-like Embedded Graph
//!
//! Uses sled for persistence and petgraph for in-memory operations.
//! Provides node/edge storage with TETH entropy tracking.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::{info, debug};

use crate::nonagon::NonagonCell;
use crate::tool_gen::GeneratedTool;

/// Node types in the forge graph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeType {
    /// Nonagon workflow cell
    Nonagon,
    /// Generated tool
    Tool,
    /// Mission load set
    MissionLoad,
    /// OSSEC rule
    OssecRule,
    /// Data source
    DataSource,
    /// Execution result
    Result,
}

/// Edge types connecting nodes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EdgeType {
    /// Tool uses nonagon cell
    UsesCell,
    /// Tool derived from mission load
    DerivedFrom,
    /// Tool executes rule
    ExecutesRule,
    /// Data flows between nodes
    DataFlow,
    /// Dependency relationship
    DependsOn,
    /// Parent-child hierarchy
    Contains,
    /// Triggers execution
    Triggers,
}

/// Graph node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeNode {
    /// Unique identifier
    pub id: String,
    /// Node type
    pub node_type: NodeType,
    /// Display label
    pub label: String,
    /// TETH entropy (if applicable)
    pub teth_entropy: Option<f64>,
    /// Additional properties
    pub properties: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified
    pub updated_at: DateTime<Utc>,
}

impl ForgeNode {
    /// Create new node
    pub fn new(node_type: NodeType, label: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: format!("node-{}", Uuid::new_v4()),
            node_type,
            label: label.into(),
            teth_entropy: None,
            properties: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Create from nonagon cell
    pub fn from_nonagon(cell: &NonagonCell) -> Self {
        let mut node = Self::new(NodeType::Nonagon, &cell.id);
        node.id = cell.id.clone();
        node.teth_entropy = Some(cell.teth_entropy);
        node.properties.insert("confidence".into(), cell.confidence.to_string());
        node
    }

    /// Create from generated tool
    pub fn from_tool(tool: &GeneratedTool) -> Self {
        let mut node = Self::new(NodeType::Tool, &tool.name);
        node.id = tool.id.clone();
        node.teth_entropy = Some(tool.nonagon.teth_entropy);
        node.properties.insert("mission_load_id".into(), tool.mission_load_id.clone());
        node
    }

    /// Set property
    pub fn set_property(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.properties.insert(key.into(), value.into());
        self.updated_at = Utc::now();
    }
}

/// Graph edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeEdge {
    /// Unique identifier
    pub id: String,
    /// Edge type
    pub edge_type: EdgeType,
    /// Source node ID
    pub source_id: String,
    /// Target node ID
    pub target_id: String,
    /// Edge weight (for pathfinding)
    pub weight: f64,
    /// Additional properties
    pub properties: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl ForgeEdge {
    /// Create new edge
    pub fn new(
        edge_type: EdgeType,
        source_id: impl Into<String>,
        target_id: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("edge-{}", Uuid::new_v4()),
            edge_type,
            source_id: source_id.into(),
            target_id: target_id.into(),
            weight: 1.0,
            properties: HashMap::new(),
            created_at: Utc::now(),
        }
    }

    /// Set weight
    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight;
        self
    }
}

/// GLAF Forge Graph - Neo4j-like embedded graph storage
pub struct ForgeGraph {
    /// Sled database for persistence
    db: sled::Db,
    /// In-memory graph for fast traversal
    graph: DiGraph<String, EdgeType>,
    /// Node ID to graph index mapping
    node_indices: HashMap<String, NodeIndex>,
    /// Node storage tree
    nodes_tree: sled::Tree,
    /// Edge storage tree
    edges_tree: sled::Tree,
}

impl ForgeGraph {
    /// Create new graph with sled persistence
    pub async fn new(db_path: &str) -> anyhow::Result<Self> {
        info!("Initializing ForgeGraph at {}", db_path);

        let db = sled::open(db_path)?;
        let nodes_tree = db.open_tree("nodes")?;
        let edges_tree = db.open_tree("edges")?;

        let mut graph = Self {
            db,
            graph: DiGraph::new(),
            node_indices: HashMap::new(),
            nodes_tree,
            edges_tree,
        };

        // Load existing nodes and edges from sled
        graph.load_from_sled().await?;

        info!("ForgeGraph initialized with {} nodes", graph.node_count());
        Ok(graph)
    }

    /// Load graph from sled persistence
    async fn load_from_sled(&mut self) -> anyhow::Result<()> {
        // Load nodes
        for result in self.nodes_tree.iter() {
            let (key, value) = result?;
            let node: ForgeNode = serde_json::from_slice(&value)?;
            let idx = self.graph.add_node(node.id.clone());
            self.node_indices.insert(node.id.clone(), idx);
            debug!("Loaded node: {}", String::from_utf8_lossy(&key));
        }

        // Load edges
        for result in self.edges_tree.iter() {
            let (_, value) = result?;
            let edge: ForgeEdge = serde_json::from_slice(&value)?;

            if let (Some(&src_idx), Some(&tgt_idx)) = (
                self.node_indices.get(&edge.source_id),
                self.node_indices.get(&edge.target_id),
            ) {
                self.graph.add_edge(src_idx, tgt_idx, edge.edge_type);
            }
        }

        Ok(())
    }

    /// Add node to graph
    pub async fn add_node(&mut self, node: ForgeNode) -> anyhow::Result<()> {
        let id = node.id.clone();

        // Add to petgraph
        let idx = self.graph.add_node(id.clone());
        self.node_indices.insert(id.clone(), idx);

        // Persist to sled
        let data = serde_json::to_vec(&node)?;
        self.nodes_tree.insert(id.as_bytes(), data)?;

        debug!("Added node: {}", id);
        Ok(())
    }

    /// Add tool node to graph
    pub async fn add_tool_node(&mut self, tool: &GeneratedTool) -> anyhow::Result<()> {
        let node = ForgeNode::from_tool(tool);
        self.add_node(node).await
    }

    /// Add nonagon cell node
    pub async fn add_nonagon_node(&mut self, cell: &NonagonCell) -> anyhow::Result<()> {
        let node = ForgeNode::from_nonagon(cell);
        self.add_node(node).await
    }

    /// Add edge to graph
    pub async fn add_edge(&mut self, edge: ForgeEdge) -> anyhow::Result<()> {
        let src_idx = self.node_indices.get(&edge.source_id)
            .ok_or_else(|| anyhow::anyhow!("Source node not found: {}", edge.source_id))?;
        let tgt_idx = self.node_indices.get(&edge.target_id)
            .ok_or_else(|| anyhow::anyhow!("Target node not found: {}", edge.target_id))?;

        // Add to petgraph
        self.graph.add_edge(*src_idx, *tgt_idx, edge.edge_type.clone());

        // Persist to sled
        let data = serde_json::to_vec(&edge)?;
        self.edges_tree.insert(edge.id.as_bytes(), data)?;

        debug!("Added edge: {} -> {}", edge.source_id, edge.target_id);
        Ok(())
    }

    /// Get node by ID
    pub fn get_node(&self, id: &str) -> anyhow::Result<Option<ForgeNode>> {
        match self.nodes_tree.get(id.as_bytes())? {
            Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
            None => Ok(None),
        }
    }

    /// Get neighbors of a node
    pub fn get_neighbors(&self, id: &str, direction: Direction) -> Vec<String> {
        if let Some(&idx) = self.node_indices.get(id) {
            self.graph
                .neighbors_directed(idx, direction)
                .filter_map(|n| self.graph.node_weight(n).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get outgoing neighbors
    pub fn get_outgoing(&self, id: &str) -> Vec<String> {
        self.get_neighbors(id, Direction::Outgoing)
    }

    /// Get incoming neighbors
    pub fn get_incoming(&self, id: &str) -> Vec<String> {
        self.get_neighbors(id, Direction::Incoming)
    }

    /// Find nodes by type
    pub fn find_by_type(&self, node_type: NodeType) -> anyhow::Result<Vec<ForgeNode>> {
        let mut results = Vec::new();
        for result in self.nodes_tree.iter() {
            let (_, value) = result?;
            let node: ForgeNode = serde_json::from_slice(&value)?;
            if node.node_type == node_type {
                results.push(node);
            }
        }
        Ok(results)
    }

    /// Find nodes by TETH entropy threshold
    pub fn find_by_entropy(&self, min_entropy: f64) -> anyhow::Result<Vec<ForgeNode>> {
        let mut results = Vec::new();
        for result in self.nodes_tree.iter() {
            let (_, value) = result?;
            let node: ForgeNode = serde_json::from_slice(&value)?;
            if let Some(entropy) = node.teth_entropy {
                if entropy >= min_entropy {
                    results.push(node);
                }
            }
        }
        Ok(results)
    }

    /// Get total node count
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get total edge count
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Flush changes to disk
    pub async fn flush(&self) -> anyhow::Result<()> {
        self.db.flush_async().await?;
        Ok(())
    }

    /// Export graph to DOT format for visualization
    pub fn to_dot(&self) -> String {
        use petgraph::dot::{Dot, Config};
        format!("{:?}", Dot::with_config(&self.graph, &[Config::EdgeNoLabel]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_graph_creation() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test-graph");

        let graph = ForgeGraph::new(path.to_str().unwrap()).await.unwrap();
        assert_eq!(graph.node_count(), 0);
    }

    #[tokio::test]
    async fn test_add_node() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test-graph");

        let mut graph = ForgeGraph::new(path.to_str().unwrap()).await.unwrap();

        let node = ForgeNode::new(NodeType::Tool, "Test Tool");
        graph.add_node(node.clone()).await.unwrap();

        assert_eq!(graph.node_count(), 1);

        let retrieved = graph.get_node(&node.id).unwrap().unwrap();
        assert_eq!(retrieved.label, "Test Tool");
    }
}
