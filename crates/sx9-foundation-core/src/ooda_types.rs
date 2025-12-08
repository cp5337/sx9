//! OODA Core Types - Under 100 lines ALWAYS
//! Observe, Orient, Decide, Act loop types

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OODALoop {
    pub loop_id: String,
    pub current_phase: OODAPhase,
    pub phase_history: Vec<PhaseTransition>,
    pub cognitive_state: CognitiveState,
    pub decision_matrix: DecisionMatrix,
    pub action_queue: Vec<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OODAPhase {
    Observe,
    Orient,
    Decide,
    Act,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseTransition {
    pub from_phase: OODAPhase,
    pub to_phase: OODAPhase,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub trigger: TransitionTrigger,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransitionTrigger {
    TimeElapsed,
    ThresholdReached,
    ExternalEvent,
    CognitiveDecision,
    SystemCommand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveState {
    pub awareness_level: AwarenessLevel,
    pub threat_assessment: ThreatAssessment,
    pub resource_availability: ResourceAvailability,
    pub operational_tempo: OperationalTempo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AwarenessLevel {
    Unaware,
    Situational,
    Tactical,
    Strategic,
    Predictive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAssessment {
    pub threat_level: ThreatLevel,
    pub threat_vectors: Vec<String>,
    pub confidence: f32,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAvailability {
    pub available_tools: Vec<String>,
    pub available_crates: Vec<String>,
    pub system_capacity: f32,
    pub operational_readiness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationalTempo {
    Steady,
    Accelerated,
    Crisis,
    Recovery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionMatrix {
    pub criteria: Vec<DecisionCriterion>,
    pub weights: HashMap<String, f32>,
    pub thresholds: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionCriterion {
    pub criterion_id: String,
    pub criterion_name: String,
    pub criterion_type: CriterionType,
    pub priority: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CriterionType {
    ThreatBased,
    ResourceBased,
    TimeBased,
    RiskBased,
    OpportunityBased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_id: String,
    pub action_type: ActionType,
    pub target: String,
    pub parameters: HashMap<String, String>,
    pub priority: ActionPriority,
    pub estimated_duration: std::time::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    ThreatResponse,
    ResourceAllocation,
    SystemReconfiguration,
    IntelligenceGathering,
    CountermeasureDeployment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionPriority {
    Immediate,
    High,
    Medium,
    Low,
}
