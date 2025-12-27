//! Voice Activity Detection
//!
//! RFC-9107 §5.1: VAD for speech endpoint detection.

use anyhow::Result;
use thiserror::Error;
use tracing::debug;

/// VAD errors
#[derive(Error, Debug)]
pub enum VadError {
    #[error("VAD initialization failed: {0}")]
    InitFailed(String),

    #[error("Invalid sample rate: {0} (must be 8000, 16000, 32000, or 48000)")]
    InvalidSampleRate(u32),

    #[error("Invalid frame length")]
    InvalidFrameLength,
}

/// VAD aggressiveness mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VadMode {
    /// Quality mode - less aggressive, fewer false negatives
    Quality = 0,
    /// Low bitrate mode
    LowBitrate = 1,
    /// Aggressive mode
    Aggressive = 2,
    /// Very aggressive mode - more false negatives but fewer false positives
    VeryAggressive = 3,
}

impl Default for VadMode {
    fn default() -> Self {
        VadMode::Aggressive
    }
}

/// Voice Activity Detector
pub struct VoiceActivityDetector {
    mode: VadMode,
    sample_rate: u32,
    frame_duration_ms: u32,
    /// Speech probability threshold (0.0-1.0)
    threshold: f32,
    /// Frames of speech needed to trigger
    hangover_frames: u32,
    /// Current speech frame count
    speech_frames: u32,
    /// Is currently in speech
    in_speech: bool,
}

impl VoiceActivityDetector {
    /// Create a new VAD with default settings
    pub fn new() -> Result<Self, VadError> {
        Self::with_config(VadMode::default(), 16000, 30)
    }

    /// Create VAD with custom configuration
    pub fn with_config(mode: VadMode, sample_rate: u32, frame_duration_ms: u32) -> Result<Self, VadError> {
        // Validate sample rate
        if ![8000, 16000, 32000, 48000].contains(&sample_rate) {
            return Err(VadError::InvalidSampleRate(sample_rate));
        }

        Ok(Self {
            mode,
            sample_rate,
            frame_duration_ms,
            threshold: 0.5,
            hangover_frames: 10,
            speech_frames: 0,
            in_speech: false,
        })
    }

    /// Set speech probability threshold
    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold.clamp(0.0, 1.0);
    }

    /// Set hangover frames (speech frames needed to trigger)
    pub fn set_hangover(&mut self, frames: u32) {
        self.hangover_frames = frames;
    }

    /// Process a frame of audio and detect voice activity
    ///
    /// Returns true if voice activity detected in this frame.
    pub fn process_frame(&mut self, samples: &[i16]) -> Result<bool, VadError> {
        let expected_samples = (self.sample_rate * self.frame_duration_ms / 1000) as usize;

        if samples.len() != expected_samples {
            debug!(
                "Frame length mismatch: expected {}, got {}",
                expected_samples,
                samples.len()
            );
        }

        // Calculate energy-based VAD (simplified)
        let energy = self.calculate_frame_energy(samples);
        let threshold = self.get_energy_threshold();

        let is_speech = energy > threshold;

        // Apply hangover logic
        if is_speech {
            self.speech_frames = self.speech_frames.saturating_add(1);
            if self.speech_frames >= self.hangover_frames {
                self.in_speech = true;
            }
        } else {
            self.speech_frames = self.speech_frames.saturating_sub(1);
            if self.speech_frames == 0 {
                self.in_speech = false;
            }
        }

        Ok(self.in_speech)
    }

    /// Check if currently in speech segment
    pub fn is_speech(&self) -> bool {
        self.in_speech
    }

    /// Reset VAD state
    pub fn reset(&mut self) {
        self.speech_frames = 0;
        self.in_speech = false;
    }

    /// Calculate frame energy (RMS)
    fn calculate_frame_energy(&self, samples: &[i16]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }

        let sum_squares: f64 = samples
            .iter()
            .map(|&s| (s as f64).powi(2))
            .sum();

        let rms = (sum_squares / samples.len() as f64).sqrt();
        (rms / 32767.0) as f32
    }

    /// Get energy threshold based on mode
    fn get_energy_threshold(&self) -> f32 {
        match self.mode {
            VadMode::Quality => 0.01,
            VadMode::LowBitrate => 0.02,
            VadMode::Aggressive => 0.03,
            VadMode::VeryAggressive => 0.05,
        }
    }

    /// Get frame size in samples
    pub fn frame_size(&self) -> usize {
        (self.sample_rate * self.frame_duration_ms / 1000) as usize
    }

    /// Get sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

impl Default for VoiceActivityDetector {
    fn default() -> Self {
        Self::new().expect("Default VAD config should be valid")
    }
}

/// Detect speech segments in audio buffer
pub fn detect_speech_segments(
    samples: &[i16],
    sample_rate: u32,
) -> Result<Vec<(usize, usize)>, VadError> {
    let mut vad = VoiceActivityDetector::with_config(VadMode::Aggressive, sample_rate, 30)?;
    let frame_size = vad.frame_size();
    let mut segments = Vec::new();
    let mut segment_start: Option<usize> = None;

    for (i, frame) in samples.chunks(frame_size).enumerate() {
        if frame.len() < frame_size {
            break; // Skip incomplete final frame
        }

        let is_speech = vad.process_frame(frame)?;
        let sample_pos = i * frame_size;

        if is_speech && segment_start.is_none() {
            segment_start = Some(sample_pos);
        } else if !is_speech && segment_start.is_some() {
            segments.push((segment_start.unwrap(), sample_pos));
            segment_start = None;
        }
    }

    // Close any open segment
    if let Some(start) = segment_start {
        segments.push((start, samples.len()));
    }

    Ok(segments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vad_creation() {
        let vad = VoiceActivityDetector::new().unwrap();
        assert_eq!(vad.sample_rate(), 16000);
    }

    #[test]
    fn test_vad_invalid_sample_rate() {
        let result = VoiceActivityDetector::with_config(VadMode::default(), 22050, 30);
        assert!(result.is_err());
    }

    #[test]
    fn test_vad_silence() {
        let mut vad = VoiceActivityDetector::new().unwrap();
        let silence = vec![0i16; vad.frame_size()];
        let result = vad.process_frame(&silence).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_vad_loud() {
        let mut vad = VoiceActivityDetector::new().unwrap();
        let loud = vec![10000i16; vad.frame_size()];

        // Process multiple frames to trigger hangover
        for _ in 0..15 {
            vad.process_frame(&loud).unwrap();
        }

        assert!(vad.is_speech());
    }

    #[test]
    fn test_detect_segments() {
        let sample_rate = 16000;
        let frame_size = 480; // 30ms at 16kHz

        // Create audio with silence, speech, silence
        let mut samples = vec![0i16; frame_size * 5]; // Silence
        samples.extend(vec![10000i16; frame_size * 20]); // Speech
        samples.extend(vec![0i16; frame_size * 5]); // Silence

        let segments = detect_speech_segments(&samples, sample_rate).unwrap();
        assert!(!segments.is_empty());
    }
}
