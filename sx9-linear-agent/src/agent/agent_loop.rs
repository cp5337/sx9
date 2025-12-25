//! Agent Loop
//!
//! Main orchestration loop for autonomous agent processing.
//! Polls Linear for assigned issues and dispatches to appropriate agents.

use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use super::{
    AgentAction, AgentConfig, AgentState, AgentTask, CoderAgent, HandoffAgent,
    HandoffPayload, InitializerAgent, QaReport,
};
use crate::linear::Client as LinearClient;
use crate::mcp::{SerenaClient, SlackMCP};

/// Agent loop orchestrator
pub struct AgentLoop {
    /// Linear client
    linear: LinearClient,

    /// Slack notifications
    slack: SlackMCP,

    /// Serena code generation
    serena: SerenaClient,

    /// Current state
    state: Arc<RwLock<AgentState>>,

    /// Configuration
    config: AgentConfig,

    /// Running flag
    running: Arc<RwLock<bool>>,
}

impl AgentLoop {
    /// Create new agent loop
    pub fn new(linear: LinearClient, slack: SlackMCP, serena: SerenaClient) -> Self {
        Self {
            linear,
            slack,
            serena,
            state: Arc::new(RwLock::new(AgentState::Idle)),
            config: AgentConfig::default(),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Configure the agent loop
    pub fn with_config(mut self, config: AgentConfig) -> Self {
        self.config = config;
        self
    }

    /// Run the agent loop
    pub async fn run(&self) -> Result<()> {
        info!("Starting agent loop");
        *self.running.write().await = true;

        while *self.running.read().await {
            // Poll for assigned issues
            match self.poll_issues().await {
                Ok(tasks) => {
                    for task in tasks {
                        if let Err(e) = self.process_task(task).await {
                            error!("Task processing failed: {}", e);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to poll issues: {}", e);
                }
            }

            // Wait before next poll
            tokio::time::sleep(self.config.poll_interval).await;
        }

        info!("Agent loop stopped");
        Ok(())
    }

    /// Stop the agent loop
    pub async fn stop(&self) {
        *self.running.write().await = false;
    }

    /// Poll Linear for assigned issues
    async fn poll_issues(&self) -> Result<Vec<AgentTask>> {
        debug!("Polling Linear for assigned issues");

        let issues = self.linear.get_assigned_issues().await?;

        let tasks: Vec<AgentTask> = issues
            .into_iter()
            .filter_map(|issue| self.issue_to_task(issue))
            .collect();

        if !tasks.is_empty() {
            info!("Found {} tasks to process", tasks.len());
        }

        Ok(tasks)
    }

    /// Convert Linear issue to agent task
    fn issue_to_task(&self, issue: crate::linear::Issue) -> Option<AgentTask> {
        // Parse task metadata from issue description
        Some(AgentTask {
            issue_id: issue.id.clone(),
            identifier: issue.identifier.clone(),
            title: issue.title.clone(),
            description: issue.description.clone().unwrap_or_default(),
            role: super::AgentRole::Coder, // Default to coder
            priority: issue.priority.unwrap_or(3),
            target_files: Vec::new(),
            branch: Some(self.generate_branch_name(&issue)),
            parent_id: None,
        })
    }

    /// Generate branch name from issue
    fn generate_branch_name(&self, issue: &crate::linear::Issue) -> String {
        let slug: String = issue
            .title
            .chars()
            .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { '-' })
            .take(40)
            .collect();

        format!("feat/{}-{}", issue.identifier, slug.trim_matches('-'))
    }

    /// Process a single task
    async fn process_task(&self, task: AgentTask) -> Result<()> {
        info!("Processing task: {} - {}", task.identifier, task.title);

        // Update state
        *self.state.write().await = AgentState::Processing;

        // Update Linear status
        self.linear.update_status(&task.issue_id, "in_progress").await?;

        // Notify Slack
        self.notify_task_started(&task).await?;

        // Run the appropriate agent
        let result = match task.role {
            super::AgentRole::Initializer => {
                self.run_initializer(&task).await
            }
            super::AgentRole::Coder => {
                self.run_coder(&task).await
            }
            super::AgentRole::Handoff => {
                self.run_handoff(&task).await
            }
            super::AgentRole::Orchestrator => {
                // Orchestrator doesn't process tasks directly
                Ok(AgentAction::Error {
                    message: "Orchestrator cannot process tasks".to_string(),
                    recoverable: true,
                })
            }
        };

        // Handle result
        match result {
            Ok(action) => {
                self.handle_action(&task, action).await?;
            }
            Err(e) => {
                error!("Agent failed: {}", e);
                self.handle_failure(&task, &e.to_string()).await?;
            }
        }

        // Update state
        *self.state.write().await = AgentState::Idle;

        Ok(())
    }

    /// Run initializer agent
    async fn run_initializer(&self, task: &AgentTask) -> Result<AgentAction> {
        let agent = InitializerAgent::new(&self.linear, &self.serena);
        agent.run(task).await
    }

    /// Run coder agent
    async fn run_coder(&self, task: &AgentTask) -> Result<AgentAction> {
        let agent = CoderAgent::new(&self.serena, &self.config);
        let action = agent.run(task).await?;

        // Run QA gates
        if let AgentAction::CodeGenerated { ref files, .. } = action {
            let qa_result = self.run_qa_gates(files).await?;

            if !qa_result.certified {
                return Ok(AgentAction::QaFailed {
                    report: qa_result.clone(),
                    blocking_gates: qa_result
                        .issues
                        .iter()
                        .filter(|i| i.severity == super::QaSeverity::Error)
                        .map(|i| i.gate.clone())
                        .collect(),
                });
            }

            return Ok(AgentAction::QaPassed { report: qa_result });
        }

        Ok(action)
    }

    /// Run handoff agent
    async fn run_handoff(&self, task: &AgentTask) -> Result<AgentAction> {
        let agent = HandoffAgent::new(&self.linear);
        agent.run(task).await
    }

    /// Run QA gates on generated files
    async fn run_qa_gates(&self, files: &[super::GeneratedFile]) -> Result<QaReport> {
        debug!("Running QA gates on {} files", files.len());

        // Placeholder - would integrate with sx9-harness QA gates
        Ok(QaReport {
            static_passed: true,
            arch_passed: true,
            pattern_passed: true,
            semantic_passed: true,
            certified: true,
            issues: Vec::new(),
        })
    }

    /// Handle successful action
    async fn handle_action(&self, task: &AgentTask, action: AgentAction) -> Result<()> {
        match action {
            AgentAction::CodeGenerated { files, tests } => {
                info!(
                    "Generated {} files and {} tests",
                    files.len(),
                    tests.len()
                );

                // Create PR
                if !self.config.dry_run {
                    let pr_url = self.create_pr(task, &files).await?;
                    self.linear.add_comment(&task.issue_id, &format!("PR created: {}", pr_url)).await?;
                }
            }

            AgentAction::IssueCreated { identifier, .. } => {
                info!("Created issue: {}", identifier);
            }

            AgentAction::PrCreated { pr_url, pr_number } => {
                info!("Created PR #{}: {}", pr_number, pr_url);
                self.linear.update_status(&task.issue_id, "done").await?;
            }

            AgentAction::QaPassed { report } => {
                info!("QA passed - certified: {}", report.certified);
                self.notify_qa_result(task, true, &report).await?;
            }

            AgentAction::QaFailed { report, blocking_gates } => {
                warn!("QA failed - blocking gates: {:?}", blocking_gates);
                self.notify_qa_result(task, false, &report).await?;
                self.linear.add_comment(
                    &task.issue_id,
                    &format!("QA failed. Blocking gates: {}", blocking_gates.join(", ")),
                ).await?;
            }

            AgentAction::StateSaved { handoff_data } => {
                info!("State saved for handoff ({} bytes)", handoff_data.len());
            }

            AgentAction::CommentAdded { comment_id } => {
                debug!("Added comment: {}", comment_id);
            }

            AgentAction::Error { message, recoverable } => {
                if recoverable {
                    warn!("Recoverable error: {}", message);
                } else {
                    error!("Unrecoverable error: {}", message);
                }
            }
        }

        Ok(())
    }

    /// Handle task failure
    async fn handle_failure(&self, task: &AgentTask, error: &str) -> Result<()> {
        // Update Linear
        self.linear.add_comment(
            &task.issue_id,
            &format!("Agent error: {}", error),
        ).await?;

        // Notify Slack
        self.slack.send_message(
            "#sx9-agent-alerts",
            &format!("Task {} failed: {}", task.identifier, error),
        ).await?;

        Ok(())
    }

    /// Notify task started
    async fn notify_task_started(&self, task: &AgentTask) -> Result<()> {
        self.slack.send_message(
            "#sx9-agent-status",
            &format!("Started: {} - {}", task.identifier, task.title),
        ).await
    }

    /// Notify QA result
    async fn notify_qa_result(&self, task: &AgentTask, passed: bool, report: &QaReport) -> Result<()> {
        let status = if passed { "PASSED" } else { "FAILED" };
        self.slack.send_message(
            "#sx9-agent-status",
            &format!(
                "QA {}: {} ({} issues)",
                status,
                task.identifier,
                report.issues.len()
            ),
        ).await
    }

    /// Create PR for generated files
    async fn create_pr(
        &self,
        task: &AgentTask,
        _files: &[super::GeneratedFile],
    ) -> Result<String> {
        // Placeholder - would use git operations
        let branch = task.branch.as_deref().unwrap_or("feature/agent-generated");
        Ok(format!(
            "https://github.com/synaptix9/sx9/pull/new/{}",
            branch
        ))
    }

    /// Get current state
    pub async fn state(&self) -> AgentState {
        *self.state.read().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_config_default() {
        let config = AgentConfig::default();
        assert_eq!(config.max_concurrent, 3);
        assert!(!config.dry_run);
    }
}
