//! Error types for CTAS-7 agent system

use thiserror::Error;

/// Result type alias for CTAS-7 operations
pub type Result<T> = std::result::Result<T, AgentError>;

/// Core error types for agent operations
#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Agent not found: {id}")]
    AgentNotFound { id: String },

    #[error("Task execution failed: {task_id}, reason: {reason}")]
    TaskExecutionFailed { task_id: String, reason: String },

    #[error("Message routing failed: {message_id}, reason: {reason}")]
    MessageRoutingFailed { message_id: String, reason: String },

    #[error("Agent registration failed: {agent_id}, reason: {reason}")]
    RegistrationFailed { agent_id: String, reason: String },

    #[error("Health check failed: {component}, reason: {reason}")]
    HealthCheckFailed { component: String, reason: String },

    #[error("Configuration error: {field}, reason: {reason}")]
    ConfigurationError { field: String, reason: String },

    #[error("Network error: {operation}, reason: {reason}")]
    NetworkError { operation: String, reason: String },

    #[error("Serialization error: {reason}")]
    SerializationError { reason: String },

    #[error("Authentication error: {reason}")]
    AuthenticationError { reason: String },

    #[error("Permission denied: {operation}, agent: {agent_id}")]
    PermissionDenied { operation: String, agent_id: String },

    #[error("Timeout error: {operation}, timeout_ms: {timeout_ms}")]
    TimeoutError { operation: String, timeout_ms: u64 },

    #[error("Resource exhausted: {resource}, limit: {limit}")]
    ResourceExhausted { resource: String, limit: String },

    #[error("Invalid input: {field}, reason: {reason}")]
    InvalidInput { field: String, reason: String },

    #[error("Internal error: {reason}")]
    InternalError { reason: String },
}

impl From<anyhow::Error> for AgentError {
    fn from(err: anyhow::Error) -> Self {
        AgentError::InternalError {
            reason: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for AgentError {
    fn from(err: serde_json::Error) -> Self {
        AgentError::SerializationError {
            reason: err.to_string(),
        }
    }
}