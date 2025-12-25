//! Anthropic Claude provider
//!
//! This module implements the LLM provider trait for the Anthropic Claude API.

use async_trait::async_trait;

use super::{LlmProvider, ProviderConfig};
use crate::client::ClaudeClient;
use crate::messages::{MessageRequest, MessageResponse};
use crate::streaming::MessageStream;
use crate::Result;

/// Anthropic Claude provider
pub struct AnthropicProvider {
    client: ClaudeClient,
}

impl AnthropicProvider {
    /// Create a new Anthropic provider
    pub fn new(config: ProviderConfig) -> Result<Self> {
        let mut client = ClaudeClient::new(&config.api_key)?;

        if let Some(base_url) = config.base_url {
            client = client.with_base_url(base_url);
        }

        Ok(Self { client })
    }

    /// Create from environment variables
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            client: ClaudeClient::from_env()?,
        })
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    fn name(&self) -> &str {
        "anthropic"
    }

    fn models(&self) -> Vec<String> {
        vec![
            "claude-opus-4-5-20251101".to_string(),
            "claude-sonnet-4-20250514".to_string(),
            "claude-3-5-sonnet-20241022".to_string(),
            "claude-3-5-haiku-20241022".to_string(),
        ]
    }

    async fn message(&self, request: MessageRequest) -> Result<MessageResponse> {
        self.client.message(request).await
    }

    async fn stream(&self, request: MessageRequest) -> Result<MessageStream> {
        self.client.stream(request).await
    }
}

impl std::fmt::Debug for AnthropicProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnthropicProvider")
            .field("client", &self.client)
            .finish()
    }
}
