//! Matroid Convergence System (RFC-9023)
//!
//! Implements "Latent Matroid" theory for Information Independence.
//! Used to calculate H2 Scores (Information Density) for cognitive signals.
//!
//! Ground Truth: "A signal is only valuable if it is independent from existing knowledge."

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Latent Matroid structure representing a subspace of information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatentMatroid {
    pub dimension: usize,
    pub basis_vectors: Vec<Vec<f64>>,
    pub independence_threshold: f64,
}

impl LatentMatroid {
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            basis_vectors: Vec::new(),
            independence_threshold: 0.1, // Minimum angle difference for independence
        }
    }

    /// Calculate H2 Score (Information Independence/Density)
    /// Returns 0.0 (Redundant) to 1.0 (Completely Orthogonal/New)
    pub fn calculate_h2_score(&self, input_vector: &[f64]) -> f64 {
        if self.basis_vectors.is_empty() {
            return 1.0; // First piece of info is always 100% new
        }

        // Simplified projection logic (Gram-Schmidt inspired)
        // In a real system, this would use biological neural manifolds

        let max_similarity = self
            .basis_vectors
            .iter()
            .map(|basis| self.cosine_similarity(basis, input_vector))
            .fold(0.0f64, |a, b| a.max(b));

        // H2 Score is inverse of max similarity
        (1.0 - max_similarity).max(0.0)
    }

    /// Add vector to basis if independent enough
    pub fn assimilate(&mut self, input_vector: Vec<f64>) -> bool {
        let h2 = self.calculate_h2_score(&input_vector);
        if h2 > self.independence_threshold {
            self.basis_vectors.push(input_vector);
            return true;
        }
        false
    }

    fn cosine_similarity(&self, a: &[f64], b: &[f64]) -> f64 {
        let dot_product: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }
}

/// H2 Convergence Tracker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceTracker {
    pub global_h2: f64,
    pub iteration: u64,
}
