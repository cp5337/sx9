//! Knowledge Graph - Integrates with GLAF Core
//!
//! Provides graph storage for OSINT entities and relationships

use crate::{config::GraphConfig, LeptoseError, Result};
use chrono::{DateTime, Utc};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef; // Import EdgeRef trait for source()/target() methods
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// OSINT node types (mirrors Python GNN OSINT)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NodeType {
    Entity,          // Person, organization, location
    Document,        // Reports, articles, files
    Event,           // Incidents, activities, timeline events
    Indicator,       // IOCs, TTPs, observable patterns
    Infrastructure,  // Domains, IPs, servers
    Campaign,        // Operations, threat campaigns
    Actor,           // Threat actors, groups
    Vulnerability,   // CVEs, exploits
    Artifact,        // Files, malware, tools
    EEI,             // Essential Element of Information
    AttackTechnique, // MITRE ATT&CK technique
}

/// Relationship types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RelationType {
    AssociatedWith,
    BelongsTo,
    CommunicatesWith,
    Uses,
    Targets,
    AttributedTo,
    Exploits,
    Hosts,
    References,
    TemporallyRelated,
    GeographicallyRelated,
    Satisfies, // EEI satisfaction
    Answers,   // EEI answer
}

/// Knowledge node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    pub id: Uuid,
    pub node_type: NodeType,
    pub name: String,
    pub description: String,
    pub properties: HashMap<String, serde_json::Value>,
    pub confidence: f64,
    pub source: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Dual trivariate hash (h1: semantic, h2: operational)
    pub hash_h1: Option<u64>,
    pub hash_h2: Option<u64>,
    /// TETH entropy score
    pub entropy: f64,
    /// Vector embedding (if computed)
    #[serde(skip)]
    pub embedding: Option<Vec<f32>>,
}

/// Knowledge edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEdge {
    pub relation_type: RelationType,
    pub weight: f64,
    pub confidence: f64,
    pub evidence: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Knowledge Graph
pub struct KnowledgeGraph {
    config: GraphConfig,
    graph: Arc<RwLock<DiGraph<KnowledgeNode, KnowledgeEdge>>>,
    node_index: Arc<RwLock<HashMap<Uuid, NodeIndex>>>,
    type_index: Arc<RwLock<HashMap<NodeType, Vec<Uuid>>>>,
}

impl KnowledgeGraph {
    pub fn new(config: GraphConfig) -> Self {
        Self {
            config,
            graph: Arc::new(RwLock::new(DiGraph::new())),
            node_index: Arc::new(RwLock::new(HashMap::new())),
            type_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add a node to the graph
    pub async fn add_node(&self, node: KnowledgeNode) -> Result<Uuid> {
        let id = node.id;
        let node_type = node.node_type.clone();

        let mut graph = self.graph.write().await;
        let mut index = self.node_index.write().await;
        let mut type_idx = self.type_index.write().await;

        // Check if already exists
        if index.contains_key(&id) {
            return Err(LeptoseError::GraphError(format!(
                "Node {} already exists",
                id
            )));
        }

        // Add to graph
        let node_idx = graph.add_node(node);
        index.insert(id, node_idx);

        // Update type index
        type_idx.entry(node_type).or_default().push(id);

        Ok(id)
    }

    /// Add an edge between nodes
    pub async fn add_edge(&self, from_id: Uuid, to_id: Uuid, edge: KnowledgeEdge) -> Result<()> {
        let graph_guard = self.graph.write().await;
        let index = self.node_index.read().await;

        let from_idx = index
            .get(&from_id)
            .ok_or_else(|| LeptoseError::GraphError(format!("Node {} not found", from_id)))?;
        let to_idx = index
            .get(&to_id)
            .ok_or_else(|| LeptoseError::GraphError(format!("Node {} not found", to_id)))?;

        drop(graph_guard);
        drop(index);

        let mut graph = self.graph.write().await;
        let index = self.node_index.read().await;

        let from_idx = *index.get(&from_id).unwrap();
        let to_idx = *index.get(&to_id).unwrap();

        graph.add_edge(from_idx, to_idx, edge);
        Ok(())
    }

    /// Get node by ID
    pub async fn get_node(&self, id: Uuid) -> Option<KnowledgeNode> {
        let graph = self.graph.read().await;
        let index = self.node_index.read().await;

        index.get(&id).map(|idx| graph[*idx].clone())
    }

    /// Get nodes by type
    pub async fn get_nodes_by_type(&self, node_type: NodeType) -> Vec<KnowledgeNode> {
        let graph = self.graph.read().await;
        let index = self.node_index.read().await;
        let type_idx = self.type_index.read().await;

        type_idx
            .get(&node_type)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| index.get(id).map(|idx| graph[*idx].clone()))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Calculate TETH entropy for the entire graph
    pub async fn calculate_graph_entropy(&self) -> f64 {
        let graph = self.graph.read().await;

        if graph.node_count() == 0 {
            return 0.0;
        }

        // Calculate total degree
        let total_degree: f64 = graph
            .node_indices()
            .map(|idx| {
                (graph
                    .edges_directed(idx, petgraph::Direction::Outgoing)
                    .count()
                    + graph
                        .edges_directed(idx, petgraph::Direction::Incoming)
                        .count()) as f64
            })
            .sum();

        if total_degree == 0.0 {
            return 0.0;
        }

        // Calculate Shannon entropy
        let mut entropy = 0.0;
        for idx in graph.node_indices() {
            let degree = (graph
                .edges_directed(idx, petgraph::Direction::Outgoing)
                .count()
                + graph
                    .edges_directed(idx, petgraph::Direction::Incoming)
                    .count()) as f64;

            if degree > 0.0 {
                let p = degree / total_degree;
                entropy -= p * p.ln();
            }
        }

        entropy
    }

    /// Find nodes related to an EEI
    pub async fn find_eei_satisfiers(&self, eei_id: Uuid) -> Vec<KnowledgeNode> {
        let graph = self.graph.read().await;
        let index = self.node_index.read().await;

        let Some(eei_idx) = index.get(&eei_id) else {
            return vec![];
        };

        // Find all nodes connected via Satisfies or Answers relations
        let mut satisfiers = vec![];
        for edge_ref in graph.edges_directed(*eei_idx, petgraph::Direction::Incoming) {
            let edge_data = edge_ref.weight();
            if matches!(
                edge_data.relation_type,
                RelationType::Satisfies | RelationType::Answers
            ) {
                // Get source node using edge_ref.source() which returns NodeIndex
                let source_idx = edge_ref.source();
                satisfiers.push(graph[source_idx].clone());
            }
        }

        satisfiers
    }

    /// Export graph statistics
    pub async fn stats(&self) -> GraphStats {
        let graph = self.graph.read().await;
        let type_idx = self.type_index.read().await;

        let mut type_counts = HashMap::new();
        for (node_type, ids) in type_idx.iter() {
            type_counts.insert(format!("{:?}", node_type), ids.len());
        }

        GraphStats {
            total_nodes: graph.node_count(),
            total_edges: graph.edge_count(),
            type_counts,
            entropy: drop(graph),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphStats {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub type_counts: HashMap<String, usize>,
    pub entropy: (),
}

impl Default for KnowledgeGraph {
    fn default() -> Self {
        Self::new(GraphConfig::default())
    }
}

impl Default for GraphConfig {
    fn default() -> Self {
        Self {
            enable_glaf: true,
            teth_threshold: 0.5,
            max_nodes: 100_000,
            relationship_types: vec![],
        }
    }
}
