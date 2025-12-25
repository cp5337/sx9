//! Agent Types
//!
//! Core types for the autonomous agent system.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Agent state in the workflow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentState {
    /// Waiting for work
    Idle,

    /// Processing an issue
    Processing,

    /// Waiting for QA gates
    AwaitingQa,

    /// Creating PR
    CreatingPr,

    /// Handing off to next agent
    HandingOff,

    /// Error state
    Error,
}

/// Agent role in the pipeline
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentRole {
    /// Creates issues from specs
    Initializer,

    /// Implements code changes
    Coder,

    /// Saves state and hands off
    Handoff,

    /// Orchestrates the pipeline
    Orchestrator,
}

impl AgentRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            AgentRole::Initializer => "initializer",
            AgentRole::Coder => "coder",
            AgentRole::Handoff => "handoff",
            AgentRole::Orchestrator => "orchestrator",
        }
    }
}

/// Agent task from Linear
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    /// Linear issue ID
    pub issue_id: String,

    /// Issue identifier (e.g., SX9-123)
    pub identifier: String,

    /// Task title
    pub title: String,

    /// Task description
    pub description: String,

    /// Assigned agent role
    pub role: AgentRole,

    /// Task priority (1=urgent, 4=low)
    pub priority: u8,

    /// Files to modify
    pub target_files: Vec<String>,

    /// Branch name
    pub branch: Option<String>,

    /// Parent task (for sub-tasks)
    pub parent_id: Option<String>,
}

/// Agent action result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentAction {
    /// Code was generated
    CodeGenerated {
        files: Vec<GeneratedFile>,
        tests: Vec<GeneratedFile>,
    },

    /// Issue was created
    IssueCreated {
        issue_id: String,
        identifier: String,
    },

    /// PR was created
    PrCreated {
        pr_url: String,
        pr_number: u32,
    },

    /// Comment was added
    CommentAdded {
        comment_id: String,
    },

    /// State was saved for handoff
    StateSaved {
        handoff_data: String,
    },

    /// QA gates passed
    QaPassed {
        report: QaReport,
    },

    /// QA gates failed
    QaFailed {
        report: QaReport,
        blocking_gates: Vec<String>,
    },

    /// Error occurred
    Error {
        message: String,
        recoverable: bool,
    },
}

/// Generated file from agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    /// File path
    pub path: String,

    /// File content
    pub content: String,

    /// Whether this is a new file
    pub is_new: bool,
}

/// QA gate report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QaReport {
    /// Static analysis passed
    pub static_passed: bool,

    /// Architecture check passed
    pub arch_passed: bool,

    /// Pattern check passed
    pub pattern_passed: bool,

    /// Semantic check passed
    pub semantic_passed: bool,

    /// Overall certification status
    pub certified: bool,

    /// Issues found
    pub issues: Vec<QaIssue>,
}

/// QA issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QaIssue {
    /// Issue severity
    pub severity: QaSeverity,

    /// Issue message
    pub message: String,

    /// Gate that flagged this
    pub gate: String,

    /// File path (if applicable)
    pub file: Option<String>,

    /// Line number (if applicable)
    pub line: Option<u32>,
}

/// QA severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QaSeverity {
    Error,
    Warning,
    Info,
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Poll interval for Linear issues
    pub poll_interval: Duration,

    /// Maximum concurrent tasks
    pub max_concurrent: usize,

    /// Auto-approve PRs below this complexity
    pub auto_approve_threshold: f32,

    /// Enable dry-run mode (no actual changes)
    pub dry_run: bool,

    /// Repository path
    pub repo_path: String,

    /// Default branch
    pub default_branch: String,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(60),
            max_concurrent: 3,
            auto_approve_threshold: 0.3,
            dry_run: false,
            repo_path: ".".to_string(),
            default_branch: "main".to_string(),
        }
    }
}

/// Handoff payload for session continuity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandoffPayload {
    /// Current agent state
    pub state: AgentState,

    /// Task being processed
    pub task: AgentTask,

    /// Actions taken so far
    pub actions: Vec<AgentAction>,

    /// Context for next agent
    pub context: String,

    /// Files modified
    pub modified_files: Vec<String>,

    /// Timestamp
    pub timestamp: String,
}
