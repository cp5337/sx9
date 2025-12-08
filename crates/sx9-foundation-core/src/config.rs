use anyhow::Result;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct VoiceConfig {
    pub provider: String, // "elevenlabs" | "azure"
    pub voice: String,    // ElevenLabs voice_id or Azure voice name
    pub accent: String,   // metadata only
}

#[derive(Debug, Deserialize)]
pub struct Agent {
    pub id: String,
    pub role: String,
    pub elite_team_name: String,
    pub stack: String, // "stack1" (Azure STT + ElevenLabs TTS) or "stack2" (Azure STT + Azure TTS)
    pub voice_config: VoiceConfig,
}

#[derive(Debug, Deserialize)]
pub struct AgentsConfig {
    pub agents: Vec<Agent>,
}

#[derive(Debug)]
pub struct VoiceBridgeConfig {
    pub mcp_base: String,
    pub default_recording_duration: u64,
    pub vad_enabled: bool,
    pub vad_threshold: f32,
    pub silence_duration: f32,
    pub max_recording_duration: u64,
    pub save_audio_files: bool,
}

impl VoiceBridgeConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            mcp_base: env::var("MCP_BASE").unwrap_or_else(|_| "http://localhost:15180".to_string()),
            default_recording_duration: env::var("DEFAULT_RECORDING_DURATION")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            vad_enabled: cfg!(feature = "vad"),
            vad_threshold: env::var("VAD_THRESHOLD")
                .unwrap_or_else(|_| "0.02".to_string())
                .parse()
                .unwrap_or(0.02),
            silence_duration: env::var("SILENCE_DURATION")
                .unwrap_or_else(|_| "2.0".to_string())
                .parse()
                .unwrap_or(2.0),
            max_recording_duration: env::var("MAX_RECORDING_DURATION")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            save_audio_files: env::var("SAVE_AUDIO_FILES")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        })
    }
}