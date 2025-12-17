//! GLAF Core Graph Engine
//!
//! Provides graph storage and query interface for neural operations

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::types::{apply_node_changes, Edge, Node, NodeChange};

/// GLAF core graph engine
#[derive(Debug)]
pub struct GLAFCore {
    // We use a Vec for ordered iteration (like xyflow layers) and a Map could be added for O(1) if needed.
    // For now, keeping it simple with RwLock<Vec<Node>> as per the "apply_node_changes" pattern which works on lists.
    // In production, we might want a dual structure (Vec + Map).
    nodes: Arc<RwLock<Vec<Node>>>,
    edges: Arc<RwLock<Vec<Edge>>>,
}

impl GLAFCore {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(Vec::new())),
            edges: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Apply a batch of node changes
    pub async fn apply_changes(&self, changes: Vec<NodeChange>) {
        let mut nodes = self.nodes.write().await;
        apply_node_changes(changes, &mut nodes);
    }

    /// Get all nodes (snapshot)
    pub async fn get_all_nodes(&self) -> Vec<Node> {
        let nodes = self.nodes.read().await;
        nodes.clone()
    }

    /// Get all edges (snapshot)
    pub async fn get_edges(&self) -> Vec<Edge> {
        let edges = self.edges.read().await;
        edges.clone()
    }

    pub async fn get_activity_sequence(&self) -> Vec<String> {
        let nodes = self.nodes.read().await;
        nodes
            .iter()
            .filter_map(|n| {
                n.data
                    .get("activity")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .collect()
    }

    /// Legacy compatibility (optional, or we can remove GlafNode later)
    /// but for now let's just use the new types.

    pub async fn add_node(&self, node: Node) {
        let mut nodes = self.nodes.write().await;
        // Check if exists
        if !nodes.iter().any(|n| n.id == node.id) {
            nodes.push(node);
        }
    }

    pub async fn add_edge(&self, edge: Edge) {
        let mut edges = self.edges.write().await;
        edges.push(edge);
    }
}

impl Default for GLAFCore {
    fn default() -> Self {
        Self::new()
    }
}
