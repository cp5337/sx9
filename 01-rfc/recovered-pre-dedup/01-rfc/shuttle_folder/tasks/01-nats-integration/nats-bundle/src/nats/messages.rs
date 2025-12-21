//! Common NATS Message Types
//!
//! Shared message structures for SX9 services.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════
// Common Header
// ═══════════════════════════════════════════════════════════════════════════

/// Standard header for all SX9 NATS messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatsHeader {
    /// Unique correlation ID for request/response tracking
    pub correlation_id: String,
    
    /// HashRef for lineage tracking (hex encoded, 32 chars)
    /// Format: {sch:16}{heredity:16}
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_ref: Option<String>,
    
    /// Source service name
    pub source: String,
    
    /// Message timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Priority: 0=normal, 1=urgent, 2=critical
    #[serde(default)]
    pub priority: u8,
    
    /// Trace ID for distributed tracing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    
    /// Parent span ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_span_id: Option<String>,
}

impl NatsHeader {
    /// Create new header with generated correlation ID
    pub fn new(source: &str) -> Self {
        Self {
            correlation_id: Uuid::new_v4().to_string(),
            hash_ref: None,
            source: source.to_string(),
            timestamp: Utc::now(),
            priority: 0,
            trace_id: None,
            parent_span_id: None,
        }
    }
    
    /// Create with specific correlation ID
    pub fn with_correlation_id(source: &str, correlation_id: &str) -> Self {
        Self {
            correlation_id: correlation_id.to_string(),
            hash_ref: None,
            source: source.to_string(),
            timestamp: Utc::now(),
            priority: 0,
            trace_id: None,
            parent_span_id: None,
        }
    }
    
    /// Set hash ref for lineage tracking
    pub fn with_hash_ref(mut self, hash_ref: &str) -> Self {
        self.hash_ref = Some(hash_ref.to_string());
        self
    }
    
    /// Set priority
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }
    
    /// Set trace context
    pub fn with_trace(mut self, trace_id: &str, parent_span_id: Option<&str>) -> Self {
        self.trace_id = Some(trace_id.to_string());
        self.parent_span_id = parent_span_id.map(String::from);
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Generic Message Wrapper
// ═══════════════════════════════════════════════════════════════════════════

/// Generic NATS message with header and typed payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NatsMessage<T> {
    pub header: NatsHeader,
    pub payload: T,
}

impl<T> NatsMessage<T> {
    pub fn new(source: &str, payload: T) -> Self {
        Self {
            header: NatsHeader::new(source),
            payload,
        }
    }
    
    pub fn with_header(header: NatsHeader, payload: T) -> Self {
        Self { header, payload }
    }
}

impl<T: Serialize> NatsMessage<T> {
    /// Serialize to JSON bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }
}

impl<T: for<'de> Deserialize<'de>> NatsMessage<T> {
    /// Deserialize from JSON bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Kali Messages
// ═══════════════════════════════════════════════════════════════════════════

/// Execution environment
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExecEnv {
    OrbStack,
    Firefly,
    Native,
    Container,
    Wasm,
}

impl Default for ExecEnv {
    fn default() -> Self {
        Self::OrbStack
    }
}

/// Kali tool execution request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaliExecRequest {
    /// Tool Unicode rune (e.g., "E000" for nmap)
    pub tool_rune: String,
    
    /// Command arguments
    pub args: Vec<String>,
    
    /// Target (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    
    /// Timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_secs: u32,
    
    /// Execution environment
    #[serde(default)]
    pub exec_env: ExecEnv,
    
    /// Working directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    
    /// Environment variables
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env_vars: Vec<(String, String)>,
}

fn default_timeout() -> u32 { 300 }

/// Kali execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KaliExecResult {
    /// Exit code
    pub exit_code: i32,
    
    /// Stdout (may be truncated)
    pub stdout: String,
    
    /// Stderr
    pub stderr: String,
    
    /// Duration in milliseconds
    pub duration_ms: u64,
    
    /// Response rune (U+E7E0 = success, U+E7E1 = failure)
    pub response_rune: String,
    
    /// Truncated flag
    #[serde(default)]
    pub truncated: bool,
}

impl KaliExecResult {
    pub fn success(stdout: String, stderr: String, duration_ms: u64) -> Self {
        Self {
            exit_code: 0,
            stdout,
            stderr,
            duration_ms,
            response_rune: "E7E0".to_string(), // Success
            truncated: false,
        }
    }
    
    pub fn failure(exit_code: i32, stdout: String, stderr: String, duration_ms: u64) -> Self {
        Self {
            exit_code,
            stdout,
            stderr,
            duration_ms,
            response_rune: "E7E1".to_string(), // Failure
            truncated: false,
        }
    }
    
    pub fn timeout(duration_ms: u64) -> Self {
        Self {
            exit_code: -1,
            stdout: String::new(),
            stderr: "Execution timed out".to_string(),
            duration_ms,
            response_rune: "E7E2".to_string(), // Timeout
            truncated: false,
        }
    }
}

/// Tool chain request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolChainRequest {
    /// Chain ID
    pub chain_id: String,
    
    /// Tool runes in execution order
    pub tools: Vec<String>,
    
    /// Arguments per tool (parallel array)
    pub args: Vec<Vec<String>>,
    
    /// Initial target
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    
    /// Chain timeout in seconds
    #[serde(default = "default_chain_timeout")]
    pub timeout_secs: u32,
}

fn default_chain_timeout() -> u32 { 1800 } // 30 minutes

// ═══════════════════════════════════════════════════════════════════════════
// Hash Messages
// ═══════════════════════════════════════════════════════════════════════════

/// Hash computation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashComputeRequest {
    /// Content to hash
    pub content: String,
    
    /// Hash type
    pub hash_type: HashType,
    
    /// Context for CUID generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<HashContext>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HashType {
    Trivariate,
    Sch,
    Cuid,
    HashRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashContext {
    pub exec_env: Option<String>,
    pub agent_id: Option<String>,
    pub delta_angle: Option<f32>,
    pub state: Option<String>,
}

/// Hash computation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashComputeResult {
    /// Full trivariate (Base96, 48 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trivariate: Option<String>,
    
    /// HashRef (hex, 32 chars)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_ref: Option<String>,
    
    /// Individual components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Health Messages
// ═══════════════════════════════════════════════════════════════════════════

/// Heartbeat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    /// Service name
    pub service: String,
    
    /// Health status
    pub status: HealthStatus,
    
    /// Uptime in seconds
    pub uptime_secs: u64,
    
    /// Version
    pub version: String,
    
    /// Optional metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_creation() {
        let header = NatsHeader::new("test-service");
        assert!(!header.correlation_id.is_empty());
        assert_eq!(header.source, "test-service");
        assert_eq!(header.priority, 0);
    }

    #[test]
    fn test_message_serialization() {
        let msg = NatsMessage::new("test", KaliExecRequest {
            tool_rune: "E000".to_string(),
            args: vec!["-sV".to_string(), "192.168.1.1".to_string()],
            target: Some("192.168.1.1".to_string()),
            timeout_secs: 300,
            exec_env: ExecEnv::OrbStack,
            working_dir: None,
            env_vars: vec![],
        });
        
        let bytes = msg.to_bytes().unwrap();
        let decoded: NatsMessage<KaliExecRequest> = NatsMessage::from_bytes(&bytes).unwrap();
        
        assert_eq!(decoded.payload.tool_rune, "E000");
    }

    #[test]
    fn test_kali_result_status() {
        let success = KaliExecResult::success("output".into(), "".into(), 100);
        assert_eq!(success.response_rune, "E7E0");
        
        let failure = KaliExecResult::failure(1, "".into(), "error".into(), 100);
        assert_eq!(failure.response_rune, "E7E1");
    }
}
