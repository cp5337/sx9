//! Tool definitions and execution for the Claude API
//!
//! This module provides types for defining tools that Claude can use,
//! and for handling tool execution results.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Result;

/// A tool definition for Claude to use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Tool name (must match regex: ^[a-zA-Z0-9_-]{1,64}$)
    pub name: String,

    /// Human-readable description of what the tool does
    pub description: String,

    /// JSON Schema for the tool's input parameters
    pub input_schema: Value,
}

impl Tool {
    /// Create a new tool definition
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        input_schema: Value,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema,
        }
    }

    /// Create a tool with a simple string parameter
    pub fn with_string_param(
        name: impl Into<String>,
        description: impl Into<String>,
        param_name: impl Into<String>,
        param_description: impl Into<String>,
    ) -> Self {
        let param_name = param_name.into();
        Self {
            name: name.into(),
            description: description.into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    param_name.clone(): {
                        "type": "string",
                        "description": param_description.into()
                    }
                },
                "required": [param_name]
            }),
        }
    }
}

/// Tool choice strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolChoice {
    /// Let Claude decide whether to use a tool
    Auto,
    /// Force Claude to use any tool
    Any,
    /// Force Claude to use a specific tool
    Tool { name: String },
}

impl ToolChoice {
    /// Force use of a specific tool
    pub fn tool(name: impl Into<String>) -> Self {
        ToolChoice::Tool { name: name.into() }
    }
}

/// A tool use request from Claude
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUse {
    /// Unique ID for this tool use
    pub id: String,

    /// Name of the tool to use
    pub name: String,

    /// Input parameters for the tool
    pub input: Value,
}

impl ToolUse {
    /// Parse the input as a specific type
    pub fn parse_input<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        serde_json::from_value(self.input.clone()).map_err(|e| crate::ClaudeError::Json(e))
    }
}

/// Result from executing a tool
#[derive(Debug, Clone)]
pub struct ToolResult {
    /// The tool use ID this is responding to
    pub tool_use_id: String,

    /// The result content (typically JSON or text)
    pub content: String,

    /// Whether this result indicates an error
    pub is_error: bool,
}

impl ToolResult {
    /// Create a successful tool result
    pub fn success(tool_use_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            tool_use_id: tool_use_id.into(),
            content: content.into(),
            is_error: false,
        }
    }

    /// Create an error tool result
    pub fn error(tool_use_id: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            tool_use_id: tool_use_id.into(),
            content: error.into(),
            is_error: true,
        }
    }

    /// Create a result from a JSON-serializable value
    pub fn json<T: Serialize>(tool_use_id: impl Into<String>, value: &T) -> Result<Self> {
        Ok(Self {
            tool_use_id: tool_use_id.into(),
            content: serde_json::to_string(value)?,
            is_error: false,
        })
    }
}

/// Trait for implementing tool handlers
#[async_trait]
pub trait ToolHandler: Send + Sync {
    /// Execute the tool with the given input
    async fn execute(&self, input: Value) -> Result<ToolResult>;

    /// Get the tool definition
    fn definition(&self) -> Tool;
}

/// A registry of tool handlers
#[derive(Default)]
pub struct ToolRegistry {
    handlers: std::collections::HashMap<String, Box<dyn ToolHandler>>,
}

impl ToolRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a tool handler
    pub fn register(&mut self, handler: Box<dyn ToolHandler>) {
        let name = handler.definition().name.clone();
        self.handlers.insert(name, handler);
    }

    /// Get all tool definitions
    pub fn definitions(&self) -> Vec<Tool> {
        self.handlers.values().map(|h| h.definition()).collect()
    }

    /// Execute a tool by name
    pub async fn execute(&self, name: &str, input: Value) -> Result<ToolResult> {
        let handler = self
            .handlers
            .get(name)
            .ok_or_else(|| crate::ClaudeError::Tool(format!("Unknown tool: {}", name)))?;

        handler.execute(input).await
    }

    /// Check if a tool is registered
    pub fn has_tool(&self, name: &str) -> bool {
        self.handlers.contains_key(name)
    }
}

/// Common tool definitions for SX9 agents
pub mod common {
    use super::*;

    /// Create a file read tool definition
    pub fn read_file_tool() -> Tool {
        Tool::new(
            "read_file",
            "Read the contents of a file at the specified path",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "The path to the file to read"
                    }
                },
                "required": ["path"]
            }),
        )
    }

    /// Create a file write tool definition
    pub fn write_file_tool() -> Tool {
        Tool::new(
            "write_file",
            "Write content to a file at the specified path",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "The path to the file to write"
                    },
                    "content": {
                        "type": "string",
                        "description": "The content to write to the file"
                    }
                },
                "required": ["path", "content"]
            }),
        )
    }

    /// Create a bash command tool definition
    pub fn bash_tool() -> Tool {
        Tool::new(
            "bash",
            "Execute a bash command in the shell",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The bash command to execute"
                    }
                },
                "required": ["command"]
            }),
        )
    }

    /// Create a search tool definition
    pub fn search_tool() -> Tool {
        Tool::new(
            "search",
            "Search for files or content in the codebase",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query"
                    },
                    "path": {
                        "type": "string",
                        "description": "Optional path to search within"
                    }
                },
                "required": ["query"]
            }),
        )
    }
}
