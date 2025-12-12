//! Unicode Operation Emitter for DSL Operations
//!
//! Emits Unicode operations from DSL operations for Neural Mux routing.
//! Provides the bridge between DSL parser and Neural Mux execution engine.

use crate::dsl::hash_extractor::{extract_all_components, hash_to_unicode, HashComponent};
use crate::dsl::operations::DSLOperation;
use crate::dsl::unicode_registry::DSLUnicodeRegistry;
use crate::dsl::{DSLError, DSLResult};

/// Unicode operation emitter
pub struct UnicodeEmitter {
    registry: DSLUnicodeRegistry,
}

impl UnicodeEmitter {
    /// Create new Unicode emitter
    pub fn new() -> Self {
        Self {
            registry: DSLUnicodeRegistry::new(),
        }
    }

    /// Emit Unicode operations from DSL operation
    pub fn emit_unicode_operation(&self, dsl_op: &DSLOperation) -> Vec<char> {
        self.registry.operation_to_unicode(dsl_op)
    }

    /// Emit primary Unicode trigger for DSL operation
    pub fn emit_primary_unicode(&self, dsl_op: &DSLOperation) -> char {
        self.registry.operation_to_primary_unicode(dsl_op)
    }

    /// Emit Unicode operations from operational hash
    /// Emit Unicode operations from operational hash
    pub fn bridge_to_legacy(
        &self,
        operational_hash: &str,
    ) -> Result<Vec<char>, Box<dyn std::error::Error>> {
        let _components = extract_all_components(operational_hash)?;

        Ok(vec![
            hash_to_unicode(HashComponent::SCH),
            hash_to_unicode(HashComponent::CUID),
            hash_to_unicode(HashComponent::UUID),
        ])
    }

    /// Helper to emit from hash string
    pub fn emit_from_hash(&self, hash: &str) -> DSLResult<Vec<char>> {
        // Validation via extraction
        let _ = extract_all_components(hash)
            .map_err(|e| DSLError::HashExtractionFailed(e.to_string()))?;

        // Return standard hash unicode triggers
        Ok(vec!['\u{E100}', '\u{E200}', '\u{E000}'])
    }

    /// Emit Unicode operations with hash context
    pub fn emit_with_hash_context(
        &self,
        dsl_op: &DSLOperation,
        operational_hash: Option<&str>,
    ) -> DSLResult<Vec<char>> {
        let mut unicode_ops = self.emit_unicode_operation(dsl_op);

        // Add hash-based Unicode triggers if operational hash provided
        if let Some(hash) = operational_hash {
            let hash_ops = self.emit_from_hash(hash)?;
            unicode_ops.extend(hash_ops);
        }

        Ok(unicode_ops)
    }

    /// Format Unicode operations as string for logging/debugging
    pub fn format_unicode_ops(&self, unicode_ops: &[char]) -> String {
        unicode_ops
            .iter()
            .map(|c| format!("\\u{{{:04X}}}", *c as u32))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl Default for UnicodeEmitter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsl::operations::*;

    #[test]
    fn test_emit_intel_collection() {
        let emitter = UnicodeEmitter::new();
        let op = DSLOperation::IntelCollection(IntelCollectionOp {
            hash: None,
            semantic_hash: None,
            unicode: '\u{E300}',
            tool: "kali_recon".to_string(),
            gpu_tier: "high".to_string(),
            isolation: "threat_intel".to_string(),
            max_cost: 0.0,              // Default for test
            timeout: "30s".to_string(), // Default for test
            depends_on: vec![],         // Default for test
        });

        let unicode_ops = emitter.emit_unicode_operation(&op);
        assert!(unicode_ops.contains(&'\u{E300}'));
        assert!(unicode_ops.contains(&'\u{E320}'));
        assert!(unicode_ops.contains(&'\u{E800}'));
    }

    #[test]
    fn test_emit_with_hash() {
        let emitter = UnicodeEmitter::new();
        let hash = "3kJ9mP4xQ7R8sN2mK5fH9nL8vC3dF6gH2jK9mP4xQ7R8sN2m";

        let unicode_ops = emitter.emit_from_hash(hash).unwrap();
        assert_eq!(unicode_ops.len(), 3);
        assert!(unicode_ops.contains(&'\u{E100}'));
        assert!(unicode_ops.contains(&'\u{E200}'));
        assert!(unicode_ops.contains(&'\u{E000}'));
    }

    #[test]
    fn test_format_unicode_ops() {
        let emitter = UnicodeEmitter::new();
        let ops = vec!['\u{E100}', '\u{E200}', '\u{E300}'];
        let formatted = emitter.format_unicode_ops(&ops);

        assert!(formatted.contains("E100"));
        assert!(formatted.contains("E200"));
        assert!(formatted.contains("E300"));
    }
}
