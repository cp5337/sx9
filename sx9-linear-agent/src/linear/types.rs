//! Linear API Types

use serde::{Deserialize, Serialize};

/// Linear issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    /// Issue ID
    pub id: String,

    /// Human-readable identifier (e.g., SX9-123)
    pub identifier: String,

    /// Issue title
    pub title: String,

    /// Issue description (markdown)
    pub description: Option<String>,

    /// Current state
    pub state: Option<IssueState>,

    /// Priority (1=urgent, 4=low)
    pub priority: Option<u8>,

    /// Team ID
    pub team_id: String,

    /// Assignee ID
    pub assignee_id: Option<String>,

    /// URL
    pub url: String,
}

/// Issue state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueState {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub state_type: String,
}

/// Linear comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub body: String,
    pub user_id: Option<String>,
    pub created_at: String,
}

/// Issue create input
#[derive(Debug, Clone, Serialize)]
pub struct IssueCreateInput {
    pub title: String,
    pub description: Option<String>,
    pub team_id: String,
    pub parent_id: Option<String>,
    pub priority: Option<u8>,
}

/// Issue update input
#[derive(Debug, Clone, Serialize)]
pub struct IssueUpdateInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub state_id: Option<String>,
    pub priority: Option<u8>,
}
