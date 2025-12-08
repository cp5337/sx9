//! SlotGraph Integration - Direct Legion ECS Integration
//!
//! Integrates the offensive tools with the main CTAS SlotGraph Legion ECS system.
//! Creates direct Entity-to-Tool mappings with real-time task execution.

use legion::prelude::*;
use crate::{
    EVMError,
    offsec_kali_launcher::{OffSecKaliLauncher, OffSecTool, OffSecConfig},
    offensive_tools_graph::OffensiveToolsGraph,
    slotgraph_task_tool_mapper::{HD4Phase, TaskToolMapping},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// SlotGraph Integration System for Legion
pub struct SlotGraphOffsecSystem {
    world: World,
    resources: Resources,
    schedule: Schedule,
}

impl SlotGraphOffsecSystem {
    pub fn new() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();
        
        // Initialize resources
        resources.insert(OffsecIntegrationConfig::default());
        resources.insert(ToolExecutionQueue::default());
        resources.insert(HD4PhaseTracker::default());
        
        // Build schedule with systems
        let schedule = Schedule::builder()
            .add_system(task_tool_mapping_system())
            .add_system(tool_execution_system())
            .add_system(hd4_phase_progression_system())
            .build();

        Self {
            world,
            resources,
            schedule,
        }
    }

    pub fn update(&mut self) {
        self.schedule.execute(&mut self.world, &mut self.resources);
    }

    pub fn add_task_node(&mut self, task: SlotGraphTaskNode) -> Entity {
        self.world.push((
            task,
            TaskStatus::Pending,
            ToolMapping::None,
        ))
    }
}

/// OffSec integration configuration
#[derive(Default)]
pub struct OffsecIntegrationConfig {
    pub auto_tool_mapping: bool,
    pub real_execution_enabled: bool,
    pub safe_mode: bool,
    pub max_concurrent_executions: usize,
}

/// Tool execution queue
#[derive(Default)]
pub struct ToolExecutionQueue {
    pub pending_executions: Vec<ToolExecution>,
    pub active_executions: HashMap<Entity, ActiveExecution>,
}

/// HD4 phase progression tracker
#[derive(Default)]
pub struct HD4PhaseTracker {
    pub current_phase: HD4Phase,
    pub phase_completion: HashMap<HD4Phase, f64>,
    pub total_tasks_by_phase: HashMap<HD4Phase, u32>,
    pub completed_tasks_by_phase: HashMap<HD4Phase, u32>,
}

/// SlotGraph Task Node - Legion ECS compatible
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotGraphTaskNode {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub hd4_phase: HD4Phase,
    pub required_capabilities: Vec<String>,
    pub priority: TaskPriority,
    pub dependencies: Vec<Uuid>,
    pub estimated_execution_time: std::time::Duration,
}

/// Task execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Mapped,      // Tool has been mapped to task
    Queued,      // Ready for execution
    Executing,   // Currently executing
    Completed,   // Successfully completed
    Failed,      // Execution failed
    Blocked,     // Dependencies not met
    Cancelled,   // Manually cancelled
}

/// Task priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Critical = 5,
    High = 4,
    Normal = 3,
    Low = 2,
    Background = 1,
}

/// Tool mapping to tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolMapping {
    None,
    Single(OffSecTool),
    Multiple(Vec<OffSecTool>),
    Pipeline(Vec<OffSecTool>),  // Sequential execution
}

/// Tool execution details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecution {
    pub entity: Entity,
    pub task_id: Uuid,
    pub tool: OffSecTool,
    pub config: OffSecConfig,
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
}

/// Active execution tracking
#[derive(Debug, Clone)]
pub struct ActiveExecution {
    pub tool_execution: ToolExecution,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub process_handle: Option<tokio::process::Child>,
    pub output_buffer: Vec<String>,
}

/// Execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub task_id: Uuid,
    pub tool: OffSecTool,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time: std::time::Duration,
    pub exit_code: Option<i32>,
}

// Legion Systems

/// Task-to-tool mapping system
fn task_tool_mapping_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("task_tool_mapping_system")
        .read_resource::<OffsecIntegrationConfig>()
        .with_query(<(Entity, &SlotGraphTaskNode, &mut ToolMapping, &TaskStatus)>::query())
        .build(|_, world, config, query| {
            if !config.auto_tool_mapping {
                return;
            }

            for (entity, task, mut mapping, status) in query.iter_mut(world) {
                if matches!(*status, TaskStatus::Pending) && matches!(*mapping, ToolMapping::None) {
                    // Auto-map tools based on task requirements
                    let mapped_tools = auto_map_tools_for_task(task);
                    
                    if !mapped_tools.is_empty() {
                        *mapping = if mapped_tools.len() == 1 {
                            ToolMapping::Single(mapped_tools[0].clone())
                        } else {
                            ToolMapping::Multiple(mapped_tools)
                        };
                        
                        tracing::info!("Auto-mapped {} tools to task: {}", 
                                     match mapping {
                                         ToolMapping::Single(_) => 1,
                                         ToolMapping::Multiple(ref tools) => tools.len(),
                                         _ => 0,
                                     }, 
                                     task.name);
                    }
                }
            }
        })
}

/// Tool execution system
fn tool_execution_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("tool_execution_system")
        .read_resource::<OffsecIntegrationConfig>()
        .write_resource::<ToolExecutionQueue>()
        .with_query(<(Entity, &SlotGraphTaskNode, &ToolMapping, &mut TaskStatus)>::query())
        .build(|commands, world, (config, queue), query| {
            if !config.real_execution_enabled {
                return;
            }

            // Execute pending tasks
            for (entity, task, mapping, mut status) in query.iter_mut(world) {
                if matches!(*status, TaskStatus::Mapped) {
                    match mapping {
                        ToolMapping::Single(tool) => {
                            let execution = ToolExecution {
                                entity: *entity,
                                task_id: task.id,
                                tool: tool.clone(),
                                config: create_tool_config(task, tool),
                                scheduled_at: chrono::Utc::now(),
                            };
                            
                            queue.pending_executions.push(execution);
                            *status = TaskStatus::Queued;
                        }
                        ToolMapping::Multiple(tools) => {
                            // Create execution for first tool in sequence
                            if let Some(first_tool) = tools.first() {
                                let execution = ToolExecution {
                                    entity: *entity,
                                    task_id: task.id,
                                    tool: first_tool.clone(),
                                    config: create_tool_config(task, first_tool),
                                    scheduled_at: chrono::Utc::now(),
                                };
                                
                                queue.pending_executions.push(execution);
                                *status = TaskStatus::Queued;
                            }
                        }
                        _ => {}
                    }
                }
            }

            // Process execution queue
            let max_concurrent = config.max_concurrent_executions;
            while queue.active_executions.len() < max_concurrent && !queue.pending_executions.is_empty() {
                if let Some(execution) = queue.pending_executions.pop() {
                    start_tool_execution(&execution, queue, commands);
                }
            }
        })
}

/// HD4 phase progression system
fn hd4_phase_progression_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("hd4_phase_progression_system")
        .write_resource::<HD4PhaseTracker>()
        .with_query(<(&SlotGraphTaskNode, &TaskStatus)>::query())
        .build(|_, world, tracker, query| {
            // Reset phase counters
            tracker.total_tasks_by_phase.clear();
            tracker.completed_tasks_by_phase.clear();
            
            // Count tasks by phase and status
            for (task, status) in query.iter(world) {
                let phase = &task.hd4_phase;
                *tracker.total_tasks_by_phase.entry(phase.clone()).or_insert(0) += 1;
                
                if matches!(*status, TaskStatus::Completed) {
                    *tracker.completed_tasks_by_phase.entry(phase.clone()).or_insert(0) += 1;
                }
            }
            
            // Calculate completion percentages
            for (phase, total) in &tracker.total_tasks_by_phase {
                let completed = tracker.completed_tasks_by_phase.get(phase).unwrap_or(&0);
                let completion_rate = if *total > 0 {
                    *completed as f64 / *total as f64
                } else {
                    0.0
                };
                tracker.phase_completion.insert(phase.clone(), completion_rate);
            }
            
            // Determine current phase based on completion
            let current_phase = determine_current_hd4_phase(tracker);
            if current_phase != tracker.current_phase {
                tracing::info!("HD4 phase transition: {:?} -> {:?}", tracker.current_phase, current_phase);
                tracker.current_phase = current_phase;
            }
        })
}

// Helper Functions

/// Auto-map tools based on task requirements
fn auto_map_tools_for_task(task: &SlotGraphTaskNode) -> Vec<OffSecTool> {
    let mut tools = Vec::new();
    
    for capability in &task.required_capabilities {
        match capability.as_str() {
            "network_scanning" => {
                tools.push(OffSecTool::Nmap);
                tools.push(OffSecTool::Masscan);
            }
            "vulnerability_assessment" => {
                tools.push(OffSecTool::OpenVAS);
                tools.push(OffSecTool::Nessus);
            }
            "web_application_testing" => {
                tools.push(OffSecTool::BurpSuite);
                tools.push(OffSecTool::OWASP_ZAP);
            }
            "exploitation" => {
                tools.push(OffSecTool::Metasploit);
                tools.push(OffSecTool::ExploitDB);
            }
            "password_attacks" => {
                tools.push(OffSecTool::John);
                tools.push(OffSecTool::Hashcat);
                tools.push(OffSecTool::Hydra);
            }
            "wireless_testing" => {
                tools.push(OffSecTool::Aircrack);
                tools.push(OffSecTool::Reaver);
            }
            "forensics" => {
                tools.push(OffSecTool::Autopsy);
                tools.push(OffSecTool::Volatility);
            }
            _ => {
                tracing::warn!("Unknown capability for tool mapping: {}", capability);
            }
        }
    }
    
    tools
}

/// Create tool configuration for task
fn create_tool_config(task: &SlotGraphTaskNode, tool: &OffSecTool) -> OffSecConfig {
    OffSecConfig {
        tool: tool.clone(),
        target: "127.0.0.1".to_string(), // Default target
        options: HashMap::new(),
        output_format: "json".to_string(),
        timeout_seconds: task.estimated_execution_time.as_secs() as u32,
        safe_mode: true,
        dry_run: false,
    }
}

/// Start tool execution
fn start_tool_execution(
    execution: &ToolExecution,
    queue: &mut ToolExecutionQueue,
    commands: &mut CommandBuffer,
) {
    tracing::info!("Starting tool execution: {:?} for task {}", execution.tool, execution.task_id);
    
    let active_execution = ActiveExecution {
        tool_execution: execution.clone(),
        started_at: chrono::Utc::now(),
        process_handle: None, // Would be set when actually executing
        output_buffer: Vec::new(),
    };
    
    queue.active_executions.insert(execution.entity, active_execution);
    
    // Update entity status to Executing
    commands.add_component(execution.entity, TaskStatus::Executing);
}

/// Determine current HD4 phase based on completion rates
fn determine_current_hd4_phase(tracker: &HD4PhaseTracker) -> HD4Phase {
    use HD4Phase::*;
    
    let phases = [Hunt, Detect, Disrupt, Disable, Dominate];
    
    for phase in phases.iter() {
        let completion = tracker.phase_completion.get(phase).unwrap_or(&0.0);
        if *completion < 0.8 {  // 80% completion threshold
            return phase.clone();
        }
    }
    
    Dominate // All phases complete
}