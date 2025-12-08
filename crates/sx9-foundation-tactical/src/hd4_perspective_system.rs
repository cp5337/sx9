//! CTAS HD4 Perspective System - Defensive vs Offensive Operations
//! 
//! The HD4 framework (Hunt, Detect, Disrupt, Disable, Dominate) completely flips
//! depending on whether we're operating defensively or offensively:
//! 
//! **DEFENSIVE HD4**: We Hunt/Detect/Disrupt/Disable/Dominate the **adversary**
//! **OFFENSIVE HD4**: We Hunt/Detect/Disrupt/Disable/Dominate the **target systems**

use bevy::prelude::*;
use crate::slotgraph_task_tool_mapper::{SlotGraphTaskMapper, TaskExecutionStrategy};
use ctas_slotgraph_tools::{
    node_types::{TaskMetadata, HD4Phase, TaskPriority, SlotGraphNode, SlotNodeType, NodeState, NodeStatus},
    cognigraph::universal_nodes::{UniversalNodeType, NodeFunctionExecutor}
};
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// HD4 Operational Perspective - fundamentally changes what we target
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum HD4Perspective {
    Defensive, // We HD4 against adversaries (defensive cybersecurity)
    Offensive, // We HD4 against target systems (offensive operations)
}

/// HD4 operation with perspective-aware targeting
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct HD4Operation {
    pub operation_id: Uuid,
    pub perspective: HD4Perspective,
    pub current_phase: HD4Phase,
    pub target_context: TargetContext,
    pub operational_environment: OperationalEnvironment,
    pub phase_objectives: HashMap<HD4Phase, String>,
    pub success_metrics: HashMap<HD4Phase, f32>,
    pub escalation_triggers: Vec<String>,
}

/// Target context changes based on perspective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetContext {
    pub primary_target: String,
    pub target_type: TargetType,
    pub target_characteristics: HashMap<String, String>,
    pub threat_level: f32,
    pub operational_value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetType {
    // Defensive targets (what we're protecting FROM)
    AdversaryGroup,
    MaliciousActor,
    AttackCampaign,
    ThreatVector,
    CompromisedAsset,
    
    // Offensive targets (what we're attacking)
    NetworkInfrastructure,
    HostSystems,
    ApplicationServices,
    DataRepositories,
    CommunicationChannels,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalEnvironment {
    pub domain: OperationalDomain,
    pub constraints: Vec<String>,
    pub available_resources: Vec<String>,
    pub rules_of_engagement: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationalDomain {
    CyberDefense,    // Blue team operations
    CyberOffense,    // Red team operations
    ThreatHunting,   // Hybrid defensive/offensive
    PenTesting,      // Authorized offensive testing
    IncidentResponse, // Defensive response operations
}

/// System for executing HD4 operations with perspective awareness
pub fn hd4_perspective_execution_system(
    mut hd4_query: Query<(Entity, &mut HD4Operation, &mut NodeState)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut operation, mut node_state) in hd4_query.iter_mut() {
        if node_state.status != NodeStatus::Ready && node_state.status != NodeStatus::Executing {
            continue;
        }
        
        // Execute current HD4 phase based on perspective
        let phase_result = match operation.perspective {
            HD4Perspective::Defensive => execute_defensive_hd4_phase(&mut operation),
            HD4Perspective::Offensive => execute_offensive_hd4_phase(&mut operation),
        };
        
        // Update operation based on phase execution results
        match phase_result {
            HD4PhaseResult::PhaseComplete(next_phase) => {
                info!("🎯 {:?} HD4 Phase {:?} completed. Advancing to {:?}", 
                    operation.perspective, operation.current_phase, next_phase);
                
                operation.current_phase = next_phase;
                node_state.execution_count += 1;
                node_state.last_update = time.elapsed_seconds();
                
                // Spawn phase transition entity
                spawn_phase_transition(&mut commands, entity, &operation, next_phase);
            }
            HD4PhaseResult::PhaseInProgress(progress) => {
                // Update progress metrics
                operation.success_metrics.insert(operation.current_phase, progress);
                node_state.last_update = time.elapsed_seconds();
                node_state.status = NodeStatus::Executing;
            }
            HD4PhaseResult::PhaseStalled(reason) => {
                warn!("⚠️ HD4 Phase {:?} stalled: {}", operation.current_phase, reason);
                node_state.status = NodeStatus::Failed;
                node_state.failure_count += 1;
            }
            HD4PhaseResult::OperationComplete => {
                info!("✅ HD4 Operation {} completed successfully", operation.operation_id);
                node_state.status = NodeStatus::Completed;
                
                // Generate final operation assessment
                generate_operation_assessment(&mut commands, entity, &operation);
            }
        }
    }
}

/// Execute defensive HD4 phase - targeting adversaries
fn execute_defensive_hd4_phase(operation: &mut HD4Operation) -> HD4PhaseResult {
    match operation.current_phase {
        HD4Phase::Hunt => {
            // DEFENSIVE HUNT: Hunt for adversaries in our environment
            info!("🔍 Defensive Hunt: Searching for adversary '{}' in our systems", 
                operation.target_context.primary_target);
            
            match operation.target_context.target_type {
                TargetType::AdversaryGroup => {
                    // Hunt for specific adversary group indicators
                    if operation.success_metrics.get(&HD4Phase::Hunt).unwrap_or(&0.0) < &0.8 {
                        HD4PhaseResult::PhaseInProgress(0.6)
                    } else {
                        HD4PhaseResult::PhaseComplete(HD4Phase::Detect)
                    }
                }
                TargetType::MaliciousActor => {
                    // Hunt for specific malicious actor TTPs
                    HD4PhaseResult::PhaseComplete(HD4Phase::Detect)
                }
                TargetType::ThreatVector => {
                    // Hunt for threat vector evidence
                    HD4PhaseResult::PhaseComplete(HD4Phase::Detect)
                }
                _ => HD4PhaseResult::PhaseStalled("Invalid target type for defensive hunt".to_string()),
            }
        }
        
        HD4Phase::Detect => {
            // DEFENSIVE DETECT: Detect adversary activities and techniques
            info!("🎯 Defensive Detect: Identifying adversary '{}' activities", 
                operation.target_context.primary_target);
            
            // Simulate detection activities
            HD4PhaseResult::PhaseComplete(HD4Phase::Disrupt)
        }
        
        HD4Phase::Disrupt => {
            // DEFENSIVE DISRUPT: Disrupt adversary operations
            info!("⚡ Defensive Disrupt: Disrupting adversary '{}' operations", 
                operation.target_context.primary_target);
            
            // Implement adversary disruption tactics
            HD4PhaseResult::PhaseComplete(HD4Phase::Disable)
        }
        
        HD4Phase::Disable => {
            // DEFENSIVE DISABLE: Disable adversary capabilities
            info!("🛡️ Defensive Disable: Disabling adversary '{}' capabilities", 
                operation.target_context.primary_target);
            
            // Execute adversary capability disabling
            HD4PhaseResult::PhaseComplete(HD4Phase::Dominate)
        }
        
        HD4Phase::Dominate => {
            // DEFENSIVE DOMINATE: Establish control over adversary threat
            info!("👑 Defensive Dominate: Establishing control over adversary threat '{}'", 
                operation.target_context.primary_target);
            
            // Complete adversary neutralization
            HD4PhaseResult::OperationComplete
        }
    }
}

/// Execute offensive HD4 phase - targeting enemy systems
fn execute_offensive_hd4_phase(operation: &mut HD4Operation) -> HD4PhaseResult {
    match operation.current_phase {
        HD4Phase::Hunt => {
            // OFFENSIVE HUNT: Hunt for target systems and vulnerabilities
            info!("🔍 Offensive Hunt: Searching for target system '{}' vulnerabilities", 
                operation.target_context.primary_target);
            
            match operation.target_context.target_type {
                TargetType::NetworkInfrastructure => {
                    // Hunt for network access points and vulnerabilities
                    HD4PhaseResult::PhaseComplete(HD4Phase::Detect)
                }
                TargetType::HostSystems => {
                    // Hunt for host system vulnerabilities
                    HD4PhaseResult::PhaseComplete(HD4Phase::Detect)
                }
                TargetType::ApplicationServices => {
                    // Hunt for application vulnerabilities
                    HD4PhaseResult::PhaseComplete(HD4Phase::Detect)
                }
                _ => HD4PhaseResult::PhaseStalled("Invalid target type for offensive hunt".to_string()),
            }
        }
        
        HD4Phase::Detect => {
            // OFFENSIVE DETECT: Detect target system configurations and weaknesses
            info!("🎯 Offensive Detect: Analyzing target system '{}' weaknesses", 
                operation.target_context.primary_target);
            
            // Implement target system reconnaissance
            HD4PhaseResult::PhaseComplete(HD4Phase::Disrupt)
        }
        
        HD4Phase::Disrupt => {
            // OFFENSIVE DISRUPT: Disrupt target system normal operations
            info!("⚡ Offensive Disrupt: Disrupting target system '{}' operations", 
                operation.target_context.primary_target);
            
            // Execute target system disruption
            HD4PhaseResult::PhaseComplete(HD4Phase::Disable)
        }
        
        HD4Phase::Disable => {
            // OFFENSIVE DISABLE: Disable target system defenses and capabilities
            info!("🔓 Offensive Disable: Disabling target system '{}' defenses", 
                operation.target_context.primary_target);
            
            // Execute target system defense disabling
            HD4PhaseResult::PhaseComplete(HD4Phase::Dominate)
        }
        
        HD4Phase::Dominate => {
            // OFFENSIVE DOMINATE: Establish control over target systems
            info!("👑 Offensive Dominate: Establishing control over target system '{}'", 
                operation.target_context.primary_target);
            
            // Complete target system compromise
            HD4PhaseResult::OperationComplete
        }
    }
}

/// Result of HD4 phase execution
#[derive(Debug, Clone)]
pub enum HD4PhaseResult {
    PhaseComplete(HD4Phase),
    PhaseInProgress(f32), // Progress percentage
    PhaseStalled(String), // Reason for stall
    OperationComplete,
}

/// Spawn phase transition entity
fn spawn_phase_transition(
    commands: &mut Commands,
    parent_entity: Entity,
    operation: &HD4Operation,
    next_phase: HD4Phase,
) {
    let transition_description = match operation.perspective {
        HD4Perspective::Defensive => {
            format!("Defensive HD4: Transitioning from {:?} to {:?} against adversary '{}'",
                operation.current_phase, next_phase, operation.target_context.primary_target)
        }
        HD4Perspective::Offensive => {
            format!("Offensive HD4: Transitioning from {:?} to {:?} against target '{}'",
                operation.current_phase, next_phase, operation.target_context.primary_target)
        }
    };
    
    let mut metadata = HashMap::new();
    metadata.insert("perspective".to_string(), format!("{:?}", operation.perspective));
    metadata.insert("previous_phase".to_string(), format!("{:?}", operation.current_phase));
    metadata.insert("next_phase".to_string(), format!("{:?}", next_phase));
    metadata.insert("target".to_string(), operation.target_context.primary_target.clone());
    
    let transition_node = SlotGraphNode::new(
        SlotNodeType::Event,
        Vec3::new(0.0, 0.0, 0.0),
        metadata,
    );
    
    commands.spawn((
        HD4PhaseTransition {
            transition_id: Uuid::new_v4(),
            parent_operation: parent_entity,
            from_phase: operation.current_phase,
            to_phase: next_phase,
            perspective: operation.perspective,
            description: transition_description,
        },
        transition_node,
        Name::new(format!("HD4Transition-{:?}-{:?}", operation.current_phase, next_phase)),
    ));
}

/// Generate final operation assessment
fn generate_operation_assessment(
    commands: &mut Commands,
    parent_entity: Entity,
    operation: &HD4Operation,
) {
    let assessment_summary = match operation.perspective {
        HD4Perspective::Defensive => {
            format!("Defensive HD4 operation against '{}' completed. Adversary threat neutralized through systematic HD4 progression.",
                operation.target_context.primary_target)
        }
        HD4Perspective::Offensive => {
            format!("Offensive HD4 operation against '{}' completed. Target system successfully compromised through systematic HD4 progression.",
                operation.target_context.primary_target)
        }
    };
    
    let mut metadata = HashMap::new();
    metadata.insert("operation_type".to_string(), format!("{:?}", operation.perspective));
    metadata.insert("target".to_string(), operation.target_context.primary_target.clone());
    metadata.insert("phases_completed".to_string(), "5".to_string());
    metadata.insert("overall_success".to_string(), "true".to_string());
    
    let assessment_node = SlotGraphNode::new(
        SlotNodeType::Intelligence,
        Vec3::new(0.0, 0.0, 0.0),
        metadata,
    );
    
    commands.spawn((
        HD4OperationAssessment {
            assessment_id: Uuid::new_v4(),
            parent_operation: parent_entity,
            perspective: operation.perspective,
            summary: assessment_summary,
            phases_executed: vec![
                HD4Phase::Hunt,
                HD4Phase::Detect,
                HD4Phase::Disrupt,
                HD4Phase::Disable,
                HD4Phase::Dominate,
            ],
            success_rate: calculate_success_rate(&operation.success_metrics),
        },
        assessment_node,
        Name::new(format!("HD4Assessment-{:?}", operation.perspective)),
    ));
}

/// Calculate overall success rate from phase metrics
fn calculate_success_rate(metrics: &HashMap<HD4Phase, f32>) -> f32 {
    if metrics.is_empty() {
        return 0.0;
    }
    
    metrics.values().sum::<f32>() / metrics.len() as f32
}

/// Component for phase transitions
#[derive(Component, Debug, Clone)]
pub struct HD4PhaseTransition {
    pub transition_id: Uuid,
    pub parent_operation: Entity,
    pub from_phase: HD4Phase,
    pub to_phase: HD4Phase,
    pub perspective: HD4Perspective,
    pub description: String,
}

/// Component for operation assessments
#[derive(Component, Debug, Clone)]
pub struct HD4OperationAssessment {
    pub assessment_id: Uuid,
    pub parent_operation: Entity,
    pub perspective: HD4Perspective,
    pub summary: String,
    pub phases_executed: Vec<HD4Phase>,
    pub success_rate: f32,
}

/// Bundle for creating HD4 operations
#[derive(Bundle)]
pub struct HD4OperationBundle {
    pub operation: HD4Operation,
    pub slotgraph_node: SlotGraphNode,
    pub node_state: NodeState,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl HD4OperationBundle {
    /// Create defensive HD4 operation (targeting adversary)
    pub fn defensive(target_adversary: String, adversary_type: TargetType) -> Self {
        let operation_id = Uuid::new_v4();
        
        let mut phase_objectives = HashMap::new();
        phase_objectives.insert(HD4Phase::Hunt, "Hunt for adversary presence in our systems".to_string());
        phase_objectives.insert(HD4Phase::Detect, "Detect adversary activities and techniques".to_string());
        phase_objectives.insert(HD4Phase::Disrupt, "Disrupt adversary operations".to_string());
        phase_objectives.insert(HD4Phase::Disable, "Disable adversary capabilities".to_string());
        phase_objectives.insert(HD4Phase::Dominate, "Neutralize adversary threat completely".to_string());
        
        let mut metadata = HashMap::new();
        metadata.insert("perspective".to_string(), "Defensive".to_string());
        metadata.insert("target_adversary".to_string(), target_adversary.clone());
        metadata.insert("operation_type".to_string(), "Cyber Defense".to_string());
        
        let slotgraph_node = SlotGraphNode::new(
            SlotNodeType::Task,
            Vec3::new(0.0, 0.0, 0.0),
            metadata,
        );
        
        Self {
            operation: HD4Operation {
                operation_id,
                perspective: HD4Perspective::Defensive,
                current_phase: HD4Phase::Hunt,
                target_context: TargetContext {
                    primary_target: target_adversary,
                    target_type: adversary_type,
                    target_characteristics: HashMap::new(),
                    threat_level: 0.8,
                    operational_value: 0.9,
                },
                operational_environment: OperationalEnvironment {
                    domain: OperationalDomain::CyberDefense,
                    constraints: vec!["Legal compliance".to_string(), "Minimal disruption".to_string()],
                    available_resources: vec!["SIEM systems".to_string(), "Threat intelligence".to_string()],
                    rules_of_engagement: vec!["Defensive only".to_string(), "Minimize collateral".to_string()],
                },
                phase_objectives,
                success_metrics: HashMap::new(),
                escalation_triggers: vec![
                    "Active data exfiltration detected".to_string(),
                    "Critical system compromise".to_string(),
                ],
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
    
    /// Create offensive HD4 operation (targeting enemy systems)
    pub fn offensive(target_system: String, system_type: TargetType) -> Self {
        let operation_id = Uuid::new_v4();
        
        let mut phase_objectives = HashMap::new();
        phase_objectives.insert(HD4Phase::Hunt, "Hunt for target system vulnerabilities".to_string());
        phase_objectives.insert(HD4Phase::Detect, "Detect target system weaknesses and configurations".to_string());
        phase_objectives.insert(HD4Phase::Disrupt, "Disrupt target system normal operations".to_string());
        phase_objectives.insert(HD4Phase::Disable, "Disable target system defenses".to_string());
        phase_objectives.insert(HD4Phase::Dominate, "Establish control over target system".to_string());
        
        let mut metadata = HashMap::new();
        metadata.insert("perspective".to_string(), "Offensive".to_string());
        metadata.insert("target_system".to_string(), target_system.clone());
        metadata.insert("operation_type".to_string(), "Cyber Offense".to_string());
        
        let slotgraph_node = SlotGraphNode::new(
            SlotNodeType::Task,
            Vec3::new(0.0, 0.0, 0.0),
            metadata,
        );
        
        Self {
            operation: HD4Operation {
                operation_id,
                perspective: HD4Perspective::Offensive,
                current_phase: HD4Phase::Hunt,
                target_context: TargetContext {
                    primary_target: target_system,
                    target_type: system_type,
                    target_characteristics: HashMap::new(),
                    threat_level: 0.7,
                    operational_value: 0.8,
                },
                operational_environment: OperationalEnvironment {
                    domain: OperationalDomain::CyberOffense,
                    constraints: vec!["Rules of engagement".to_string(), "Authorization scope".to_string()],
                    available_resources: vec!["Exploit frameworks".to_string(), "C2 infrastructure".to_string()],
                    rules_of_engagement: vec!["Authorized targets only".to_string(), "Document all actions".to_string()],
                },
                phase_objectives,
                success_metrics: HashMap::new(),
                escalation_triggers: vec![
                    "Target system isolation detected".to_string(),
                    "Defensive countermeasures activated".to_string(),
                ],
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