//! Combinatorial Optimizer
//!
//! RFC-9021: GLAF APOC++ greedy optimization procedure

use crate::fragment::Fragment;
use crate::matroid::LatentMatroid;
use crate::GlafConfig;
use serde::{Deserialize, Serialize};

/// Result of optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignmentResult {
    /// IDs of fragments assigned to maximize score
    pub assigned_fragments: Vec<u64>,
    /// Maximum convergence score achieved
    pub max_convergence_score: f64,
    /// Number of fragments used
    pub fragments_used_count: usize,
    /// Optimization iterations performed
    pub iterations: usize,
}

/// Combinatorial optimizer for fragment selection
pub struct CombinatorialOptimizer<'a> {
    fragment_pool: &'a [Fragment],
    matroid: &'a LatentMatroid,
    config: GlafConfig,
}

impl<'a> CombinatorialOptimizer<'a> {
    /// Create new optimizer
    pub fn new(fragment_pool: &'a [Fragment], matroid: &'a LatentMatroid, config: GlafConfig) -> Self {
        Self {
            fragment_pool,
            matroid,
            config,
        }
    }

    /// Implements the GLAF APOC++ greedy optimization procedure (RFC-9021 §3.1)
    ///
    /// Goal: Find subset of fragments maximizing weighted convergence score
    /// Score = Fragment.confidence * Convergence_Weight + Matroid_Rank * Rank_Weight
    /// Constraint: Fragments must be within time-of-value threshold
    pub fn greedy_optimize(&self) -> AssignmentResult {
        let mut best_score = 0.0;
        let mut current_indices: Vec<usize> = Vec::new();
        let mut iterations = 0;

        // 1. Filter fragments based on time-of-value decay
        let relevant_fragments: Vec<(usize, &Fragment)> = self.fragment_pool.iter()
            .enumerate()
            .filter(|(_, f)| f.is_valid(self.config.time_of_value_hours))
            .collect();

        // 2. Greedy selection - iterate through fragments selecting highest marginal gain
        for (index, fragment) in &relevant_fragments {
            iterations += 1;

            // Calculate potential H2 (Semantic) gain from this fragment
            let rank_gain = self.matroid.rank_delta(&current_indices, *index);

            // Calculate total marginal objective gain
            let marginal_gain = (fragment.confidence * self.config.convergence_weight)
                + (rank_gain as f64 * self.config.matroid_weight);

            // Decision: Commit if marginal gain is positive
            let next_score = best_score + marginal_gain;

            if next_score > best_score {
                best_score = next_score;
                current_indices.push(*index);
            }
        }

        // 3. Generate final result
        let final_fragments: Vec<u64> = current_indices.iter()
            .filter_map(|&i| self.fragment_pool.get(i).map(|f| f.id))
            .collect();

        AssignmentResult {
            assigned_fragments: final_fragments.clone(),
            max_convergence_score: best_score,
            fragments_used_count: final_fragments.len(),
            iterations,
        }
    }

    /// Optimize with maximum fragment count constraint
    pub fn greedy_optimize_bounded(&self, max_fragments: usize) -> AssignmentResult {
        let mut best_score = 0.0;
        let mut current_indices: Vec<usize> = Vec::new();
        let mut iterations = 0;

        let relevant_fragments: Vec<(usize, &Fragment)> = self.fragment_pool.iter()
            .enumerate()
            .filter(|(_, f)| f.is_valid(self.config.time_of_value_hours))
            .collect();

        for (index, fragment) in &relevant_fragments {
            if current_indices.len() >= max_fragments {
                break;
            }

            iterations += 1;

            let rank_gain = self.matroid.rank_delta(&current_indices, *index);
            let marginal_gain = (fragment.confidence * self.config.convergence_weight)
                + (rank_gain as f64 * self.config.matroid_weight);

            let next_score = best_score + marginal_gain;

            if next_score > best_score {
                best_score = next_score;
                current_indices.push(*index);
            }
        }

        let final_fragments: Vec<u64> = current_indices.iter()
            .filter_map(|&i| self.fragment_pool.get(i).map(|f| f.id))
            .collect();

        AssignmentResult {
            assigned_fragments: final_fragments.clone(),
            max_convergence_score: best_score,
            fragments_used_count: final_fragments.len(),
            iterations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_fragments() -> Vec<Fragment> {
        vec![
            Fragment::new(1, vec![1.0, 0.0, 0.0], 0.9),
            Fragment::new(2, vec![0.0, 1.0, 0.0], 0.8),
            Fragment::new(3, vec![0.0, 0.0, 1.0], 0.7),
        ]
    }

    #[test]
    fn test_greedy_optimize() {
        let fragments = make_test_fragments();
        let matroid = LatentMatroid::new(fragments.clone(), 3);
        let config = GlafConfig::default();
        let optimizer = CombinatorialOptimizer::new(&fragments, &matroid, config);

        let result = optimizer.greedy_optimize();
        assert!(!result.assigned_fragments.is_empty());
        assert!(result.max_convergence_score > 0.0);
    }

    #[test]
    fn test_bounded_optimize() {
        let fragments = make_test_fragments();
        let matroid = LatentMatroid::new(fragments.clone(), 3);
        let config = GlafConfig::default();
        let optimizer = CombinatorialOptimizer::new(&fragments, &matroid, config);

        let result = optimizer.greedy_optimize_bounded(2);
        assert!(result.fragments_used_count <= 2);
    }
}
