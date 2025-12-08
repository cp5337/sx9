//! QA Level Definitions and Orchestration
//! 
//! This module defines the core QA level abstractions and orchestration logic.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, error, debug};
use uuid::Uuid;

use crate::QASystemConfig;

/// QA Level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QALevel {
    QA0Census,
    QA1Standards,
    QA2Fratricide,
    QA3AIIntegration,
}

impl QALevel {
    pub fn name(&self) -> &'static str {
        match self {
            QALevel::QA0Census => "QA0: Code Census & Baseline",
            QALevel::QA1Standards => "QA1: Standards Enforcement",
            QALevel::QA2Fratricide => "QA2: Fratricide Detection",
            QALevel::QA3AIIntegration => "QA3: AI Integration & Validation",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            QALevel::QA0Census => "Code census, genetic hashing, and baseline analysis",
            QALevel::QA1Standards => "Code standards enforcement and compliance checking",
            QALevel::QA2Fratricide => "Fratricide detection and conflict analysis",
            QALevel::QA3AIIntegration => "AI-powered analysis and comprehensive validation",
        }
    }
}

/// QA Result for individual levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAResult {
    pub level: QALevel,
    pub status: QAStatus,
    pub score: f64, // 0.0 to 100.0
    pub issues: Vec<QAIssue>,
    pub metrics: HashMap<String, QAMetric>,
    pub execution_time: Duration,
    pub timestamp: DateTime<Utc>,
    pub id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QAStatus {
    Success,
    Warning,
    Error,
    Critical,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAIssue {
    pub severity: IssueSeverity,
    pub category: String,
    pub message: String,
    pub file_path: Option<String>,
    pub line_number: Option<u32>,
    pub suggestion: Option<String>,
    pub auto_fixable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QAMetric {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub description: String,
}

impl QAResult {
    pub fn new(level: QALevel) -> Self {
        Self {
            level,
            status: QAStatus::Success,
            score: 0.0,
            issues: Vec::new(),
            metrics: HashMap::new(),
            execution_time: Duration::from_secs(0),
            timestamp: Utc::now(),
            id: Uuid::new_v4(),
        }
    }

    pub fn add_issue(&mut self, issue: QAIssue) {
        // Update status based on issue severity
        match issue.severity {
            IssueSeverity::Critical => self.status = QAStatus::Critical,
            IssueSeverity::Error if matches!(self.status, QAStatus::Success | QAStatus::Warning) => {
                self.status = QAStatus::Error;
            }
            IssueSeverity::Warning if matches!(self.status, QAStatus::Success) => {
                self.status = QAStatus::Warning;
            }
            _ => {}
        }
        self.issues.push(issue);
    }

    pub fn add_metric(&mut self, name: String, metric: QAMetric) {
        self.metrics.insert(name, metric);
    }

    pub fn calculate_score(&mut self) {
        let total_issues = self.issues.len() as f64;
        if total_issues == 0.0 {
            self.score = 100.0;
            return;
        }

        let weight_sum: f64 = self.issues.iter().map(|issue| {
            match issue.severity {
                IssueSeverity::Critical => 10.0,
                IssueSeverity::Error => 5.0,
                IssueSeverity::Warning => 2.0,
                IssueSeverity::Info => 0.5,
            }
        }).sum();

        // Score decreases based on weighted issues
        self.score = (100.0 - weight_sum).max(0.0);
    }
}

/// QA System Orchestrator
pub struct QASystemOrchestrator {
    config: QASystemConfig,
    active_levels: Vec<QALevel>,
}

impl QASystemOrchestrator {
    pub fn new(config: QASystemConfig) -> Self {
        let mut active_levels = Vec::new();
        
        if config.qa_levels.qa0_census {
            active_levels.push(QALevel::QA0Census);
        }
        if config.qa_levels.qa1_standards {
            active_levels.push(QALevel::QA1Standards);
        }
        if config.qa_levels.qa2_fratricide {
            active_levels.push(QALevel::QA2Fratricide);
        }
        if config.qa_levels.qa3_ai_integration {
            active_levels.push(QALevel::QA3AIIntegration);
        }

        Self {
            config,
            active_levels,
        }
    }

    pub fn get_active_levels(&self) -> &[QALevel] {
        &self.active_levels
    }

    pub async fn execute_level(&self, level: QALevel, _repo_path: &str) -> Result<QAResult> {
        let start_time = std::time::Instant::now();
        info!("🔍 Executing {}", level.name());
        
        let mut result = QAResult::new(level);
        
        match level {
            QALevel::QA0Census => {
                // This will be handled by the census module
                debug!("QA0 Census execution delegated to census module");
            }
            QALevel::QA1Standards => {
                // This will be handled by the standards module
                debug!("QA1 Standards execution delegated to standards module");
            }
            QALevel::QA2Fratricide => {
                // This will be handled by the fratricide module
                debug!("QA2 Fratricide execution delegated to fratricide module");
            }
            QALevel::QA3AIIntegration => {
                // This will be handled by the integration module
                debug!("QA3 AI Integration execution delegated to integration module");
            }
        }

        result.execution_time = start_time.elapsed();
        result.calculate_score();
        
        info!("✅ {} completed with score: {:.2}", level.name(), result.score);
        
        Ok(result)
    }

    pub async fn execute_all_levels(&self, repo_path: &str) -> Result<Vec<QAResult>> {
        let mut results = Vec::new();
        
        for &level in &self.active_levels {
            match self.execute_level(level, repo_path).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    error!("Failed to execute {}: {}", level.name(), e);
                    let mut error_result = QAResult::new(level);
                    error_result.status = QAStatus::Error;
                    error_result.add_issue(QAIssue {
                        severity: IssueSeverity::Error,
                        category: "Execution".to_string(),
                        message: format!("Failed to execute level: {}", e),
                        file_path: None,
                        line_number: None,
                        suggestion: None,
                        auto_fixable: false,
                    });
                    results.push(error_result);
                }
            }
        }
        
        Ok(results)
    }

    pub fn get_overall_score(&self, results: &[QAResult]) -> f64 {
        if results.is_empty() {
            return 0.0;
        }
        
        let total: f64 = results.iter().map(|r| r.score).sum();
        total / results.len() as f64
    }

    pub fn get_critical_issues<'a>(&self, results: &'a [QAResult]) -> Vec<&'a QAIssue> {
        results.iter()
            .flat_map(|r| &r.issues)
            .filter(|issue| matches!(issue.severity, IssueSeverity::Critical))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qa_result_score_calculation() {
        let mut result = QAResult::new(QALevel::QA0Census);
        
        // Add a critical issue
        result.add_issue(QAIssue {
            severity: IssueSeverity::Critical,
            category: "Test".to_string(),
            message: "Critical issue".to_string(),
            file_path: None,
            line_number: None,
            suggestion: None,
            auto_fixable: false,
        });
        
        result.calculate_score();
        
        assert_eq!(result.score, 90.0); // 100 - 10 (critical weight)
        assert!(matches!(result.status, QAStatus::Critical));
    }

    #[test]
    fn test_orchestrator_active_levels() {
        let config = QASystemConfig::default();
        let orchestrator = QASystemOrchestrator::new(config);
        
        assert_eq!(orchestrator.get_active_levels().len(), 4);
        assert!(orchestrator.get_active_levels().contains(&QALevel::QA0Census));
        assert!(orchestrator.get_active_levels().contains(&QALevel::QA1Standards));
        assert!(orchestrator.get_active_levels().contains(&QALevel::QA2Fratricide));
        assert!(orchestrator.get_active_levels().contains(&QALevel::QA3AIIntegration));
    }
}
