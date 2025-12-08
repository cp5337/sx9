//! Relational Properties - Connectivity, dependencies, interaction matrix
//! 
//! Core relational constraints for cognitive atoms in Universal Cognigraph.

use std::collections::HashMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Relational Properties (from Universal Cognigraph)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationalProperties {
    pub connectivity: u32,          // κ: Connectivity capacity ∈ ℕ
    pub dependencies: Vec<Uuid>,    // δ: Dependency vector
    pub interaction_matrix: HashMap<Uuid, f32>, // ι: Interaction strength matrix
}

impl Default for RelationalProperties {
    fn default() -> Self {
        Self {
            connectivity: 5,
            dependencies: Vec::new(),
            interaction_matrix: HashMap::new(),
        }
    }
}

impl RelationalProperties {
    /// Add dependency relationship
    pub fn add_dependency(&mut self, target_id: Uuid, strength: f32) {
        if !self.dependencies.contains(&target_id) {
            self.dependencies.push(target_id);
        }
        self.interaction_matrix.insert(target_id, strength);
    }
    
    /// Check if at connectivity limit
    pub fn is_at_capacity(&self) -> bool {
        self.dependencies.len() >= self.connectivity as usize
    }
}
