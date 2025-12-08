//! Latent Matroid implementation
//!
//! RFC-9021: Matroid Rank calculation for information independence

use nalgebra::DMatrix;
use crate::fragment::Fragment;

/// Matroid rank result
#[derive(Debug, Clone, Copy)]
pub struct MatroidRank {
    pub rank: usize,
    pub max_possible: usize,
}

impl MatroidRank {
    /// Get normalized rank (0.0 - 1.0)
    pub fn normalized(&self) -> f64 {
        if self.max_possible > 0 {
            self.rank as f64 / self.max_possible as f64
        } else {
            0.0
        }
    }
}

/// Latent Matroid for measuring information independence
///
/// The ground set E is the collection of all fragments.
/// Rank r(S) is the maximum number of linearly independent vectors in S.
pub struct LatentMatroid {
    /// Ground set of fragments
    pub ground_set: Vec<Fragment>,
    /// Vector dimension
    dimension: usize,
    /// Numerical tolerance for rank calculation
    tolerance: f64,
}

impl LatentMatroid {
    /// Create new matroid from fragments
    pub fn new(fragments: Vec<Fragment>, dimension: usize) -> Self {
        Self {
            ground_set: fragments,
            dimension,
            tolerance: 1e-6,
        }
    }

    /// Set numerical tolerance for rank calculation
    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Calculate rank of a subset of fragments
    ///
    /// Rank r(S) = maximum number of linearly independent vectors in S.
    /// This measures Information Independence (RFC-9021 §3.3).
    /// Runs in Zone C (Analytical) time window.
    pub fn calculate_rank(&self, subset_indices: &[usize]) -> MatroidRank {
        if subset_indices.is_empty() {
            return MatroidRank { rank: 0, max_possible: self.ground_set.len() };
        }

        // Collect vectors for the subset
        let vectors: Vec<&Vec<f64>> = subset_indices.iter()
            .filter_map(|&i| self.ground_set.get(i).map(|f| &f.vector))
            .collect();

        if vectors.is_empty() {
            return MatroidRank { rank: 0, max_possible: self.ground_set.len() };
        }

        // Build matrix where each column is a fragment vector
        let num_cols = vectors.len();
        let num_rows = vectors[0].len();

        let matrix_data: Vec<f64> = vectors.into_iter()
            .flat_map(|v| v.iter().copied())
            .collect();

        let matrix = DMatrix::from_column_slice(num_rows, num_cols, &matrix_data);

        // Compute rank (number of independent columns)
        let rank = matrix.rank(self.tolerance);

        MatroidRank {
            rank,
            max_possible: self.ground_set.len(),
        }
    }

    /// Calculate normalized rank for a subset
    pub fn normalized_rank(&self, subset_indices: &[usize]) -> f64 {
        self.calculate_rank(subset_indices).normalized()
    }

    /// Measure change in information independence when adding a fragment
    ///
    /// High rank delta = high H2 contribution
    pub fn rank_delta(&self, existing_indices: &[usize], new_index: usize) -> usize {
        let old_rank = self.calculate_rank(existing_indices).rank;

        let mut new_indices = existing_indices.to_vec();
        if !new_indices.contains(&new_index) {
            new_indices.push(new_index);
        }

        let new_rank = self.calculate_rank(&new_indices).rank;

        new_rank.saturating_sub(old_rank)
    }

    /// Check if adding a fragment increases independence
    pub fn is_independent(&self, existing_indices: &[usize], new_index: usize) -> bool {
        self.rank_delta(existing_indices, new_index) > 0
    }

    /// Get ground set size
    pub fn size(&self) -> usize {
        self.ground_set.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_fragments() -> Vec<Fragment> {
        vec![
            Fragment::new(1, vec![1.0, 0.0, 0.0], 0.9),
            Fragment::new(2, vec![0.0, 1.0, 0.0], 0.8),
            Fragment::new(3, vec![0.0, 0.0, 1.0], 0.7),
            Fragment::new(4, vec![1.0, 1.0, 0.0], 0.6), // Linear combination of 1 and 2
        ]
    }

    #[test]
    fn test_matroid_rank_independent() {
        let fragments = make_fragments();
        let matroid = LatentMatroid::new(fragments, 3);

        // Three orthogonal vectors should have rank 3
        let rank = matroid.calculate_rank(&[0, 1, 2]);
        assert_eq!(rank.rank, 3);
    }

    #[test]
    fn test_matroid_rank_dependent() {
        let fragments = make_fragments();
        let matroid = LatentMatroid::new(fragments, 3);

        // Fragment 4 is a linear combination of 1 and 2
        let rank = matroid.calculate_rank(&[0, 1, 3]);
        assert_eq!(rank.rank, 2);
    }

    #[test]
    fn test_rank_delta() {
        let fragments = make_fragments();
        let matroid = LatentMatroid::new(fragments, 3);

        // Adding orthogonal vector should increase rank
        let delta = matroid.rank_delta(&[0], 1);
        assert_eq!(delta, 1);

        // Adding dependent vector should not increase rank
        let delta = matroid.rank_delta(&[0, 1], 3);
        assert_eq!(delta, 0);
    }
}
