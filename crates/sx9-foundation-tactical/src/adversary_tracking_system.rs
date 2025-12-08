//! CTAS Adversary Tracking System - 1st Person (1n) vs 2nd Person (2n) Paradigms
//! 
//! Demonstrates the difference between tracking adversaries (1n) and executing tasks (2n)

use bevy::prelude::*;
use crate::slotgraph_task_tool_mapper::{SlotGraphTaskMapper, TaskExecutionStrategy};
use ctas_slotgraph_tools::{
    node_types::{TaskMetadata, HD4Phase, TaskPriority, SlotGraphNode, SlotNodeType, NodeState, NodeStatus},
    cognigraph::universal_nodes::{UniversalNodeType, NodeFunctionExecutor}
};
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// **1ST PERSON (1n) - ADVERSARY TRACKING**
/// "I am the adversary being tracked by CTAS"
/// Focus: Persona-driven behavior simulation, first-person narrative

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct AdversaryPersona {
    pub persona_id: Uuid,
    pub name: String,
    pub persona_type: AdversaryType,
    pub current_objective: String,
    pub psychological_state: PsychologicalState,
    pub capabilities: Vec<String>,
    pub behavioral_patterns: Vec<String>,
    pub knowledge_base: HashMap<String, String>,
    pub narrative_state: String, // "I am currently..." first person narrative
    pub environmental_awareness: EnvironmentalAwareness,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdversaryType {
    JihadistOperator,
    NationStateAPT,
    CybercriminalGroup,
    InsiderThreat,
    HacktivistCell,
    OrganizedCrime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsychologicalState {
    pub stress_level: f32,      // 0.0 to 1.0
    pub confidence: f32,        // 0.0 to 1.0
    pub paranoia_level: f32,    // 0.0 to 1.0
    pub operational_tempo: f32, // 0.0 to 1.0
    pub risk_tolerance: f32,    // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalAwareness {
    pub metoc_sensitivity: f32,     // Weather/oceanographic impact
    pub traffic_awareness: f32,     // Network traffic consciousness
    pub illumination_dependency: f32, // Daylight/darkness preference
    pub resource_constraints: f32,  // Available resources awareness
}

/// **2ND PERSON (2n) - TASK EXECUTION**  
/// "Execute this scan task" - operator commanding system
/// Focus: Command-driven execution, imperative actions

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct OperatorTask {
    pub task_id: Uuid,
    pub command: String,           // "Execute port scan on target"
    pub target: String,
    pub operator_id: String,
    pub task_type: TaskType,
    pub execution_parameters: HashMap<String, String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub expected_outcome: String,  // "System will scan and report results"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    PortScan,
    VulnerabilityAssessment,
    NetworkReconnaissance,
    ExploitDelivery,
    DataExfiltration,
    PersistenceEstablishment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Queued,
    Executing,
    Completed,
    Failed,
    Cancelled,
}

/// System for 1st Person Adversary Simulation
pub fn adversary_simulation_system(
    mut adversary_query: Query<(Entity, &mut AdversaryPersona, &mut NodeState)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut adversary, mut node_state) in adversary_query.iter_mut() {
        // Update adversary psychological state based on environment
        update_adversary_psychology(&mut adversary, time.elapsed_seconds());
        
        // Generate first-person narrative
        adversary.narrative_state = generate_adversary_narrative(&adversary);
        
        // Make autonomous decisions based on persona
        let decision = make_adversary_decision(&adversary);
        if let Some(action) = decision {
            info!("🎭 {} decides: {}", adversary.name, action);
            
            // Adversary spawns their own actions (1st person agency)
            spawn_adversary_action(&mut commands, entity, &adversary, action);
        }
        
        // Update node state to reflect adversary activity
        node_state.last_update = time.elapsed_seconds();
        
        // Log first-person perspective
        if time.elapsed_seconds() as u32 % 5 == 0 {
            info!("🎭 {}: {}", adversary.name, adversary.narrative_state);
        }
    }
}

/// System for 2nd Person Task Execution
pub fn operator_task_execution_system(
    mut task_query: Query<(Entity, &mut OperatorTask, &mut NodeState)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut task, mut node_state) in task_query.iter_mut() {
        if task.status != TaskStatus::Queued {
            continue;
        }
        
        info!("⚡ Executing operator command: {}", task.command);
        
        // Update status to executing
        task.status = TaskStatus::Executing;
        node_state.status = NodeStatus::Executing;
        node_state.last_update = time.elapsed_seconds();
        
        // Execute the commanded task
        let result = execute_operator_command(&task);
        
        // Update based on execution result
        match result {
            Ok(output) => {
                task.status = TaskStatus::Completed;
                node_state.status = NodeStatus::Completed;
                info!("✅ Task completed: {} -> {}", task.command, output);
            }
            Err(error) => {
                task.status = TaskStatus::Failed;
                node_state.status = NodeStatus::Failed;
                node_state.failure_count += 1;
                warn!("❌ Task failed: {} -> {}", task.command, error);
            }
        }
        
        node_state.execution_count += 1;
        node_state.last_update = time.elapsed_seconds();
    }
}

/// Update adversary psychological state (1n - internal experience)
fn update_adversary_psychology(adversary: &mut AdversaryPersona, elapsed_time: f32) {
    // Stress increases with operational tempo
    if adversary.psychological_state.operational_tempo > 0.8 {
        adversary.psychological_state.stress_level += 0.1 * (elapsed_time / 10.0);
        adversary.psychological_state.stress_level = adversary.psychological_state.stress_level.min(1.0);
    }
    
    // Paranoia increases with environmental sensitivity
    let env_pressure = adversary.environmental_awareness.traffic_awareness * 
                      adversary.environmental_awareness.resource_constraints;
    adversary.psychological_state.paranoia_level += env_pressure * 0.05 * (elapsed_time / 10.0);
    adversary.psychological_state.paranoia_level = adversary.psychological_state.paranoia_level.min(1.0);
    
    // Confidence decreases with high stress and paranoia
    let psychological_pressure = adversary.psychological_state.stress_level + 
                                adversary.psychological_state.paranoia_level;
    if psychological_pressure > 1.0 {
        adversary.psychological_state.confidence -= 0.1 * (elapsed_time / 10.0);
        adversary.psychological_state.confidence = adversary.psychological_state.confidence.max(0.0);
    }
}

/// Generate first-person adversary narrative (1n perspective)
fn generate_adversary_narrative(adversary: &AdversaryPersona) -> String {
    let stress_desc = if adversary.psychological_state.stress_level > 0.8 {
        "I am feeling intense pressure from operational demands"
    } else if adversary.psychological_state.stress_level > 0.5 {
        "I am managing moderate operational stress"
    } else {
        "I am operating calmly and effectively"
    };
    
    let paranoia_desc = if adversary.psychological_state.paranoia_level > 0.8 {
        "I suspect I may be under surveillance"
    } else if adversary.psychological_state.paranoia_level > 0.5 {
        "I am maintaining operational security awareness"
    } else {
        "I feel secure in my current position"
    };
    
    let confidence_desc = if adversary.psychological_state.confidence > 0.8 {
        "I am confident in my capabilities and mission success"
    } else if adversary.psychological_state.confidence > 0.5 {
        "I am proceeding with moderate confidence"
    } else {
        "I am questioning my operational effectiveness"
    };
    
    format!(
        "I am {}, currently focused on '{}'. {}. {}. {}.",
        adversary.name,
        adversary.current_objective,
        stress_desc,
        paranoia_desc,
        confidence_desc
    )
}

/// Make autonomous adversary decisions (1n agency)
fn make_adversary_decision(adversary: &AdversaryPersona) -> Option<String> {
    // High paranoia leads to defensive actions
    if adversary.psychological_state.paranoia_level > 0.8 {
        return Some("I need to improve my operational security".to_string());
    }
    
    // High confidence with low stress leads to aggressive actions
    if adversary.psychological_state.confidence > 0.7 && 
       adversary.psychological_state.stress_level < 0.3 {
        return Some("I will accelerate my operational timeline".to_string());
    }
    
    // High stress leads to caution
    if adversary.psychological_state.stress_level > 0.8 {
        return Some("I need to reduce my operational tempo".to_string());
    }
    
    // Resource constraints lead to adaptation
    if adversary.environmental_awareness.resource_constraints > 0.7 {
        return Some("I must adapt my approach due to resource limitations".to_string());
    }
    
    None
}

/// Execute operator-commanded task (2n execution)
fn execute_operator_command(task: &OperatorTask) -> Result<String, String> {
    match task.task_type {
        TaskType::PortScan => {
            Ok(format!("Port scan completed on {}: 22/SSH, 80/HTTP, 443/HTTPS open", task.target))
        }
        TaskType::VulnerabilityAssessment => {
            Ok(format!("Vulnerability assessment of {} found 3 critical, 7 high, 12 medium issues", task.target))
        }
        TaskType::NetworkReconnaissance => {
            Ok(format!("Network reconnaissance of {} identified 15 active hosts", task.target))
        }
        TaskType::ExploitDelivery => {
            if task.target.contains("192.168.1") {
                Ok(format!("Exploit delivered to {} - shell established", task.target))
            } else {
                Err("Exploit delivery failed - target hardened".to_string())
            }
        }
        TaskType::DataExfiltration => {
            Ok(format!("Data exfiltration from {} - 2.3GB transferred", task.target))
        }
        TaskType::PersistenceEstablishment => {
            Ok(format!("Persistence established on {} via scheduled task", task.target))
        }
    }
}

/// Spawn adversary-initiated actions (1n self-generated actions)
fn spawn_adversary_action(
    commands: &mut Commands,
    parent_entity: Entity,
    adversary: &AdversaryPersona,
    action: String,
) {
    let action_node = SlotGraphNode::new(
        SlotNodeType::Event,
        Vec3::new(0.0, 0.0, 0.0),
        {
            let mut metadata = HashMap::new();
            metadata.insert("action".to_string(), action.clone());
            metadata.insert("adversary_id".to_string(), adversary.persona_id.to_string());
            metadata.insert("adversary_type".to_string(), format!("{:?}", adversary.persona_type));
            metadata.insert("confidence_level".to_string(), adversary.psychological_state.confidence.to_string());
            metadata
        },
    );
    
    commands.spawn((
        AdversaryAction {
            action_id: Uuid::new_v4(),
            adversary_entity: parent_entity,
            description: action,
            initiated_by_adversary: true, // 1st person initiated
        },
        action_node,
        Name::new(format!("AdversaryAction-{}", adversary.name)),
    ));
}

/// Component for adversary-initiated actions (1n)
#[derive(Component, Debug, Clone)]
pub struct AdversaryAction {
    pub action_id: Uuid,
    pub adversary_entity: Entity,
    pub description: String,
    pub initiated_by_adversary: bool,
}

/// Bundle for creating adversary entities (1n)
#[derive(Bundle)]
pub struct AdversaryPersonaBundle {
    pub adversary: AdversaryPersona,
    pub slotgraph_node: SlotGraphNode,
    pub node_state: NodeState,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl AdversaryPersonaBundle {
    /// Create jihadist operator adversary (1n)
    pub fn jihadist_operator(name: String, objective: String) -> Self {
        let persona_id = Uuid::new_v4();
        
        let mut metadata = HashMap::new();
        metadata.insert("persona_type".to_string(), "JihadistOperator".to_string());
        metadata.insert("name".to_string(), name.clone());
        metadata.insert("perspective".to_string(), "first_person".to_string());
        
        let slotgraph_node = SlotGraphNode::new(
            SlotNodeType::People,
            Vec3::new(0.0, 0.0, 0.0),
            metadata,
        );
        
        Self {
            adversary: AdversaryPersona {
                persona_id,
                name,
                persona_type: AdversaryType::JihadistOperator,
                current_objective: objective,
                psychological_state: PsychologicalState {
                    stress_level: 0.3,
                    confidence: 0.7,
                    paranoia_level: 0.4,
                    operational_tempo: 0.6,
                    risk_tolerance: 0.8,
                },
                capabilities: vec![
                    "Network infiltration".to_string(),
                    "Social engineering".to_string(),
                    "Operational security".to_string(),
                ],
                behavioral_patterns: vec![
                    "Operates in cell structure".to_string(),
                    "High operational security".to_string(),
                    "Adapts to pressure".to_string(),
                ],
                knowledge_base: HashMap::new(),
                narrative_state: "I am preparing for operational phase".to_string(),
                environmental_awareness: EnvironmentalAwareness {
                    metoc_sensitivity: 0.6,
                    traffic_awareness: 0.9,
                    illumination_dependency: 0.7,
                    resource_constraints: 0.8,
                },
            },
            slotgraph_node,
            node_state: NodeState {
                status: NodeStatus::Ready,
                last_update: 0.0,
                health_score: 1.0,
                execution_count: 0,
                failure_count: 0,
            },
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

/// Bundle for creating operator tasks (2n)
#[derive(Bundle)]
pub struct OperatorTaskBundle {
    pub task: OperatorTask,
    pub slotgraph_node: SlotGraphNode,
    pub node_state: NodeState,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl OperatorTaskBundle {
    /// Create operator command task (2n)
    pub fn new(command: String, target: String, task_type: TaskType, operator_id: String) -> Self {
        let task_id = Uuid::new_v4();
        
        let mut metadata = HashMap::new();
        metadata.insert("command".to_string(), command.clone());
        metadata.insert("target".to_string(), target.clone());
        metadata.insert("perspective".to_string(), "second_person".to_string());
        metadata.insert("operator_id".to_string(), operator_id.clone());
        
        let slotgraph_node = SlotGraphNode::new(
            SlotNodeType::Task,
            Vec3::new(0.0, 0.0, 0.0),
            metadata,
        );
        
        Self {
            task: OperatorTask {
                task_id,
                command,
                target,
                operator_id,
                task_type,
                execution_parameters: HashMap::new(),
                status: TaskStatus::Queued,
                priority: TaskPriority::High,
                expected_outcome: "System will execute command and report results".to_string(),
            },
            slotgraph_node,
            node_state: NodeState {
                status: NodeStatus::Ready,
                last_update: 0.0,
                health_score: 1.0,
                execution_count: 0,
                failure_count: 0,
            },
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}