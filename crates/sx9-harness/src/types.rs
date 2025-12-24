//! Type definitions for sx9-harness
//!
//! Converted from TypeScript types in forge-unified-v5

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Connection Status Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
    Offline,
    Connecting,
    Ready,
    Querying,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeptoseConnection {
    pub status: ConnectionStatus,
    pub last_query: Option<i64>,
    pub latency_ms: Option<u32>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromaDBConnection {
    pub status: ConnectionStatus,
    pub collections: Vec<String>,
    pub last_query: Option<i64>,
    pub latency_ms: Option<u32>,
    pub error: Option<String>,
}

// ============================================================================
// Query Result Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternSuggestion {
    pub interview_id: String,
    pub pattern: String,
    pub similarity: f32,
    pub voice_narrative: String,
    pub metadata: Option<PatternMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMetadata {
    pub created_at: Option<String>,
    pub forge_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRecommendation {
    pub tool_name: String,
    pub category: String,
    pub entropy: f32,  // TETH entropy score
    pub similarity: f32,
    pub why_relevant: String,
    pub capabilities: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatScenario {
    pub scenario_id: String,
    pub apt_group: String,
    pub techniques: Vec<String>,
    pub tools_used: Vec<String>,
    pub detection_rules: Vec<String>,
    pub description: Option<String>,
}

// ============================================================================
// QA Gate Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub severity: Severity,
    pub score: f32,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StaticReport {
    pub schema_version: String,
    pub loadset_id: String,
    pub structure_score: u32,
    pub complexity_score: u32,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchReport {
    pub schema_version: String,
    pub loadset_id: String,
    pub score: u32,
    pub ecs_layer: Option<String>,
    pub bevy_free: bool,
    pub tcr_compliant: bool,
    pub rune_valid: bool,
    pub slot_valid: bool,
    pub violations: Vec<Violation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub code: String,
    pub severity: String,
    pub file: String,
    pub line: usize,
    pub message: String,
}

// ============================================================================
// Executor Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutorConfig {
    pub max_retries: u32,
    pub timeout_ms: u64,
    pub nats_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: Option<String>,
    pub error: Option<String>,
    pub duration_ms: u64,
}
