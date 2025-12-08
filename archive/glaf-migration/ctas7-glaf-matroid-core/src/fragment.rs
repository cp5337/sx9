//! Intelligence Fragment representation
//!
//! RFC-9021: Fragment vectors for convergence analysis

use serde::{Deserialize, Serialize};

/// Intelligence fragment (vector/embedding)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fragment {
    /// Fragment ID (corresponds to trivariate hash)
    pub id: u64,
    /// Vector embedding (384-dim per RFC-9021: all-MiniLM-L6-v2)
    pub vector: Vec<f64>,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Timestamp of collection
    pub collected_at: chrono::DateTime<chrono::Utc>,
    /// Source identifier
    pub source: String,
}

impl Fragment {
    /// Create new fragment with given dimension
    pub fn new(id: u64, vector: Vec<f64>, confidence: f64) -> Self {
        Self {
            id,
            vector,
            confidence: confidence.clamp(0.0, 1.0),
            collected_at: chrono::Utc::now(),
            source: String::new(),
        }
    }

    /// Create fragment with source
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = source.into();
        self
    }

    /// Get vector dimension
    pub fn dimension(&self) -> usize {
        self.vector.len()
    }

    /// Check if fragment is within time-of-value window
    pub fn is_valid(&self, hours_threshold: i64) -> bool {
        let threshold = chrono::Duration::hours(hours_threshold);
        let age = chrono::Utc::now() - self.collected_at;
        age < threshold
    }

    /// Calculate dot product with another fragment
    pub fn dot(&self, other: &Fragment) -> f64 {
        self.vector.iter()
            .zip(other.vector.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    /// Calculate L2 norm
    pub fn norm(&self) -> f64 {
        self.vector.iter()
            .map(|x| x * x)
            .sum::<f64>()
            .sqrt()
    }

    /// Calculate cosine similarity with another fragment
    pub fn cosine_similarity(&self, other: &Fragment) -> f64 {
        let dot = self.dot(other);
        let norm_product = self.norm() * other.norm();
        if norm_product > 0.0 {
            dot / norm_product
        } else {
            0.0
        }
    }
}

impl PartialEq for Fragment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Fragment {}

impl std::hash::Hash for Fragment {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fragment_creation() {
        let f = Fragment::new(1, vec![1.0, 0.0, 0.0], 0.9);
        assert_eq!(f.id, 1);
        assert_eq!(f.dimension(), 3);
        assert_eq!(f.confidence, 0.9);
    }

    #[test]
    fn test_cosine_similarity() {
        let f1 = Fragment::new(1, vec![1.0, 0.0, 0.0], 1.0);
        let f2 = Fragment::new(2, vec![1.0, 0.0, 0.0], 1.0);
        let f3 = Fragment::new(3, vec![0.0, 1.0, 0.0], 1.0);

        assert!((f1.cosine_similarity(&f2) - 1.0).abs() < 0.001);
        assert!((f1.cosine_similarity(&f3)).abs() < 0.001);
    }
}
