//! Voice Mapping - Agent to ElevenLabs Voice ID Mapping
//!
//! RFC-9107 §5.2: Maps SX9 agents to their corresponding ElevenLabs voices.

use crate::config::{Agent, AgentsConfig, VoiceConfig};
use std::collections::HashMap;
use tracing::warn;

/// Default voice mappings (RFC-9107 §5.2)
pub static VOICE_MAPPINGS: &[(&str, &str, &str, &str)] = &[
    // (agent_id, voice_id, language, domain)
    ("natasha", "EXAVITQu4vr4xnSDxMaL", "ru", "Geopolitical Intel"),
    ("elena", "oWAxZDx7w5VEj9dCyTzz", "es", "Cartel Operations"),
    ("zoe", "21m00Tcm4TlvDq8ikWAM", "en", "Orbital Control"),
    ("cove", "pNInz6obpgDQGcFmaJgB", "en", "Repository Operations"),
    ("marcus", "VR6AewLTigWG4xSOukaG", "en", "Neural Mux"),
    ("grok", "ErXwobaYiN019PkySvjV", "en", "Space Engineering"),
    ("altair", "EXAVITQu4vr4xnSDxMaL", "en", "Space Domain Awareness"),
    ("claude", "21m00Tcm4TlvDq8ikWAM", "en", "Meta-Agent Orchestration"),
    ("gpt", "pNInz6obpgDQGcFmaJgB", "en", "Tactical Operations"),
];

/// Get ElevenLabs voice ID for an agent
pub fn get_elevenlabs_voice_id(agent_id: &str) -> Option<&'static str> {
    VOICE_MAPPINGS
        .iter()
        .find(|(id, _, _, _)| *id == agent_id.to_lowercase())
        .map(|(_, voice_id, _, _)| *voice_id)
}

/// Get all agent IDs with voice mappings
pub fn get_voice_enabled_agents() -> Vec<&'static str> {
    VOICE_MAPPINGS.iter().map(|(id, _, _, _)| *id).collect()
}

/// Get agent's language
pub fn get_agent_language(agent_id: &str) -> Option<&'static str> {
    VOICE_MAPPINGS
        .iter()
        .find(|(id, _, _, _)| *id == agent_id.to_lowercase())
        .map(|(_, _, lang, _)| *lang)
}

/// Get agent's domain
pub fn get_agent_domain(agent_id: &str) -> Option<&'static str> {
    VOICE_MAPPINGS
        .iter()
        .find(|(id, _, _, _)| *id == agent_id.to_lowercase())
        .map(|(_, _, _, domain)| *domain)
}

/// Create a VoiceConfig for an agent
pub fn create_voice_config(agent_id: &str) -> Option<VoiceConfig> {
    let voice_id = get_elevenlabs_voice_id(agent_id)?;
    let name = agent_id
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string() + &agent_id[1..])
        .unwrap_or_else(|| agent_id.to_string());

    Some(
        VoiceConfig::new(voice_id, &name)
            .with_stability(0.5)
            .with_similarity(0.75),
    )
}

/// Voice mapping registry with custom overrides
#[derive(Debug, Clone)]
pub struct VoiceMappingRegistry {
    mappings: HashMap<String, VoiceMapping>,
}

/// A single voice mapping entry
#[derive(Debug, Clone)]
pub struct VoiceMapping {
    pub agent_id: String,
    pub voice_id: String,
    pub language: String,
    pub domain: String,
    pub stability: f32,
    pub similarity_boost: f32,
}

impl VoiceMappingRegistry {
    /// Create a new registry with default mappings
    pub fn new() -> Self {
        let mut mappings = HashMap::new();

        for (agent_id, voice_id, language, domain) in VOICE_MAPPINGS {
            mappings.insert(
                agent_id.to_string(),
                VoiceMapping {
                    agent_id: agent_id.to_string(),
                    voice_id: voice_id.to_string(),
                    language: language.to_string(),
                    domain: domain.to_string(),
                    stability: 0.5,
                    similarity_boost: 0.75,
                },
            );
        }

        Self { mappings }
    }

    /// Get voice mapping for an agent
    pub fn get(&self, agent_id: &str) -> Option<&VoiceMapping> {
        self.mappings.get(&agent_id.to_lowercase())
    }

    /// Add or update a mapping
    pub fn set(&mut self, mapping: VoiceMapping) {
        self.mappings
            .insert(mapping.agent_id.to_lowercase(), mapping);
    }

    /// Remove a mapping
    pub fn remove(&mut self, agent_id: &str) -> Option<VoiceMapping> {
        self.mappings.remove(&agent_id.to_lowercase())
    }

    /// Get all mappings
    pub fn all(&self) -> impl Iterator<Item = &VoiceMapping> {
        self.mappings.values()
    }

    /// Get voice ID for agent
    pub fn get_voice_id(&self, agent_id: &str) -> Option<&str> {
        self.get(agent_id).map(|m| m.voice_id.as_str())
    }

    /// Convert to AgentsConfig
    pub fn to_agents_config(&self) -> AgentsConfig {
        let agents = self
            .mappings
            .values()
            .map(|m| Agent {
                id: m.agent_id.clone(),
                name: m
                    .agent_id
                    .chars()
                    .next()
                    .map(|c| c.to_uppercase().to_string() + &m.agent_id[1..])
                    .unwrap_or_else(|| m.agent_id.clone()),
                voice: VoiceConfig::new(&m.voice_id, &m.agent_id)
                    .with_stability(m.stability)
                    .with_similarity(m.similarity_boost),
                language: m.language.clone(),
                domain: m.domain.clone(),
            })
            .collect();

        AgentsConfig { agents }
    }
}

impl Default for VoiceMappingRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Resolve voice ID with fallback
pub fn resolve_voice_id(agent_id: &str, fallback: &str) -> String {
    match get_elevenlabs_voice_id(agent_id) {
        Some(voice_id) => voice_id.to_string(),
        None => {
            warn!(
                "No voice mapping for agent '{}', using fallback '{}'",
                agent_id, fallback
            );
            fallback.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_voice_id() {
        assert_eq!(
            get_elevenlabs_voice_id("natasha"),
            Some("EXAVITQu4vr4xnSDxMaL")
        );
        assert_eq!(get_elevenlabs_voice_id("zoe"), Some("21m00Tcm4TlvDq8ikWAM"));
        assert_eq!(get_elevenlabs_voice_id("unknown"), None);
    }

    #[test]
    fn test_get_voice_id_case_insensitive() {
        assert_eq!(
            get_elevenlabs_voice_id("NATASHA"),
            Some("EXAVITQu4vr4xnSDxMaL")
        );
        assert_eq!(
            get_elevenlabs_voice_id("Natasha"),
            Some("EXAVITQu4vr4xnSDxMaL")
        );
    }

    #[test]
    fn test_get_agent_language() {
        assert_eq!(get_agent_language("natasha"), Some("ru"));
        assert_eq!(get_agent_language("elena"), Some("es"));
        assert_eq!(get_agent_language("zoe"), Some("en"));
    }

    #[test]
    fn test_create_voice_config() {
        let config = create_voice_config("natasha").unwrap();
        assert_eq!(config.voice_id, "EXAVITQu4vr4xnSDxMaL");
        assert_eq!(config.name, "Natasha");
    }

    #[test]
    fn test_voice_mapping_registry() {
        let registry = VoiceMappingRegistry::new();
        assert!(registry.get("natasha").is_some());
        assert!(registry.get("unknown").is_none());
    }

    #[test]
    fn test_registry_to_agents_config() {
        let registry = VoiceMappingRegistry::new();
        let config = registry.to_agents_config();
        assert!(!config.agents.is_empty());
    }

    #[test]
    fn test_resolve_voice_id() {
        assert_eq!(
            resolve_voice_id("natasha", "default"),
            "EXAVITQu4vr4xnSDxMaL"
        );
        assert_eq!(resolve_voice_id("unknown", "fallback-id"), "fallback-id");
    }
}
