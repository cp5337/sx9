//! GLAF Core Graph Engine
//!
//! Provides graph storage and query interface for neural operations

use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// GLAF graph node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlafNode {
    pub id: String,
    pub labels: Vec<String>,
    pub properties: HashMap<String, Value>,
}

/// GLAF graph relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlafRelationship {
    pub id: String,
    pub from: String,
    pub to: String,
    pub rel_type: String,
    pub properties: HashMap<String, Value>,
}

/// GLAF core graph engine
pub struct GLAFCore {
    nodes: Arc<RwLock<HashMap<String, GlafNode>>>,
    relationships: Arc<RwLock<Vec<GlafRelationship>>>,
}

impl GLAFCore {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            relationships: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_node(&self, node: GlafNode) {
        let mut nodes = self.nodes.write().await;
        nodes.insert(node.id.clone(), node);
    }

    pub async fn add_relationship(&self, rel: GlafRelationship) {
        let mut rels = self.relationships.write().await;
        rels.push(rel);
    }

    pub async fn get_node(&self, id: &str) -> Option<GlafNode> {
        let nodes = self.nodes.read().await;
        nodes.get(id).cloned()
    }

    pub async fn get_all_nodes(&self) -> Vec<GlafNode> {
        let nodes = self.nodes.read().await;
        nodes.values().cloned().collect()
    }

    pub async fn get_activity_sequence(&self) -> Vec<String> {
        // Extract activity sequence from nodes
        let nodes = self.nodes.read().await;
        nodes.values()
            .filter_map(|n| {
                n.properties.get("activity")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            })
            .collect()
    }

    pub async fn get_relationships(&self) -> Vec<GlafRelationship> {
        let rels = self.relationships.read().await;
        rels.clone()
    }
}

impl Default for GLAFCore {
    fn default() -> Self {
        Self::new()
    }
}

