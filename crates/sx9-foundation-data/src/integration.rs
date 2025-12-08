//! Integration Module for External Systems
//! 
//! This module handles integration with repo-prompt, AI-CLI, Python scripts,
//! and hook systems.

use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;
use tracing::{info, warn};

use crate::levels::QAResult;

/// Repo-Prompt Integration
pub struct RepoPromptIntegration {
    client: Client,
    base_url: String,
}

impl RepoPromptIntegration {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "http://localhost:8080".to_string(), // Default repo-prompt port
        }
    }

    pub fn with_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    pub async fn analyze_repository(&self, repo_path: &str) -> Result<RepoPromptAnalysis> {
        info!("🔗 Connecting to repo-prompt for analysis");
        
        let request = RepoPromptRequest {
            repo_path: repo_path.to_string(),
            analysis_type: "comprehensive".to_string(),
            include_metrics: true,
            persona: "qa-analyst".to_string(),
        };

        let response = self.client
            .post(&format!("{}/api/analyze", self.base_url))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let analysis: RepoPromptAnalysis = response.json().await?;
            info!("✅ Repo-prompt analysis completed");
            Ok(analysis)
        } else {
            let error_text = response.text().await?;
            anyhow::bail!("Repo-prompt analysis failed: {}", error_text);
        }
    }

    pub async fn get_persona_recommendations(&self, code_snippet: &str) -> Result<Vec<String>> {
        let request = PersonaRequest {
            code: code_snippet.to_string(),
            persona: "code-reviewer".to_string(),
        };

        let response = self.client
            .post(&format!("{}/api/persona/recommend", self.base_url))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let recommendations: PersonaResponse = response.json().await?;
            Ok(recommendations.recommendations)
        } else {
            warn!("Failed to get persona recommendations");
            Ok(Vec::new())
        }
    }
}

#[derive(Debug, Serialize)]
struct RepoPromptRequest {
    repo_path: String,
    analysis_type: String,
    include_metrics: bool,
    persona: String,
}

#[derive(Debug, Serialize)]
struct PersonaRequest {
    code: String,
    persona: String,
}

#[derive(Debug, Deserialize)]
struct PersonaResponse {
    recommendations: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct RepoPromptAnalysis {
    pub summary: String,
    pub quality_score: f64,
    pub issues: Vec<RepoPromptIssue>,
    pub metrics: HashMap<String, f64>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RepoPromptIssue {
    pub severity: String,
    pub category: String,
    pub message: String,
    pub file_path: Option<String>,
    pub line_number: Option<u32>,
}

/// AI-CLI Integration
pub struct AiCliIntegration {
    enabled: bool,
}

impl AiCliIntegration {
    pub fn new() -> Self {
        Self {
            enabled: Self::check_ai_cli_available(),
        }
    }

    fn check_ai_cli_available() -> bool {
        std::process::Command::new("ai")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    pub async fn analyze_code_quality(&self, file_path: &str) -> Result<AiCliAnalysis> {
        if !self.enabled {
            anyhow::bail!("AI-CLI is not available or installed");
        }

        info!("🤖 Running AI-CLI analysis on: {}", file_path);

        let output = Command::new("ai")
            .args(&[
                "analyze",
                "--quality",
                "--format", "json",
                file_path
            ])
            .output()
            .await?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let analysis: AiCliAnalysis = serde_json::from_str(&stdout)?;
            info!("✅ AI-CLI analysis completed");
            Ok(analysis)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("AI-CLI analysis failed: {}", stderr);
        }
    }

    pub async fn get_code_suggestions(&self, code: &str) -> Result<Vec<String>> {
        if !self.enabled {
            return Ok(Vec::new());
        }

        let output = Command::new("ai")
            .args(&["suggest", "--format", "json"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut child = output;
        if let Some(stdin) = child.stdin.as_mut() {
            use tokio::io::AsyncWriteExt;
            stdin.write_all(code.as_bytes()).await?;
        }

        let output = child.wait_with_output().await?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let suggestions: SuggestionsResponse = serde_json::from_str(&stdout)?;
            Ok(suggestions.suggestions)
        } else {
            warn!("Failed to get AI-CLI suggestions");
            Ok(Vec::new())
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AiCliAnalysis {
    pub quality_score: f64,
    pub complexity_score: f64,
    pub maintainability: f64,
    pub issues: Vec<AiCliIssue>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AiCliIssue {
    pub severity: String,
    pub category: String,
    pub message: String,
    pub line: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct SuggestionsResponse {
    suggestions: Vec<String>,
}

/// Python Integration
pub struct PythonIntegration {
    python_executable: String,
}

impl PythonIntegration {
    pub fn new() -> Self {
        let python_executable = Self::find_python_executable();
        Self { python_executable }
    }

    fn find_python_executable() -> String {
        // Try different Python executables
        for python in &["python3", "python", "py"] {
            if let Ok(output) = std::process::Command::new(python)
                .arg("--version")
                .output()
            {
                if output.status.success() {
                    return python.to_string();
                }
            }
        }
        "python3".to_string() // Default fallback
    }

    pub async fn run_codebase_census(&self, repo_path: &str) -> Result<PythonCensusResult> {
        info!("🐍 Running Python codebase census");

        let script_path = Path::new(repo_path).join("codebase_census.py");
        if !script_path.exists() {
            anyhow::bail!("codebase_census.py not found in repository");
        }

        let output = Command::new(&self.python_executable)
            .args(&[
                script_path.to_str().unwrap(),
                "--json",
                "--repo", repo_path
            ])
            .output()
            .await?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let result: PythonCensusResult = serde_json::from_str(&stdout)?;
            info!("✅ Python census completed");
            Ok(result)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Python census failed: {}", stderr);
        }
    }

    pub async fn run_comprehensive_analyzer(&self, repo_path: &str) -> Result<PythonAnalysisResult> {
        info!("🐍 Running Python comprehensive code analyzer");

        let script_path = Path::new(repo_path).join("comprehensive_code_analyzer.py");
        if !script_path.exists() {
            anyhow::bail!("comprehensive_code_analyzer.py not found in repository");
        }

        let output = Command::new(&self.python_executable)
            .args(&[
                script_path.to_str().unwrap(),
                "--format", "json",
                repo_path
            ])
            .output()
            .await?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let result: PythonAnalysisResult = serde_json::from_str(&stdout)?;
            info!("✅ Python comprehensive analysis completed");
            Ok(result)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Python comprehensive analysis failed: {}", stderr);
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PythonCensusResult {
    pub total_files: u32,
    pub languages: HashMap<String, u32>,
    pub total_lines: u32,
    pub code_lines: u32,
    pub comment_lines: u32,
    pub blank_lines: u32,
    pub file_types: HashMap<String, u32>,
}

#[derive(Debug, Deserialize)]
pub struct PythonAnalysisResult {
    pub quality_metrics: HashMap<String, f64>,
    pub issues: Vec<PythonIssue>,
    pub complexity_analysis: HashMap<String, f64>,
    pub security_findings: Vec<PythonSecurityIssue>,
}

#[derive(Debug, Deserialize)]
pub struct PythonIssue {
    pub severity: String,
    pub category: String,
    pub message: String,
    pub file_path: String,
    pub line_number: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct PythonSecurityIssue {
    pub severity: String,
    pub cwe_id: Option<String>,
    pub message: String,
    pub file_path: String,
    pub line_number: Option<u32>,
}

/// Hook System Integration
pub struct HookSystemIntegration {
    hooks_enabled: bool,
}

impl HookSystemIntegration {
    pub fn new() -> Self {
        Self {
            hooks_enabled: true,
        }
    }

    pub async fn trigger_pre_analysis_hooks(&self, repo_path: &str) -> Result<()> {
        if !self.hooks_enabled {
            return Ok(());
        }

        info!("🪝 Triggering pre-analysis hooks");
        
        // Call CTAS hook system
        let output = Command::new("ctas-hooks")
            .args(&["trigger", "pre-analysis", repo_path])
            .output()
            .await;

        match output {
            Ok(result) if result.status.success() => {
                info!("✅ Pre-analysis hooks completed successfully");
                Ok(())
            }
            Ok(result) => {
                let stderr = String::from_utf8_lossy(&result.stderr);
                warn!("Pre-analysis hooks completed with warnings: {}", stderr);
                Ok(())
            }
            Err(e) => {
                warn!("Failed to run pre-analysis hooks: {}", e);
                Ok(()) // Don't fail the entire analysis
            }
        }
    }

    pub async fn trigger_post_analysis_hooks(&self, repo_path: &str, results: &[QAResult]) -> Result<()> {
        if !self.hooks_enabled {
            return Ok(());
        }

        info!("🪝 Triggering post-analysis hooks");

        // Create temporary results file for hooks
        let results_json = serde_json::to_string_pretty(results)?;
        let temp_results = tempfile::NamedTempFile::new()?;
        tokio::fs::write(temp_results.path(), results_json).await?;

        let output = Command::new("ctas-hooks")
            .args(&[
                "trigger", "post-analysis",
                "--results", temp_results.path().to_str().unwrap(),
                repo_path
            ])
            .output()
            .await;

        match output {
            Ok(result) if result.status.success() => {
                info!("✅ Post-analysis hooks completed successfully");
                Ok(())
            }
            Ok(result) => {
                let stderr = String::from_utf8_lossy(&result.stderr);
                warn!("Post-analysis hooks completed with warnings: {}", stderr);
                Ok(())
            }
            Err(e) => {
                warn!("Failed to run post-analysis hooks: {}", e);
                Ok(()) // Don't fail the entire analysis
            }
        }
    }
}

impl Default for RepoPromptIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AiCliIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PythonIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for HookSystemIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_integration_creation() {
        let integration = PythonIntegration::new();
        assert!(!integration.python_executable.is_empty());
    }

    #[test]
    fn test_ai_cli_integration_creation() {
        let integration = AiCliIntegration::new();
        // Just test that it creates without panicking
        assert!(!integration.enabled || integration.enabled);
    }

    #[test]
    fn test_repo_prompt_integration_with_custom_url() {
        let integration = RepoPromptIntegration::new()
            .with_url("http://localhost:9090".to_string());
        assert_eq!(integration.base_url, "http://localhost:9090");
    }
}
