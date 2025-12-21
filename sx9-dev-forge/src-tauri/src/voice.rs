//! Voice integration with ElevenLabs for SX9 Dev Forge

use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VoiceError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("No API key configured")]
    NoApiKey,
    #[error("API error: {0}")]
    Api(String),
    #[error("Audio error: {0}")]
    Audio(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    pub api_key: Option<String>,
    pub voice_id: String,
    pub model_id: String,
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            voice_id: "21m00Tcm4TlvDq8ikWAM".to_string(), // Default voice
            model_id: "eleven_monolingual_v1".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeechToTextRequest {
    pub audio_data: Vec<u8>,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeechToTextResponse {
    pub text: String,
    pub confidence: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextToSpeechRequest {
    pub text: String,
    pub voice_id: Option<String>,
    pub model_id: Option<String>,
}

pub struct VoiceClient {
    client: reqwest::Client,
    api_key: String,
    config: VoiceConfig,
}

impl VoiceClient {
    const API_URL: &'static str = "https://api.elevenlabs.io/v1";

    pub fn new(api_key: String, config: VoiceConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            config,
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            "xi-api-key",
            HeaderValue::from_str(&self.api_key).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }

    /// Convert text to speech using ElevenLabs
    pub async fn text_to_speech(&self, text: &str, voice_id: Option<&str>) -> Result<Vec<u8>, VoiceError> {
        let voice = voice_id.unwrap_or(&self.config.voice_id);
        let url = format!("{}/text-to-speech/{}", Self::API_URL, voice);

        let body = serde_json::json!({
            "text": text,
            "model_id": self.config.model_id,
            "voice_settings": {
                "stability": 0.5,
                "similarity_boost": 0.75
            }
        });

        let response = self.client
            .post(&url)
            .headers(self.headers())
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(VoiceError::Api(error_text));
        }

        let audio_data = response.bytes().await?.to_vec();
        Ok(audio_data)
    }

    /// Get available voices
    pub async fn list_voices(&self) -> Result<Vec<Voice>, VoiceError> {
        let url = format!("{}/voices", Self::API_URL);

        let response = self.client
            .get(&url)
            .headers(self.headers())
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(VoiceError::Api(error_text));
        }

        #[derive(Deserialize)]
        struct VoicesResponse {
            voices: Vec<Voice>,
        }

        let data: VoicesResponse = response.json().await?;
        Ok(data.voices)
    }

    /// Test connection to ElevenLabs API
    pub async fn test_connection(&self) -> Result<bool, VoiceError> {
        let voices = self.list_voices().await?;
        Ok(!voices.is_empty())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voice {
    pub voice_id: String,
    pub name: String,
    pub category: Option<String>,
}

/// Browser-based speech recognition (Web Speech API)
/// This is handled on the frontend, but we provide types for serialization
#[derive(Debug, Serialize, Deserialize)]
pub struct VoiceRecognitionResult {
    pub transcript: String,
    pub confidence: f32,
    pub is_final: bool,
}
