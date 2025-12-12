//! DSL-to-Unicode Operation Registry
//!
//! Maps DSL macros to Unicode Private Use Area (U+E000-E9FF) operations
//! for Neural Mux routing. Provides bidirectional lookup tables.

use crate::dsl::operations::DSLOperation;
use std::collections::HashMap;

/// Unicode Private Use Area ranges for CTAS-7 DSL operations
pub mod unicode_ranges {
    /// Core System Operations (256 ops)
    pub const CORE_SYSTEM_START: u32 = 0xE000;
    pub const CORE_SYSTEM_END: u32 = 0xE0FF;

    /// Trivariate Hash Components - SCH (256 ops)
    pub const TRIVARIATE_SCH_START: u32 = 0xE100;
    pub const TRIVARIATE_SCH_END: u32 = 0xE1FF;

    /// Context System Nodes - CUID (256 ops)
    pub const CONTEXT_CUID_START: u32 = 0xE200;
    pub const CONTEXT_CUID_END: u32 = 0xE2FF;

    /// Intelligence Nodes - H₃ semantic (256 ops)
    pub const INTELLIGENCE_START: u32 = 0xE300;
    pub const INTELLIGENCE_END: u32 = 0xE3FF;

    /// Environmental Mask Nodes (256 ops)
    pub const ENVIRONMENTAL_START: u32 = 0xE400;
    pub const ENVIRONMENTAL_END: u32 = 0xE4FF;

    /// XSD Integration (256 ops) - DEPRECATED, use for playbook encoding
    pub const XSD_START: u32 = 0xE500;
    pub const XSD_END: u32 = 0xE5FF;

    /// Reserved (L* learning, GNN) (512 ops)
    pub const RESERVED_START: u32 = 0xE600;
    pub const RESERVED_END: u32 = 0xE7FF;

    /// Kali Tools (256 ops)
    pub const KALI_TOOLS_START: u32 = 0xE800;
    pub const KALI_TOOLS_END: u32 = 0xE8FF;

    /// WASM Sensors (256 ops)
    pub const WASM_SENSORS_START: u32 = 0xE900;
    pub const WASM_SENSORS_END: u32 = 0xE9FF;
}

/// DSL macro to Unicode operation mapping
pub struct DSLUnicodeRegistry {
    /// DSL operation → Unicode operations (one-to-many)
    dsl_to_unicode: HashMap<String, Vec<char>>,
    /// Unicode operation → DSL operations (many-to-one)
    unicode_to_dsl: HashMap<char, String>,
    /// DSL operation → Primary Unicode trigger
    dsl_primary_unicode: HashMap<String, char>,
}

impl DSLUnicodeRegistry {
    /// Create new registry with default mappings
    pub fn new() -> Self {
        let mut registry = Self {
            dsl_to_unicode: HashMap::new(),
            unicode_to_dsl: HashMap::new(),
            dsl_primary_unicode: HashMap::new(),
        };

        // Initialize default mappings
        registry.initialize_default_mappings();
        registry
    }

    /// Initialize default DSL → Unicode mappings
    fn initialize_default_mappings(&mut self) {
        // hash_trigger! → U+E100 (SCH trivariate processor) + U+E120 (CUID) + U+E140 (UUID)
        self.register_dsl_operation(
            "hash_trigger",
            vec!['\u{E100}', '\u{E120}', '\u{E140}'],
            '\u{E100}',
        );

        // intel_collection! → U+E300 (ptie) + U+E320 (usim-hash) + U+E800 (kali-tool)
        self.register_dsl_operation(
            "intel_collection",
            vec!['\u{E300}', '\u{E320}', '\u{E800}'],
            '\u{E300}',
        );

        // pentest_spawn! → U+E200 (geo) + U+E800 (kali-tool) + U+E440 (threat-env)
        self.register_dsl_operation(
            "pentest_spawn",
            vec!['\u{E200}', '\u{E800}', '\u{E440}'],
            '\u{E200}',
        );

        // ephemeral_asset! → U+E000 (init) + U+E006 (term) + U+E140 (uuid)
        self.register_dsl_operation(
            "ephemeral_asset",
            vec!['\u{E000}', '\u{E006}', '\u{E140}'],
            '\u{E000}',
        );

        // node_interview! → U+E300 (ptie) + U+E301 (eei) + U+E302 (threat)
        self.register_dsl_operation(
            "node_interview",
            vec!['\u{E300}', '\u{E301}', '\u{E302}'],
            '\u{E300}',
        );

        // kali_tool! → U+E800-E8FF (tool-specific Unicode)
        self.register_dsl_operation(
            "kali_tool",
            vec!['\u{E800}'], // Base trigger, tool-specific codes in range
            '\u{E800}',
        );

        // workflow! → U+E000 (system controller) + U+E001 (sequence)
        self.register_dsl_operation("workflow", vec!['\u{E000}', '\u{E001}'], '\u{E000}');

        // parallel! → U+E000 (system controller) + U+E002 (parallel)
        self.register_dsl_operation("parallel", vec!['\u{E000}', '\u{E002}'], '\u{E000}');

        // conditional! → U+E000 (system controller) + U+E003 (conditional)
        self.register_dsl_operation("conditional", vec!['\u{E000}', '\u{E003}'], '\u{E000}');
    }

    /// Register a DSL operation with its Unicode mappings
    fn register_dsl_operation(&mut self, dsl_name: &str, unicode_ops: Vec<char>, primary: char) {
        self.dsl_to_unicode
            .insert(dsl_name.to_string(), unicode_ops.clone());
        self.dsl_primary_unicode
            .insert(dsl_name.to_string(), primary);

        // Map each Unicode operation back to DSL
        for unicode_op in unicode_ops {
            self.unicode_to_dsl.insert(unicode_op, dsl_name.to_string());
        }
    }

    /// Get Unicode operations for a DSL operation
    pub fn get_unicode_ops(&self, dsl_name: &str) -> Option<&Vec<char>> {
        self.dsl_to_unicode.get(dsl_name)
    }

    /// Get primary Unicode trigger for a DSL operation
    pub fn get_primary_unicode(&self, dsl_name: &str) -> Option<char> {
        self.dsl_primary_unicode.get(dsl_name).copied()
    }

    /// Get DSL operation name from Unicode operation
    pub fn get_dsl_from_unicode(&self, unicode_op: char) -> Option<&String> {
        self.unicode_to_dsl.get(&unicode_op)
    }

    /// Map DSL operation enum to Unicode operations
    pub fn operation_to_unicode(&self, op: &DSLOperation) -> Vec<char> {
        match op {
            DSLOperation::HashTrigger(_) => self
                .get_unicode_ops("hash_trigger")
                .cloned()
                .unwrap_or_else(|| vec!['\u{E100}']),
            DSLOperation::IntelCollection(_) => self
                .get_unicode_ops("intel_collection")
                .cloned()
                .unwrap_or_else(|| vec!['\u{E300}']),
            DSLOperation::PentestSpawn(_) => self
                .get_unicode_ops("pentest_spawn")
                .cloned()
                .unwrap_or_else(|| vec!['\u{E200}']),
            DSLOperation::EphemeralAsset(_) => self
                .get_unicode_ops("ephemeral_asset")
                .cloned()
                .unwrap_or_else(|| vec!['\u{E000}']),
            DSLOperation::NodeInterview(_) => self
                .get_unicode_ops("node_interview")
                .cloned()
                .unwrap_or_else(|| vec!['\u{E300}']),
            DSLOperation::KaliTool(_) => self
                .get_unicode_ops("kali_tool")
                .cloned()
                .unwrap_or_else(|| vec!['\u{E800}']),
            DSLOperation::Workflow(_) => self
                .get_unicode_ops("workflow")
                .cloned()
                .unwrap_or_else(|| vec!['\u{E000}']),
            DSLOperation::Parallel(_) => self
                .get_unicode_ops("parallel")
                .cloned()
                .unwrap_or_else(|| vec!['\u{E000}']),
            DSLOperation::Conditional(_) => self
                .get_unicode_ops("conditional")
                .cloned()
                .unwrap_or_else(|| vec!['\u{E000}']),
        }
    }

    /// Get primary Unicode trigger for DSL operation
    pub fn operation_to_primary_unicode(&self, op: &DSLOperation) -> char {
        match op {
            DSLOperation::HashTrigger(_) => self
                .get_primary_unicode("hash_trigger")
                .unwrap_or('\u{E100}'),
            DSLOperation::IntelCollection(_) => self
                .get_primary_unicode("intel_collection")
                .unwrap_or('\u{E300}'),
            DSLOperation::PentestSpawn(_) => self
                .get_primary_unicode("pentest_spawn")
                .unwrap_or('\u{E200}'),
            DSLOperation::EphemeralAsset(_) => self
                .get_primary_unicode("ephemeral_asset")
                .unwrap_or('\u{E000}'),
            DSLOperation::NodeInterview(_) => self
                .get_primary_unicode("node_interview")
                .unwrap_or('\u{E300}'),
            DSLOperation::KaliTool(_) => {
                self.get_primary_unicode("kali_tool").unwrap_or('\u{E800}')
            }
            DSLOperation::Workflow(_) => self.get_primary_unicode("workflow").unwrap_or('\u{E000}'),
            DSLOperation::Parallel(_) => self.get_primary_unicode("parallel").unwrap_or('\u{E000}'),
            DSLOperation::Conditional(_) => self
                .get_primary_unicode("conditional")
                .unwrap_or('\u{E000}'),
        }
    }
}

impl Default for DSLUnicodeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_initialization() {
        let registry = DSLUnicodeRegistry::new();

        // Test hash_trigger mapping
        let unicode_ops = registry.get_unicode_ops("hash_trigger").unwrap();
        assert!(unicode_ops.contains(&'\u{E100}'));

        // Test intel_collection mapping
        let unicode_ops = registry.get_unicode_ops("intel_collection").unwrap();
        assert!(unicode_ops.contains(&'\u{E300}'));
        assert!(unicode_ops.contains(&'\u{E320}'));
        assert!(unicode_ops.contains(&'\u{E800}'));
    }

    #[test]
    fn test_bidirectional_mapping() {
        let registry = DSLUnicodeRegistry::new();

        // DSL → Unicode
        let unicode_ops = registry.get_unicode_ops("node_interview").unwrap();
        assert!(!unicode_ops.is_empty());

        // Unicode → DSL
        let dsl_name = registry.get_dsl_from_unicode('\u{E300}').unwrap();
        assert!(dsl_name == "intel_collection" || dsl_name == "node_interview");
    }

    #[test]
    fn test_primary_unicode() {
        let registry = DSLUnicodeRegistry::new();

        let primary = registry.get_primary_unicode("intel_collection").unwrap();
        assert_eq!(primary, '\u{E300}');

        let primary = registry.get_primary_unicode("ephemeral_asset").unwrap();
        assert_eq!(primary, '\u{E000}');
    }
}
