//! Agent Types for Team Member Onboarding
//!
//! Agents are registered as team members in Linear and Slack so they
//! appear in @ mention autocomplete lists.
//!
//! CLSGS Annex A.2 compliant: Agents declare behavioral scope via N-V-N-N.

use serde::{Deserialize, Serialize};
use sx9_foundation_core::data::{DateTime, Utc, Uuid};

// ============================================================================
// CLSGS ANNEX A.2: BEHAVIORAL SCOPE (N-V-N-N)
// ============================================================================

/// Behavioral scope declaration following N-V-N-N pattern.
///
/// Per CLSGS Annex A.2: Agents declare a single dominant ROLE,
/// bounded ACTION scope, explicit CONSTRAINT ownership, and stable OBJECT domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralScope {
    /// Noun: The role this agent fulfills (e.g., "CodeGenerator", "SecurityAuditor")
    pub role: String,

    /// Verb: The bounded action scope (e.g., "generate", "review", "analyze")
    pub action: String,

    /// Noun: The constraint ownership (e.g., "rust_crate", "api_endpoint")
    pub constraint: String,

    /// Noun: The object domain (e.g., "source_code", "configuration")
    pub object: String,
}

impl BehavioralScope {
    /// Format as N-V-N-N annotation string
    pub fn as_annotation(&self) -> String {
        format!(
            "// {}_{}_{}_{}",
            self.role.to_uppercase(),
            self.action.to_uppercase(),
            self.constraint.to_uppercase(),
            self.object.to_uppercase()
        )
    }

    /// Validate scope declaration is non-empty
    pub fn is_valid(&self) -> bool {
        !self.role.is_empty()
            && !self.action.is_empty()
            && !self.constraint.is_empty()
            && !self.object.is_empty()
    }
}

/// Drift signal from Semantic QA (CLSGS Annex A.3)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftSignal {
    /// Drift vector type
    pub vector: DriftVector,

    /// Normalized score (0.0 = no drift, 0.8+ = governance concern)
    pub score: f32,

    /// Delta angle in degrees (0-180)
    pub delta_angle: f32,

    /// Human-readable explanation
    pub explanation: String,

    /// Timestamp of detection
    pub detected_at: DateTime<Utc>,
}

/// Drift vector types per RFC-9142
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftVector {
    /// Component assumes undeclared responsibilities
    Role,
    /// Stated limits erode or disappear
    Constraint,
    /// Dependencies exceed expectations
    Coupling,
    /// Decision-making migrates unexpectedly
    Authority,
    /// N-V-N-N patterns structurally present but behaviorally violated
    Pattern,
}

/// QA status attached to task/issue (CLSGS Annex A.3.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QaStatus {
    /// Static QA passed (required for progress)
    pub static_passed: bool,

    /// Semantic drift signals (advisory)
    pub drift_signals: Vec<DriftSignal>,

    /// Governance gate level
    pub gate_level: GovernanceGate,

    /// Last QA run timestamp
    pub last_checked: DateTime<Utc>,
}

/// Governance gate levels per RFC-9142 Section 7.1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GovernanceGate {
    /// Log only
    Observe,
    /// Annotate PR/Issue
    Warn,
    /// Require human acknowledgment
    Gate,
    /// Block release pending review
    Escalate,
}

impl Default for GovernanceGate {
    fn default() -> Self {
        GovernanceGate::Observe
    }
}

// ============================================================================
// EXISTING TYPES (EXTENDED)
// ============================================================================

/// AI Provider backend for agent routing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AiProvider {
    Claude,
    Gpt,
    Gemini,
    Grok,
    Cursor,
    Wolfram,
    /// Local/self-hosted model
    Local,
}

impl AiProvider {
    /// Get the NATS subject prefix for this provider
    pub fn nats_subject(&self) -> &'static str {
        match self {
            AiProvider::Claude => "sx9.agent.claude",
            AiProvider::Gpt => "sx9.agent.gpt",
            AiProvider::Gemini => "sx9.agent.gemini",
            AiProvider::Grok => "sx9.agent.grok",
            AiProvider::Cursor => "sx9.agent.cursor",
            AiProvider::Wolfram => "sx9.agent.wolfram",
            AiProvider::Local => "sx9.agent.local",
        }
    }

    /// Environment variable for API key
    pub fn api_key_env(&self) -> &'static str {
        match self {
            AiProvider::Claude => "ANTHROPIC_API_KEY",
            AiProvider::Gpt => "OPENAI_API_KEY",
            AiProvider::Gemini => "GOOGLE_API_KEY",
            AiProvider::Grok => "XAI_API_KEY",
            AiProvider::Cursor => "CURSOR_API_KEY",
            AiProvider::Wolfram => "WOLFRAM_API_KEY",
            AiProvider::Local => "LOCAL_MODEL_ENDPOINT",
        }
    }
}

/// Agent status for availability tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentStatus {
    /// Ready to accept tasks
    Available,
    /// Currently processing a task
    Busy,
    /// Agent is offline/unreachable
    Offline,
    /// Agent encountered an error
    Error,
}

/// Agent capability categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentCapability {
    /// Code generation and implementation
    CodeGeneration,
    /// Code review and analysis
    CodeReview,
    /// Architecture and design
    Architecture,
    /// Security analysis
    Security,
    /// Research and information gathering
    Research,
    /// Documentation writing
    Documentation,
    /// Mathematical/analytical tasks
    Analysis,
    /// Strategic planning
    Planning,
    /// Infrastructure/DevOps
    Infrastructure,
    /// Custom capability
    Custom(String),
}

/// An AI Agent registered as a team member
///
/// This struct contains all information needed to:
/// - Show agent in Linear team member dropdown
/// - Show agent in Slack @ mention autocomplete
/// - Route tasks via NATS to the correct AI provider
///
/// CLSGS Annex A.2: Each agent declares a behavioral scope (N-V-N-N).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Internal unique identifier
    pub id: Uuid,

    /// Display name (e.g., "Forge", "Axiom", "Claude")
    pub name: String,

    /// Handle for @ mentions (e.g., "@forge", "@claude")
    pub handle: String,

    /// Description shown in team member lists
    pub description: String,

    /// Avatar URL for display in UI
    pub avatar_url: Option<String>,

    /// AI provider backend
    pub provider: AiProvider,

    /// Specific model to use (e.g., "claude-3-5-sonnet", "gpt-4")
    pub model: String,

    /// Agent capabilities for task routing
    pub capabilities: Vec<AgentCapability>,

    /// Keywords that trigger this agent (for auto-assignment)
    pub trigger_keywords: Vec<String>,

    /// Behavioral scope declaration (N-V-N-N) per CLSGS Annex A.2
    pub behavioral_scope: Option<BehavioralScope>,

    /// Linear integration
    pub linear: Option<LinearIntegration>,

    /// Slack integration
    pub slack: Option<SlackIntegration>,

    /// Current status
    pub status: AgentStatus,

    /// When agent was registered
    pub registered_at: DateTime<Utc>,

    /// Last heartbeat/activity
    pub last_seen: Option<DateTime<Utc>>,
}

/// Linear team member integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearIntegration {
    /// Linear user ID for this agent
    pub user_id: String,

    /// Linear team ID
    pub team_id: String,

    /// Can be assigned issues
    pub can_be_assigned: bool,

    /// Auto-assign based on labels
    pub auto_assign_labels: Vec<String>,
}

/// Slack bot user integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackIntegration {
    /// Slack bot user ID (e.g., "U12345678")
    pub bot_user_id: String,

    /// Slack app ID
    pub app_id: String,

    /// Channels the agent monitors
    pub channels: Vec<String>,

    /// Can respond to DMs
    pub allow_dm: bool,
}

/// Agent registration request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRegistration {
    pub name: String,
    pub handle: String,
    pub description: String,
    pub provider: AiProvider,
    pub model: String,
    pub capabilities: Vec<AgentCapability>,
    pub trigger_keywords: Vec<String>,
}

/// Agent heartbeat for status tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentHeartbeat {
    pub agent_id: Uuid,
    pub status: AgentStatus,
    pub current_task: Option<String>,
    pub metrics: AgentMetrics,
    pub timestamp: DateTime<Utc>,
}

/// Agent performance metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentMetrics {
    /// Tasks completed in last hour
    pub tasks_completed_hour: u32,
    /// Average response time in ms
    pub avg_response_ms: u64,
    /// Success rate (0-100)
    pub success_rate: u8,
    /// Current queue depth
    pub queue_depth: u32,
}

/// Task assignment to an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    pub task_id: Uuid,
    pub agent_id: Uuid,
    pub source: TaskSource,
    pub content: String,
    pub context: TaskContext,
    pub priority: TaskPriority,
    pub assigned_at: DateTime<Utc>,
}

/// Where the task originated
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskSource {
    /// @ mention in Slack
    SlackMention {
        channel: String,
        thread_ts: Option<String>,
        user_id: String,
    },
    /// Linear issue assignment
    LinearIssue {
        issue_id: String,
        project_id: Option<String>,
    },
    /// Direct NATS request
    NatsRequest {
        reply_subject: String,
    },
    /// Internal system request
    System {
        component: String,
    },
}

/// Additional context for task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskContext {
    /// Files to consider
    pub files: Vec<String>,
    /// Previous messages in thread
    pub thread_history: Vec<String>,
    /// Related Linear issues
    pub related_issues: Vec<String>,
    /// Memory snapshot ID
    pub memory_snapshot: Option<String>,
}

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Urgent,
}

impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::Normal
    }
}

/// Task completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResponse {
    pub task_id: Uuid,
    pub agent_id: Uuid,
    pub success: bool,
    pub response: String,
    pub artifacts: Vec<TaskArtifact>,
    pub duration_ms: u64,
    pub completed_at: DateTime<Utc>,
}

/// Artifact produced by task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskArtifact {
    pub name: String,
    pub artifact_type: String,
    pub path: Option<String>,
    pub content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_nats_subject() {
        assert_eq!(AiProvider::Claude.nats_subject(), "sx9.agent.claude");
        assert_eq!(AiProvider::Gpt.nats_subject(), "sx9.agent.gpt");
        assert_eq!(AiProvider::Gemini.nats_subject(), "sx9.agent.gemini");
    }

    #[test]
    fn test_provider_api_key_env() {
        assert_eq!(AiProvider::Claude.api_key_env(), "ANTHROPIC_API_KEY");
        assert_eq!(AiProvider::Gpt.api_key_env(), "OPENAI_API_KEY");
    }

    #[test]
    fn test_task_priority_default() {
        assert_eq!(TaskPriority::default(), TaskPriority::Normal);
    }
}
