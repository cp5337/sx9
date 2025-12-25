//! Message types for the Claude API
//!
//! This module defines the core types for interacting with the Claude Messages API.

use serde::{Deserialize, Serialize};

use crate::tools::{Tool, ToolChoice, ToolUse};

/// A conversation message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// The role of the message sender
    pub role: Role,

    /// The content of the message
    pub content: MessageContent,
}

impl Message {
    /// Create a user message with text content
    pub fn user(text: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: MessageContent::Text(text.into()),
        }
    }

    /// Create an assistant message with text content
    pub fn assistant(text: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: MessageContent::Text(text.into()),
        }
    }

    /// Create a message with multiple content blocks
    pub fn with_blocks(role: Role, blocks: Vec<ContentBlock>) -> Self {
        Self {
            role,
            content: MessageContent::Blocks(blocks),
        }
    }
}

/// Message sender role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

/// Message content - either simple text or structured blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    /// Simple text content
    Text(String),
    /// Structured content blocks
    Blocks(Vec<ContentBlock>),
}

/// A content block within a message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    /// Text content
    Text { text: String },

    /// Image content (base64 encoded)
    Image {
        source: ImageSource,
    },

    /// Tool use request from assistant
    ToolUse(ToolUse),

    /// Tool result from user
    ToolResult {
        tool_use_id: String,
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_error: Option<bool>,
    },
}

impl ContentBlock {
    /// Create a text content block
    pub fn text(text: impl Into<String>) -> Self {
        ContentBlock::Text { text: text.into() }
    }

    /// Create a tool result block
    pub fn tool_result(tool_use_id: impl Into<String>, content: impl Into<String>) -> Self {
        ContentBlock::ToolResult {
            tool_use_id: tool_use_id.into(),
            content: content.into(),
            is_error: None,
        }
    }

    /// Create an error tool result block
    pub fn tool_error(tool_use_id: impl Into<String>, error: impl Into<String>) -> Self {
        ContentBlock::ToolResult {
            tool_use_id: tool_use_id.into(),
            content: error.into(),
            is_error: Some(true),
        }
    }

    /// Extract text content if this is a text block
    pub fn as_text(&self) -> Option<&str> {
        match self {
            ContentBlock::Text { text } => Some(text),
            _ => None,
        }
    }
}

/// Image source for image content blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSource {
    /// Source type (currently only "base64")
    #[serde(rename = "type")]
    pub source_type: String,
    /// Media type (e.g., "image/png")
    pub media_type: String,
    /// Base64 encoded image data
    pub data: String,
}

/// Request to create a message
#[derive(Debug, Clone, Serialize)]
pub struct MessageRequest {
    /// Model to use (e.g., "claude-sonnet-4-20250514")
    pub model: String,

    /// Conversation messages
    pub messages: Vec<Message>,

    /// Maximum tokens to generate
    pub max_tokens: u32,

    /// System prompt (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// Available tools (optional)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,

    /// Tool choice strategy (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// Temperature for sampling (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Top-p sampling (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// Stop sequences (optional)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stop_sequences: Vec<String>,

    /// Enable streaming (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Metadata (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MessageMetadata>,
}

impl Default for MessageRequest {
    fn default() -> Self {
        Self {
            model: crate::DEFAULT_MODEL.to_string(),
            messages: Vec::new(),
            max_tokens: crate::DEFAULT_MAX_TOKENS,
            system: None,
            tools: Vec::new(),
            tool_choice: None,
            temperature: None,
            top_p: None,
            stop_sequences: Vec::new(),
            stream: None,
            metadata: None,
        }
    }
}

/// Message metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMetadata {
    /// User ID for tracking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Response from the Messages API
#[derive(Debug, Clone, Deserialize)]
pub struct MessageResponse {
    /// Unique message ID
    pub id: String,

    /// Object type (always "message")
    #[serde(rename = "type")]
    pub object_type: String,

    /// Role (always "assistant")
    pub role: Role,

    /// Response content blocks
    pub content: Vec<ContentBlock>,

    /// Model used
    pub model: String,

    /// Reason generation stopped
    pub stop_reason: Option<StopReason>,

    /// Stop sequence that triggered stop (if any)
    pub stop_sequence: Option<String>,

    /// Token usage
    pub usage: Usage,
}

impl MessageResponse {
    /// Extract all text content from the response
    pub fn content_text(&self) -> String {
        self.content
            .iter()
            .filter_map(|block| block.as_text())
            .collect::<Vec<_>>()
            .join("")
    }

    /// Get all tool use requests from the response
    pub fn tool_uses(&self) -> Vec<&ToolUse> {
        self.content
            .iter()
            .filter_map(|block| match block {
                ContentBlock::ToolUse(tool_use) => Some(tool_use),
                _ => None,
            })
            .collect()
    }

    /// Check if the response contains tool use requests
    pub fn has_tool_use(&self) -> bool {
        self.content.iter().any(|block| matches!(block, ContentBlock::ToolUse(_)))
    }
}

/// Reason why generation stopped
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    /// Model finished generating
    EndTurn,
    /// Max tokens reached
    MaxTokens,
    /// Stop sequence encountered
    StopSequence,
    /// Tool use requested
    ToolUse,
}

/// Token usage information
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Usage {
    /// Input tokens consumed
    pub input_tokens: u32,
    /// Output tokens generated
    pub output_tokens: u32,
}

impl Usage {
    /// Total tokens used
    pub fn total(&self) -> u32 {
        self.input_tokens + self.output_tokens
    }
}

/// API error response
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    /// Error type
    #[serde(rename = "type")]
    pub error_type: String,
    /// Error details
    pub error: ErrorDetail,
}

/// Error details
#[derive(Debug, Deserialize)]
pub struct ErrorDetail {
    /// Error type
    #[serde(rename = "type")]
    pub error_type: String,
    /// Error message
    pub message: String,
}
