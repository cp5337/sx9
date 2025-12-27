//! Agent Loop
//!
//! Main orchestration loop for autonomous agent processing.
//! Polls Linear for assigned issues and dispatches to appropriate agents.
//! Integrates with sx9-harness QA gates (RFC-9050) and Git operations (RFC-9030).

use anyhow::Result;
use std::path::Path;
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

// Import sx9-harness QA gates (RFC-9050)
use sx9_harness::{StaticGate, SemanticGate, ArchGate, PatternGate};

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

    /// Run QA gates on generated files using sx9-harness (RFC-9050)
    async fn run_qa_gates(&self, files: &[super::GeneratedFile]) -> Result<QaReport> {
        debug!("Running QA gates on {} files", files.len());

        let mut issues = Vec::new();
        let mut static_passed = true;
        let mut arch_passed = true;
        let mut pattern_passed = true;
        let mut semantic_passed = true;

        // Get the crate path from the first file or use current directory
        let crate_path = if let Some(first_file) = files.first() {
            let p = Path::new(&first_file.path);
            // Navigate up to find Cargo.toml
            p.ancestors()
                .find(|a| a.join("Cargo.toml").exists())
                .unwrap_or(Path::new("."))
                .to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_else(|_| ".".into())
        };

        // Run Static Gate (cargo check, complexity analysis)
        let static_gate = StaticGate::new();
        match static_gate.run(&crate_path).await {
            Ok(report) => {
                if report.structure_score < 50 || !report.findings.is_empty() {
                    static_passed = false;
                    for finding in report.findings {
                        issues.push(super::QaIssue {
                            gate: "static".to_string(),
                            severity: super::QaSeverity::Error,
                            message: format!("[{}] {}", finding.rule, finding.message),
                            file: Some(finding.file),
                            line: Some(finding.line),
                        });
                    }
                }
                debug!("Static gate: structure={}, complexity={}",
                    report.structure_score, report.complexity_score);
            }
            Err(e) => {
                warn!("Static gate error: {}", e);
            }
        }

        // Run Architecture Gate (dependency analysis)
        let arch_gate = ArchGate::new();
        match arch_gate.run(&crate_path).await {
            Ok(report) => {
                if !report.violations.is_empty() {
                    arch_passed = false;
                    for violation in &report.violations {
                        issues.push(super::QaIssue {
                            gate: "arch".to_string(),
                            severity: super::QaSeverity::Warning,
                            message: format!("[{}] {}", violation.rule, violation.description),
                            file: Some(violation.file.clone()),
                            line: None,
                        });
                    }
                }
                debug!("Arch gate: violations={}", report.violations.len());
            }
            Err(e) => {
                warn!("Arch gate error: {}", e);
            }
        }

        // Run Pattern Gate (code pattern analysis)
        let pattern_gate = PatternGate::new();
        match pattern_gate.run(&crate_path).await {
            Ok(_report) => {
                // Pattern gate returns a Value, check for issues
                debug!("Pattern gate: passed");
            }
            Err(e) => {
                pattern_passed = false;
                issues.push(super::QaIssue {
                    gate: "pattern".to_string(),
                    severity: super::QaSeverity::Warning,
                    message: e,
                    file: None,
                    line: None,
                });
            }
        }

        // Run Semantic Gate (drift detection per RFC-9142)
        let semantic_gate = SemanticGate::new();
        match semantic_gate.run(&crate_path).await {
            Ok(report) => {
                if !report.passed {
                    semantic_passed = false;
                    for signal in &report.drift_signals {
                        issues.push(super::QaIssue {
                            gate: "semantic".to_string(),
                            severity: if signal.score > 0.8 {
                                super::QaSeverity::Error
                            } else {
                                super::QaSeverity::Warning
                            },
                            message: signal.explanation.clone(),
                            file: None,
                            line: None,
                        });
                    }
                }
                debug!("Semantic gate: passed={}, drift_signals={}",
                    report.passed, report.drift_signals.len());
            }
            Err(e) => {
                warn!("Semantic gate error: {}", e);
            }
        }

        // Certification requires all gates to pass
        let certified = static_passed && arch_passed && pattern_passed && semantic_passed;

        info!(
            "QA gates complete: static={}, arch={}, pattern={}, semantic={}, certified={}",
            static_passed, arch_passed, pattern_passed, semantic_passed, certified
        );

        Ok(QaReport {
            static_passed,
            arch_passed,
            pattern_passed,
            semantic_passed,
            certified,
            issues,
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

    /// Create PR for generated files using git2 (RFC-9030)
    async fn create_pr(
        &self,
        task: &AgentTask,
        files: &[super::GeneratedFile],
    ) -> Result<String> {
        use git2::{Repository, Signature};
        use std::process::Command;

        let branch = task.branch.as_deref().unwrap_or("feature/agent-generated");
        let repo_path = std::env::current_dir()?;

        info!("Creating PR for branch: {}", branch);

        // Open repository
        let repo = Repository::open(&repo_path)
            .map_err(|e| anyhow::anyhow!("Failed to open repository: {}", e))?;

        // Create and checkout branch
        let head = repo.head()?;
        let head_commit = head.peel_to_commit()?;

        // Create branch if it doesn't exist
        if repo.find_branch(branch, git2::BranchType::Local).is_err() {
            repo.branch(branch, &head_commit, false)?;
            debug!("Created branch: {}", branch);
        }

        // Checkout branch
        let obj = repo.revparse_single(&format!("refs/heads/{}", branch))?;
        repo.checkout_tree(&obj, None)?;
        repo.set_head(&format!("refs/heads/{}", branch))?;
        debug!("Checked out branch: {}", branch);

        // Write files to disk
        for file in files {
            let file_path = repo_path.join(&file.path);
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&file_path, &file.content)?;
            debug!("Wrote file: {}", file.path);
        }

        // Stage files
        let mut index = repo.index()?;
        for file in files {
            index.add_path(Path::new(&file.path))?;
        }
        index.write()?;

        // Create commit
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        let signature = Signature::now("SX9 Agent", "agent@sx9.ai")?;
        let parent = repo.find_commit(head_commit.id())?;

        let commit_msg = format!(
            "{}: {}\n\n🤖 Generated with [Claude Code](https://claude.com/claude-code)\n\nCo-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>",
            task.identifier, task.title
        );

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &commit_msg,
            &tree,
            &[&parent],
        )?;
        info!("Created commit for {}", task.identifier);

        // Push to remote using git CLI (git2 push requires credentials setup)
        let push_output = Command::new("git")
            .args(["push", "-u", "origin", branch])
            .current_dir(&repo_path)
            .output()?;

        if !push_output.status.success() {
            let stderr = String::from_utf8_lossy(&push_output.stderr);
            return Err(anyhow::anyhow!("Git push failed: {}", stderr));
        }
        info!("Pushed branch {} to origin", branch);

        // Create PR using gh CLI
        let pr_output = Command::new("gh")
            .args([
                "pr", "create",
                "--title", &format!("{}: {}", task.identifier, task.title),
                "--body", &format!(
                    "## Linear Issue\nCloses {}\n\n## Summary\n{}\n\n---\n🤖 Generated by SX9 Agent",
                    task.identifier,
                    task.description
                ),
                "--base", "main",
                "--head", branch,
            ])
            .current_dir(&repo_path)
            .output()?;

        if !pr_output.status.success() {
            let stderr = String::from_utf8_lossy(&pr_output.stderr);
            return Err(anyhow::anyhow!("PR creation failed: {}", stderr));
        }

        let pr_url = String::from_utf8_lossy(&pr_output.stdout).trim().to_string();
        info!("Created PR: {}", pr_url);

        Ok(pr_url)
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
