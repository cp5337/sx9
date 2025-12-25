//! Error types for the Claude SDK

use thiserror::Error;

/// Result type alias for Claude SDK operations
pub type Result<T> = std::result::Result<T, ClaudeError>;

/// Errors that can occur when using the Claude SDK
#[derive(Debug, Error)]
pub enum ClaudeError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// API returned an error response
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        error_type: String,
        message: String,
    },

    /// Rate limit exceeded
    #[error("Rate limit exceeded, retry after {retry_after_seconds}s")]
    RateLimited { retry_after_seconds: u64 },

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Invalid request
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Stream parsing error
    #[error("Stream error: {0}")]
    Stream(String),

    /// Memory provider error
    #[error("Memory error: {0}")]
    Memory(String),

    /// Tool execution error
    #[error("Tool error: {0}")]
    Tool(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Environment variable missing
    #[error("Environment variable not set: {0}")]
    EnvVar(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// MCP protocol error
    #[error("MCP error: {0}")]
    Mcp(String),

    /// Network error
    #[error("Network error: {0}")]
    Network(String),

    /// Parse error
    #[error("Parse error: {0}")]
    Parse(String),
}

impl ClaudeError {
    /// Create an API error from response details
    pub fn api_error(status: u16, error_type: impl Into<String>, message: impl Into<String>) -> Self {
        ClaudeError::Api {
            status,
            error_type: error_type.into(),
            message: message.into(),
        }
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ClaudeError::RateLimited { .. }
                | ClaudeError::Http(_)
                | ClaudeError::Api { status: 500..=599, .. }
        )
    }
}
