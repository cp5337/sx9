//! Cognitive Computing Components
//!
//! Phi-3 integrated cognitive systems that represent thoughts, memories,
//! and reasoning processes as Legion ECS entities.

use chrono::{DateTime, Utc};
use legion::systems::{Runnable, SystemBuilder};
use legion::world::SubWorld;
use legion::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

/// Cognitive state of an entity (threats, agents, reasoning processes)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CognitiveState {
    /// Current cognitive load (0.0 to 1.0)
    pub cognitive_load: f32,

    /// Attention focus (entity being analyzed)
    pub attention_focus: Option<Entity>,

    /// Cognitive mode (analytical, reactive, predictive)
    pub mode: CognitiveMode,

    /// Processing priority (higher = more urgent)
    pub priority: f32,

    /// Last cognitive update timestamp
    pub last_update: DateTime<Utc>,
}

impl CognitiveState {
    pub fn new() -> Self {
        Self {
            cognitive_load: 0.0,
            attention_focus: None,
            mode: CognitiveMode::Analytical,
            priority: 0.5,
            last_update: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CognitiveMode {
    /// Deep analysis mode (high accuracy, slower)
    Analytical,

    /// Fast reaction mode (quick decisions)
    Reactive,

    /// Future prediction mode (scenario planning)
    Predictive,

    /// Pattern recognition mode (finding connections)
    Pattern,

    /// Learning mode (updating knowledge)
    Learning,
}

/// Thought process entity - represents active reasoning
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThoughtProcess {
    /// Unique thought ID
    pub id: Uuid,

    /// Subject of the thought (what entity is being reasoned about)
    pub subject: Entity,

    /// Current reasoning step
    pub current_step: usize,

    /// Reasoning chain (step-by-step analysis)
    pub reasoning_chain: Vec<ReasoningStep>,

    /// Confidence in current reasoning (0.0 to 1.0)
    pub confidence: f32,

    /// Processing deadline (for real-time constraints)
    pub deadline: DateTime<Utc>,

    /// Assigned to specific Phi-3 inference worker
    pub phi3_worker_id: Option<u32>,
}

impl ThoughtProcess {
    pub fn new(subject: Entity) -> Self {
        Self {
            id: Uuid::new_v4(),
            subject,
            current_step: 0,
            reasoning_chain: Vec::new(),
            confidence: 0.0,
            deadline: Utc::now() + chrono::Duration::milliseconds(100), // 100ms deadline
            phi3_worker_id: None,
        }
    }

    pub fn add_reasoning_step(&mut self, step: ReasoningStep) {
        self.reasoning_chain.push(step);
        self.current_step += 1;
        self.confidence = self.calculate_confidence();
    }

    fn calculate_confidence(&self) -> f32 {
        if self.reasoning_chain.is_empty() {
            return 0.0;
        }

        let avg_confidence: f32 = self
            .reasoning_chain
            .iter()
            .map(|step| step.confidence)
            .sum::<f32>()
            / self.reasoning_chain.len() as f32;

        // Decay confidence with reasoning chain length (avoid overthinking)
        let length_penalty = 1.0 - (self.reasoning_chain.len() as f32 * 0.1).min(0.3);
        avg_confidence * length_penalty
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReasoningStep {
    /// Step description
    pub description: String,

    /// Phi-3 generated reasoning text
    pub reasoning_text: String,

    /// Confidence in this step (0.0 to 1.0)
    pub confidence: f32,

    /// Related entities discovered in this step
    pub related_entities: Vec<Entity>,

    /// Timestamp when step was completed
    pub timestamp: DateTime<Utc>,

    /// Phi-3 model response metadata
    pub model_metadata: Phi3StepMetadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Phi3StepMetadata {
    /// Inference time in milliseconds
    pub inference_time_ms: f32,

    /// Token count in response
    pub token_count: usize,

    /// Model confidence score
    pub model_confidence: f32,

    /// Temperature used for generation
    pub temperature: f32,
}

/// Memory trace - represents learned knowledge and experiences
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryTrace {
    /// Unique memory ID
    pub id: Uuid,

    /// Memory type
    pub memory_type: MemoryType,

    /// Content of the memory
    pub content: MemoryContent,

    /// Strength of memory (0.0 to 1.0, decays over time)
    pub strength: f32,

    /// Number of times this memory has been accessed
    pub access_count: u32,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last access timestamp
    pub last_accessed: DateTime<Utc>,

    /// Associated entities
    pub associated_entities: Vec<Entity>,
}

impl MemoryTrace {
    pub fn new(memory_type: MemoryType, content: MemoryContent) -> Self {
        Self {
            id: Uuid::new_v4(),
            memory_type,
            content,
            strength: 1.0,
            access_count: 0,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            associated_entities: Vec::new(),
        }
    }

    pub fn access(&mut self) {
        self.access_count += 1;
        self.last_accessed = Utc::now();

        // Strengthen memory on access (spaced repetition)
        self.strength = (self.strength + 0.1).min(1.0);
    }

    pub fn decay(&mut self, decay_rate: f32) {
        self.strength = (self.strength - decay_rate).max(0.0);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MemoryType {
    /// Episodic memory (specific events and experiences)
    Episodic,

    /// Semantic memory (facts and knowledge)
    Semantic,

    /// Procedural memory (how to do things)
    Procedural,

    /// Working memory (temporary processing)
    Working,

    /// Pattern memory (recognized patterns)
    Pattern,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MemoryContent {
    /// Text-based memory (insights, facts, observations)
    Text(String),

    /// Structured threat analysis
    ThreatAnalysis {
        threat_id: String,
        analysis: String,
        severity_assessment: f32,
        recommended_actions: Vec<String>,
    },

    /// Relationship pattern
    RelationshipPattern {
        pattern_type: String,
        entities: Vec<Entity>,
        pattern_strength: f32,
        description: String,
    },

    /// Decision memory (what decisions were made and outcomes)
    Decision {
        decision: String,
        context: String,
        outcome: Option<String>,
        success_rate: f32,
    },
}

/// NASA MAV-inspired autonomous agent assignment
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssignedAgent {
    /// Agent type (based on NASA DSAS controller types)
    pub agent_type: AgentType,

    /// Agent instance ID
    pub agent_id: u32,

    /// Current workload (number of entities managed)
    pub workload: u32,

    /// Agent capability/efficiency rating
    pub efficiency: f32,

    /// Specialization areas
    pub specializations: Vec<String>,
}

impl AssignedAgent {
    pub fn threat_analyst() -> Self {
        Self {
            agent_type: AgentType::ThreatAnalyst,
            agent_id: rand::random(),
            workload: 0,
            efficiency: 1.0,
            specializations: vec!["malware_analysis".to_string(), "apt_tracking".to_string()],
        }
    }

    pub fn pattern_detector() -> Self {
        Self {
            agent_type: AgentType::PatternDetector,
            agent_id: rand::random(),
            workload: 0,
            efficiency: 1.0,
            specializations: vec![
                "correlation_analysis".to_string(),
                "temporal_patterns".to_string(),
            ],
        }
    }
}

/// Agent types inspired by NASA DSAS controller hierarchy
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AgentType {
    /// Strategic threat analysis (like DSAS TRACON controllers)
    ThreatAnalyst,

    /// Pattern detection and correlation (like enroute controllers)
    PatternDetector,

    /// Real-time decision making (like tower controllers)
    DecisionMaker,

    /// Memory and knowledge management
    MemoryManager,

    /// Coordination and planning
    Coordinator,
}

// Cognitive Systems for Legion Schedule

/// Thought generation system - creates new reasoning processes
pub fn thought_generation_system() -> impl Runnable {
    SystemBuilder::new("thought_generation")
        .with_query(<(Entity, &mut CognitiveState, Option<&ThoughtProcess>)>::query())
        .build(move |cmd, world, _resources, query| {
            for (entity, mut cognitive_state, thought_process) in query.iter_mut(world) {
                // Only create new thoughts if not already thinking and cognitive load allows
                if thought_process.is_none() && cognitive_state.cognitive_load < 0.8 {
                    // Create new thought process for this entity
                    cmd.add_component(*entity, ThoughtProcess::new(*entity));
                    cognitive_state.cognitive_load += 0.3;
                    cognitive_state.last_update = Utc::now();
                }
            }
        })
}

/// Memory consolidation system - manages memory decay and strengthening
pub fn memory_consolidation_system() -> impl Runnable {
    SystemBuilder::new("memory_consolidation")
        .with_query(<&mut MemoryTrace>::query())
        .build(move |_cmd, world, _resources, query| {
            for mut memory in query.iter_mut(world) {
                // Decay memory strength over time
                let time_since_access = Utc::now()
                    .signed_duration_since(memory.last_accessed)
                    .num_seconds() as f32;

                let decay_rate = match memory.memory_type {
                    MemoryType::Working => 0.01 * time_since_access / 60.0, // Fast decay for working memory
                    MemoryType::Episodic => 0.001 * time_since_access / 3600.0, // Slow decay for episodes
                    MemoryType::Semantic => 0.0001 * time_since_access / 86400.0, // Very slow for facts
                    _ => 0.005 * time_since_access / 1800.0, // Medium decay for others
                };

                memory.decay(decay_rate);
            }
        })
}

/// Reasoning chain system - advances thought processes
pub fn reasoning_chain_system() -> impl Runnable {
    SystemBuilder::new("reasoning_chain")
        .with_query(<&mut ThoughtProcess>::query())
        .build(move |_cmd, world, _resources, query| {
            for mut thought in query.iter_mut(world) {
                // Check if reasoning chain needs advancement
                if thought.current_step < 5 && Utc::now() < thought.deadline {
                    // This will be connected to Phi-3 inference in the next step
                    // For now, simulate reasoning step advancement
                    if thought.reasoning_chain.len() < 3 {
                        let step = ReasoningStep {
                            description: format!("Reasoning step {}", thought.current_step + 1),
                            reasoning_text: "Analyzing...".to_string(),
                            confidence: 0.8,
                            related_entities: Vec::new(),
                            timestamp: Utc::now(),
                            model_metadata: Phi3StepMetadata {
                                inference_time_ms: 15.0,
                                token_count: 50,
                                model_confidence: 0.85,
                                temperature: 0.7,
                            },
                        };
                        thought.add_reasoning_step(step);
                    }
                }
            }
        })
}

/// Inference dispatch system - queues thoughts for Phi-3 processing
pub fn inference_dispatch_system() -> impl Runnable {
    SystemBuilder::new("inference_dispatch")
        .with_query(<&mut ThoughtProcess>::query().filter(component::<CognitiveState>()))
        .build(move |_cmd, world, _resources, query| {
            for mut thought in query.iter_mut(world) {
                // Assign Phi-3 worker if not already assigned
                if thought.phi3_worker_id.is_none() && thought.reasoning_chain.len() < 3 {
                    // Round-robin assignment to Phi-3 workers
                    thought.phi3_worker_id = Some(thought.id.as_u128() as u32 % 4);
                    // 4 Phi-3 workers
                }
            }
        })
}
