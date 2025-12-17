//! SX9 DSL for Playbook Modernization (RFC-9011-B)
//!
//! Defines the canonical `Sx9Playbook` structure and associated types
//! for the transition from XSD to TOML-based threat response playbooks.

use std::collections::HashMap;
use sx9_foundation_manifold::core::data::serde::{Deserialize, Serialize};

/// Executable playbook derived from SX9 DSL (RFC-9011-B)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Sx9Playbook {
    /// Unique playbook identifier (UUID)
    pub playbook_id: String,

    /// Human-readable name
    pub name: String,

    /// Description/summary of the playbook
    pub description: String,

    // -- HD4 Context --
    /// Primary HD4 phase (e.g., "HUNT", "DISRUPT")
    pub hd4_phase: String,

    /// Primary PTCC primitive (e.g., 0x08 for EXECUTE)
    pub ptcc_primitive: u8,

    // -- Source References --
    /// List of source artifact references (e.g., "sigma:abc123")
    pub source_artifacts: Vec<String>,

    /// Covered ATT&CK technique IDs (e.g., "T1059.001")
    pub attack_techniques: Vec<String>,

    // -- Execution --
    /// Ordered list of actions to execute
    pub actions: Vec<PlaybookAction>,

    /// Target platforms (e.g., "windows", "linux")
    pub platforms: Vec<String>,

    // -- Requirements --
    /// Required capabilities or tools
    pub prerequisites: Vec<String>,

    /// Required log sources or data feeds
    pub data_sources: Vec<String>,

    // -- Metadata --
    /// Severity level (info, low, medium, high, critical)
    pub severity: String,

    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,

    /// Creation timestamp (ISO 8601)
    pub created_at: String,

    /// Trivariate hash for content addressing and integrity
    pub trivariate_hash: TrivariateHash,
}

/// Trivariate hash components (v7.3.1)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrivariateHash {
    /// Semantic Content Hash (SCH-T)
    pub sch_t: String,

    /// Contextual Unique ID (CUID-T)
    pub cuid_t: String,

    /// Universally Unique ID (UUID)
    pub uuid: String,
}

/// Polymorphic playbook action types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlaybookAction {
    /// Detection action (e.g., SIEM query)
    Detect {
        engine: String,
        query_type: String,
        logsource: HashMap<String, String>,
        detection_logic: sx9_foundation_manifold::core::data::serde_json::Value,
        #[serde(default)]
        level: String,
    },

    /// Execution action (e.g., command execution, script)
    Execute {
        #[serde(skip_serializing_if = "Option::is_none")]
        platform: Option<String>,
        executor: String,
        command: String,
        #[serde(default)]
        cleanup: String,
        #[serde(default)]
        payloads: Vec<String>,
        #[serde(default)]
        elevation_required: bool,
    },

    /// Response action (e.g., isolate host, block IP)
    Response {
        action: String,
        target: String,
        #[serde(default)]
        params: HashMap<String, String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml;

    #[test]
    fn test_playbook_serialization() {
        let playbook = Sx9Playbook {
            playbook_id: "550e8400-e29b-41d4-a716-446655440000".to_string(),
            name: "Test Playbook".to_string(),
            description: "A test playbook".to_string(),
            hd4_phase: "HUNT".to_string(),
            ptcc_primitive: 0x08,
            source_artifacts: vec!["sigma:123".to_string()],
            attack_techniques: vec!["T1059".to_string()],
            actions: vec![PlaybookAction::Execute {
                platform: Some("windows".to_string()),
                executor: "powershell".to_string(),
                command: "Get-Process".to_string(),
                cleanup: "".to_string(),
                payloads: vec![],
                elevation_required: false,
            }],
            platforms: vec!["windows".to_string()],
            prerequisites: vec![],
            data_sources: vec![],
            severity: "high".to_string(),
            confidence: 0.9,
            created_at: "2025-12-12T00:00:00Z".to_string(),
            trivariate_hash: TrivariateHash {
                sch_t: "abc".to_string(),
                cuid_t: "def".to_string(),
                uuid: "123".to_string(),
            },
        };

        let toml_string = toml::to_string(&playbook).expect("Failed to serialize to TOML");
        println!("TOML Output:\n{}", toml_string);

        let deserialized: Sx9Playbook =
            toml::from_str(&toml_string).expect("Failed to deserialize from TOML");
        assert_eq!(playbook, deserialized);
    }
}
