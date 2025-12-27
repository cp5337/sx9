//! Voice Bridge Configuration
//!
//! RFC-9107 §5.3: Configuration for ElevenLabs voice integration.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Configuration errors
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing API key: set ELEVEN_KEY or ELEVENLABS_API_KEY environment variable")]
    MissingApiKey,

    #[error("Invalid configuration: {0}")]
    Invalid(String),

    #[error("Environment error: {0}")]
    EnvError(#[from] std::env::VarError),
}

/// Main voice bridge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceBridgeConfig {
    /// ElevenLabs API key
    pub api_key: String,

    /// Base URL for ElevenLabs API
    pub base_url: String,

    /// Default model ID
    pub model_id: String,

    /// Request timeout in seconds
    pub timeout_secs: u64,

    /// Maximum retries for failed requests
    pub max_retries: u32,

    /// Voice stability (0.0-1.0)
    pub stability: f32,

    /// Similarity boost (0.0-1.0)
    pub similarity_boost: f32,

    /// Style (0.0-1.0, only for v2 models)
    pub style: f32,

    /// Use speaker boost
    pub use_speaker_boost: bool,
}

impl Default for VoiceBridgeConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.elevenlabs.io/v1".to_string(),
            model_id: "eleven_monolingual_v1".to_string(),
            timeout_secs: 30,
            max_retries: 3,
            stability: 0.5,
            similarity_boost: 0.75,
            style: 0.0,
            use_speaker_boost: true,
        }
    }
}

impl VoiceBridgeConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        let api_key = std::env::var("ELEVEN_KEY")
            .or_else(|_| std::env::var("ELEVENLABS_API_KEY"))
            .map_err(|_| ConfigError::MissingApiKey)?;

        let model_id = std::env::var("ELEVEN_MODEL")
            .unwrap_or_else(|_| "eleven_monolingual_v1".to_string());

        let stability: f32 = std::env::var("ELEVEN_STABILITY")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.5);

        let similarity_boost: f32 = std::env::var("ELEVEN_SIMILARITY")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0.75);

        Ok(Self {
            api_key,
            model_id,
            stability,
            similarity_boost,
            ..Default::default()
        })
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.api_key.is_empty() {
            return Err(ConfigError::MissingApiKey);
        }
        if self.stability < 0.0 || self.stability > 1.0 {
            return Err(ConfigError::Invalid("stability must be 0.0-1.0".into()));
        }
        if self.similarity_boost < 0.0 || self.similarity_boost > 1.0 {
            return Err(ConfigError::Invalid("similarity_boost must be 0.0-1.0".into()));
        }
        Ok(())
    }
}

/// Voice configuration for a specific voice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    /// Voice ID from ElevenLabs
    pub voice_id: String,

    /// Display name
    pub name: String,

    /// Voice stability override
    pub stability: Option<f32>,

    /// Similarity boost override
    pub similarity_boost: Option<f32>,

    /// Style override
    pub style: Option<f32>,
}

impl VoiceConfig {
    /// Create a new voice configuration
    pub fn new(voice_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            voice_id: voice_id.into(),
            name: name.into(),
            stability: None,
            similarity_boost: None,
            style: None,
        }
    }

    /// Set stability
    pub fn with_stability(mut self, stability: f32) -> Self {
        self.stability = Some(stability);
        self
    }

    /// Set similarity boost
    pub fn with_similarity(mut self, similarity: f32) -> Self {
        self.similarity_boost = Some(similarity);
        self
    }
}

/// Agent voice configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Agent identifier
    pub id: String,

    /// Agent display name
    pub name: String,

    /// Voice configuration
    pub voice: VoiceConfig,

    /// Language code (e.g., "en", "ru", "es")
    pub language: String,

    /// Domain/specialty
    pub domain: String,
}

/// Collection of agent configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentsConfig {
    /// List of configured agents
    pub agents: Vec<Agent>,
}

impl AgentsConfig {
    /// Create default agent configurations (RFC-9107 §5.2)
    pub fn default_agents() -> Self {
        Self {
            agents: vec![
                Agent {
                    id: "natasha".to_string(),
                    name: "Natasha".to_string(),
                    voice: VoiceConfig::new("EXAVITQu4vr4xnSDxMaL", "Natasha")
                        .with_stability(0.7)
                        .with_similarity(0.8),
                    language: "ru".to_string(),
                    domain: "Geopolitical Intel".to_string(),
                },
                Agent {
                    id: "elena".to_string(),
                    name: "Elena".to_string(),
                    voice: VoiceConfig::new("oWAxZDx7w5VEj9dCyTzz", "Elena")
                        .with_stability(0.6)
                        .with_similarity(0.75),
                    language: "es".to_string(),
                    domain: "Cartel Operations".to_string(),
                },
                Agent {
                    id: "zoe".to_string(),
                    name: "Zoe".to_string(),
                    voice: VoiceConfig::new("21m00Tcm4TlvDq8ikWAM", "Zoe")
                        .with_stability(0.5)
                        .with_similarity(0.75),
                    language: "en".to_string(),
                    domain: "Orbital Control".to_string(),
                },
                Agent {
                    id: "cove".to_string(),
                    name: "Cove".to_string(),
                    voice: VoiceConfig::new("pNInz6obpgDQGcFmaJgB", "Cove")
                        .with_stability(0.5)
                        .with_similarity(0.7),
                    language: "en".to_string(),
                    domain: "Repository Operations".to_string(),
                },
                Agent {
                    id: "marcus".to_string(),
                    name: "Marcus".to_string(),
                    voice: VoiceConfig::new("VR6AewLTigWG4xSOukaG", "Marcus")
                        .with_stability(0.6)
                        .with_similarity(0.8),
                    language: "en".to_string(),
                    domain: "Neural Mux".to_string(),
                },
            ],
        }
    }

    /// Get agent by ID
    pub fn get_agent(&self, id: &str) -> Option<&Agent> {
        self.agents.iter().find(|a| a.id == id)
    }

    /// Get voice ID for agent
    pub fn get_voice_id(&self, agent_id: &str) -> Option<&str> {
        self.get_agent(agent_id).map(|a| a.voice.voice_id.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = VoiceBridgeConfig::default();
        assert_eq!(config.model_id, "eleven_monolingual_v1");
        assert!(config.stability >= 0.0 && config.stability <= 1.0);
    }

    #[test]
    fn test_default_agents() {
        let agents = AgentsConfig::default_agents();
        assert_eq!(agents.agents.len(), 5);
        assert!(agents.get_agent("natasha").is_some());
        assert!(agents.get_agent("zoe").is_some());
    }

    #[test]
    fn test_voice_config() {
        let voice = VoiceConfig::new("test-id", "Test Voice")
            .with_stability(0.8)
            .with_similarity(0.9);
        assert_eq!(voice.stability, Some(0.8));
        assert_eq!(voice.similarity_boost, Some(0.9));
    }
}
