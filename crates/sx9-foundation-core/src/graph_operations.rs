//! Neural Context Engine - Graph Operations Module
//!
//! This module provides core graph manipulation operations including
//! node insertion, edge creation, and graph analysis functions.

use crate::types::{ContextualNode, ContextualNeuralGraph, GraphEdge, EdgeType, NodeType};
use anyhow::{anyhow, Result};
use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

impl ContextualNeuralGraph {
    /// Adds a new node to the graph
    ///
    /// # Arguments
    /// * `node` - The node to add to the graph
    ///
    /// # Returns
    /// * `Result<()>` - Success or error if node already exists
    pub fn add_node(&mut self, mut node: ContextualNode) -> Result<()> {
        if self.nodes.contains_key(&node.id) {
            return Err(anyhow!("Node with ID {} already exists", node.id));
        }

        // Update timestamps
        node.last_accessed = Utc::now();

        // Initialize adjacency list entry
        self.adjacency_list.insert(node.id.clone(), Vec::new());

        // Insert node
        self.nodes.insert(node.id.clone(), node);

        self.update_version();
        Ok(())
    }

    /// Creates and adds an edge between two nodes
    ///
    /// # Arguments
    /// * `source_id` - Source node identifier
    /// * `target_id` - Target node identifier
    /// * `weight` - Edge weight (0.0-1.0)
    /// * `edge_type` - Type of relationship
    ///
    /// # Returns
    /// * `Result<String>` - Edge ID or error
    pub fn add_edge(
        &mut self,
        source_id: &str,
        target_id: &str,
        weight: f32,
        edge_type: EdgeType,
    ) -> Result<String> {
        // Validate nodes exist
        if !self.nodes.contains_key(source_id) {
            return Err(anyhow!("Source node {} not found", source_id));
        }
        if !self.nodes.contains_key(target_id) {
            return Err(anyhow!("Target node {} not found", target_id));
        }

        // Validate weight range
        if !(0.0..=1.0).contains(&weight) {
            return Err(anyhow!("Weight must be between 0.0 and 1.0"));
        }

        let edge_id = Uuid::new_v4().to_string();
        let edge = GraphEdge {
            id: edge_id.clone(),
            source_node: source_id.to_string(),
            target_node: target_id.to_string(),
            weight,
            edge_type,
            created_at: Utc::now(),
            metadata: HashMap::new(),
        };

        // Update adjacency list
        self.adjacency_list
            .entry(source_id.to_string())
            .or_default()
            .push(target_id.to_string());

        // Add edge
        self.edges.insert(edge_id.clone(), edge);

        self.update_version();
        Ok(edge_id)
    }

    /// Finds nodes by type
    ///
    /// # Arguments
    /// * `node_type` - The type of nodes to find
    ///
    /// # Returns
    /// * `Vec<&ContextualNode>` - References to matching nodes
    pub fn find_nodes_by_type(&self, node_type: &NodeType) -> Vec<&ContextualNode> {
        self.nodes
            .values()
            .filter(|node| std::mem::discriminant(&node.node_type) == std::mem::discriminant(node_type))
            .collect()
    }

    /// Gets connected nodes for a given node
    ///
    /// # Arguments
    /// * `node_id` - Node to find connections for
    ///
    /// # Returns
    /// * `Result<Vec<&ContextualNode>>` - Connected nodes or error
    pub fn get_connected_nodes(&self, node_id: &str) -> Result<Vec<&ContextualNode>> {
        let connected_ids = self.adjacency_list
            .get(node_id)
            .ok_or_else(|| anyhow!("Node {} not found", node_id))?;

        let connected_nodes: Vec<&ContextualNode> = connected_ids
            .iter()
            .filter_map(|id| self.nodes.get(id))
            .collect();

        Ok(connected_nodes)
    }

    /// Calculates graph density (edges / max_possible_edges)
    ///
    /// # Returns
    /// * `f32` - Graph density value between 0.0 and 1.0
    pub fn calculate_density(&self) -> f32 {
        let node_count = self.nodes.len() as f32;
        if node_count <= 1.0 {
            return 0.0;
        }

        let max_edges = node_count * (node_count - 1.0);
        let actual_edges = self.edges.len() as f32;

        actual_edges / max_edges
    }

    /// Finds the most connected node (highest degree)
    ///
    /// # Returns
    /// * `Option<&ContextualNode>` - Most connected node if any exist
    pub fn find_hub_node(&self) -> Option<&ContextualNode> {
        self.adjacency_list
            .iter()
            .max_by_key(|(_, connections)| connections.len())
            .and_then(|(node_id, _)| self.nodes.get(node_id))
    }

    /// Updates the graph version and modification timestamp
    fn update_version(&mut self) {
        self.version += 1;
        self.last_modified = Utc::now();
    }

    /// Removes a node and all its edges
    ///
    /// # Arguments
    /// * `node_id` - Node to remove
    ///
    /// # Returns
    /// * `Result<()>` - Success or error if node not found
    pub fn remove_node(&mut self, node_id: &str) -> Result<()> {
        if !self.nodes.contains_key(node_id) {
            return Err(anyhow!("Node {} not found", node_id));
        }

        // Remove all edges involving this node
        let edges_to_remove: Vec<String> = self.edges
            .iter()
            .filter(|(_, edge)| edge.source_node == node_id || edge.target_node == node_id)
            .map(|(edge_id, _)| edge_id.clone())
            .collect();

        for edge_id in edges_to_remove {
            self.edges.remove(&edge_id);
        }

        // Clean up adjacency list
        self.adjacency_list.remove(node_id);
        for connections in self.adjacency_list.values_mut() {
            connections.retain(|id| id != node_id);
        }

        // Remove the node
        self.nodes.remove(node_id);

        self.update_version();
        Ok(())
    }
}