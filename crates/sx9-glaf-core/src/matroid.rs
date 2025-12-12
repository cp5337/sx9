//! Latent Matroid Rank Calculations
//!
//! RFC-9021: Measures information independence structure
//! High rank change = new independent information
//! Low rank change = redundant information

use crate::glaf_core::GlafNode;
use anyhow::Result;

/// Matroid rank calculator
pub struct MatroidRank {
    previous_rank: f64,
}

impl MatroidRank {
    pub fn new() -> Self {
        Self { previous_rank: 0.0 }
    }

    /// Calculate rank for a set of nodes
    ///
    /// Rank = max |X| where X ⊆ S and X is independent
    pub fn calculate_rank(&mut self, nodes: &[GlafNode]) -> f64 {
        // Simplified: rank is number of unique information sources
        // Count nodes with unique properties
        let mut unique_sources = std::collections::HashSet::new();

        for node in nodes {
            // Use node ID + primary label as uniqueness key
            let key = format!(
                "{}:{}",
                node.id,
                node.labels.first().unwrap_or(&"Unknown".to_string())
            );
            unique_sources.insert(key);
        }

        let rank = unique_sources.len() as f64;
        self.previous_rank = rank;
        rank
    }

    /// Calculate rank delta (change from previous calculation)
    pub fn calculate_rank_delta(&self, current_rank: f64) -> f64 {
        current_rank - self.previous_rank
    }
}

/// Calculate rank for a set of nodes
pub async fn calculate_rank(nodes: &[GlafNode]) -> f64 {
    let mut calculator = MatroidRank::new();
    calculator.calculate_rank(nodes)
}

/// Calculate rank delta
pub async fn calculate_rank_delta(previous_rank: f64, current_rank: f64) -> f64 {
    current_rank - previous_rank
}
