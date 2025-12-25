//! Agent Dispatch System
//!
//! Routes Linear issues to appropriate agents based on labels and content.

use std::sync::Arc;

use super::{LinearAgentTask, LinearIssue, LinearTaskContext};
use crate::agents::{Agent, AgentCapability, AgentRegistry};
use sx9_foundation_core::data::{Uuid, Utc};

/// Agent dispatcher for Linear issues
pub struct AgentDispatcher {
    /// Agent registry
    agents: Arc<AgentRegistry>,
}

impl AgentDispatcher {
    /// Create new dispatcher
    pub fn new(agents: Arc<AgentRegistry>) -> Self {
        Self { agents }
    }

    /// Dispatch issue to appropriate agent
    pub fn dispatch_issue(&self, issue: &LinearIssue) -> Result<LinearAgentTask, String> {
        // Select agent based on issue content
        let agent = self.select_agent(issue)?;

        // Build task context
        let context = self.build_context(issue);

        // Create branch name
        let branch = self.generate_branch_name(issue);

        // Create task
        let task = LinearAgentTask {
            task_id: Uuid::new_v4(),
            issue_id: issue.id.clone(),
            issue_identifier: issue.identifier.clone(),
            title: issue.title.clone(),
            description: issue.description.clone().unwrap_or_default(),
            agent_type: agent.name.clone(),
            context: LinearTaskContext {
                repository: Some("sx9".to_string()),
                files_mentioned: context.files,
                branch: Some(branch),
                labels: issue.label_ids.clone(),
                related_issues: context.related,
            },
            priority: issue.priority,
            created_at: Utc::now(),
        };

        Ok(task)
    }

    /// Select agent based on issue labels and content
    fn select_agent(&self, issue: &LinearIssue) -> Result<Agent, String> {
        // First try to match by keyword in content
        let text = format!(
            "{} {}",
            issue.title,
            issue.description.as_deref().unwrap_or("")
        );

        if let Some(agent) = self.agents.find_best_agent(&text) {
            return Ok(agent);
        }

        // Fall back to default agent (Forge)
        self.agents
            .get_by_handle("forge")
            .ok_or_else(|| "No default agent found".to_string())
    }

    /// Match agent by capability
    #[allow(dead_code)]
    fn match_capability(&self, labels: &[String]) -> Option<AgentCapability> {
        for label in labels {
            let label_lower = label.to_lowercase();
            if label_lower.contains("bug") || label_lower.contains("frontend") {
                return Some(AgentCapability::CodeGeneration);
            }
            if label_lower.contains("security") {
                return Some(AgentCapability::Security);
            }
            if label_lower.contains("qa") || label_lower.contains("test") {
                return Some(AgentCapability::CodeReview);
            }
            if label_lower.contains("docs") || label_lower.contains("documentation") {
                return Some(AgentCapability::Documentation);
            }
            if label_lower.contains("infra") || label_lower.contains("devops") {
                return Some(AgentCapability::Infrastructure);
            }
            if label_lower.contains("architecture") || label_lower.contains("design") {
                return Some(AgentCapability::Architecture);
            }
        }
        None
    }

    /// Build task context from issue
    fn build_context(&self, issue: &LinearIssue) -> IssueContext {
        let mut files = Vec::new();
        let mut related = Vec::new();

        // Extract file references from description
        if let Some(desc) = &issue.description {
            // Look for file paths
            for word in desc.split_whitespace() {
                if word.contains('/') && (word.ends_with(".rs") || word.ends_with(".ts") || word.ends_with(".tsx")) {
                    files.push(word.to_string());
                }
                // Look for issue references (SX9-NNN)
                let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '-');
                if cleaned.starts_with("SX9-") {
                    related.push(cleaned.to_string());
                }
            }
        }

        IssueContext { files, related }
    }

    /// Generate branch name from issue
    fn generate_branch_name(&self, issue: &LinearIssue) -> String {
        let slug = issue
            .title
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .trim_matches('-')
            .split("--")
            .collect::<Vec<_>>()
            .join("-");

        // Truncate slug to reasonable length
        let slug = if slug.len() > 40 {
            &slug[..40]
        } else {
            &slug
        };

        format!("feat/{}-{}", issue.identifier, slug)
    }
}

/// Extracted context from issue
struct IssueContext {
    files: Vec<String>,
    related: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear::{LinearPriority, LinearState};
    use sx9_foundation_core::data::Utc;

    fn test_issue() -> LinearIssue {
        LinearIssue {
            id: "issue_123".to_string(),
            identifier: "SX9-123".to_string(),
            title: "Fix plasma dashboard SSE connection".to_string(),
            description: Some("The SSE connection in src/hooks/use-plasma-stream.ts is dropping. Related to SX9-100.".to_string()),
            state: LinearState::Todo,
            priority: LinearPriority::High,
            team_id: "team_123".to_string(),
            project_id: None,
            cycle_id: None,
            label_ids: vec!["bug".to_string(), "frontend".to_string()],
            assignee_id: None,
            creator_id: "user_123".to_string(),
            url: "https://linear.app/sx9/issue/SX9-123".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            // CLSGS Annex A.3 extensions
            intent_anchors: Vec::new(),
            qa_signal: None,
            can_progress: true,
        }
    }

    #[test]
    fn test_dispatch_issue() {
        let agents = Arc::new(AgentRegistry::with_defaults());
        let dispatcher = AgentDispatcher::new(agents);

        let issue = test_issue();
        let result = dispatcher.dispatch_issue(&issue);

        assert!(result.is_ok());
        let task = result.unwrap();
        assert_eq!(task.issue_identifier, "SX9-123");
        assert!(!task.agent_type.is_empty());
    }

    #[test]
    fn test_generate_branch_name() {
        let agents = Arc::new(AgentRegistry::with_defaults());
        let dispatcher = AgentDispatcher::new(agents);

        let issue = test_issue();
        let branch = dispatcher.generate_branch_name(&issue);

        assert!(branch.starts_with("feat/SX9-123-"));
        assert!(branch.contains("plasma"));
    }

    #[test]
    fn test_build_context() {
        let agents = Arc::new(AgentRegistry::with_defaults());
        let dispatcher = AgentDispatcher::new(agents);

        let issue = test_issue();
        let context = dispatcher.build_context(&issue);

        assert!(context.files.iter().any(|f| f.contains("use-plasma-stream.ts")));
        assert!(context.related.contains(&"SX9-100".to_string()));
    }

    #[test]
    fn test_match_capability() {
        let agents = Arc::new(AgentRegistry::with_defaults());
        let dispatcher = AgentDispatcher::new(agents);

        let labels = vec!["bug".to_string(), "frontend".to_string()];
        let cap = dispatcher.match_capability(&labels);
        assert_eq!(cap, Some(AgentCapability::CodeGeneration));

        let labels = vec!["security".to_string()];
        let cap = dispatcher.match_capability(&labels);
        assert_eq!(cap, Some(AgentCapability::Security));

        let labels = vec!["random".to_string()];
        let cap = dispatcher.match_capability(&labels);
        assert_eq!(cap, None);
    }
}
