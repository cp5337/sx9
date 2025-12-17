//! OODA Processor - Observe, Orient, Decide, Act loop processing
//! Manages cognitive state transitions and decision making

use crate::ooda_types::*;
use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use chrono::{DateTime, Utc};

pub struct OODAProcessor {
    pub active_loops: Arc<RwLock<HashMap<String, OODALoop>>>,
    pub decision_engine: Arc<RwLock<DecisionEngine>>,
    pub cognitive_state_manager: Arc<RwLock<CognitiveStateManager>>,
}

#[derive(Debug, Clone)]
pub struct DecisionEngine {
    pub decision_rules: Vec<DecisionRule>,
    pub threshold_config: ThresholdConfig,
    pub action_templates: HashMap<String, ActionTemplate>,
}

#[derive(Debug, Clone)]
pub struct DecisionRule {
    pub rule_id: String,
    pub rule_name: String,
    pub conditions: Vec<DecisionCondition>,
    pub actions: Vec<String>,
    pub priority: f32,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct DecisionCondition {
    pub condition_type: ConditionType,
    pub threshold: f32,
    pub comparison: ComparisonOperator,
    pub target_value: String,
}

#[derive(Debug, Clone)]
pub enum ConditionType {
    ThreatLevel,
    ResourceAvailability,
    TimeElapsed,
    ConfidenceLevel,
    OperationalTempo,
}

#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

#[derive(Debug, Clone)]
pub struct ThresholdConfig {
    pub threat_thresholds: HashMap<ThreatLevel, f32>,
    pub resource_thresholds: HashMap<String, f32>,
    pub time_thresholds: HashMap<String, std::time::Duration>,
    pub confidence_thresholds: HashMap<String, f32>,
}

#[derive(Debug, Clone)]
pub struct ActionTemplate {
    pub template_id: String,
    pub action_type: ActionType,
    pub parameters: HashMap<String, String>,
    pub estimated_duration: std::time::Duration,
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CognitiveStateManager {
    pub state_history: Vec<CognitiveStateSnapshot>,
    pub transition_rules: Vec<TransitionRule>,
    pub awareness_models: HashMap<String, AwarenessModel>,
}

#[derive(Debug, Clone)]
pub struct CognitiveStateSnapshot {
    pub timestamp: DateTime<Utc>,
    pub cognitive_state: CognitiveState,
    pub trigger: String,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct TransitionRule {
    pub rule_id: String,
    pub from_awareness: AwarenessLevel,
    pub to_awareness: AwarenessLevel,
    pub conditions: Vec<TransitionCondition>,
    pub probability: f32,
}

#[derive(Debug, Clone)]
pub struct TransitionCondition {
    pub condition_type: TransitionConditionType,
    pub threshold: f32,
    pub operator: ComparisonOperator,
}

#[derive(Debug, Clone)]
pub enum TransitionConditionType {
    ThreatIncrease,
    ResourceDecrease,
    TimePressure,
    ConfidenceDrop,
    ExternalEvent,
}

#[derive(Debug, Clone)]
pub struct AwarenessModel {
    pub model_id: String,
    pub awareness_level: AwarenessLevel,
    pub information_processing_rate: f32,
    pub decision_making_speed: f32,
    pub response_capability: f32,
}

impl OODAProcessor {
    pub fn new() -> Self {
        Self {
            active_loops: Arc::new(RwLock::new(HashMap::new())),
            decision_engine: Arc::new(RwLock::new(DecisionEngine::new())),
            cognitive_state_manager: Arc::new(RwLock::new(CognitiveStateManager::new())),
        }
    }

    /// Create a new OODA loop
    pub async fn create_loop(&self, loop_id: String) -> Result<OODALoop> {
        let ooda_loop = OODALoop {
            loop_id: loop_id.clone(),
            current_phase: OODAPhase::Observe,
            phase_history: Vec::new(),
            cognitive_state: CognitiveState::default(),
            decision_matrix: DecisionMatrix::default(),
            action_queue: Vec::new(),
        };

        let mut loops = self.active_loops.write().await;
        loops.insert(loop_id.clone(), ooda_loop.clone());
        Ok(ooda_loop)
    }

    /// Process the current phase of an OODA loop
    pub async fn process_phase(&self, loop_id: &str) -> Result<PhaseProcessingResult> {
        let mut loops = self.active_loops.write().await;
        let loop_ref = loops.get_mut(loop_id)
            .ok_or_else(|| anyhow::anyhow!("OODA loop not found: {}", loop_id))?;

        let current_phase = loop_ref.current_phase.clone();
        let processing_result = match current_phase {
            OODAPhase::Observe => self.process_observe_phase(loop_ref).await?,
            OODAPhase::Orient => self.process_orient_phase(loop_ref).await?,
            OODAPhase::Decide => self.process_decide_phase(loop_ref).await?,
            OODAPhase::Act => self.process_act_phase(loop_ref).await?,
        };

        // Record phase transition
        let transition = PhaseTransition {
            from_phase: current_phase,
            to_phase: processing_result.next_phase.clone(),
            timestamp: Utc::now(),
            trigger: processing_result.transition_trigger.clone(),
            confidence: processing_result.confidence,
        };

        loop_ref.phase_history.push(transition);
        loop_ref.current_phase = processing_result.next_phase;

        Ok(processing_result)
    }

    /// Process the Observe phase
    async fn process_observe_phase(&self, ooda_loop: &mut OODALoop) -> Result<PhaseProcessingResult> {
        let cognitive_manager = self.cognitive_state_manager.read().await;
        
        // Update cognitive state based on observations
        let new_awareness = self.calculate_awareness_level(&ooda_loop.cognitive_state);
        ooda_loop.cognitive_state.awareness_level = new_awareness;

        // Determine if we should transition to Orient
        let should_transition = self.should_transition_to_orient(&ooda_loop.cognitive_state).await?;
        
        let next_phase = if should_transition {
            OODAPhase::Orient
        } else {
            OODAPhase::Observe
        };

        Ok(PhaseProcessingResult {
            next_phase,
            transition_trigger: TransitionTrigger::ThresholdReached,
            confidence: 0.8,
            actions_generated: Vec::new(),
        })
    }

    /// Process the Orient phase
    async fn process_orient_phase(&self, ooda_loop: &mut OODALoop) -> Result<PhaseProcessingResult> {
        // Update threat assessment and resource availability
        self.update_threat_assessment(&mut ooda_loop.cognitive_state).await?;
        self.update_resource_availability(&mut ooda_loop.cognitive_state).await?;

        // Determine if we should transition to Decide
        let should_transition = self.should_transition_to_decide(&ooda_loop.cognitive_state).await?;
        
        let next_phase = if should_transition {
            OODAPhase::Decide
        } else {
            OODAPhase::Orient
        };

        Ok(PhaseProcessingResult {
            next_phase,
            transition_trigger: TransitionTrigger::CognitiveDecision,
            confidence: 0.85,
            actions_generated: Vec::new(),
        })
    }

    /// Process the Decide phase
    async fn process_decide_phase(&self, ooda_loop: &mut OODALoop) -> Result<PhaseProcessingResult> {
        let decision_engine = self.decision_engine.read().await;
        
        // Generate actions based on decision matrix
        let actions = self.generate_actions(&ooda_loop.cognitive_state, &ooda_loop.decision_matrix).await?;
        ooda_loop.action_queue.extend(actions.clone());

        // Determine if we should transition to Act
        let should_transition = !actions.is_empty();
        
        let next_phase = if should_transition {
            OODAPhase::Act
        } else {
            OODAPhase::Decide
        };

        Ok(PhaseProcessingResult {
            next_phase,
            transition_trigger: TransitionTrigger::SystemCommand,
            confidence: 0.9,
            actions_generated: actions,
        })
    }

    /// Process the Act phase
    async fn process_act_phase(&self, ooda_loop: &mut OODALoop) -> Result<PhaseProcessingResult> {
        // Execute actions from the queue
        let executed_actions = self.execute_actions(&mut ooda_loop.action_queue).await?;

        // Update operational tempo based on actions taken
        self.update_operational_tempo(&mut ooda_loop.cognitive_state, &executed_actions).await?;

        // Determine if we should cycle back to Observe
        let should_cycle = self.should_cycle_to_observe(&ooda_loop.cognitive_state).await?;
        
        let next_phase = if should_cycle {
            OODAPhase::Observe
        } else {
            OODAPhase::Act
        };

        Ok(PhaseProcessingResult {
            next_phase,
            transition_trigger: TransitionTrigger::TimeElapsed,
            confidence: 0.95,
            actions_generated: executed_actions,
        })
    }

    /// Calculate awareness level based on cognitive state
    fn calculate_awareness_level(&self, cognitive_state: &CognitiveState) -> AwarenessLevel {
        let threat_score = match cognitive_state.threat_assessment.threat_level {
            ThreatLevel::None => 0.0,
            ThreatLevel::Low => 0.25,
            ThreatLevel::Medium => 0.5,
            ThreatLevel::High => 0.75,
            ThreatLevel::Critical => 1.0,
        };

        let resource_score = cognitive_state.resource_availability.operational_readiness;
        let overall_score = (threat_score + resource_score) / 2.0;

        match overall_score {
            s if s < 0.2 => AwarenessLevel::Unaware,
            s if s < 0.4 => AwarenessLevel::Situational,
            s if s < 0.6 => AwarenessLevel::Tactical,
            s if s < 0.8 => AwarenessLevel::Strategic,
            _ => AwarenessLevel::Predictive,
        }
    }

    /// Check if should transition to Orient phase
    async fn should_transition_to_orient(&self, cognitive_state: &CognitiveState) -> Result<bool> {
        // Transition if awareness level is sufficient
        Ok(matches!(cognitive_state.awareness_level, 
            AwarenessLevel::Situational | AwarenessLevel::Tactical | 
            AwarenessLevel::Strategic | AwarenessLevel::Predictive))
    }

    /// Check if should transition to Decide phase
    ///
    /// RFC-9025: "Convergence Limit" is set at 75% (0.75).
    /// Decisions MUST NOT be made until 75% of required information is available and verified.
    async fn should_transition_to_decide(&self, cognitive_state: &CognitiveState) -> Result<bool> {
        // Enforce 75% Convergence Threshold
        Ok(cognitive_state.threat_assessment.confidence >= 0.75 && 
           cognitive_state.resource_availability.operational_readiness >= 0.75)
    }

    /// Check if should cycle back to Observe phase
    async fn should_cycle_to_observe(&self, cognitive_state: &CognitiveState) -> Result<bool> {
        // Cycle back if actions are complete or new information is needed
        Ok(cognitive_state.threat_assessment.confidence < 0.5 || 
           cognitive_state.resource_availability.operational_readiness < 0.3)
    }

    /// Update threat assessment
    async fn update_threat_assessment(&self, cognitive_state: &mut CognitiveState) -> Result<()> {
        // Implementation would integrate with threat intelligence systems
        cognitive_state.threat_assessment.last_updated = Utc::now();
        Ok(())
    }

    /// Update resource availability
    async fn update_resource_availability(&self, cognitive_state: &mut CognitiveState) -> Result<()> {
        // Implementation would check actual system resources
        Ok(())
    }

    /// Generate actions based on cognitive state and decision matrix
    async fn generate_actions(&self, cognitive_state: &CognitiveState, decision_matrix: &DecisionMatrix) -> Result<Vec<Action>> {
        let mut actions = Vec::new();
        
        // Generate actions based on threat level
        match cognitive_state.threat_assessment.threat_level {
            ThreatLevel::Critical => {
                actions.push(Action {
                    action_id: format!("threat_response_{}", Utc::now().timestamp()),
                    action_type: ActionType::ThreatResponse,
                    target: "system".to_string(),
                    parameters: HashMap::new(),
                    priority: ActionPriority::Immediate,
                    estimated_duration: std::time::Duration::from_secs(30),
                });
            },
            ThreatLevel::High => {
                actions.push(Action {
                    action_id: format!("intelligence_gathering_{}", Utc::now().timestamp()),
                    action_type: ActionType::IntelligenceGathering,
                    target: "network".to_string(),
                    parameters: HashMap::new(),
                    priority: ActionPriority::High,
                    estimated_duration: std::time::Duration::from_secs(60),
                });
            },
            _ => {}
        }

        Ok(actions)
    }

    /// Execute actions from the queue
    async fn execute_actions(&self, action_queue: &mut Vec<Action>) -> Result<Vec<Action>> {
        let mut executed = Vec::new();
        
        while let Some(action) = action_queue.pop() {
            // Execute the action (implementation would integrate with actual systems)
            executed.push(action);
        }

        Ok(executed)
    }

    /// Update operational tempo based on executed actions
    async fn update_operational_tempo(&self, cognitive_state: &mut CognitiveState, executed_actions: &[Action]) -> Result<()> {
        // Update operational tempo based on action types and frequency
        if executed_actions.iter().any(|a| matches!(a.priority, ActionPriority::Immediate)) {
            cognitive_state.operational_tempo = OperationalTempo::Crisis;
        } else if executed_actions.len() > 3 {
            cognitive_state.operational_tempo = OperationalTempo::Accelerated;
        } else {
            cognitive_state.operational_tempo = OperationalTempo::Steady;
        }
        Ok(())
    }

    /// Get active OODA loop by ID
    pub async fn get_loop(&self, loop_id: &str) -> Result<Option<OODALoop>> {
        let loops = self.active_loops.read().await;
        Ok(loops.get(loop_id).cloned())
    }

    /// Get all active OODA loops
    pub async fn get_all_loops(&self) -> Result<Vec<OODALoop>> {
        let loops = self.active_loops.read().await;
        Ok(loops.values().cloned().collect())
    }
}

#[derive(Debug, Clone)]
pub struct PhaseProcessingResult {
    pub next_phase: OODAPhase,
    pub transition_trigger: TransitionTrigger,
    pub confidence: f32,
    pub actions_generated: Vec<Action>,
}

impl Default for CognitiveState {
    fn default() -> Self {
        Self {
            awareness_level: AwarenessLevel::Unaware,
            threat_assessment: ThreatAssessment {
                threat_level: ThreatLevel::None,
                threat_vectors: Vec::new(),
                confidence: 0.0,
                last_updated: Utc::now(),
            },
            resource_availability: ResourceAvailability {
                available_tools: Vec::new(),
                available_crates: Vec::new(),
                system_capacity: 1.0,
                operational_readiness: 1.0,
            },
            operational_tempo: OperationalTempo::Steady,
        }
    }
}

impl Default for DecisionMatrix {
    fn default() -> Self {
        Self {
            criteria: Vec::new(),
            weights: HashMap::new(),
            thresholds: HashMap::new(),
        }
    }
}

impl DecisionEngine {
    fn new() -> Self {
        Self {
            decision_rules: Vec::new(),
            threshold_config: ThresholdConfig {
                threat_thresholds: HashMap::new(),
                resource_thresholds: HashMap::new(),
                time_thresholds: HashMap::new(),
                confidence_thresholds: HashMap::new(),
            },
            action_templates: HashMap::new(),
        }
    }
}

impl CognitiveStateManager {
    fn new() -> Self {
        Self {
            state_history: Vec::new(),
            transition_rules: Vec::new(),
            awareness_models: HashMap::new(),
        }
    }
}
