//! Hash Extraction Utilities for DSL Operations
//!
//! Provides functions to extract SCH, CUID, and UUID components from
//! operational trivariate hashes and map them to Unicode triggers.

use crate::dsl::{DSLError, DSLResult};
use serde::{Deserialize, Serialize};
use std::ops::Range;

/// Operational hash position ranges (0-indexed)
pub const SCH_RANGE: Range<usize> = 0..16;
pub const CUID_RANGE: Range<usize> = 16..32;
pub const UUID_RANGE: Range<usize> = 32..48;

/// Hash component types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashComponent {
    /// SCH (Positions 0-15): Spatial/Content Hash
    SCH,
    /// CUID (Positions 16-31): Context Unique ID
    CUID,
    /// UUID (Positions 32-47): Universal Unique ID
    UUID,
}

/// Extract SCH (positions 0-15) from operational hash
pub fn extract_sch(operational_hash: &str) -> DSLResult<String> {
    if operational_hash.len() != 48 {
        return Err(DSLError::InvalidHash(format!(
            "Expected 48 characters, got {}",
            operational_hash.len()
        )));
    }
    Ok(operational_hash[SCH_RANGE].to_string())
}

/// Extract CUID (positions 16-31) from operational hash
pub fn extract_cuid(operational_hash: &str) -> DSLResult<String> {
    if operational_hash.len() != 48 {
        return Err(DSLError::InvalidHash(format!(
            "Expected 48 characters, got {}",
            operational_hash.len()
        )));
    }
    Ok(operational_hash[CUID_RANGE].to_string())
}

/// Extract UUID (positions 32-47) from operational hash
pub fn extract_uuid(operational_hash: &str) -> DSLResult<String> {
    if operational_hash.len() != 48 {
        return Err(DSLError::InvalidHash(format!(
            "Expected 48 characters, got {}",
            operational_hash.len()
        )));
    }
    Ok(operational_hash[UUID_RANGE].to_string())
}

/// Map hash component to Unicode trigger for Neural Mux routing
pub fn hash_to_unicode(component_type: HashComponent) -> char {
    match component_type {
        HashComponent::SCH => '\u{E100}',  // Trivariate processor
        HashComponent::CUID => '\u{E200}', // Context processor
        HashComponent::UUID => '\u{E000}', // System controller
    }
}

/// Extract all components from operational hash
pub fn extract_all_components(operational_hash: &str) -> DSLResult<HashComponents> {
    Ok(HashComponents {
        sch: extract_sch(operational_hash)?,
        cuid: extract_cuid(operational_hash)?,
        uuid: extract_uuid(operational_hash)?,
        full_hash: operational_hash.to_string(),
    })
}

/// Container for extracted hash components
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HashComponents {
    pub sch: String,
    pub cuid: String,
    pub uuid: String,
    pub full_hash: String,
}

impl HashComponents {
    /// Get Unicode triggers for all components
    pub fn unicode_triggers(&self) -> [char; 3] {
        [
            hash_to_unicode(HashComponent::SCH),
            hash_to_unicode(HashComponent::CUID),
            hash_to_unicode(HashComponent::UUID),
        ]
    }
}

/// Semantic hash (H₃) structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticHash {
    /// SHA256 of interview content
    pub block_id: String,
    /// Belief Core semantic classification
    pub semantic_hash: String,
    /// Ed25519 signature
    pub auth_sig: String,
}

impl SemanticHash {
    /// Create new semantic hash
    pub fn new(block_id: String, semantic_hash: String, auth_sig: String) -> Self {
        Self {
            block_id,
            semantic_hash,
            auth_sig,
        }
    }

    /// Get Unicode triggers for semantic hash components
    pub fn unicode_triggers(&self) -> [char; 3] {
        [
            '\u{E300}', // Block ID (integrity)
            '\u{E320}', // Semantic (classification)
            '\u{E400}', // Auth sig (verification)
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_extraction() {
        let hash = "3kJ9mP4xQ7R8sN2mK5fH9nL8vC3dF6gH2jK9mP4xQ7R8sN2m";

        assert_eq!(extract_sch(hash).unwrap(), "3kJ9mP4xQ7R8sN2m");
        assert_eq!(extract_cuid(hash).unwrap(), "K5fH9nL8vC3dF6gH");
        assert_eq!(extract_uuid(hash).unwrap(), "2jK9mP4xQ7R8sN2m");
    }

    #[test]
    fn test_invalid_hash_length() {
        let short_hash = "too_short";
        assert!(extract_sch(short_hash).is_err());
        assert!(extract_cuid(short_hash).is_err());
        assert!(extract_uuid(short_hash).is_err());
    }

    #[test]
    fn test_unicode_mapping() {
        assert_eq!(hash_to_unicode(HashComponent::SCH), '\u{E100}');
        assert_eq!(hash_to_unicode(HashComponent::CUID), '\u{E200}');
        assert_eq!(hash_to_unicode(HashComponent::UUID), '\u{E000}');
    }

    #[test]
    fn test_extract_all_components() {
        let hash = "3kJ9mP4xQ7R8sN2mK5fH9nL8vC3dF6gH2jK9mP4xQ7R8sN2m";
        let components = extract_all_components(hash).unwrap();

        assert_eq!(components.sch, "3kJ9mP4xQ7R8sN2m");
        assert_eq!(components.cuid, "K5fH9nL8vC3dF6gH");
        assert_eq!(components.uuid, "2jK9mP4xQ7R8sN2m");
        assert_eq!(components.full_hash, hash);

        let triggers = components.unicode_triggers();
        assert_eq!(triggers[0], '\u{E100}');
        assert_eq!(triggers[1], '\u{E200}');
        assert_eq!(triggers[2], '\u{E000}');
    }

    #[test]
    fn test_semantic_hash() {
        let h3 = SemanticHash::new(
            "a7f3e2c9".to_string(),
            "b8e4c3d1".to_string(),
            "c9f5d4e2".to_string(),
        );

        let triggers = h3.unicode_triggers();
        assert_eq!(triggers[0], '\u{E300}');
        assert_eq!(triggers[1], '\u{E320}');
        assert_eq!(triggers[2], '\u{E400}');
    }
}
