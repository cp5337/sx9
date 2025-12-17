//! Threat-Specific ECS Components for Plasma Defender
//!
//! These components are tuned for threat detection and response,
//! separate from the general-purpose sx9-plasma-ecs components.

use serde::{Deserialize, Serialize};

// =============================================================================
// CORE THREAT COMPONENTS
// =============================================================================

/// Threat entity component (Legion Layer 2)
/// Central component for tracking detected threats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEntityComponent {
    /// Unique entity ID
    pub entity_id: u64,
    /// Trivariate hash of the threat
    pub threat_hash: u64,
    /// Unicode trigger point (U+E000-E9FF)
    pub unicode_trigger: u32,
    /// 32-bit primitive bitfield (RFC-9001)
    pub primitive_bitfield: u64,
    /// Speed class for routing (0=Hot, 1=Warm, 2=Cold)
    pub speed_class: u8,
    /// SlotGraph slot ID
    pub slot_id: u64,
    /// Threat confidence score (0.0-1.0)
    pub confidence: f32,
    /// Timestamp when first detected
    pub first_seen_ns: u64,
    /// Timestamp of last activity
    pub last_seen_ns: u64,
}

impl Default for ThreatEntityComponent {
    fn default() -> Self {
        Self {
            entity_id: 0,
            threat_hash: 0,
            unicode_trigger: 0xE000,
            primitive_bitfield: 0,
            speed_class: 2, // Cold by default
            slot_id: 0,
            confidence: 0.0,
            first_seen_ns: 0,
            last_seen_ns: 0,
        }
    }
}

// =============================================================================
// OSSEC ALERT COMPONENTS
// =============================================================================

/// OSSEC alert component
/// Maps OSSEC rule triggers to ECS entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OssecAlertComponent {
    /// OSSEC rule ID
    pub rule_id: u32,
    /// Alert level (0-15)
    pub level: u8,
    /// MITRE ATT&CK technique (e.g., "T1059.001")
    pub mitre_technique: Option<String>,
    /// MITRE ATT&CK tactic (e.g., "execution")
    pub mitre_tactic: Option<String>,
    /// Rule description
    pub description: String,
    /// Source IP (if applicable)
    pub src_ip: Option<String>,
    /// Destination IP (if applicable)
    pub dst_ip: Option<String>,
    /// Timestamp of alert
    pub timestamp: u64,
    /// Raw alert data (JSON)
    pub raw_data: Option<String>,
}

impl Default for OssecAlertComponent {
    fn default() -> Self {
        Self {
            rule_id: 0,
            level: 0,
            mitre_technique: None,
            mitre_tactic: None,
            description: String::new(),
            src_ip: None,
            dst_ip: None,
            timestamp: 0,
            raw_data: None,
        }
    }
}

// =============================================================================
// HD4 KILL CHAIN COMPONENTS
// =============================================================================

/// HD4 phase enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum Hd4Phase {
    #[default]
    Hunt = 0,
    Detect = 1,
    Disrupt = 2,
    Disable = 3,
    Dominate = 4,
}

impl Hd4Phase {
    /// Y-axis value per RFC-9301 (0.0-1.0 normalized)
    pub fn y_axis(&self) -> f64 {
        match self {
            Self::Hunt => 0.0,
            Self::Detect => 0.25,
            Self::Disrupt => 0.5,
            Self::Disable => 0.75,
            Self::Dominate => 1.0,
        }
    }

    /// Delta angle for plasma state (degrees)
    pub fn delta_angle(&self) -> f32 {
        match self {
            Self::Hunt => 5.0,
            Self::Detect => 15.0,
            Self::Disrupt => 30.0,
            Self::Disable => 60.0,
            Self::Dominate => 90.0,
        }
    }

    /// Normalized delta angle (0.0-1.0) per RFC-9016
    pub fn delta_normalized(&self) -> f64 {
        self.y_axis()
    }
}

/// HD4 phase component for threat entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hd4PhaseComponent {
    /// Current HD4 phase
    pub phase: Hd4Phase,
    /// Time entered this phase
    pub entered_at: u64,
    /// Tick count in this phase
    pub ticks_in_phase: u64,
    /// Previous phase (for tracking transitions)
    pub previous_phase: Option<Hd4Phase>,
    /// Actions taken in this phase
    pub actions_taken: u32,
}

impl Default for Hd4PhaseComponent {
    fn default() -> Self {
        Self {
            phase: Hd4Phase::Hunt,
            entered_at: 0,
            ticks_in_phase: 0,
            previous_phase: None,
            actions_taken: 0,
        }
    }
}

// =============================================================================
// CRYSTAL EVALUATION COMPONENTS
// =============================================================================

/// Crystal family for threat evaluation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum DefensiveCrystalFamily {
    #[default]
    Defensive = 0, // Standard threat response
    Honeypot = 1,    // Deception/trap
    Sentinel = 2,    // High-priority monitoring
    Quarantine = 3,  // Isolation mode
    Retaliation = 4, // Active countermeasures
}

/// Crystal evaluation component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystalEvalComponent {
    /// Crystal family for this threat
    pub family: DefensiveCrystalFamily,
    /// Ring strength from crystal resonance
    pub ring_strength: f32,
    /// Resonance frequency
    pub resonance_frequency: f64,
    /// Last evaluation timestamp
    pub last_eval_ns: u64,
    /// Evaluation count
    pub eval_count: u32,
}

impl Default for CrystalEvalComponent {
    fn default() -> Self {
        Self {
            family: DefensiveCrystalFamily::Defensive,
            ring_strength: 0.0,
            resonance_frequency: 0.0,
            last_eval_ns: 0,
            eval_count: 0,
        }
    }
}

// =============================================================================
// SDT GATE COMPONENTS
// =============================================================================

/// SDT state for threat gates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[repr(u8)]
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

/// SDT gate component for threat latching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdtGateComponent {
    /// Gate ID
    pub gate_id: u32,
    /// Current state
    pub state: SdtState,
    /// Trigger threshold
    pub threshold: f32,
    /// Current accumulated value
    pub current_value: f32,
    /// Last trigger tick
    pub last_trigger_tick: u64,
    /// Trigger count
    pub trigger_count: u32,
}

impl Default for SdtGateComponent {
    fn default() -> Self {
        Self {
            gate_id: 0,
            state: SdtState::Off,
            threshold: 0.5,
            current_value: 0.0,
            last_trigger_tick: 0,
            trigger_count: 0,
        }
    }
}

// =============================================================================
// EEI CORRELATION COMPONENTS
// =============================================================================

/// EEI (Essential Elements of Information) correlation component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EeiCorrelationComponent {
    /// EEI query ID that matched
    pub eei_id: Option<u64>,
    /// Correlation score (0.0-1.0)
    pub correlation_score: f32,
    /// Matched keywords
    pub matched_keywords: Vec<String>,
    /// Time-of-Value remaining (ms)
    pub tov_remaining_ms: u64,
    /// Last correlation timestamp
    pub last_correlation_ns: u64,
}

impl Default for EeiCorrelationComponent {
    fn default() -> Self {
        Self {
            eei_id: None,
            correlation_score: 0.0,
            matched_keywords: Vec::new(),
            tov_remaining_ms: 0,
            last_correlation_ns: 0,
        }
    }
}

// =============================================================================
// TOOL OUTPUT COMPONENTS
// =============================================================================

/// Tool output component for processed tool results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutputComponent {
    /// Tool trivariate hash
    pub tool_hash: u64,
    /// Tool name
    pub tool_name: String,
    /// Output hash for deduplication
    pub output_hash: u64,
    /// Output size in bytes
    pub output_size: u32,
    /// MITRE technique from tool output
    pub mitre_technique: Option<String>,
    /// Parsed indicators
    pub indicators: Vec<String>,
    /// Processing timestamp
    pub processed_at: u64,
    /// JetStream sequence number (for replay)
    pub jetstream_seq: Option<u64>,
}

impl Default for ToolOutputComponent {
    fn default() -> Self {
        Self {
            tool_hash: 0,
            tool_name: String::new(),
            output_hash: 0,
            output_size: 0,
            mitre_technique: None,
            indicators: Vec::new(),
            processed_at: 0,
            jetstream_seq: None,
        }
    }
}

// =============================================================================
// ANN OBSERVER COMPONENTS (adapted from sx9-plasma-ecs)
// =============================================================================

/// ANN observer mode for threat pattern detection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ThreatObserverMode {
    #[default]
    Passive, // Just watching
    Active,     // Actively correlating
    Learning,   // Training on patterns
    Predictive, // Generating predictions
    Hunting,    // Active threat hunting
}

/// ANN observer component for threat patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatObserverComponent {
    /// Observer ID
    pub observer_id: String,
    /// Current mode
    pub mode: ThreatObserverMode,
    /// Observation count
    pub observation_count: u64,
    /// Last observation hash
    pub last_observation_hash: u64,
    /// Confidence score
    pub confidence_score: f32,
    /// Pattern buffer for ANN input
    pub pattern_buffer: Vec<f32>,
    /// Anomaly score (higher = more anomalous)
    pub anomaly_score: f32,
}

impl Default for ThreatObserverComponent {
    fn default() -> Self {
        Self {
            observer_id: String::new(),
            mode: ThreatObserverMode::Passive,
            observation_count: 0,
            last_observation_hash: 0,
            confidence_score: 0.0,
            pattern_buffer: Vec::with_capacity(128),
            anomaly_score: 0.0,
        }
    }
}

// =============================================================================
// AGENT COMPONENTS
// =============================================================================

/// Agent type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AgentType {
    #[default]
    NetworkMonitor,
    ThreatHunter,
    CanaryWatcher,
    AnomalyDetector,
    OssecAgent,
}

/// Agent component for threat monitoring agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentComponent {
    /// Agent ID
    pub agent_id: String,
    /// Agent type
    pub agent_type: AgentType,
    /// Is agent active
    pub active: bool,
    /// Last tick processed
    pub last_tick: u64,
    /// Events generated
    pub events_generated: u64,
    /// Last error (if any)
    pub last_error: Option<String>,
}

impl Default for AgentComponent {
    fn default() -> Self {
        Self {
            agent_id: String::new(),
            agent_type: AgentType::NetworkMonitor,
            active: false,
            last_tick: 0,
            events_generated: 0,
            last_error: None,
        }
    }
}
