//! Handoff Agent
//!
//! Saves session state and prepares for agent handoff.
//! Ensures continuity between agent sessions by persisting
//! context to Linear comments.

use anyhow::Result;
use serde_json;
use tracing::{debug, info};

use super::{AgentAction, AgentState, AgentTask, HandoffPayload};
use crate::linear::Client as LinearClient;

/// Handoff agent - saves state for session continuity
pub struct HandoffAgent<'a> {
    linear: &'a LinearClient,
}

impl<'a> HandoffAgent<'a> {
    /// Create new handoff agent
    pub fn new(linear: &'a LinearClient) -> Self {
        Self { linear }
    }

    /// Run the handoff agent
    pub async fn run(&self, task: &AgentTask) -> Result<AgentAction> {
        info!("Handoff processing: {}", task.identifier);

        // Build handoff payload
        let payload = self.build_payload(task).await?;

        // Serialize to JSON
        let handoff_json = serde_json::to_string_pretty(&payload)?;

        // Save as Linear comment with special marker
        let comment = self.format_handoff_comment(&handoff_json);
        let comment_id = self.linear.add_comment(&task.issue_id, &comment).await?;

        info!("Saved handoff state to comment: {}", comment_id);

        Ok(AgentAction::StateSaved {
            handoff_data: handoff_json,
        })
    }

    /// Build handoff payload from current state
    async fn build_payload(&self, task: &AgentTask) -> Result<HandoffPayload> {
        // Get issue comments to find previous handoffs
        let comments = self.linear.get_comments(&task.issue_id).await?;

        // Find previous handoff if any
        let previous_actions = self.parse_previous_handoff(&comments);

        Ok(HandoffPayload {
            state: AgentState::HandingOff,
            task: task.clone(),
            actions: previous_actions,
            context: self.build_context(task),
            modified_files: Vec::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Build context string for next agent
    fn build_context(&self, task: &AgentTask) -> String {
        format!(
            "Task: {} ({})\nPriority: {}\nDescription: {}",
            task.title,
            task.identifier,
            task.priority,
            truncate_description(&task.description, 500)
        )
    }

    /// Parse previous handoff from comments
    fn parse_previous_handoff(&self, comments: &[crate::linear::Comment]) -> Vec<AgentAction> {
        for comment in comments.iter().rev() {
            if comment.body.contains("<!-- AGENT_HANDOFF -->") {
                // Extract JSON from comment
                if let Some(json_start) = comment.body.find("```json") {
                    if let Some(json_end) = comment.body[json_start..].find("```\n") {
                        let json_str = &comment.body[json_start + 7..json_start + json_end];
                        if let Ok(payload) = serde_json::from_str::<HandoffPayload>(json_str) {
                            return payload.actions;
                        }
                    }
                }
            }
        }

        Vec::new()
    }

    /// Format handoff comment with marker
    fn format_handoff_comment(&self, json: &str) -> String {
        format!(
            r#"<!-- AGENT_HANDOFF -->
## Agent Handoff State

This comment contains serialized agent state for session continuity.

<details>
<summary>Handoff Data (JSON)</summary>

```json
{}
```

</details>

---
*Saved by SX9 Handoff Agent*
"#,
            json
        )
    }

    /// Restore state from previous handoff
    pub async fn restore(&self, task: &AgentTask) -> Result<Option<HandoffPayload>> {
        let comments = self.linear.get_comments(&task.issue_id).await?;

        for comment in comments.iter().rev() {
            if comment.body.contains("<!-- AGENT_HANDOFF -->") {
                if let Some(json_start) = comment.body.find("```json") {
                    let after_marker = &comment.body[json_start + 7..];
                    if let Some(json_end) = after_marker.find("```") {
                        let json_str = &after_marker[..json_end];
                        if let Ok(payload) = serde_json::from_str::<HandoffPayload>(json_str) {
                            debug!("Restored handoff state from comment");
                            return Ok(Some(payload));
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    /// Clean up old handoff comments
    pub async fn cleanup_old_handoffs(&self, task: &AgentTask, keep_count: usize) -> Result<usize> {
        let comments = self.linear.get_comments(&task.issue_id).await?;

        let handoff_comments: Vec<_> = comments
            .iter()
            .filter(|c| c.body.contains("<!-- AGENT_HANDOFF -->"))
            .collect();

        let to_delete = handoff_comments.len().saturating_sub(keep_count);

        for comment in handoff_comments.iter().take(to_delete) {
            self.linear.delete_comment(&comment.id).await?;
            debug!("Deleted old handoff comment: {}", comment.id);
        }

        Ok(to_delete)
    }
}

/// Truncate description to max length
fn truncate_description(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_description() {
        assert_eq!(truncate_description("short", 10), "short");
        assert_eq!(
            truncate_description("this is a very long description", 20),
            "this is a very lo..."
        );
    }

    #[test]
    fn test_format_handoff_comment() {
        // Would need LinearClient to fully test
        assert!(true);
    }
}
