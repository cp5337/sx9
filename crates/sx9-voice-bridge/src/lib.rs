//! SX9 Voice Bridge - ElevenLabs TTS/STT Implementation
//!
//! RFC-9107 §5: Full voice integration for the SX9 agent system.
//!
//! This crate provides the actual ElevenLabs API implementation that
//! `sx9-foundation-voice` depends on via feature flag.
//!
//! # Features
//!
//! - `tts` (default): Text-to-Speech via ElevenLabs API
//! - `audio_capture` (default): Microphone recording via cpal
//! - `audio_playback`: Audio playback via rodio
//! - `vad` (default): Voice Activity Detection via webrtc-vad
//!
//! # Usage
//!
//! ```rust,ignore
//! use sx9_voice_bridge::{tts::elevenlabs_tts, config::VoiceBridgeConfig};
//!
//! let config = VoiceBridgeConfig::from_env()?;
//! let audio = elevenlabs_tts(&config, "EXAVITQu4vr4xnSDxMaL", "Hello from SX9").await?;
//! ```

pub mod audio;
pub mod config;
pub mod session;
pub mod tts;
pub mod vad;
pub mod voice_mapping;

// Re-exports for convenience
pub use config::{Agent, AgentsConfig, VoiceBridgeConfig, VoiceConfig};
pub use session::SessionManager;
pub use tts::{elevenlabs_tts, test_tts_config, tts_with_retry, validate_tts_text};
pub use vad::VoiceActivityDetector;
pub use voice_mapping::{create_voice_config, get_elevenlabs_voice_id};

/// Crate version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Check if ElevenLabs API key is configured
pub fn is_elevenlabs_configured() -> bool {
    std::env::var("ELEVEN_KEY").is_ok() || std::env::var("ELEVENLABS_API_KEY").is_ok()
}

/// Get the voice model ID (default: eleven_monolingual_v1)
pub fn get_voice_model() -> String {
    std::env::var("ELEVEN_MODEL")
        .unwrap_or_else(|_| "eleven_monolingual_v1".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_voice_model_default() {
        let model = get_voice_model();
        assert!(model.contains("eleven"));
    }
}
