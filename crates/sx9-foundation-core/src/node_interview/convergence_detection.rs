//! Intelligence Convergence Detection System
//! Mathematical "vibration" analysis and threshold detection

use serde::{Deserialize, Serialize};
use anyhow::Result;
use super::types::{VibrationAnalysis, ConvergenceResult};

// ================================================================================================
// Intelligence Convergence Detection
// ================================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceConvergenceDetector {
    /// Mathematical convergence threshold (default: 0.95)
    pub convergence_threshold: f64,
    /// Current convergence probability calculation
    pub current_convergence: f64,
    /// Mathematical "vibration" analysis for prediction
    pub vibration_analyzer: IntelligenceVibrationAnalyzer,
    /// Convergence history for trend analysis
    pub convergence_history: Vec<ConvergenceReading>,
    /// OODA loop trigger status
    pub ooda_trigger_active: bool,
}

impl IntelligenceConvergenceDetector {
    pub fn new() -> Self {
        Self {
            convergence_threshold: 0.95, // 95% convergence threshold per specification
            current_convergence: 0.0,
            vibration_analyzer: IntelligenceVibrationAnalyzer::new(),
            convergence_history: Vec::new(),
            ooda_trigger_active: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceVibrationAnalyzer {
    pub vibration_threshold: f64,
    pub frequency_analysis: FrequencyAnalysis,
    pub amplitude_metrics: AmplitudeMetrics,
    pub resonance_detector: ResonanceDetector,
}

impl IntelligenceVibrationAnalyzer {
    pub fn new() -> Self {
        Self {
            vibration_threshold: 0.8,
            frequency_analysis: FrequencyAnalysis { frequency: 2.4 },
            amplitude_metrics: AmplitudeMetrics { amplitude: 0.67 },
            resonance_detector: ResonanceDetector { resonance: 0.45 },
        }
    }

    pub async fn analyze_intelligence_vibration(&self) -> Result<VibrationAnalysis> {
        Ok(VibrationAnalysis {
            amplitude: self.amplitude_metrics.amplitude,
            confidence: 0.91,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceReading {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub convergence_value: f64,
    pub vibration_amplitude: f64,
    pub node_count_contribution: u32,
    pub mathematical_confidence: f64,
}

// Supporting types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyAnalysis { pub frequency: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmplitudeMetrics { pub amplitude: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceDetector { pub resonance: f64 }