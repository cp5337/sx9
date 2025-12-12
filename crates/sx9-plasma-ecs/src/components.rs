//! ECS Components for Plasma State
//!
//! Defines component types for Plasma entities with serde support.
//! Uses local serializable versions of atlas-bus types.

use serde::{Deserialize, Serialize};

// Local serializable versions of atlas-bus types
// These mirror the atlas-bus types but with serde derives

/// SDT state (local serializable version)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SdtState {
    #[default]
    Off = 0,
    Primed = 1,
    Conducting = 2,
    Latched = 3,
}

impl From<u8> for SdtState {
    fn from(v: u8) -> Self {
        match v {
            0 => SdtState::Off,
            1 => SdtState::Primed,
            2 => SdtState::Conducting,
            3 => SdtState::Latched,
            _ => SdtState::Off,
        }
    }
}

/// Crystal family (local serializable version)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CrystalFamily {
    #[default]
    Orbital,
    GroundStation,
    TarPit,
    Silent,
    Resonant,
    Critical,
}

impl From<u8> for CrystalFamily {
    fn from(v: u8) -> Self {
        match v {
            0 => CrystalFamily::Orbital,
            1 => CrystalFamily::GroundStation,
            2 => CrystalFamily::TarPit,
            3 => CrystalFamily::Silent,
            4 => CrystalFamily::Resonant,
            5 => CrystalFamily::Critical,
            _ => CrystalFamily::Orbital,
        }
    }
}

/// Plasma entity component (Legion)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlasmaComponent {
    pub delta_angle: u16,
    pub entropy: u32,
    pub sdt_state: SdtState,
    pub crystal_family: CrystalFamily,
}

impl Default for PlasmaComponent {
    fn default() -> Self {
        Self {
            delta_angle: 0,
            entropy: 0,
            sdt_state: SdtState::Off,
            crystal_family: CrystalFamily::Orbital,
        }
    }
}

/// Threat agent component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAgentComponent {
    pub agent_id: String,
    pub agent_type: String,
    pub threat_level: f32,
}

/// Crystal resonance component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystalResonanceComponent {
    pub ring_strength: f32,
    pub resonance_frequency: f64,
}

/// SDT gate component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdtGateComponent {
    pub state: SdtState,
    pub threshold: f32,
    pub current_value: f32,
}

// ANN Observer Components (RFC-9114 Rev 1.1)
// Observer tracks neural patterns across entity state changes

/// ANN observer mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AnnObserverMode {
    #[default]
    Passive,
    Active,
    Learning,
    Predictive,
}

/// ANN neuron activation component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnNeuronComponent {
    pub layer_id: u32,
    pub neuron_id: u32,
    pub activation: f32,
    pub bias: f32,
    pub weights: Vec<f32>,
}

impl Default for AnnNeuronComponent {
    fn default() -> Self {
        Self {
            layer_id: 0,
            neuron_id: 0,
            activation: 0.0,
            bias: 0.0,
            weights: Vec::new(),
        }
    }
}

/// ANN observer component for tracking entity patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnObserverComponent {
    pub observer_id: String,
    pub mode: AnnObserverMode,
    pub observation_count: u64,
    pub last_observation_hash: u64,
    pub confidence_score: f32,
    pub pattern_buffer: Vec<f32>,
}

impl Default for AnnObserverComponent {
    fn default() -> Self {
        Self {
            observer_id: String::new(),
            mode: AnnObserverMode::Passive,
            observation_count: 0,
            last_observation_hash: 0,
            confidence_score: 0.0,
            pattern_buffer: Vec::with_capacity(128),
        }
    }
}

/// ANN layer configuration component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnLayerComponent {
    pub layer_id: u32,
    pub layer_type: AnnLayerType,
    pub input_size: usize,
    pub output_size: usize,
    pub activation_fn: ActivationFunction,
}

/// ANN layer type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AnnLayerType {
    #[default]
    Dense,
    Convolutional,
    Recurrent,
    Attention,
}

/// Activation function enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ActivationFunction {
    #[default]
    ReLU,
    Sigmoid,
    Tanh,
    Softmax,
    Linear,
}

/// ANN prediction output component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnPredictionComponent {
    pub prediction_id: u64,
    pub output_vector: Vec<f32>,
    pub confidence: f32,
    pub timestamp_ns: u64,
}

impl Default for AnnPredictionComponent {
    fn default() -> Self {
        Self {
            prediction_id: 0,
            output_vector: Vec::new(),
            confidence: 0.0,
            timestamp_ns: 0,
        }
    }
}
