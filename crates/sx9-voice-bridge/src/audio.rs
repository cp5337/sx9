//! Audio Capture and Processing
//!
//! RFC-9107 §5.1: Audio capture, encoding, and playback.

use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

/// Audio sample rate (16kHz for voice)
pub const SAMPLE_RATE: u32 = 16000;

/// Audio channels (mono for voice)
pub const CHANNELS: u16 = 1;

/// Bits per sample
pub const BITS_PER_SAMPLE: u16 = 16;

/// Audio buffer for recording
#[derive(Debug, Clone)]
pub struct AudioBuffer {
    /// Raw PCM samples (i16)
    pub samples: Vec<i16>,
    /// Sample rate
    pub sample_rate: u32,
    /// Number of channels
    pub channels: u16,
}

impl AudioBuffer {
    /// Create a new empty audio buffer
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
            sample_rate: SAMPLE_RATE,
            channels: CHANNELS,
        }
    }

    /// Create from raw samples
    pub fn from_samples(samples: Vec<i16>, sample_rate: u32, channels: u16) -> Self {
        Self {
            samples,
            sample_rate,
            channels,
        }
    }

    /// Duration in seconds
    pub fn duration_secs(&self) -> f32 {
        self.samples.len() as f32 / (self.sample_rate as f32 * self.channels as f32)
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.samples.clear();
    }
}

impl Default for AudioBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate RMS (Root Mean Square) of audio samples
///
/// Returns a value between 0.0 and 1.0 representing audio level.
pub fn calculate_rms(samples: &[i16]) -> f32 {
    if samples.is_empty() {
        return 0.0;
    }

    let sum_squares: f64 = samples
        .iter()
        .map(|&s| (s as f64).powi(2))
        .sum();

    let mean_square = sum_squares / samples.len() as f64;
    let rms = mean_square.sqrt();

    // Normalize to 0.0-1.0 (i16 max is 32767)
    (rms / 32767.0) as f32
}

/// Convert audio to mono 16kHz
pub fn to_mono_16k(samples: &[i16], source_rate: u32, source_channels: u16) -> Vec<i16> {
    let mut mono_samples = samples.to_vec();

    // Convert to mono if stereo
    if source_channels == 2 {
        mono_samples = samples
            .chunks(2)
            .map(|chunk| {
                if chunk.len() == 2 {
                    ((chunk[0] as i32 + chunk[1] as i32) / 2) as i16
                } else {
                    chunk[0]
                }
            })
            .collect();
    }

    // Resample to 16kHz if needed
    if source_rate != SAMPLE_RATE {
        mono_samples = resample(&mono_samples, source_rate, SAMPLE_RATE);
    }

    mono_samples
}

/// Simple linear resampling
fn resample(samples: &[i16], source_rate: u32, target_rate: u32) -> Vec<i16> {
    if source_rate == target_rate {
        return samples.to_vec();
    }

    let ratio = source_rate as f64 / target_rate as f64;
    let new_len = (samples.len() as f64 / ratio) as usize;
    let mut result = Vec::with_capacity(new_len);

    for i in 0..new_len {
        let src_idx = (i as f64 * ratio) as usize;
        let src_idx_next = (src_idx + 1).min(samples.len() - 1);
        let frac = (i as f64 * ratio) - src_idx as f64;

        // Linear interpolation
        let sample = samples[src_idx] as f64 * (1.0 - frac) + samples[src_idx_next] as f64 * frac;
        result.push(sample as i16);
    }

    result
}

/// Encode audio buffer to WAV format (PCM 16-bit mono 16kHz)
pub fn encode_wav_pcm16_mono_16k(buffer: &AudioBuffer) -> Result<Vec<u8>> {
    let mut wav_data = Vec::new();

    // WAV header
    let data_size = buffer.samples.len() * 2; // 16-bit = 2 bytes per sample
    let file_size = 36 + data_size;

    // RIFF header
    wav_data.extend_from_slice(b"RIFF");
    wav_data.extend_from_slice(&(file_size as u32).to_le_bytes());
    wav_data.extend_from_slice(b"WAVE");

    // fmt chunk
    wav_data.extend_from_slice(b"fmt ");
    wav_data.extend_from_slice(&16u32.to_le_bytes()); // chunk size
    wav_data.extend_from_slice(&1u16.to_le_bytes()); // PCM format
    wav_data.extend_from_slice(&buffer.channels.to_le_bytes());
    wav_data.extend_from_slice(&buffer.sample_rate.to_le_bytes());
    let byte_rate = buffer.sample_rate * buffer.channels as u32 * 2;
    wav_data.extend_from_slice(&byte_rate.to_le_bytes());
    let block_align = buffer.channels * 2;
    wav_data.extend_from_slice(&block_align.to_le_bytes());
    wav_data.extend_from_slice(&BITS_PER_SAMPLE.to_le_bytes());

    // data chunk
    wav_data.extend_from_slice(b"data");
    wav_data.extend_from_slice(&(data_size as u32).to_le_bytes());

    // Audio data
    for sample in &buffer.samples {
        wav_data.extend_from_slice(&sample.to_le_bytes());
    }

    Ok(wav_data)
}

/// Audio recorder using cpal
#[cfg(feature = "audio_capture")]
pub struct AudioRecorder {
    buffer: Arc<Mutex<AudioBuffer>>,
    is_recording: Arc<Mutex<bool>>,
}

#[cfg(feature = "audio_capture")]
impl AudioRecorder {
    /// Create a new audio recorder
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(AudioBuffer::new())),
            is_recording: Arc::new(Mutex::new(false)),
        }
    }

    /// Check if currently recording
    pub async fn is_recording(&self) -> bool {
        *self.is_recording.lock().await
    }

    /// Get current buffer
    pub async fn get_buffer(&self) -> AudioBuffer {
        self.buffer.lock().await.clone()
    }

    /// Clear buffer
    pub async fn clear_buffer(&self) {
        self.buffer.lock().await.clear();
    }
}

#[cfg(feature = "audio_capture")]
impl Default for AudioRecorder {
    fn default() -> Self {
        Self::new()
    }
}

/// Record from microphone (simplified - returns placeholder)
///
/// Note: Full implementation requires cpal feature and platform-specific audio handling.
#[cfg(feature = "audio_capture")]
pub async fn record_mic(duration_secs: f32) -> Result<AudioBuffer> {
    info!("Recording for {} seconds", duration_secs);

    // Placeholder - actual implementation would use cpal
    // This allows the crate to compile without audio hardware
    tokio::time::sleep(std::time::Duration::from_secs_f32(duration_secs)).await;

    warn!("Audio recording not implemented - returning empty buffer");
    Ok(AudioBuffer::new())
}

/// Record with Voice Activity Detection
#[cfg(all(feature = "audio_capture", feature = "vad"))]
pub async fn record_with_vad(
    max_duration_secs: f32,
    silence_threshold_secs: f32,
) -> Result<AudioBuffer> {
    use crate::vad::VoiceActivityDetector;

    info!(
        "Recording with VAD: max={}s, silence={}s",
        max_duration_secs, silence_threshold_secs
    );

    let vad = VoiceActivityDetector::new()?;
    let buffer = AudioBuffer::new();

    // Placeholder - actual implementation would combine cpal + VAD
    warn!("VAD recording not fully implemented");

    Ok(buffer)
}

/// Play audio bytes (MP3 or WAV)
#[cfg(feature = "audio_playback")]
pub fn play_audio(audio_bytes: &[u8]) -> Result<()> {
    use rodio::{Decoder, OutputStream, Sink};
    use std::io::Cursor;

    let (_stream, stream_handle) = OutputStream::try_default()
        .context("Failed to get audio output device")?;

    let sink = Sink::try_new(&stream_handle)
        .context("Failed to create audio sink")?;

    let cursor = Cursor::new(audio_bytes.to_vec());
    let source = Decoder::new(cursor)
        .context("Failed to decode audio")?;

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}

/// Play audio asynchronously
#[cfg(feature = "audio_playback")]
pub async fn play_audio_async(audio_bytes: Vec<u8>) -> Result<()> {
    tokio::task::spawn_blocking(move || play_audio(&audio_bytes))
        .await
        .context("Audio playback task failed")?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_rms_empty() {
        assert_eq!(calculate_rms(&[]), 0.0);
    }

    #[test]
    fn test_calculate_rms_silence() {
        let samples = vec![0i16; 100];
        assert_eq!(calculate_rms(&samples), 0.0);
    }

    #[test]
    fn test_calculate_rms_max() {
        let samples = vec![32767i16; 100];
        let rms = calculate_rms(&samples);
        assert!(rms > 0.99 && rms <= 1.0);
    }

    #[test]
    fn test_audio_buffer_duration() {
        let mut buffer = AudioBuffer::new();
        buffer.samples = vec![0i16; 16000]; // 1 second at 16kHz
        assert!((buffer.duration_secs() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_to_mono_16k_passthrough() {
        let samples = vec![100i16, 200, 300];
        let result = to_mono_16k(&samples, 16000, 1);
        assert_eq!(result, samples);
    }

    #[test]
    fn test_to_mono_16k_stereo() {
        let stereo = vec![100i16, 200, 300, 400];
        let mono = to_mono_16k(&stereo, 16000, 2);
        assert_eq!(mono.len(), 2);
        assert_eq!(mono[0], 150); // (100 + 200) / 2
        assert_eq!(mono[1], 350); // (300 + 400) / 2
    }

    #[test]
    fn test_encode_wav() {
        let mut buffer = AudioBuffer::new();
        buffer.samples = vec![0, 100, -100, 0];
        let wav = encode_wav_pcm16_mono_16k(&buffer).unwrap();

        // Check RIFF header
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
    }
}
