//! LLM Provider abstractions
//!
//! This module provides abstractions for different LLM providers,
//! enabling future support for OpenAI, Gemini, and other APIs.

pub mod anthropic;

#[cfg(feature = "openai")]
pub mod openai;

#[cfg(feature = "gemini")]
pub mod gemini;

use async_trait::async_trait;

use crate::messages::{MessageRequest, MessageResponse};
use crate::streaming::MessageStream;
use crate::Result;

/// Trait for LLM providers
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Get the provider name
    fn name(&self) -> &str;

    /// Get available models
    fn models(&self) -> Vec<String>;

    /// Send a message request
    async fn message(&self, request: MessageRequest) -> Result<MessageResponse>;

    /// Stream a message response
    async fn stream(&self, request: MessageRequest) -> Result<MessageStream>;
}

/// Provider configuration
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    /// API key
    pub api_key: String,

    /// Base URL (optional, for proxies)
    pub base_url: Option<String>,

    /// Request timeout in seconds
    pub timeout_seconds: u64,

    /// Maximum retries
    pub max_retries: u32,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: None,
            timeout_seconds: 300,
            max_retries: 3,
        }
    }
}
