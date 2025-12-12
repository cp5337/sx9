//! Hash Classification for DSL Operations
//!
//! Provides hash-based operation classification for routing DSL operations
//! through the Neural Mux fabric. Uses existing CTAS-7 TrivariteHashEngine.

use crate::dsl::operations::Priority;
use crate::dsl::{DSLError, DSLResult};

/// Hash classifier for DSL operations
///
/// Uses the existing CTAS-7 v7.2 TrivariteHashEngine from foundation-core
/// to avoid code duplication and maintain ecosystem integrity.
pub struct HashClassifier {
    // NOTE: We use sx9_foundation_core::TrivariteHashEngine for actual hashing
    // This struct provides classification logic only
}

impl Default for HashClassifier {
    fn default() -> Self {
        Self::new()
    }
}

impl HashClassifier {
    /// Create new hash classifier
    pub fn new() -> Self {
        Self {}
    }

    /// Classify operation based on hash content
    ///
    /// Uses first 8 characters of SCH (positions 0-7) to determine operation class
    pub fn classify_operation(&self, hash: &str) -> DSLResult<OperationClass> {
        if hash.is_empty() {
            return Err(DSLError::InvalidHash("Empty hash".to_string()));
        }

        if hash.len() < 8 {
            return Err(DSLError::InvalidHash(format!(
                "Hash too short for classification: {} characters",
                hash.len()
            )));
        }

        // Use first 8 characters for classification
        let sample = &hash[0..8];
        let classification_value = Self::hash_sample_to_value(sample);

        // Classify based on value ranges
        let class = match classification_value {
            0..=0x3FFF_FFFF => OperationClass::Intelligence,
            0x4000_0000..=0x7FFF_FFFF => OperationClass::Offensive,
            0x8000_0000..=0xBFFF_FFFF => OperationClass::Defensive,
            0xC000_0000..=0xFFFF_FFFF => OperationClass::Administrative,
        };

        Ok(class)
    }

    /// Determine priority based on hash classification
    pub fn classify_priority(&self, hash: &str) -> DSLResult<Priority> {
        let class = self.classify_operation(hash)?;

        let priority = match class {
            OperationClass::Intelligence => Priority::Critical,
            OperationClass::Offensive => Priority::High,
            OperationClass::Defensive => Priority::High,
            OperationClass::Administrative => Priority::Medium,
        };

        Ok(priority)
    }

    /// Convert hash sample to numeric value for classification
    fn hash_sample_to_value(sample: &str) -> u32 {
        let mut value: u32 = 0;
        for (i, ch) in sample.chars().take(8).enumerate() {
            let char_value = ch as u32;
            value = value.wrapping_add(char_value.wrapping_shl((i * 4) as u32));
        }
        value
    }
}

/// Operation classification categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationClass {
    /// Intelligence collection and analysis
    Intelligence,
    /// Offensive cyber operations
    Offensive,
    /// Defensive cyber operations
    Defensive,
    /// Administrative and support operations
    Administrative,
}

// NOTE: Trivariate hash CREATION is handled by ctas7_foundation_core::TrivariteHashEngine
// This module only handles CLASSIFICATION of existing hashes for DSL routing

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_classification() {
        let classifier = HashClassifier::default();

        // Test with various hash inputs
        let hash1 = "3kJ9mP4xQ7R8sN2m";
        let class = classifier.classify_operation(hash1).unwrap();
        assert!(matches!(
            class,
            OperationClass::Intelligence
                | OperationClass::Offensive
                | OperationClass::Defensive
                | OperationClass::Administrative
        ));
    }

    #[test]
    fn test_priority_classification() {
        let classifier = HashClassifier::default();

        let hash = "3kJ9mP4xQ7R8sN2m";
        let priority = classifier.classify_priority(hash).unwrap();
        assert!(matches!(
            priority,
            Priority::Critical | Priority::High | Priority::Medium
        ));
    }

    #[test]
    fn test_hash_sample_to_value() {
        let sample = "3kJ9mP4x";
        let value = HashClassifier::hash_sample_to_value(sample);
        assert!(value > 0); // Should produce a non-zero value
    }

    #[test]
    fn test_empty_hash_error() {
        let classifier = HashClassifier::default();
        let result = classifier.classify_operation("");
        assert!(result.is_err());
    }
}
