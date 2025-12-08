//! CTAS-7 GLAF Matroid Core - Convergence Math
//!
//! RFC-9021: GLAF Matroid Specification
//!
//! Core Element: Convergence Math
//! Implementation: ctas7-glaf-matroid-core
//!
//! Zone C - Analytical processing for graph-based convergence detection
//! and matroid theory implementation.

pub mod matroid;
pub mod optimizer;
pub mod fragment;
pub mod hawkes;

pub use matroid::{LatentMatroid, MatroidRank};
pub use optimizer::{CombinatorialOptimizer, AssignmentResult};
pub use fragment::Fragment;
pub use hawkes::HawkesProcess;

use serde::{Deserialize, Serialize};

/// GLAF Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlafConfig {
    /// Time-of-value threshold in hours (default: 24)
    pub time_of_value_hours: i64,
    /// Convergence objective weight (default: 0.6)
    pub convergence_weight: f64,
    /// Matroid rank weight (default: 0.4)
    pub matroid_weight: f64,
    /// Vector dimension for fragments (384 per RFC-9021: all-MiniLM-L6-v2)
    pub vector_dimension: usize,
    /// Numerical tolerance for rank calculation
    pub rank_tolerance: f64,
}

impl Default for GlafConfig {
    fn default() -> Self {
        Self {
            time_of_value_hours: 24,
            convergence_weight: 0.6,
            matroid_weight: 0.4,
            vector_dimension: 384, // RFC-9021: all-MiniLM-L6-v2
            rank_tolerance: 1e-6,
        }
    }
}

/// Convergence score result
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ConvergenceResult {
    /// H1 - Operational convergence (fast excitatory)
    pub h1_operational: f64,
    /// H2 - Semantic convergence (slow modulatory)
    pub h2_semantic: f64,
    /// Combined weighted score
    pub combined: f64,
}

impl ConvergenceResult {
    pub fn new(h1: f64, h2: f64, config: &GlafConfig) -> Self {
        let combined = h1 * config.convergence_weight + h2 * config.matroid_weight;
        Self {
            h1_operational: h1,
            h2_semantic: h2,
            combined,
        }
    }

    /// Check if convergence exceeds threshold for phase transition
    pub fn exceeds_threshold(&self, threshold: f64) -> bool {
        self.combined > threshold
    }
}

/// GLAF Analysis Engine
pub struct GlafEngine {
    config: GlafConfig,
    matroid: Option<LatentMatroid>,
}

impl GlafEngine {
    pub fn new(config: GlafConfig) -> Self {
        Self {
            config,
            matroid: None,
        }
    }

    /// Initialize matroid from fragment pool
    pub fn initialize_matroid(&mut self, fragments: Vec<Fragment>) {
        self.matroid = Some(LatentMatroid::new(fragments, self.config.vector_dimension));
    }

    /// Calculate convergence for given subset
    pub fn calculate_convergence(&self, fragment_indices: &[usize], h1_input: f64) -> ConvergenceResult {
        let h2 = self.matroid.as_ref()
            .map(|m| m.normalized_rank(fragment_indices))
            .unwrap_or(0.0);

        ConvergenceResult::new(h1_input, h2, &self.config)
    }

    /// Get configuration
    pub fn config(&self) -> &GlafConfig {
        &self.config
    }
}

impl Default for GlafEngine {
    fn default() -> Self {
        Self::new(GlafConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convergence_result() {
        let config = GlafConfig::default();
        let result = ConvergenceResult::new(0.8, 0.6, &config);
        assert!(result.h1_operational > 0.0);
        assert!(result.h2_semantic > 0.0);
        assert!(result.combined > 0.0);
    }

    #[test]
    fn test_glaf_engine_creation() {
        let engine = GlafEngine::default();
        assert_eq!(engine.config().vector_dimension, 384); // RFC-9021
    }
}
