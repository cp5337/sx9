//! Claude API HTTP client
//!
//! This module provides the main client for interacting with the Claude API.

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use secrecy::{ExposeSecret, SecretString};
use std::time::Duration;

use crate::error::ClaudeError;
use crate::memory::MemoryProvider;
use crate::messages::{ErrorResponse, MessageRequest, MessageResponse};
use crate::streaming::MessageStream;
use crate::tools::Tool;
use crate::{Result, API_BASE_URL, DEFAULT_MAX_TOKENS, DEFAULT_MODEL};

/// Claude API client
pub struct ClaudeClient {
    /// API key (stored securely)
    api_key: SecretString,

    /// Model to use
    model: String,

    /// Maximum tokens for responses
    max_tokens: u32,

    /// HTTP client
    http_client: reqwest::Client,

    /// Base URL for API
    base_url: String,

    /// Optional memory provider
    memory: Option<Box<dyn MemoryProvider>>,

    /// Default tools
    tools: Vec<Tool>,
}

impl ClaudeClient {
    /// Create a new client with the given API key
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let api_key = SecretString::new(api_key.into().into());

        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(300)) // 5 minute timeout for long responses
            .build()?;

        Ok(Self {
            api_key,
            model: DEFAULT_MODEL.to_string(),
            max_tokens: DEFAULT_MAX_TOKENS,
            http_client,
            base_url: API_BASE_URL.to_string(),
            memory: None,
            tools: Vec::new(),
        })
    }

    /// Create a client from the ANTHROPIC_API_KEY environment variable
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| ClaudeError::EnvVar("ANTHROPIC_API_KEY".to_string()))?;

        Self::new(api_key)
    }

    /// Set the model to use
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Set the maximum tokens for responses
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Set the base URL (useful for testing or proxies)
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Set a memory provider
    pub fn with_memory(mut self, memory: Box<dyn MemoryProvider>) -> Self {
        self.memory = Some(memory);
        self
    }

    /// Add default tools to all requests
    pub fn with_tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = tools;
        self
    }

    /// Get the current model
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Build request headers
    fn build_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        headers.insert(
            "x-api-key",
            HeaderValue::from_str(self.api_key.expose_secret())
                .map_err(|_| ClaudeError::Config("Invalid API key format".to_string()))?,
        );

        headers.insert(
            "anthropic-version",
            HeaderValue::from_static("2023-06-01"),
        );

        Ok(headers)
    }

    /// Send a message and get a response
    pub async fn message(&self, mut request: MessageRequest) -> Result<MessageResponse> {
        // Apply defaults
        if request.model.is_empty() {
            request.model = self.model.clone();
        }
        if request.max_tokens == 0 {
            request.max_tokens = self.max_tokens;
        }

        // Add default tools if not already present
        if request.tools.is_empty() && !self.tools.is_empty() {
            request.tools = self.tools.clone();
        }

        // Ensure streaming is disabled for this method
        request.stream = Some(false);

        let url = format!("{}/messages", self.base_url);

        let response = self
            .http_client
            .post(&url)
            .headers(self.build_headers()?)
            .json(&request)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            return Err(self.handle_error_response(status.as_u16(), response).await);
        }

        let message_response: MessageResponse = response.json().await?;

        Ok(message_response)
    }

    /// Stream a response via SSE
    pub async fn stream(&self, mut request: MessageRequest) -> Result<MessageStream> {
        // Apply defaults
        if request.model.is_empty() {
            request.model = self.model.clone();
        }
        if request.max_tokens == 0 {
            request.max_tokens = self.max_tokens;
        }

        // Add default tools if not already present
        if request.tools.is_empty() && !self.tools.is_empty() {
            request.tools = self.tools.clone();
        }

        // Enable streaming
        request.stream = Some(true);

        let url = format!("{}/messages", self.base_url);

        let response = self
            .http_client
            .post(&url)
            .headers(self.build_headers()?)
            .json(&request)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            return Err(self.handle_error_response(status.as_u16(), response).await);
        }

        Ok(MessageStream::new(response))
    }

    /// Handle error responses from the API
    async fn handle_error_response(
        &self,
        status: u16,
        response: reqwest::Response,
    ) -> ClaudeError {
        // Check for rate limiting
        if status == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);

            return ClaudeError::RateLimited {
                retry_after_seconds: retry_after,
            };
        }

        // Try to parse error response
        match response.json::<ErrorResponse>().await {
            Ok(error_response) => ClaudeError::Api {
                status,
                error_type: error_response.error.error_type,
                message: error_response.error.message,
            },
            Err(_) => ClaudeError::Api {
                status,
                error_type: "unknown".to_string(),
                message: format!("HTTP {} error", status),
            },
        }
    }

    /// Store a memory entry (if memory provider is configured)
    pub async fn store_memory(&self, key: &str, content: &str) -> Result<()> {
        if let Some(ref memory) = self.memory {
            memory.store(key, content).await
        } else {
            Err(ClaudeError::Memory("No memory provider configured".to_string()))
        }
    }

    /// Retrieve a memory entry (if memory provider is configured)
    pub async fn retrieve_memory(&self, key: &str) -> Result<Option<String>> {
        if let Some(ref memory) = self.memory {
            memory.retrieve(key).await
        } else {
            Err(ClaudeError::Memory("No memory provider configured".to_string()))
        }
    }

    /// Search memory (if memory provider is configured)
    pub async fn search_memory(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<crate::memory::MemoryEntry>> {
        if let Some(ref memory) = self.memory {
            memory.search(query, limit).await
        } else {
            Err(ClaudeError::Memory("No memory provider configured".to_string()))
        }
    }
}

impl std::fmt::Debug for ClaudeClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClaudeClient")
            .field("model", &self.model)
            .field("max_tokens", &self.max_tokens)
            .field("base_url", &self.base_url)
            .field("api_key", &"[REDACTED]")
            .field("has_memory", &self.memory.is_some())
            .field("tools_count", &self.tools.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_debug_redacts_api_key() {
        let client = ClaudeClient::new("sk-test-key").unwrap();
        let debug = format!("{:?}", client);
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains("sk-test-key"));
    }

    #[test]
    fn test_client_model_default() {
        let client = ClaudeClient::new("sk-test-key").unwrap();
        assert_eq!(client.model(), DEFAULT_MODEL);
    }

    #[test]
    fn test_client_with_model() {
        let client = ClaudeClient::new("sk-test-key")
            .unwrap()
            .with_model("claude-opus-4-5-20251101");
        assert_eq!(client.model(), "claude-opus-4-5-20251101");
    }
}
