//! # SX9 Claude SDK
//!
//! Native Rust client for the Anthropic Claude API, designed for the SX9
//! autonomous agent SDK. This crate provides:
//!
//! - HTTP client for Claude Messages API
//! - SSE streaming response handler
//! - Tool definitions and execution
//! - Extensible memory providers (Sled-backed by default)
//! - MCP transport support (optional)
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use sx9_claude_sdk::{ClaudeClient, MessageRequest, Message};
//!
//! #[tokio::main]
//! async fn main() -> sx9_claude_sdk::Result<()> {
//!     let client = ClaudeClient::from_env()?;
//!
//!     let response = client.message(MessageRequest {
//!         messages: vec![Message::user("Hello, Claude!")],
//!         ..Default::default()
//!     }).await?;
//!
//!     println!("{}", response.content_text());
//!     Ok(())
//! }
//! ```
//!
//! ## RFC Reference
//!
//! This crate implements the specifications defined in:
//! - RFC-9145: SX9 Autonomous Agent SDK Operating Manual
//! - RFC-9141: FORGE Assembly Line & Dual-Heartbeat QA Doctrine

pub mod client;
pub mod messages;
pub mod streaming;
pub mod tools;

pub mod memory;

#[cfg(feature = "mcp")]
pub mod mcp;

pub mod providers;

mod error;

// Re-exports for convenience
pub use client::ClaudeClient;
pub use error::{ClaudeError, Result};
pub use messages::{
    ContentBlock, Message, MessageRequest, MessageResponse, Role, StopReason, Usage,
};
pub use streaming::MessageStream;
pub use tools::{Tool, ToolChoice, ToolResult, ToolUse};

pub use memory::{MemoryEntry, MemoryProvider};

/// Default Claude model to use
pub const DEFAULT_MODEL: &str = "claude-sonnet-4-20250514";

/// Maximum tokens for responses
pub const DEFAULT_MAX_TOKENS: u32 = 4096;

/// Anthropic API base URL
pub const API_BASE_URL: &str = "https://api.anthropic.com/v1";
