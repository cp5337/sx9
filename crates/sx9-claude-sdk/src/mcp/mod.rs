//! Model Context Protocol (MCP) Transport Layer
//!
//! This module provides MCP transport implementations for connecting Claude
//! to external tools and services via the standardized MCP protocol.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                     MCP TRANSPORT LAYER                      │
//! ├─────────────────────────────────────────────────────────────┤
//! │                                                              │
//! │  ┌──────────────┐       ┌──────────────┐                   │
//! │  │ StdioTransport│       │ HttpTransport │                   │
//! │  │              │       │              │                   │
//! │  │ stdin/stdout │       │ HTTP/SSE     │                   │
//! │  └──────┬───────┘       └──────┬───────┘                   │
//! │         │                      │                            │
//! │         └──────────┬───────────┘                            │
//! │                    │                                        │
//! │                    ▼                                        │
//! │         ┌──────────────────┐                               │
//! │         │   McpClient      │                               │
//! │         │                  │                               │
//! │         │ • list_tools()   │                               │
//! │         │ • call_tool()    │                               │
//! │         │ • list_prompts() │                               │
//! │         │ • get_prompt()   │                               │
//! │         │ • list_resources()│                               │
//! │         │ • read_resource()│                               │
//! │         └──────────────────┘                               │
//! │                                                              │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! // HTTP transport (e.g., for remote MCP servers)
//! let transport = HttpTransport::new("http://localhost:3000/mcp");
//! let client = McpClient::new(transport);
//!
//! // List available tools
//! let tools = client.list_tools().await?;
//!
//! // Call a tool
//! let result = client.call_tool("search", json!({"query": "rust"})).await?;
//! ```

pub mod protocol;
pub mod transport;

pub use protocol::*;
pub use transport::*;

use async_trait::async_trait;
use serde_json::Value;

use crate::Result;

/// MCP transport trait for different communication methods
#[async_trait]
pub trait McpTransport: Send + Sync {
    /// Send a request and receive a response
    async fn request(&self, method: &str, params: Option<Value>) -> Result<Value>;

    /// Check if the transport is connected
    fn is_connected(&self) -> bool;

    /// Close the transport
    async fn close(&self) -> Result<()>;
}

/// MCP client for interacting with MCP servers
pub struct McpClient<T: McpTransport> {
    transport: T,
    server_info: Option<ServerInfo>,
}

impl<T: McpTransport> McpClient<T> {
    /// Create a new MCP client with the given transport
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            server_info: None,
        }
    }

    /// Initialize connection and get server capabilities
    pub async fn initialize(&mut self, client_info: ClientInfo) -> Result<InitializeResult> {
        let params = serde_json::to_value(InitializeParams {
            protocol_version: PROTOCOL_VERSION.to_string(),
            capabilities: ClientCapabilities::default(),
            client_info,
        })?;

        let response = self.transport.request("initialize", Some(params)).await?;
        let result: InitializeResult = serde_json::from_value(response)?;

        self.server_info = Some(result.server_info.clone());

        // Send initialized notification
        self.transport.request("initialized", None).await?;

        Ok(result)
    }

    /// List available tools
    pub async fn list_tools(&self) -> Result<ListToolsResult> {
        let response = self.transport.request("tools/list", None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Call a tool with arguments
    pub async fn call_tool(&self, name: &str, arguments: Value) -> Result<CallToolResult> {
        let params = serde_json::json!({
            "name": name,
            "arguments": arguments
        });

        let response = self.transport.request("tools/call", Some(params)).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// List available prompts
    pub async fn list_prompts(&self) -> Result<ListPromptsResult> {
        let response = self.transport.request("prompts/list", None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Get a specific prompt
    pub async fn get_prompt(&self, name: &str, arguments: Option<Value>) -> Result<GetPromptResult> {
        let params = serde_json::json!({
            "name": name,
            "arguments": arguments
        });

        let response = self.transport.request("prompts/get", Some(params)).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// List available resources
    pub async fn list_resources(&self) -> Result<ListResourcesResult> {
        let response = self.transport.request("resources/list", None).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Read a resource
    pub async fn read_resource(&self, uri: &str) -> Result<ReadResourceResult> {
        let params = serde_json::json!({
            "uri": uri
        });

        let response = self.transport.request("resources/read", Some(params)).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Get server info (after initialization)
    pub fn server_info(&self) -> Option<&ServerInfo> {
        self.server_info.as_ref()
    }

    /// Close the client connection
    pub async fn close(&self) -> Result<()> {
        self.transport.close().await
    }
}

impl<T: McpTransport + std::fmt::Debug> std::fmt::Debug for McpClient<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("McpClient")
            .field("server_info", &self.server_info)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_version() {
        assert!(!PROTOCOL_VERSION.is_empty());
    }
}
