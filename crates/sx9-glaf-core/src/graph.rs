use petgraph::graph::{Graph, NodeIndex};
use petgraph::Directed;
use std::collections::HashMap;
use crate::trivariate::TrivariateHash;
use serde::{Deserialize, Serialize};

/// Node stored in the Petgraph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlaNode {
    pub id: String,
    pub trivariate: TrivariateHash,
    pub data: serde_json::Value,
    pub label: String,
}

/// Edge stored in the Petgraph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlaEdge {
    pub relation: String,
    pub weight: f64,
}

/// The High-Performance Analytical Graph Engine
pub struct GraphEngine {
    /// The core petgraph structure
    graph: Graph<GlaNode, GlaEdge, Directed>,
    
    /// Fast lookup by H1 (Identity) Hash
    h1_index: HashMap<String, NodeIndex>,
    
    /// Fast lookup by H3 (Crystal) Hash - for "Crystal Planes"
    h3_index: HashMap<String, Vec<NodeIndex>>,
}

impl GraphEngine {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            h1_index: HashMap::new(),
            h3_index: HashMap::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: GlaNode) -> NodeIndex {
        // Index update
        let h1 = node.trivariate.h1.clone();
        let h3 = node.trivariate.h3.clone();
        
        // Add to graph
        let idx = self.graph.add_node(node);
        
        // Update indices
        self.h1_index.insert(h1, idx);
        self.h3_index.entry(h3).or_insert_with(Vec::new).push(idx);
        
        idx
    }

    /// Connect two nodes (H2 Execution Edge)
    pub fn add_edge(&mut self, source_h1: &str, target_h1: &str, relation: &str, weight: f64) -> Option<()> {
        let source_idx = *self.h1_index.get(source_h1)?;
        let target_idx = *self.h1_index.get(target_h1)?;
        
        self.graph.add_edge(source_idx, target_idx, GlaEdge {
            relation: relation.to_string(),
            weight,
        });
        
        Some(())
    }

    /// "Cut" the graph: Retrieve all nodes in a specific Crystal (H3)
    pub fn get_crystal_plane(&self, h3_hash: &str) -> Vec<&GlaNode> {
        if let Some(indices) = self.h3_index.get(h3_hash) {
            indices.iter().map(|&idx| &self.graph[idx]).collect()
        } else {
            Vec::new()
        }
    }

    /// Node Count
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }
}
