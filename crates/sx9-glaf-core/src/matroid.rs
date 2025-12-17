//! Latent Matroid Rank Calculations
//!
//! RFC-9023: Measures information independence structure
//! High rank change = new independent information
//! Low rank change = redundant information

use nalgebra::{DMatrix, Vector3};

/// Represents a single intelligence fragment (Vector/Embedding)
#[derive(Debug, Clone)]
pub struct Fragment {
    /// Corresponds to the HASH of the fragment (trivariate hash)
    pub id: u64,

    /// Embedding vector (simplified to 3D for demo, usually 768)
    pub vector: Vector3<f64>,

    /// Source confidence score [0.0, 1.0]
    pub confidence: f64,
}

/// Implements the Matroid Rank function based on linear independence.
pub struct LatentMatroid {
    pub ground_set: Vec<Vector3<f64>>,
}

impl LatentMatroid {
    pub fn new(vectors: Vec<Vector3<f64>>) -> Self {
        Self {
            ground_set: vectors,
        }
    }

    /// Calculates the rank of a subset of fragments.
    pub fn calculate_rank(&self, subset_indices: &[usize]) -> usize {
        if subset_indices.is_empty() {
            return 0;
        }

        // 1. Collect the vectors for the subset
        let vectors: Vec<Vector3<f64>> = subset_indices
            .iter()
            .filter_map(|&i| self.ground_set.get(i).cloned())
            .collect();

        let num_cols = vectors.len();
        if num_cols == 0 {
            return 0;
        }

        // 2. Build matrix
        // nalgebra::DMatrix::from_iterator takes rows, cols, iterator
        // We want columns, so we can build it that way.
        let matrix_data: Vec<f64> = vectors.iter().flat_map(|v| v.iter().copied()).collect();

        // 3 rows (dim), N cols
        let matrix = DMatrix::from_column_slice(3, num_cols, &matrix_data);

        // 3. Compute rank with tolerance
        matrix.rank(1e-6)
    }
}

// Legacy placeholder compatibility (to avoid breaking current calls if any)
// Real implementations should move to LatentMatroid
pub async fn calculate_rank(nodes: &[crate::types::Node]) -> f64 {
    // Basic placeholder logic preserved for compatibility
    let mut unique = std::collections::HashSet::new();
    for n in nodes {
        unique.insert(n.id.clone());
    }
    unique.len() as f64
}

// Required export for lib.rs
pub struct MatroidRank {
    pub previous_rank: f64,
}

pub async fn calculate_rank_delta(previous: f64, current: f64) -> f64 {
    current - previous
}
