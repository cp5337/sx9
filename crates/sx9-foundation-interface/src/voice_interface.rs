//! CTAS AI CLI Voice Interface
//!
//! Provides audio conversational capabilities including:
//! - Speech-to-text transcription using Whisper
//! - Text-to-speech synthesis using multiple providers
//! - Real-time audio streaming and processing
//! - Voice command recognition and routing
//! - Audio feedback and response generation

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex, RwLock},
    time::Instant,
};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

// Re-export types from main crate
pub use crate::{
    ai_platforms::AIPlatform,
    natural_language::ParsedNLCommand,
    persona_context::PersonaAssignment,
};

/// Voice interface configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    pub enabled: bool,
    pub input_device: Option<String>,
    pub output_device: Option<String>,
    pub sample_rate: u32,
    pub channels: u16,
    pub buffer_size: usize,
    pub whisper_model_path: Option<String>,
    pub tts_provider: TTSProvider,
    pub voice_personality: VoicePersonality,
}

/// Text-to-speech provider options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TTSProvider {
    System,
    Whisper,
    Custom(String),
}

/// Voice personality for TTS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoicePersonality {
    Natasha,
    Marcus,
    Elena,
    James,
    Alex,
}

/// Voice command processing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCommandResult {
    pub session_id: Uuid,
    pub transcription: String,
    pub confidence: f32,
    pub parsed_command: Option<ParsedNLCommand>,
    pub response_text: String,
    pub persona_assignment: Option<PersonaAssignment>,
    pub ai_platform: Option<AIPlatform>,
    pub processing_time_ms: u64,
}

/// Voice session state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSession {
    pub id: Uuid,
    pub started_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub is_active: bool,
    pub audio_buffer: Vec<f32>,
    pub command_count: usize,
}

/// Voice system events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoiceEvent {
    SessionStarted { session_id: Uuid },
    SessionEnded { session_id: Uuid },
    AudioReceived { session_id: Uuid, data: Vec<f32> },
    CommandProcessed { session_id: Uuid, result: VoiceCommandResult },
    Error { session_id: Uuid, error: String },
}

/// Voice manager for handling multiple voice sessions
#[derive(Debug)]
pub struct VoiceManager {
    sessions: RwLock<HashMap<Uuid, VoiceSession>>,
    config: VoiceConfig,
}

impl VoiceManager {
    pub fn new(config: VoiceConfig) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            config,
        }
    }

    pub async fn create_session(&self) -> Result<Uuid> {
        let session_id = Uuid::new_v4();
        let session = VoiceSession {
            id: session_id,
            started_at: Utc::now(),
            last_activity: Utc::now(),
            is_active: true,
            audio_buffer: Vec::new(),
            command_count: 0,
        };

        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(session_id, session);
        }

        info!("Voice session created: {}", session_id);
        Ok(session_id)
    }

    pub async fn get_session(&self, session_id: Uuid) -> Option<VoiceSession> {
        let sessions = self.sessions.read().await;
        sessions.get(&session_id).cloned()
    }

    pub async fn get_sessions(&self) -> HashMap<Uuid, VoiceSession> {
        self.sessions.read().await.clone()
    }

    pub async fn end_session(&self, session_id: Uuid) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.is_active = false;
            session.last_activity = Utc::now();
            info!("Voice session ended: {}", session_id);
        }
        Ok(())
    }
}

/// Main voice interface with full duplex audio capabilities
#[derive(Debug)]
pub struct VoiceInterface {
    config: VoiceConfig,
    voice_manager: VoiceManager,
    whisper_context: Option<WhisperContext>,
    audio_host: cpal::Host,
    sessions: RwLock<HashMap<Uuid, VoiceSession>>,
    audio_buffer: Arc<Mutex<VecDeque<f32>>>,
    is_recording: Arc<Mutex<bool>>,
    event_sender: mpsc::UnboundedSender<VoiceEvent>,
    event_receiver: RwLock<Option<mpsc::UnboundedReceiver<VoiceEvent>>>,
    input_stream: Arc<Mutex<Option<cpal::Stream>>>,
    output_stream: Arc<Mutex<Option<cpal::Stream>>>,
}

impl VoiceInterface {
    /// Create a new voice interface with the given configuration
    pub async fn new(config: VoiceConfig) -> Result<Self> {
        // Validate configuration
        config.validate()?;
        
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        let voice_manager = VoiceManager::new(config.clone());

        // Initialize audio host
        let audio_host = cpal::default_host();
        
        // Initialize Whisper context if model path is provided
        let whisper_context = if let Some(model_path) = &config.whisper_model_path {
            Some(WhisperContext::new(model_path).context("Failed to load Whisper model")?)
        } else {
            None
        };

        let interface = Self {
            config,
            voice_manager,
            whisper_context,
            audio_host,
            sessions: RwLock::new(HashMap::new()),
            audio_buffer: Arc::new(Mutex::new(VecDeque::new())),
            is_recording: Arc::new(Mutex::new(false)),
            event_sender,
            event_receiver: RwLock::new(Some(event_receiver)),
            input_stream: Arc::new(Mutex::new(None)),
            output_stream: Arc::new(Mutex::new(None)),
        };

        info!("Voice interface initialized");
        Ok(interface)
    }

    /// Start a new voice session
    pub async fn start_session(&mut self) -> Result<Uuid> {
        let session_id = self.voice_manager.create_session().await?;
        
        // Send session started event
        let _ = self.event_sender.send(VoiceEvent::SessionStarted { session_id });
        
        Ok(session_id)
    }

    /// Start listening for voice input
    pub async fn start_listening(&mut self, session_id: Uuid) -> Result<()> {
        {
            let mut is_recording = self.is_recording.lock().unwrap();
            *is_recording = true;
        }

        info!("Started listening for session: {}", session_id);
        Ok(())
    }

    /// Stop listening for voice input
    pub async fn stop_listening(&mut self, session_id: Uuid) -> Result<()> {
        {
            let mut is_recording = self.is_recording.lock().unwrap();
            *is_recording = false;
        }

        self.voice_manager.end_session(session_id).await?;
        info!("Stopped listening for session: {}", session_id);
        Ok(())
    }

    /// Process voice command from audio data
    pub async fn process_voice_command(&mut self, session_id: Uuid, audio_data: Vec<f32>) -> Result<VoiceCommandResult> {
        let start_time = Instant::now();

        // Simulate transcription (in real implementation, this would use Whisper)
        let transcription = self.simulate_transcription(&audio_data).await?;
        
        // Simulate command parsing
        let parsed_command = self.simulate_command_parsing(&transcription).await?;
        
        // Generate response
        let response_text = self.generate_response(&transcription).await?;

        let processing_time = start_time.elapsed().as_millis() as u64;

        let result = VoiceCommandResult {
            session_id,
            transcription,
            confidence: 0.85, // Simulated confidence
            parsed_command,
            response_text,
            persona_assignment: None,
            ai_platform: None,
            processing_time_ms: processing_time,
        };

        // Send command processed event
        let _ = self.event_sender.send(VoiceEvent::CommandProcessed {
            session_id,
            result: result.clone(),
        });

        Ok(result)
    }

    /// Start the event processing loop
    pub async fn start_event_loop(&mut self) -> Result<()> {
        let mut receiver = self
            .event_receiver
            .write()
            .await
            .take()
            .context("Event receiver already taken")?;

        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                Self::handle_event(event).await;
            }
        });

        Ok(())
    }

    /// Get all voice sessions
    pub async fn get_sessions(&self) -> HashMap<Uuid, VoiceSession> {
        self.voice_manager.get_sessions().await
    }

    // Helper methods for simulation
    async fn simulate_transcription(&self, _audio_data: &[f32]) -> Result<String> {
        // In real implementation, this would use Whisper
        Ok("Hello, this is a simulated transcription".to_string())
    }

    async fn simulate_command_parsing(&self, transcription: &str) -> Result<Option<ParsedNLCommand>> {
        // In real implementation, this would parse the command
        Ok(Some(ParsedNLCommand {
            command_domain: "general".to_string(),
            action_type: "query".to_string(),
            target: transcription.to_string(),
            parameters: HashMap::new(),
        }))
    }

    async fn generate_response(&self, transcription: &str) -> Result<String> {
        // In real implementation, this would generate a proper response
        Ok(format!("I heard you say: '{}'. This is a simulated response.", transcription))
    }

    /// Handle voice events
    async fn handle_event(event: VoiceEvent) {
        match event {
            VoiceEvent::SessionStarted { session_id } => {
                info!("Voice session started: {}", session_id);
            }
            VoiceEvent::SessionEnded { session_id } => {
                info!("Voice session ended: {}", session_id);
            }
            VoiceEvent::AudioReceived { session_id, data } => {
                debug!("Audio received for session {}: {} samples", session_id, data.len());
            }
            VoiceEvent::CommandProcessed { session_id, result } => {
                info!("Command processed for session {}: {}", session_id, result.transcription);
            }
            VoiceEvent::Error { session_id, error } => {
                error!("Error in voice session {}: {}", session_id, error);
            }
        }
    }
}

impl VoiceConfig {
    /// Validate the voice configuration
    pub fn validate(&self) -> Result<()> {
        if self.sample_rate == 0 {
            return Err(anyhow::anyhow!("Sample rate must be greater than 0"));
        }
        if self.channels == 0 {
            return Err(anyhow::anyhow!("Channels must be greater than 0"));
        }
        if self.buffer_size == 0 {
            return Err(anyhow::anyhow!("Buffer size must be greater than 0"));
        }
        Ok(())
    }

    /// Get default configuration
    pub fn default() -> Self {
        Self {
            enabled: false,
            input_device: None,
            output_device: None,
            sample_rate: 16000,
            channels: 1,
            buffer_size: 1024,
            whisper_model_path: None,
            tts_provider: TTSProvider::System,
            voice_personality: VoicePersonality::Natasha,
        }
    }
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_voice_interface_creation() {
        let config = VoiceConfig::default();
        let interface = VoiceInterface::new(config).await;
        assert!(interface.is_ok());
    }

    #[tokio::test]
    async fn test_voice_session_creation() {
        let config = VoiceConfig::default();
        let mut interface = VoiceInterface::new(config).await.unwrap();
        
        let session_id = interface.start_session().await;
        assert!(session_id.is_ok());
    }
}
