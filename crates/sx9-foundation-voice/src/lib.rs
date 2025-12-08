//! Foundation Voice Synthesis - ElevenLabs Integration
//!
//! RFC-9107 §5: Voice System Integration
//!
//! This crate provides the foundation voice interface for CTAS-7 agents.
//! The primary implementation uses ElevenLabs TTS/STT via the `ctas7-voice-bridge` crate.
//!
//! # Features
//!
//! - `elevenlabs` (default): Enable ElevenLabs TTS/STT via voice-bridge
//! - `persona_management`: Agent voice persona configuration
//!
//! # Usage
//!
//! ```rust,ignore
//! use foundation_voice::tts::elevenlabs_tts;
//!
//! // Synthesize speech with ElevenLabs
//! elevenlabs_tts("EXAVITQu4vr4xnSDxMaL", "Hello from CTAS-7").await?;
//! ```

use serde::{Deserialize, Serialize};

// Foundation modules
pub mod voice_logger;
pub mod foundation_integration;

// Re-export voice_logger types
pub use voice_logger::{
    VoiceLogger, VoiceInteraction, VoiceSpeaker, ExecutionMode,
    VoicePriority, VoiceLoggerConfig, VoiceQueryFilter, VoiceLoggerStats,
    ThalamicFilterConfig, ThalamicFilterResult
};

// RFC-9107 §5.2: Re-export from voice-bridge when elevenlabs feature is enabled
#[cfg(feature = "elevenlabs")]
pub use ctas7_voice_bridge::{
    // TTS functions
    tts::elevenlabs_tts,
    tts::validate_tts_text,
    tts::tts_with_retry,
    tts::test_tts_config,
    // Audio capture and processing
    audio::record_mic,
    audio::record_with_vad,
    audio::encode_wav_pcm16_mono_16k,
    audio::to_mono_16k,
    audio::play_audio,
    audio::calculate_rms,
    // Voice activity detection
    vad::VoiceActivityDetector,
    // Session management
    session::SessionManager,
    // Configuration types
    config::{Agent, AgentsConfig, VoiceBridgeConfig, VoiceConfig},
    // Voice mapping
    voice_mapping::get_elevenlabs_voice_id,
    voice_mapping::create_voice_config,
    // Version and status
    is_elevenlabs_configured,
    get_voice_model,
    VERSION as VOICE_BRIDGE_VERSION,
};

// NVNN: Placeholder config until real modules are ported
/// Voice engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceEngineConfig {
    /// ElevenLabs API key (from ELEVEN_KEY env)
    pub api_key_env: String,
    /// Default voice ID
    pub default_voice_id: String,
    /// Model ID (eleven_monolingual_v1)
    pub model_id: String,
    /// Voice stability (0.0-1.0)
    pub stability: f32,
    /// Similarity boost (0.0-1.0)
    pub similarity_boost: f32,
}

impl Default for VoiceEngineConfig {
    fn default() -> Self {
        Self {
            api_key_env: "ELEVEN_KEY".to_string(),
            default_voice_id: "EXAVITQu4vr4xnSDxMaL".to_string(), // Default professional voice
            model_id: "eleven_monolingual_v1".to_string(),
            stability: 0.7,
            similarity_boost: 0.8,
        }
    }
}

/// Health status for voice subsystem
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Check if ElevenLabs API key is configured
pub fn is_voice_configured() -> bool {
    std::env::var("ELEVEN_KEY").is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = VoiceEngineConfig::default();
        assert_eq!(config.model_id, "eleven_monolingual_v1");
        assert!(config.stability > 0.0 && config.stability <= 1.0);
    }
}
