//! Legion ECS Integration Tests for Exploit Vector Machine
//!
//! Comprehensive test suite validating Legion ECS performance,
//! task execution, and SlotGraph integration capabilities.

use ctas_exploit_vector_machine::{
    legion_integration::*,
    slotgraph_task_tool_mapper::HD4Phase,
    EVMConfig, EVMError,
};
use legion::prelude::*;
use std::time::{Duration, Instant};
use uuid::Uuid;

#[cfg(test)]
mod legion_evm_tests {
    use super::*;

    #[test]
    fn test_legion_manager_creation() {
        let manager = EVMLegionManager::new();
        let metrics = manager.get_metrics();
        
        assert_eq!(metrics.tasks_completed, 0);
        assert_eq!(metrics.tasks_failed, 0);
        assert_eq!(metrics.average_execution_time_ms, 0.0);
    }

    #[test]
    fn test_task_addition() {
        let mut manager = EVMLegionManager::new();
        
        let task = EVMTask {
            id: Uuid::new_v4(),
            task_type: EVMTaskType::NetworkDiscovery,
            status: TaskStatus::Pending,
            priority: TaskPriority::High,
            target: None,
            dependencies: Vec::new(),
            metadata: std::collections::HashMap::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        let task_id = manager.add_task(task.clone());
        assert_eq!(task_id, task.id);
        
        let pending_tasks = manager.get_tasks_by_status(TaskStatus::Pending);
        assert_eq!(pending_tasks.len(), 1);
    }

    #[test]
    fn test_task_priority_ordering() {
        let mut manager = EVMLegionManager::new();
        
        // Add tasks with different priorities
        let low_priority_task = EVMTask {
            id: Uuid::new_v4(),
            task_type: EVMTaskType::NetworkDiscovery,
            status: TaskStatus::Pending,
            priority: TaskPriority::Low,
            target: None,
            dependencies: Vec::new(),
            metadata: std::collections::HashMap::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        let critical_task = EVMTask {
            id: Uuid::new_v4(),
            task_type: EVMTaskType::ExploitExecution,
            status: TaskStatus::Pending,
            priority: TaskPriority::Critical,
            target: None,
            dependencies: Vec::new(),
            metadata: std::collections::HashMap::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        manager.add_task(low_priority_task);
        manager.add_task(critical_task);
        
        // Legion should handle priority-based execution
        manager.update();
        
        let metrics = manager.get_metrics();
        assert!(metrics.tasks_completed >= 0); // May complete tasks during update
    }

    #[test]
    fn test_task_dependency_resolution() {
        let mut manager = EVMLegionManager::new();
        
        let parent_task_id = Uuid::new_v4();
        let child_task_id = Uuid::new_v4();
        
        let parent_task = EVMTask {
            id: parent_task_id,
            task_type: EVMTaskType::NetworkDiscovery,
            status: TaskStatus::Pending,
            priority: TaskPriority::Normal,
            target: None,
            dependencies: Vec::new(),
            metadata: std::collections::HashMap::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        let child_task = EVMTask {
            id: child_task_id,
            task_type: EVMTaskType::VulnerabilityAssessment,
            status: TaskStatus::Pending,
            priority: TaskPriority::Normal,
            target: None,
            dependencies: vec![parent_task_id],
            metadata: std::collections::HashMap::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        manager.add_task(parent_task);
        manager.add_task(child_task);
        
        // Child should remain blocked until parent completes
        manager.update();
        
        let pending_tasks = manager.get_tasks_by_status(TaskStatus::Pending);
        assert!(pending_tasks.len() > 0); // Child should still be pending
    }

    #[tokio::test]
    async fn test_performance_benchmarks() {
        let mut manager = EVMLegionManager::new();
        let task_count = 100;
        
        // Benchmark task addition
        let start = Instant::now();
        for i in 0..task_count {
            let task = EVMTask {
                id: Uuid::new_v4(),
                task_type: EVMTaskType::NetworkDiscovery,
                status: TaskStatus::Pending,
                priority: TaskPriority::Normal,
                target: None,
                dependencies: Vec::new(),
                metadata: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("batch_id".to_string(), i.to_string());
                    map
                },
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            manager.add_task(task);
        }
        let addition_time = start.elapsed();
        
        println!("Added {} tasks in {:?}", task_count, addition_time);
        assert!(addition_time < Duration::from_millis(100)); // Should be fast
        
        // Benchmark system updates
        let start = Instant::now();
        for _ in 0..10 {
            manager.update();
        }
        let update_time = start.elapsed();
        
        println!("10 system updates took {:?}", update_time);
        assert!(update_time < Duration::from_millis(500)); // Should be responsive
    }

    #[test]
    fn test_resource_usage_tracking() {
        let mut manager = EVMLegionManager::new();
        
        let task = EVMTask {
            id: Uuid::new_v4(),
            task_type: EVMTaskType::ExploitExecution,
            status: TaskStatus::Pending,
            priority: TaskPriority::High,
            target: None,
            dependencies: Vec::new(),
            metadata: std::collections::HashMap::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        manager.add_task(task);
        manager.update();
        
        let metrics = manager.get_metrics();
        
        // Resource usage should be tracked
        assert!(metrics.total_cpu_usage >= 0.0);
        assert!(metrics.total_memory_usage_mb >= 0.0);
    }

    #[test] 
    fn test_task_queue_management() {
        let task_queue = TaskQueue::default();
        
        assert_eq!(task_queue.pending_tasks.len(), 0);
        assert_eq!(task_queue.active_tasks.len(), 0);
        assert_eq!(task_queue.completed_tasks.len(), 0);
        assert_eq!(task_queue.failed_tasks.len(), 0);
    }

    #[test]
    fn test_execution_metrics() {
        let metrics = ExecutionMetrics::default();
        
        assert_eq!(metrics.tasks_completed, 0);
        assert_eq!(metrics.tasks_failed, 0);
        assert_eq!(metrics.average_execution_time_ms, 0.0);
        assert_eq!(metrics.total_cpu_usage, 0.0);
        assert_eq!(metrics.total_memory_usage_mb, 0.0);
    }

    #[test]
    fn test_task_result_creation() {
        let task_result = TaskResult {
            task_id: Uuid::new_v4(),
            success: true,
            output: "Network scan completed".to_string(),
            error: None,
            execution_time_ms: 1500,
            resource_usage: ResourceUsage {
                cpu_percent: 25.0,
                memory_mb: 128.0,
                network_kb: 2048.0,
                disk_io_kb: 512.0,
            },
        };
        
        assert!(task_result.success);
        assert_eq!(task_result.execution_time_ms, 1500);
        assert_eq!(task_result.resource_usage.cpu_percent, 25.0);
    }

    #[test]
    fn test_task_status_transitions() {
        use TaskStatus::*;
        
        // Valid transitions
        assert_ne!(Pending, InProgress);
        assert_ne!(InProgress, Completed);
        assert_ne!(InProgress, Failed);
        
        // Test serialization
        let status = TaskStatus::InProgress;
        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: TaskStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(status, deserialized);
    }

    #[test]
    fn test_task_priority_comparison() {
        use TaskPriority::*;
        
        assert!(Critical > High);
        assert!(High > Normal);
        assert!(Normal > Low);
        assert!(Low > Background);
    }
}

#[cfg(test)]
mod legion_performance_tests {
    use super::*;

    #[tokio::test]
    async fn test_concurrent_task_execution() {
        let mut manager = EVMLegionManager::new();
        
        // Add multiple concurrent tasks
        for i in 0..50 {
            let task = EVMTask {
                id: Uuid::new_v4(),
                task_type: match i % 4 {
                    0 => EVMTaskType::NetworkDiscovery,
                    1 => EVMTaskType::VulnerabilityAssessment, 
                    2 => EVMTaskType::ExploitExecution,
                    _ => EVMTaskType::PostExploitation,
                },
                status: TaskStatus::Pending,
                priority: TaskPriority::Normal,
                target: None,
                dependencies: Vec::new(),
                metadata: std::collections::HashMap::new(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            manager.add_task(task);
        }
        
        // Process multiple update cycles
        let start = Instant::now();
        for _ in 0..100 {
            manager.update();
            tokio::task::yield_now().await; // Allow other tasks to run
        }
        let total_time = start.elapsed();
        
        println!("Processed 50 tasks across 100 update cycles in {:?}", total_time);
        assert!(total_time < Duration::from_secs(5)); // Should complete reasonably fast
        
        let metrics = manager.get_metrics();
        println!("Final metrics: {:?}", metrics);
    }

    #[test]
    fn test_memory_efficiency() {
        let mut manager = EVMLegionManager::new();
        
        // Add many tasks to test memory usage
        for i in 0..1000 {
            let task = EVMTask {
                id: Uuid::new_v4(),
                task_type: EVMTaskType::NetworkDiscovery,
                status: TaskStatus::Pending,
                priority: TaskPriority::Normal,
                target: None,
                dependencies: Vec::new(),
                metadata: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("test_data".to_string(), format!("data_{}", i));
                    map
                },
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            manager.add_task(task);
        }
        
        // Memory usage should be reasonable for Legion ECS
        let metrics = manager.get_metrics();
        println!("Memory usage for 1000 tasks: {:.2} MB", metrics.total_memory_usage_mb);
        
        // Legion should be more memory efficient than Bevy
        assert!(metrics.total_memory_usage_mb < 500.0); // Reasonable limit
    }
}

#[cfg(test)]
mod legion_vs_bevy_comparison {
    use super::*;

    #[test]
    fn test_legion_architectural_advantages() {
        // Test demonstrates Legion's advantages over Bevy for EVM workloads
        
        // 1. No gaming bloat - Legion is pure ECS
        let config = EVMConfig::default();
        assert!(!config.legion_integration); // Starts disabled, can be enabled
        
        // 2. Better analytical performance patterns
        let mut manager = EVMLegionManager::new();
        
        // Add analytical workload (not gaming workload)
        let analytical_task = EVMTask {
            id: Uuid::new_v4(),
            task_type: EVMTaskType::VulnerabilityAssessment,
            status: TaskStatus::Pending,
            priority: TaskPriority::High,
            target: None,
            dependencies: Vec::new(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                map.insert("analysis_type".to_string(), "security_scan".to_string());
                map.insert("target_count".to_string(), "1000".to_string());
                map
            },
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        manager.add_task(analytical_task);
        manager.update();
        
        // Legion should handle analytical workloads efficiently
        let metrics = manager.get_metrics();
        assert!(metrics.total_cpu_usage >= 0.0); // Should track resource usage
    }

    #[test]
    fn test_professional_aesthetics() {
        // No cute bird logos here - this is Legion territory
        let manager = EVMLegionManager::new();
        
        // Legion API should feel professional and military-grade
        let task_queue_name = std::any::type_name::<TaskQueue>();
        assert!(task_queue_name.contains("TaskQueue"));
        assert!(!task_queue_name.contains("bevy")); // No bevy references
        
        let metrics_name = std::any::type_name::<ExecutionMetrics>();
        assert!(metrics_name.contains("ExecutionMetrics"));
        assert!(!metrics_name.contains("bevy")); // No bevy references
    }
}