//! Core traits for CTAS-7 agent system

use async_trait::async_trait;
use crate::{AgentId, Message, Task, TaskResult, AgentMetadata, Result};

/// Core trait for agent message handling
#[async_trait]
pub trait MessageHandler {
    /// Handle incoming message
    async fn handle_message(&self, message: Message) -> Result<()>;

    /// Send outbound message
    async fn send_message(&self, message: Message) -> Result<()>;
}

/// Core trait for task execution
#[async_trait]
pub trait TaskExecutor {
    /// Execute a task and return result
    async fn execute_task(&self, task: Task) -> Result<TaskResult>;

    /// Check if agent can handle this task
    fn can_handle(&self, task: &Task) -> bool;
}

/// Core trait for agent lifecycle management
#[async_trait]
pub trait AgentLifecycle {
    /// Initialize agent
    async fn initialize(&mut self) -> Result<()>;

    /// Start agent processing
    async fn start(&mut self) -> Result<()>;

    /// Stop agent gracefully
    async fn stop(&mut self) -> Result<()>;

    /// Get current agent metadata
    fn metadata(&self) -> AgentMetadata;
}

/// Core trait for message routing
#[async_trait]
pub trait MessageRouter {
    /// Route message to appropriate agent
    async fn route_message(&self, message: Message) -> Result<()>;

    /// Register agent for message routing
    async fn register_agent(&mut self, metadata: AgentMetadata) -> Result<()>;

    /// Unregister agent
    async fn unregister_agent(&mut self, agent_id: AgentId) -> Result<()>;
}

/// Core trait for agent discovery
#[async_trait]
pub trait AgentDiscovery {
    /// Find agents by capability
    async fn find_by_capability(&self, capability: &str) -> Result<Vec<AgentMetadata>>;

    /// Get agent by ID
    async fn get_agent(&self, agent_id: AgentId) -> Result<Option<AgentMetadata>>;

    /// List all active agents
    async fn list_agents(&self) -> Result<Vec<AgentMetadata>>;
}

/// Core trait for health monitoring
#[async_trait]
pub trait HealthMonitor {
    /// Check agent health
    async fn check_health(&self, agent_id: AgentId) -> Result<bool>;

    /// Get system health status
    async fn system_health(&self) -> Result<bool>;
}