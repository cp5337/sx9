//! QA Orchestrator: Runs all QA phases and aggregates results
use anyhow::Result;
use crate::qa_types::{PhaseReport, RehabReport};
use crate::census::CodeCensus;

pub struct QAOrchestrator;

impl QAOrchestrator {
    pub fn new() -> Self {
        Self
    }

    /// Run all QA phases and return a unified rehabilitation report
    pub async fn rehabilitate_repo(&self, repo_path: &str) -> Result<RehabReport> {
        let mut phases = Vec::new();
        // Run QA0 (baseline census)
        let census = CodeCensus::new();
        let census_metrics = census.run_initial_census(repo_path).await?;
        let mut metrics = std::collections::HashMap::new();
        metrics.insert("file_count".to_string(), census_metrics.file_count);
        metrics.insert("line_count".to_string(), census_metrics.line_count);
        let summary = format!("Files: {}, Lines: {}", census_metrics.file_count, census_metrics.line_count);
        phases.push(PhaseReport {
            phase: "QA0".to_string(),
            summary,
            metrics,
        });
        // TODO: Add QA1-QA5 phases here
        let overall_status = "Rehabilitation complete (QA0 baseline)".to_string();
        Ok(RehabReport { phases, overall_status })
    }
}
