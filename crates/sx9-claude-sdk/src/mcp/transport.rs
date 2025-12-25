//! MCP Transport Implementations
//!
//! This module provides transport layer implementations for MCP:
//! - HTTP transport for remote servers
//! - Stdio transport for local subprocess servers
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                         MCP TRANSPORTS                                   │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │                                                                          │
//! │  ┌─────────────────────────────┐    ┌─────────────────────────────┐    │
//! │  │     HttpTransport           │    │     StdioTransport          │    │
//! │  ├─────────────────────────────┤    ├─────────────────────────────┤    │
//! │  │                             │    │                             │    │
//! │  │  HTTP POST + SSE            │    │  Subprocess stdin/stdout   │    │
//! │  │  JSON-RPC over HTTPS        │    │  JSON-RPC over pipes       │    │
//! │  │                             │    │                             │    │
//! │  │  Use case:                  │    │  Use case:                  │    │
//! │  │  • Remote MCP servers       │    │  • Local MCP servers        │    │
//! │  │  • Cloud-hosted tools       │    │  • npx @mcp/server-xxx      │    │
//! │  │  • API gateways             │    │  • Python MCP servers       │    │
//! │  │                             │    │                             │    │
//! │  └─────────────────────────────┘    └─────────────────────────────┘    │
//! │                                                                          │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```

use async_trait::async_trait;
use serde_json::Value;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::sync::{Mutex, RwLock};

use super::protocol::{JsonRpcRequest, JsonRpcResponse};
use super::McpTransport;
use crate::{ClaudeError, Result};

// ============================================================================
// HTTP Transport
// ============================================================================

/// HTTP transport for remote MCP servers
///
/// Uses HTTP POST for requests and can handle SSE for streaming responses.
pub struct HttpTransport {
    /// Base URL for the MCP server
    base_url: String,

    /// HTTP client
    client: reqwest::Client,

    /// Request ID counter
    request_id: AtomicU64,

    /// Connection state
    connected: AtomicBool,
}

impl HttpTransport {
    /// Create a new HTTP transport
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: reqwest::Client::new(),
            request_id: AtomicU64::new(1),
            connected: AtomicBool::new(true),
        }
    }

    /// Create with custom HTTP client
    pub fn with_client(base_url: impl Into<String>, client: reqwest::Client) -> Self {
        Self {
            base_url: base_url.into(),
            client,
            request_id: AtomicU64::new(1),
            connected: AtomicBool::new(true),
        }
    }

    /// Get next request ID
    fn next_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }
}

#[async_trait]
impl McpTransport for HttpTransport {
    async fn request(&self, method: &str, params: Option<Value>) -> Result<Value> {
        let id = self.next_id();
        let request = JsonRpcRequest::new(id, method, params);

        let response = self
            .client
            .post(&self.base_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| ClaudeError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ClaudeError::Network(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        let rpc_response: JsonRpcResponse = response
            .json()
            .await
            .map_err(|e| ClaudeError::Parse(e.to_string()))?;

        if let Some(error) = rpc_response.error {
            return Err(ClaudeError::Mcp(format!(
                "{}: {}",
                error.code, error.message
            )));
        }

        rpc_response
            .result
            .ok_or_else(|| ClaudeError::Mcp("Empty response".to_string()))
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    async fn close(&self) -> Result<()> {
        self.connected.store(false, Ordering::SeqCst);
        Ok(())
    }
}

impl std::fmt::Debug for HttpTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpTransport")
            .field("base_url", &self.base_url)
            .field("connected", &self.connected.load(Ordering::SeqCst))
            .finish()
    }
}

// ============================================================================
// Stdio Transport
// ============================================================================

/// Stdio transport for local MCP server subprocesses
///
/// Spawns a subprocess and communicates via stdin/stdout using JSON-RPC.
pub struct StdioTransport {
    /// Child process
    child: Arc<Mutex<Option<Child>>>,

    /// Stdin writer
    stdin: Arc<Mutex<Option<ChildStdin>>>,

    /// Stdout reader
    stdout: Arc<Mutex<Option<BufReader<ChildStdout>>>>,

    /// Request ID counter
    request_id: AtomicU64,

    /// Connection state
    connected: Arc<AtomicBool>,

    /// Pending responses
    pending: Arc<RwLock<std::collections::HashMap<u64, tokio::sync::oneshot::Sender<Value>>>>,
}

impl StdioTransport {
    /// Spawn a new MCP server process
    pub async fn spawn(command: &str, args: &[&str]) -> Result<Self> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .map_err(|e| ClaudeError::Mcp(format!("Failed to spawn process: {}", e)))?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| ClaudeError::Mcp("Failed to get stdin".to_string()))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| ClaudeError::Mcp("Failed to get stdout".to_string()))?;

        let transport = Self {
            child: Arc::new(Mutex::new(Some(child))),
            stdin: Arc::new(Mutex::new(Some(stdin))),
            stdout: Arc::new(Mutex::new(Some(BufReader::new(stdout)))),
            request_id: AtomicU64::new(1),
            connected: Arc::new(AtomicBool::new(true)),
            pending: Arc::new(RwLock::new(std::collections::HashMap::new())),
        };

        // Start response reader task
        transport.start_reader();

        Ok(transport)
    }

    /// Get next request ID
    fn next_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Start background reader task
    fn start_reader(&self) {
        let stdout = self.stdout.clone();
        let pending = self.pending.clone();
        let connected = self.connected.clone();

        tokio::spawn(async move {
            loop {
                let mut line = String::new();

                {
                    let mut stdout_guard = stdout.lock().await;
                    if let Some(reader) = stdout_guard.as_mut() {
                        match reader.read_line(&mut line).await {
                            Ok(0) => {
                                // EOF
                                connected.store(false, Ordering::SeqCst);
                                break;
                            }
                            Ok(_) => {}
                            Err(e) => {
                                tracing::error!("Error reading from MCP server: {}", e);
                                connected.store(false, Ordering::SeqCst);
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }

                if line.is_empty() {
                    continue;
                }

                // Parse JSON-RPC response
                if let Ok(response) = serde_json::from_str::<JsonRpcResponse>(&line) {
                    if let super::protocol::JsonRpcId::Number(id) = response.id {
                        let mut pending_guard = pending.write().await;
                        if let Some(sender) = pending_guard.remove(&id) {
                            let value = if let Some(error) = response.error {
                                serde_json::json!({
                                    "error": {
                                        "code": error.code,
                                        "message": error.message
                                    }
                                })
                            } else {
                                response.result.unwrap_or(Value::Null)
                            };
                            let _ = sender.send(value);
                        }
                    }
                }
            }
        });
    }
}

#[async_trait]
impl McpTransport for StdioTransport {
    async fn request(&self, method: &str, params: Option<Value>) -> Result<Value> {
        if !self.is_connected() {
            return Err(ClaudeError::Mcp("Transport not connected".to_string()));
        }

        let id = self.next_id();
        let request = JsonRpcRequest::new(id, method, params);

        // Create response channel
        let (tx, rx) = tokio::sync::oneshot::channel();

        // Register pending request
        {
            let mut pending_guard = self.pending.write().await;
            pending_guard.insert(id, tx);
        }

        // Send request
        {
            let mut stdin_guard = self.stdin.lock().await;
            if let Some(stdin) = stdin_guard.as_mut() {
                let json = serde_json::to_string(&request)?;
                stdin
                    .write_all(json.as_bytes())
                    .await
                    .map_err(|e| ClaudeError::Mcp(format!("Failed to write: {}", e)))?;
                stdin
                    .write_all(b"\n")
                    .await
                    .map_err(|e| ClaudeError::Mcp(format!("Failed to write newline: {}", e)))?;
                stdin
                    .flush()
                    .await
                    .map_err(|e| ClaudeError::Mcp(format!("Failed to flush: {}", e)))?;
            } else {
                return Err(ClaudeError::Mcp("Stdin not available".to_string()));
            }
        }

        // Wait for response with timeout
        let result = tokio::time::timeout(std::time::Duration::from_secs(30), rx)
            .await
            .map_err(|_| ClaudeError::Mcp("Request timeout".to_string()))?
            .map_err(|_| ClaudeError::Mcp("Response channel closed".to_string()))?;

        // Check for error in result
        if let Some(error) = result.get("error") {
            return Err(ClaudeError::Mcp(format!(
                "{}: {}",
                error.get("code").and_then(|c| c.as_i64()).unwrap_or(-1),
                error
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error")
            )));
        }

        Ok(result)
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    async fn close(&self) -> Result<()> {
        self.connected.store(false, Ordering::SeqCst);

        // Close stdin
        {
            let mut stdin_guard = self.stdin.lock().await;
            *stdin_guard = None;
        }

        // Kill child process
        {
            let mut child_guard = self.child.lock().await;
            if let Some(mut child) = child_guard.take() {
                let _ = child.kill().await;
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for StdioTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StdioTransport")
            .field("connected", &self.connected.load(Ordering::SeqCst))
            .finish()
    }
}

// ============================================================================
// Factory Functions
// ============================================================================

/// Create an HTTP transport for a remote MCP server
pub fn http(url: impl Into<String>) -> HttpTransport {
    HttpTransport::new(url)
}

/// Spawn a stdio transport for a local MCP server
pub async fn stdio(command: &str, args: &[&str]) -> Result<StdioTransport> {
    StdioTransport::spawn(command, args).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_transport_debug() {
        let transport = HttpTransport::new("http://localhost:3000");
        let debug = format!("{:?}", transport);
        assert!(debug.contains("HttpTransport"));
        assert!(debug.contains("localhost:3000"));
    }

    #[tokio::test]
    async fn test_http_transport_close() {
        let transport = HttpTransport::new("http://localhost:3000");
        assert!(transport.is_connected());

        transport.close().await.unwrap();
        assert!(!transport.is_connected());
    }
}
