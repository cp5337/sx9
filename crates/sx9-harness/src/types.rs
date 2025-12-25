//! sx9-harness/src/types.rs
//! Recovered from conversations - TypeScript → Rust mapping reference

use serde::{Deserialize, Serialize};
use sx9_foundation_core::data::{Uuid, DateTime, Utc};

/// Harness execution modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarnessMode {
    /// Fully autonomous execution
    Autonomous,
    /// Research and analysis mode
    Research,
    /// Code generation and building
    Build,
    /// Security analysis and testing
    Security,
    /// Planning and design mode
    Planning,
}

/// Agent personas
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Persona {
    /// Factory code generation
    Forge,
    /// Mathematical analysis
    Axiom,
    /// Strategic planning
    Vector,
    /// Security operations
    Sentinel,
    /// Quality assurance
    Guardian,
}

/// Model selection for inference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Model {
    Sonnet,
    Opus,
    Haiku,
    Custom(String),
}

/// Inference parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceParams {
    pub model: Model,
    pub temperature: f32,
    pub max_tokens: u32,
    pub top_p: f32,
}

impl Default for InferenceParams {
    fn default() -> Self {
        Self {
            model: Model::Sonnet,
            temperature: 0.0,
            max_tokens: 8192,
            top_p: 1.0,
        }
    }
}

/// Context source configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSources {
    pub memory: bool,
    pub linear: bool,
    pub drive: bool,
    pub filesystem: bool,
    pub web: bool,
}

impl Default for ContextSources {
    fn default() -> Self {
        Self {
            memory: true,
            linear: true,
            drive: false,
            filesystem: true,
            web: false,
        }
    }
}

/// Mission definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub harness: HarnessMode,
    pub persona: Persona,
    pub inference: InferenceParams,
    pub context_sources: ContextSources,
    pub linear_issue_id: Option<String>,
    pub slack_channel: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionState {
    Pending,
    Running,
    Paused,
    Completed,
    Failed(String),
}

/// Session tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub mission_id: Uuid,
    pub state: ExecutionState,
    pub messages: Vec<Message>,
    pub artifacts: Vec<Artifact>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

/// Message in session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
    Tool,
}

/// Artifact produced during execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: Uuid,
    pub name: String,
    pub artifact_type: ArtifactType,
    pub path: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ArtifactType {
    Code,
    Document,
    Data,
    Image,
}

/// QA Report structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QaReport {
    pub crate_name: String,
    pub grade: Grade,
    pub score: u8,
    pub dimensions: Dimensions,
    pub refactor_directives: Vec<RefactorDirective>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Grade {
    A,
    B,
    C,
    D,
    F,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    pub structure: DimensionScore,
    pub complexity: DimensionScore,
    pub pattern: DimensionScore,
    pub architecture: DimensionScore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionScore {
    pub score: u8,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactorDirective {
    pub file: String,
    pub line: u32,
    pub issue: String,
    pub directive: String,
}

/// Redux-style action for state management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    StartMission(Mission),
    PauseMission(Uuid),
    ResumeMission(Uuid),
    CompleteMission(Uuid),
    FailMission(Uuid, String),
    AddMessage(Uuid, Message),
    AddArtifact(Uuid, Artifact),
    UpdateQa(Uuid, QaReport),
    SetLinearIssue(Uuid, String),
    SendSlackNotification(Uuid, String),
}

/// Application state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppState {
    pub missions: std::collections::HashMap<Uuid, Mission>,
    pub sessions: std::collections::HashMap<Uuid, Session>,
    pub active_mission: Option<Uuid>,
    pub qa_reports: std::collections::HashMap<Uuid, QaReport>,
}

// ========================================================================
// Gate Types (required by gates/*)
// ========================================================================

/// Executor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutorConfig {
    pub harness_mode: HarnessMode,
    pub persona: Persona,
    pub inference: InferenceParams,
    pub timeout_secs: u64,
    pub max_retries: u32,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            harness_mode: HarnessMode::Build,
            persona: Persona::Forge,
            inference: InferenceParams::default(),
            timeout_secs: 300,
            max_retries: 3,
        }
    }
}

/// Execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub output: Option<String>,
    pub error: Option<String>,
    pub duration_ms: u64,
}

/// Static analysis finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub severity: Severity,
    pub message: String,
    pub file: String,
    pub line: u32,
    pub column: Option<u32>,
    pub rule: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Hint,
}

/// Static analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticReport {
    pub schema_version: String,
    pub loadset_id: String,
    pub structure_score: u32,
    pub complexity_score: u32,
    pub findings: Vec<Finding>,
}

/// Architecture violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub rule: String,
    pub description: String,
    pub file: String,
    pub severity: Severity,
}

/// Architecture analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
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
