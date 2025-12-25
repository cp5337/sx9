//! Initializer Agent
//!
//! Creates Linear issues from specifications and requirements.
//! First agent in the pipeline - transforms ideas into actionable tasks.

use anyhow::Result;
use tracing::{debug, info};

use super::{AgentAction, AgentTask};
use crate::linear::Client as LinearClient;
use crate::mcp::SerenaClient;

/// Initializer agent - creates issues from specs
pub struct InitializerAgent<'a> {
    linear: &'a LinearClient,
    serena: &'a SerenaClient,
}

impl<'a> InitializerAgent<'a> {
    /// Create new initializer agent
    pub fn new(linear: &'a LinearClient, serena: &'a SerenaClient) -> Self {
        Self { linear, serena }
    }

    /// Run the initializer agent
    pub async fn run(&self, task: &AgentTask) -> Result<AgentAction> {
        info!("Initializer processing: {}", task.identifier);

        // Analyze the specification
        let spec = self.analyze_spec(&task.description).await?;

        // Break down into sub-tasks if needed
        let sub_tasks = self.decompose_task(&spec).await?;

        if sub_tasks.is_empty() {
            // Single task - just update and pass through
            debug!("Task {} is atomic, no decomposition needed", task.identifier);
            return Ok(AgentAction::IssueCreated {
                issue_id: task.issue_id.clone(),
                identifier: task.identifier.clone(),
            });
        }

        // Create sub-issues in Linear
        for sub_task in &sub_tasks {
            let issue = self.linear.create_issue(
                &sub_task.title,
                &sub_task.description,
                Some(&task.issue_id),
            ).await?;

            info!("Created sub-issue: {}", issue.identifier);
        }

        Ok(AgentAction::IssueCreated {
            issue_id: task.issue_id.clone(),
            identifier: task.identifier.clone(),
        })
    }

    /// Analyze specification to understand requirements
    async fn analyze_spec(&self, description: &str) -> Result<TaskSpec> {
        debug!("Analyzing specification");

        // Use Serena to analyze the spec
        let analysis = self.serena.analyze_code(description).await?;

        Ok(TaskSpec {
            summary: description.lines().next().unwrap_or("").to_string(),
            requirements: analysis.suggestions,
            complexity: analysis.quality_score,
            estimated_files: Vec::new(),
        })
    }

    /// Decompose task into sub-tasks if complex
    async fn decompose_task(&self, spec: &TaskSpec) -> Result<Vec<SubTask>> {
        // If complexity is low, no decomposition needed
        if spec.complexity < 0.5 {
            return Ok(Vec::new());
        }

        debug!("Decomposing complex task (complexity: {})", spec.complexity);

        // Create sub-tasks based on requirements
        let sub_tasks: Vec<SubTask> = spec
            .requirements
            .iter()
            .enumerate()
            .map(|(i, req)| SubTask {
                title: format!("Sub-task {}: {}", i + 1, truncate(req, 50)),
                description: req.clone(),
                order: i as u32,
            })
            .collect();

        Ok(sub_tasks)
    }
}

/// Task specification from analysis
#[derive(Debug)]
struct TaskSpec {
    summary: String,
    requirements: Vec<String>,
    complexity: f32,
    estimated_files: Vec<String>,
}

/// Sub-task for decomposition
#[derive(Debug)]
struct SubTask {
    title: String,
    description: String,
    order: u32,
}

/// Truncate string with ellipsis
fn truncate(s: &str, max_len: usize) -> String {
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
    fn test_truncate() {
        assert_eq!(truncate("short", 10), "short");
        assert_eq!(truncate("this is a long string", 10), "this is...");
    }
}
