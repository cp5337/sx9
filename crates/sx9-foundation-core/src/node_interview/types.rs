//! Core types for Node Interview Graph Detector System

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ================================================================================================
// Core Node Interview Types
// ================================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterviewBlanks {
    pub essential_elements: Vec<EssentialElement>,
    pub tactical_questions: Vec<TacticalQuestion>,
    pub context_probes: Vec<ContextProbe>,
    pub validation_checks: Vec<ValidationCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EssentialElement {
    pub eei_id: String,
    pub category: EEICategory,
    pub question: String,
    pub priority: Priority,
    pub data_type: String,
    pub validation_schema: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EEICategory {
    Geographic,
    Temporal,
    Functional,
    Relational,
    Operational,
    Technical,
    Tactical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
    Optional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalQuestion {
    pub question_id: String,
    pub domain: String,
    pub interrogation: String,
    pub expected_response_type: String,
    pub follow_up_triggers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextProbe {
    pub probe_id: String,
    pub target_attribute: String,
    pub extraction_method: String,
    pub validation_pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCheck {
    pub check_id: String,
    pub validation_type: String,
    pub constraint: String,
    pub error_handling: String,
}

// ================================================================================================
// Node State Types
// ================================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeState {
    pub node_id: String,
    pub state: NodeActivityState,
    pub mathematical_score: f64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub convergence_contribution: f64,
    pub eei_responses: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NodeActivityState {
    Normal,
    Investigating,
    Increasing,
    HighActivity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStateCounts {
    pub normal: u32,
    pub investigating: u32,
    pub increasing: u32,
    pub high_activity: u32,
    pub total: u32,
}

impl Default for NodeStateCounts {
    fn default() -> Self {
        Self { normal: 0, investigating: 0, increasing: 0, high_activity: 0, total: 0 }
    }
}

// ================================================================================================
// OODA Phase Types
// ================================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OODAPhase {
    Observe,
    Orient,
    Decide,
    Act,
}

// ================================================================================================
// Graph Algorithm Types
// ================================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphAlgorithm {
    KNNClustering,
    AStarPathfinding,
    MatroidOptimization,
    ARIMAForecasting,
    CUSUMChangePointDetection,
    FloydWarshallAllPairs,
    BellmanFordShortestPath,
}

// ================================================================================================
// Result Types
// ================================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceResult {
    pub convergence_probability: f64,
    pub vibration_analysis: VibrationAnalysis,
    pub node_analysis: NodeAnalysis,
    pub ooda_triggered: bool,
    pub mathematical_trace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInterviewResult {
    pub node_id: String,
    pub node_state: NodeState,
    pub eei_analysis: EEIAnalysisResult,
    pub adversary_narrative: AdversaryNarrative,
    pub mathematical_classification: NodeMathematicalClassification,
    pub convergence_impact: ConvergenceImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMathematicalClassification {
    pub activity_state: NodeActivityState,
    pub mathematical_score: f64,
    pub convergence_contribution: f64,
    pub cluster_assignment: u32,
    pub pattern_confidence: f64,
}

// ================================================================================================
// Supporting Analysis Types
// ================================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibrationAnalysis { pub amplitude: f64, pub confidence: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAnalysis { pub active_nodes: u32 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EEIAnalysisResult {
    pub responses: HashMap<String, Value>,
    pub patterns: Vec<String>,
    pub metrics: Vec<f64>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdversaryNarrative { pub narrative: String }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceImpact {
    pub probability_delta: f64,
    pub risk_assessment: String,
    pub recommended_actions: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterAnalysis { pub cluster_id: u32, pub confidence: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternSynthesis { pub convergence_weight: f64, pub confidence: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub probability_change: f64,
    pub risk_level: String,
    pub recommended_actions: Vec<String>
}