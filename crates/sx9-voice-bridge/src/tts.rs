//! ElevenLabs Text-to-Speech
//!
//! RFC-9107 §5.1: TTS implementation for agent voice synthesis.

use crate::config::VoiceBridgeConfig;
use anyhow::{Context, Result};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, error, info, warn};

/// TTS errors
#[derive(Error, Debug)]
pub enum TtsError {
    #[error("API error: {status} - {message}")]
    ApiError { status: u16, message: String },

    #[error("Invalid text: {0}")]
    InvalidText(String),

    #[error("Rate limited: retry after {retry_after} seconds")]
    RateLimited { retry_after: u64 },

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Configuration error: {0}")]
    Config(String),
}

/// TTS request body
#[derive(Debug, Serialize)]
struct TtsRequest {
    text: String,
    model_id: String,
    voice_settings: VoiceSettings,
}

/// Voice settings for TTS
#[derive(Debug, Serialize)]
struct VoiceSettings {
    stability: f32,
    similarity_boost: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_speaker_boost: Option<bool>,
}

/// TTS response for streaming
#[derive(Debug, Deserialize)]
struct TtsErrorResponse {
    detail: Option<ErrorDetail>,
}

#[derive(Debug, Deserialize)]
struct ErrorDetail {
    message: String,
    status: Option<String>,
}

/// Validate TTS text input
pub fn validate_tts_text(text: &str) -> Result<(), TtsError> {
    if text.is_empty() {
        return Err(TtsError::InvalidText("Text cannot be empty".into()));
    }
    if text.len() > 5000 {
        return Err(TtsError::InvalidText(
            "Text exceeds maximum length of 5000 characters".into(),
        ));
    }
    // Check for unsupported characters
    if text.chars().all(|c| c.is_whitespace()) {
        return Err(TtsError::InvalidText(
            "Text cannot be only whitespace".into(),
        ));
    }
    Ok(())
}

/// Synthesize speech with ElevenLabs API
///
/// Returns raw audio bytes (MP3 format by default).
///
/// # Arguments
/// * `config` - Voice bridge configuration with API key
/// * `voice_id` - ElevenLabs voice ID
/// * `text` - Text to synthesize
///
/// # Example
/// ```rust,ignore
/// let audio = elevenlabs_tts(&config, "EXAVITQu4vr4xnSDxMaL", "Hello world").await?;
/// ```
pub async fn elevenlabs_tts(
    config: &VoiceBridgeConfig,
    voice_id: &str,
    text: &str,
) -> Result<Bytes, TtsError> {
    validate_tts_text(text)?;

    let url = format!("{}/text-to-speech/{}", config.base_url, voice_id);

    let request_body = TtsRequest {
        text: text.to_string(),
        model_id: config.model_id.clone(),
        voice_settings: VoiceSettings {
            stability: config.stability,
            similarity_boost: config.similarity_boost,
            style: if config.style > 0.0 {
                Some(config.style)
            } else {
                None
            },
            use_speaker_boost: if config.use_speaker_boost {
                Some(true)
            } else {
                None
            },
        },
    };

    debug!("TTS request: voice={}, text_len={}", voice_id, text.len());

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(config.timeout_secs))
        .build()
        .map_err(TtsError::Network)?;

    let response = client
        .post(&url)
        .header("xi-api-key", &config.api_key)
        .header("Content-Type", "application/json")
        .header("Accept", "audio/mpeg")
        .json(&request_body)
        .send()
        .await?;

    let status = response.status();

    if status.is_success() {
        let audio_bytes = response.bytes().await?;
        info!(
            "TTS success: voice={}, audio_size={}",
            voice_id,
            audio_bytes.len()
        );
        Ok(audio_bytes)
    } else if status.as_u16() == 429 {
        // Rate limited
        let retry_after = response
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
            .unwrap_or(60);
        warn!("TTS rate limited, retry after {} seconds", retry_after);
        Err(TtsError::RateLimited { retry_after })
    } else {
        let error_text = response.text().await.unwrap_or_default();
        error!("TTS error: status={}, body={}", status, error_text);

        // Try to parse error response
        let message = serde_json::from_str::<TtsErrorResponse>(&error_text)
            .ok()
            .and_then(|e| e.detail)
            .map(|d| d.message)
            .unwrap_or(error_text);

        Err(TtsError::ApiError {
            status: status.as_u16(),
            message,
        })
    }
}

/// Synthesize speech with automatic retry on failure
///
/// Retries up to `config.max_retries` times with exponential backoff.
pub async fn tts_with_retry(
    config: &VoiceBridgeConfig,
    voice_id: &str,
    text: &str,
) -> Result<Bytes, TtsError> {
    let mut last_error = None;
    let mut delay = std::time::Duration::from_millis(500);

    for attempt in 0..config.max_retries {
        match elevenlabs_tts(config, voice_id, text).await {
            Ok(audio) => return Ok(audio),
            Err(TtsError::RateLimited { retry_after }) => {
                warn!(
                    "TTS attempt {} rate limited, waiting {} seconds",
                    attempt + 1,
                    retry_after
                );
                tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;
                last_error = Some(TtsError::RateLimited { retry_after });
            }
            Err(TtsError::Network(e)) if attempt < config.max_retries - 1 => {
                warn!("TTS attempt {} failed with network error: {}", attempt + 1, e);
                tokio::time::sleep(delay).await;
                delay *= 2; // Exponential backoff
                last_error = Some(TtsError::Network(e));
            }
            Err(e) => {
                // Non-retryable error
                return Err(e);
            }
        }
    }

    Err(last_error.unwrap_or(TtsError::Config("Max retries exceeded".into())))
}

/// Test TTS configuration by making a minimal API call
pub async fn test_tts_config(config: &VoiceBridgeConfig) -> Result<bool> {
    let url = format!("{}/user", config.base_url);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("xi-api-key", &config.api_key)
        .send()
        .await
        .context("Failed to connect to ElevenLabs API")?;

    if response.status().is_success() {
        info!("ElevenLabs API connection successful");
        Ok(true)
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        error!("ElevenLabs API test failed: {} - {}", status, body);
        Ok(false)
    }
}

/// Get available voices from ElevenLabs API
pub async fn get_available_voices(config: &VoiceBridgeConfig) -> Result<Vec<VoiceInfo>> {
    let url = format!("{}/voices", config.base_url);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("xi-api-key", &config.api_key)
        .send()
        .await
        .context("Failed to fetch voices")?;

    if response.status().is_success() {
        let voices_response: VoicesResponse = response.json().await?;
        Ok(voices_response.voices)
    } else {
        anyhow::bail!("Failed to fetch voices: {}", response.status());
    }
}

/// Voice information from ElevenLabs API
#[derive(Debug, Deserialize)]
pub struct VoiceInfo {
    pub voice_id: String,
    pub name: String,
    pub category: Option<String>,
    pub labels: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct VoicesResponse {
    voices: Vec<VoiceInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_text_empty() {
        assert!(validate_tts_text("").is_err());
    }

    #[test]
    fn test_validate_text_whitespace() {
        assert!(validate_tts_text("   ").is_err());
    }

    #[test]
    fn test_validate_text_valid() {
        assert!(validate_tts_text("Hello world").is_ok());
    }

    #[test]
    fn test_validate_text_too_long() {
        let long_text = "a".repeat(6000);
        assert!(validate_tts_text(&long_text).is_err());
    }
}
