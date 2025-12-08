//! Legion ECS Integration - Task graph system for exploit operations
//!
//! Tesla-grade Legion Entity-Component-System integration providing
//! real-time task orchestration and SlotGraph compatibility.

#[cfg(feature = "legion-integration")]
use legion::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::{EVMError, ExploitTarget, MetasploitResult};

/// EVM Task types in the SlotGraph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EVMTaskType {
    NetworkDiscovery,
    VulnerabilityAssessment,
    ExploitExecution,
    PostExploitation,
    DeceptionDeployment,
    C2BeaconSimulation,
    TwinGeneration,
    TrafficObfuscation,
    DataExfiltration,
    PersistenceEstablishment,
}

/// Task execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Blocked,
    Cancelled,
}

/// Task priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskPriority {
    Critical,
    High,
    Normal,
    Low,
    Background,
}

/// EVM Task Entity with Legion compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EVMTask {
    pub id: Uuid,
    pub task_type: EVMTaskType,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub target: Option<ExploitTarget>,
    pub dependencies: Vec<Uuid>,
    pub metadata: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Task execution result component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: Uuid,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time_ms: u64,
    pub resource_usage: ResourceUsage,
}

/// Resource usage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_percent: f64,
    pub memory_mb: f64,
    pub network_kb: f64,
    pub disk_io_kb: f64,
}

/// Legion ECS World manager for EVM tasks
#[cfg(feature = "legion-integration")]
pub struct EVMLegionManager {
    world: World,
    resources: Resources,
    schedule: Schedule,
}

#[cfg(feature = "legion-integration")]
impl EVMLegionManager {
    pub fn new() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();
        
        // Initialize EVM task scheduling system
        let schedule = Schedule::builder()
            .add_system(task_dependency_system())
            .add_system(task_execution_system())
            .add_system(task_result_processing_system())
            .add_system(resource_monitoring_system())
            .build();

        // Insert global resources
        resources.insert(TaskQueue::default());
        resources.insert(ExecutionMetrics::default());
        
        Self {
            world,
            resources,
            schedule,
        }
    }

    /// Add new EVM task to Legion world
    pub fn add_task(&mut self, task: EVMTask) -> Uuid {
        let task_id = task.id;
        
        self.world.push((
            task.clone(),
            TaskStatus::Pending,
            task.priority,
            task.task_type,
        ));

        task_id
    }

    /// Execute one frame of Legion systems
    pub fn update(&mut self) {
        self.schedule.execute(&mut self.world, &mut self.resources);
    }

    /// Query tasks by status
    pub fn get_tasks_by_status(&self, status: TaskStatus) -> Vec<Entity> {
        let mut query = <(Entity, &TaskStatus)>::query();
        
        query
            .iter(&self.world)
            .filter(|(_, task_status)| matches!(**task_status, status))
            .map(|(entity, _)| *entity)
            .collect()
    }

    /// Get task execution metrics
    pub fn get_metrics(&self) -> &ExecutionMetrics {
        self.resources.get::<ExecutionMetrics>().expect("ExecutionMetrics resource should exist")
    }
}

/// Task queue resource for Legion
#[derive(Debug, Default)]
pub struct TaskQueue {
    pub pending_tasks: Vec<Uuid>,
    pub active_tasks: HashMap<Uuid, chrono::DateTime<chrono::Utc>>,
    pub completed_tasks: Vec<Uuid>,
    pub failed_tasks: Vec<(Uuid, String)>,
}

/// Execution metrics resource
#[derive(Debug, Default)]
pub struct ExecutionMetrics {
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub average_execution_time_ms: f64,
    pub total_cpu_usage: f64,
    pub total_memory_usage_mb: f64,
}

/// Legion system for managing task dependencies
#[cfg(feature = "legion-integration")]
fn task_dependency_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("task_dependency_system")
        .read_resource::<TaskQueue>()
        .with_query(<(Entity, &EVMTask, &mut TaskStatus)>::query())
        .build(|_, world, task_queue, query| {
            for (entity, task, mut status) in query.iter_mut(world) {
                if matches!(*status, TaskStatus::Pending) {
                    // Check if all dependencies are completed
                    let dependencies_satisfied = task.dependencies.iter()
                        .all(|dep_id| task_queue.completed_tasks.contains(dep_id));
                    
                    if dependencies_satisfied {
                        *status = TaskStatus::InProgress;
                        tracing::info!("Task {} ready for execution", task.id);
                    }
                }
            }
        })
}

/// Legion system for task execution
#[cfg(feature = "legion-integration")]
fn task_execution_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("task_execution_system")
        .write_resource::<TaskQueue>()
        .write_resource::<ExecutionMetrics>()
        .with_query(<(Entity, &EVMTask, &mut TaskStatus)>::query())
        .build(|commands, world, (task_queue, metrics), query| {
            for (entity, task, mut status) in query.iter_mut(world) {
                if matches!(*status, TaskStatus::InProgress) {
                    // Execute task based on type
                    let execution_result = execute_evm_task(&task.task_type, &task.target);
                    
                    match execution_result {
                        Ok(result) => {
                            *status = TaskStatus::Completed;
                            task_queue.completed_tasks.push(task.id);
                            metrics.tasks_completed += 1;
                            
                            // Add result component
                            commands.add_component(entity, result);
                            
                            tracing::info!("Task {} completed successfully", task.id);
                        }
                        Err(error) => {
                            *status = TaskStatus::Failed;
                            task_queue.failed_tasks.push((task.id, error.to_string()));
                            metrics.tasks_failed += 1;
                            
                            tracing::error!("Task {} failed: {}", task.id, error);
                        }
                    }
                }
            }
        })
}

/// Legion system for processing task results
#[cfg(feature = "legion-integration")]
fn task_result_processing_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("task_result_processing_system")
        .write_resource::<ExecutionMetrics>()
        .with_query(<&TaskResult>::query())
        .build(|_, world, metrics, query| {
            for result in query.iter(world) {
                // Update metrics with task result
                metrics.average_execution_time_ms = 
                    (metrics.average_execution_time_ms + result.execution_time_ms as f64) / 2.0;
                
                metrics.total_cpu_usage += result.resource_usage.cpu_percent;
                metrics.total_memory_usage_mb += result.resource_usage.memory_mb;
                
                tracing::debug!("Processed result for task {}", result.task_id);
            }
        })
}

/// Legion system for monitoring resource usage
#[cfg(feature = "legion-integration")]
fn resource_monitoring_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("resource_monitoring_system")
        .read_resource::<ExecutionMetrics>()
        .with_query(<(&EVMTask, &TaskStatus)>::query())
        .build(|_, world, metrics, query| {
            let active_tasks: Vec<_> = query
                .iter(world)
                .filter(|(_, status)| matches!(**status, TaskStatus::InProgress))
                .collect();
            
            if active_tasks.len() > 10 {
                tracing::warn!("High task load detected: {} active tasks", active_tasks.len());
            }
            
            if metrics.total_memory_usage_mb > 1000.0 {
                tracing::warn!("High memory usage detected: {:.2} MB", metrics.total_memory_usage_mb);
            }
        })
}

/// Execute EVM task based on task type
fn execute_evm_task(
    task_type: &EVMTaskType,
    target: &Option<ExploitTarget>,
) -> Result<TaskResult, EVMError> {
    let start_time = std::time::Instant::now();
    let task_id = Uuid::new_v4();
    
    // Simulate task execution based on type
    let output = match task_type {
        EVMTaskType::NetworkDiscovery => {
            tracing::info!("Executing network discovery task");
            "Network discovery completed - 15 hosts found".to_string()
        }
        EVMTaskType::VulnerabilityAssessment => {
            tracing::info!("Executing vulnerability assessment task");
            "Vulnerability scan completed - 3 high-risk vulnerabilities found".to_string()
        }
        EVMTaskType::ExploitExecution => {
            tracing::info!("Executing exploit task");
            if target.is_some() {
                "Exploit executed successfully - shell obtained".to_string()
            } else {
                return Err(EVMError::ConfigurationError("No target specified for exploit".to_string()));
            }
        }
        EVMTaskType::DeceptionDeployment => {
            tracing::info!("Deploying deception assets");
            "Deception assets deployed - 5 honeypots active".to_string()
        }
        _ => {
            format!("Task {:?} executed", task_type)
        }
    };
    
    let execution_time = start_time.elapsed().as_millis() as u64;
    
    Ok(TaskResult {
        task_id,
        success: true,
        output,
        error: None,
        execution_time_ms: execution_time,
        resource_usage: ResourceUsage {
            cpu_percent: fastrand::f64() * 50.0, // Simulated CPU usage
            memory_mb: fastrand::f64() * 100.0,  // Simulated memory usage
            network_kb: fastrand::f64() * 1024.0, // Simulated network usage
            disk_io_kb: fastrand::f64() * 512.0,  // Simulated disk I/O
        },
    })
}

#[cfg(not(feature = "legion-integration"))]
pub struct EVMLegionManager;

#[cfg(not(feature = "legion-integration"))]
impl EVMLegionManager {
    pub fn new() -> Self {
        Self
    }
    
    pub fn add_task(&mut self, _task: EVMTask) -> Uuid {
        Uuid::new_v4()
    }
    
    pub fn update(&mut self) {
        // No-op when Legion integration disabled
    }
}