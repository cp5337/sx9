//! Linear Gateway Types
//!
//! Types for Linear API integration following RFC-9030.
//!
//! CLSGS Annex A.3 compliant: Linear tasks map to behavioral regions via
//! intent anchors. QA signals (static and semantic) attach to task state.

use serde::{Deserialize, Serialize};
use sx9_foundation_core::data::{serde_json, DateTime, Utc, Uuid};

use crate::agents::{GovernanceGate, QaStatus};

// ============================================================================
// CLSGS ANNEX A.3: LINEAR INTENT ANCHORS
// ============================================================================

/// Intent anchor mapping a Linear task to behavioral regions in code.
///
/// Per CLSGS Annex A.3.1: Each Linear task represents a unit of declared intent
/// that maps to one or more behavioral regions and corresponding N-V-N-N annotations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentAnchor {
    /// N-V-N-N annotation pattern this intent maps to
    pub nvnn_pattern: String,

    /// File paths containing the behavioral region
    pub file_paths: Vec<String>,

    /// Specific code regions (file:line_start-line_end)
    pub code_regions: Vec<String>,

    /// Bounded agentic scope (which agent owns this region)
    pub agent_scope: Option<String>,

    /// Intent confidence score (0.0-1.0)
    pub confidence: f32,
}

/// QA signal attachment to a Linear task.
///
/// Per CLSGS Annex A.3.2: QA outcomes map to task validity and risk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearQaSignal {
    /// Static QA passed (required for task progression)
    pub static_passed: bool,

    /// Semantic QA status with drift signals
    pub qa_status: Option<QaStatus>,

    /// Governance gate blocking task (if any)
    pub blocking_gate: Option<GovernanceGate>,

    /// Human-readable QA summary
    pub summary: String,

    /// Last QA check timestamp
    pub checked_at: DateTime<Utc>,
}

/// Linear issue state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LinearState {
    Backlog,
    Todo,
    InProgress,
    Done,
    Canceled,
}

impl LinearState {
    pub fn as_str(&self) -> &'static str {
        match self {
            LinearState::Backlog => "backlog",
            LinearState::Todo => "todo",
            LinearState::InProgress => "in_progress",
            LinearState::Done => "done",
            LinearState::Canceled => "canceled",
        }
    }
}

/// Linear issue priority (1=urgent, 4=low)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinearPriority {
    None = 0,
    Urgent = 1,
    High = 2,
    Medium = 3,
    Low = 4,
}

impl From<i32> for LinearPriority {
    fn from(val: i32) -> Self {
        match val {
            1 => LinearPriority::Urgent,
            2 => LinearPriority::High,
            3 => LinearPriority::Medium,
            4 => LinearPriority::Low,
            _ => LinearPriority::None,
        }
    }
}

/// Linear issue representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearIssue {
    /// Linear internal ID
    pub id: String,

    /// Human-readable identifier (e.g., "SX9-123")
    pub identifier: String,

    /// Issue title
    pub title: String,

    /// Issue description (markdown)
    pub description: Option<String>,

    /// Current state
    pub state: LinearState,

    /// Priority level
    pub priority: LinearPriority,

    /// Team ID
    pub team_id: String,

    /// Project ID (optional)
    pub project_id: Option<String>,

    /// Cycle ID (sprint)
    pub cycle_id: Option<String>,

    /// Labels
    pub label_ids: Vec<String>,

    /// Assignee user ID
    pub assignee_id: Option<String>,

    /// Creator user ID
    pub creator_id: String,

    /// URL to issue
    pub url: String,

    /// Created timestamp
    pub created_at: DateTime<Utc>,

    /// Updated timestamp
    pub updated_at: DateTime<Utc>,

    // ========================================================================
    // CLSGS ANNEX A.3 EXTENSIONS
    // ========================================================================

    /// Intent anchors mapping task to behavioral regions (A.3.1)
    #[serde(default)]
    pub intent_anchors: Vec<IntentAnchor>,

    /// QA signal attachment (A.3.2)
    #[serde(default)]
    pub qa_signal: Option<LinearQaSignal>,

    /// Whether this task can progress (false if Static QA failed)
    #[serde(default = "default_can_progress")]
    pub can_progress: bool,
}

fn default_can_progress() -> bool {
    true
}

impl LinearIssue {
    /// Check if static QA allows task progression (CLSGS A.3.2)
    pub fn static_qa_passed(&self) -> bool {
        self.qa_signal
            .as_ref()
            .map(|s| s.static_passed)
            .unwrap_or(true)
    }

    /// Check if any governance gate is blocking (CLSGS A.3.2)
    pub fn is_gate_blocked(&self) -> bool {
        self.qa_signal
            .as_ref()
            .and_then(|s| s.blocking_gate)
            .is_some()
    }

    /// Get the highest severity drift signal score
    pub fn max_drift_score(&self) -> f32 {
        self.qa_signal
            .as_ref()
            .and_then(|s| s.qa_status.as_ref())
            .map(|qa| {
                qa.drift_signals
                    .iter()
                    .map(|d| d.score)
                    .fold(0.0_f32, f32::max)
            })
            .unwrap_or(0.0)
    }

    /// Attach an intent anchor to this issue
    pub fn add_intent_anchor(&mut self, anchor: IntentAnchor) {
        self.intent_anchors.push(anchor);
    }

    /// Set QA signal (blocks progression if static_passed=false)
    pub fn set_qa_signal(&mut self, signal: LinearQaSignal) {
        self.can_progress = signal.static_passed && signal.blocking_gate.is_none();
        self.qa_signal = Some(signal);
    }
}

/// Linear comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearComment {
    pub id: String,
    pub issue_id: String,
    pub user_id: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

/// Linear webhook payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearWebhook {
    /// Action type: create, update, remove
    pub action: String,

    /// Actor (user who triggered)
    pub actor_id: Option<String>,

    /// Webhook type: Issue, Comment, Project, etc.
    #[serde(rename = "type")]
    pub webhook_type: String,

    /// Timestamp
    pub created_at: DateTime<Utc>,

    /// Issue data (if issue webhook)
    pub data: serde_json::Value,

    /// Organization ID
    pub organization_id: String,

    /// Webhook ID for deduplication
    pub webhook_id: String,
}

impl LinearWebhook {
    /// Extract issue from webhook data
    pub fn to_issue(&self) -> Option<LinearIssue> {
        if self.webhook_type == "Issue" {
            serde_json::from_value(self.data.clone()).ok()
        } else {
            None
        }
    }

    /// Extract comment from webhook data
    pub fn to_comment(&self) -> Option<LinearComment> {
        if self.webhook_type == "Comment" {
            serde_json::from_value(self.data.clone()).ok()
        } else {
            None
        }
    }
}

/// Linear webhook event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebhookEventType {
    IssueCreated,
    IssueUpdated,
    IssueRemoved,
    CommentCreated,
    CommentUpdated,
    Unknown,
}

impl WebhookEventType {
    pub fn from_webhook(webhook: &LinearWebhook) -> Self {
        match (webhook.webhook_type.as_str(), webhook.action.as_str()) {
            ("Issue", "create") => WebhookEventType::IssueCreated,
            ("Issue", "update") => WebhookEventType::IssueUpdated,
            ("Issue", "remove") => WebhookEventType::IssueRemoved,
            ("Comment", "create") => WebhookEventType::CommentCreated,
            ("Comment", "update") => WebhookEventType::CommentUpdated,
            _ => WebhookEventType::Unknown,
        }
    }
}

/// Agent task derived from Linear issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearAgentTask {
    /// Task ID
    pub task_id: Uuid,

    /// Source Linear issue
    pub issue_id: String,

    /// Issue identifier
    pub issue_identifier: String,

    /// Task title
    pub title: String,

    /// Task description
    pub description: String,

    /// Target agent type
    pub agent_type: String,

    /// Task context
    pub context: LinearTaskContext,

    /// Priority
    pub priority: LinearPriority,

    /// Created at
    pub created_at: DateTime<Utc>,
}

/// Context for Linear-derived task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearTaskContext {
    /// Repository name
    pub repository: Option<String>,

    /// Files mentioned in issue
    pub files_mentioned: Vec<String>,

    /// Branch name (if auto-created)
    pub branch: Option<String>,

    /// Labels from issue
    pub labels: Vec<String>,

    /// Related issues
    pub related_issues: Vec<String>,
}

/// Linear gateway configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearGatewayConfig {
    /// Linear API key
    pub api_key: String,

    /// Team ID
    pub team_id: String,

    /// Workspace ID
    pub workspace_id: String,

    /// Webhook secret for signature verification
    pub webhook_secret: String,

    /// Gateway port (default: 18120)
    pub port: u16,

    /// NATS URL
    pub nats_url: String,
}

impl Default for LinearGatewayConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            team_id: String::new(),
            workspace_id: String::new(),
            webhook_secret: String::new(),
            port: 18120,
            nats_url: "nats://localhost:4222".to_string(),
        }
    }
}

/// Gateway health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayHealth {
    pub status: String,
    pub linear_connected: bool,
    pub nats_connected: bool,
    pub agents_active: usize,
    pub uptime_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_priority_from_i32() {
        assert_eq!(LinearPriority::from(1), LinearPriority::Urgent);
        assert_eq!(LinearPriority::from(2), LinearPriority::High);
        assert_eq!(LinearPriority::from(3), LinearPriority::Medium);
        assert_eq!(LinearPriority::from(4), LinearPriority::Low);
        assert_eq!(LinearPriority::from(0), LinearPriority::None);
        assert_eq!(LinearPriority::from(99), LinearPriority::None);
    }

    #[test]
    fn test_linear_state_as_str() {
        assert_eq!(LinearState::InProgress.as_str(), "in_progress");
        assert_eq!(LinearState::Todo.as_str(), "todo");
    }

    #[test]
    fn test_gateway_config_default() {
        let config = LinearGatewayConfig::default();
        assert_eq!(config.port, 18120);
        assert_eq!(config.nats_url, "nats://localhost:4222");
    }
}
