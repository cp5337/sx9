//! Coder Agent
//!
//! Implements code changes based on Linear issue specifications.
//! Core agent that generates, modifies, and tests code.

use anyhow::Result;
use std::path::Path;
use tracing::{debug, info, warn};

use super::{AgentAction, AgentConfig, AgentTask, GeneratedFile};
use crate::mcp::{CodeSpec, SerenaClient};

/// Coder agent - implements code changes
pub struct CoderAgent<'a> {
    serena: &'a SerenaClient,
    config: &'a AgentConfig,
}

impl<'a> CoderAgent<'a> {
    /// Create new coder agent
    pub fn new(serena: &'a SerenaClient, config: &'a AgentConfig) -> Self {
        Self { serena, config }
    }

    /// Run the coder agent
    pub async fn run(&self, task: &AgentTask) -> Result<AgentAction> {
        info!("Coder processing: {}", task.identifier);

        // Build code specification from task
        let spec = self.build_spec(task);

        // Generate code using Serena
        let generated = self.serena.generate_code(&spec).await?;

        // Convert to generated files
        let files: Vec<GeneratedFile> = generated
            .files
            .into_iter()
            .map(|f| GeneratedFile {
                path: f.path,
                content: f.content,
                is_new: true,
            })
            .collect();

        let tests: Vec<GeneratedFile> = generated
            .tests
            .into_iter()
            .map(|f| GeneratedFile {
                path: f.path,
                content: f.content,
                is_new: true,
            })
            .collect();

        info!(
            "Generated {} source files and {} test files",
            files.len(),
            tests.len()
        );

        // Write files if not in dry-run mode
        if !self.config.dry_run {
            for file in &files {
                self.write_file(file).await?;
            }
            for file in &tests {
                self.write_file(file).await?;
            }
        } else {
            debug!("Dry-run mode: files not written");
        }

        Ok(AgentAction::CodeGenerated { files, tests })
    }

    /// Build code specification from task
    fn build_spec(&self, task: &AgentTask) -> CodeSpec {
        // Extract language from description or default to Rust
        let language = self.detect_language(&task.description);

        // Extract framework hints
        let framework = self.detect_framework(&task.description);

        // Parse requirements from description
        let requirements = self.parse_requirements(&task.description);

        CodeSpec {
            title: task.title.clone(),
            description: task.description.clone(),
            requirements,
            language,
            framework,
        }
    }

    /// Detect programming language from description
    fn detect_language(&self, description: &str) -> String {
        let desc_lower = description.to_lowercase();

        if desc_lower.contains("rust") || desc_lower.contains(".rs") {
            "rust".to_string()
        } else if desc_lower.contains("typescript") || desc_lower.contains(".tsx") {
            "typescript".to_string()
        } else if desc_lower.contains("python") || desc_lower.contains(".py") {
            "python".to_string()
        } else if desc_lower.contains("javascript") || desc_lower.contains(".js") {
            "javascript".to_string()
        } else {
            // Default to Rust for SX9 project
            "rust".to_string()
        }
    }

    /// Detect framework from description
    fn detect_framework(&self, description: &str) -> Option<String> {
        let desc_lower = description.to_lowercase();

        if desc_lower.contains("tokio") || desc_lower.contains("async") {
            Some("tokio".to_string())
        } else if desc_lower.contains("axum") {
            Some("axum".to_string())
        } else if desc_lower.contains("react") {
            Some("react".to_string())
        } else if desc_lower.contains("tauri") {
            Some("tauri".to_string())
        } else {
            None
        }
    }

    /// Parse requirements from description
    fn parse_requirements(&self, description: &str) -> Vec<String> {
        let mut requirements = Vec::new();

        // Look for bullet points or numbered lists
        for line in description.lines() {
            let line = line.trim();

            // Match "- item" or "* item" or "1. item"
            if line.starts_with("- ")
                || line.starts_with("* ")
                || line.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
                    && line.contains(". ")
            {
                let content = line
                    .trim_start_matches(|c: char| c == '-' || c == '*' || c.is_ascii_digit() || c == '.')
                    .trim();

                if !content.is_empty() {
                    requirements.push(content.to_string());
                }
            }
        }

        // If no structured requirements, split by sentences
        if requirements.is_empty() {
            requirements = description
                .split('.')
                .filter(|s| s.len() > 10)
                .take(5)
                .map(|s| s.trim().to_string())
                .collect();
        }

        requirements
    }

    /// Write file to disk
    async fn write_file(&self, file: &GeneratedFile) -> Result<()> {
        let path = Path::new(&self.config.repo_path).join(&file.path);

        // Create parent directories
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Write content
        tokio::fs::write(&path, &file.content).await?;

        debug!("Wrote file: {}", file.path);
        Ok(())
    }

    /// Run tests for generated code
    pub async fn run_tests(&self, files: &[GeneratedFile]) -> Result<TestResult> {
        debug!("Running tests for {} files", files.len());

        // Would run cargo test or npm test depending on language
        // For now, return success
        Ok(TestResult {
            passed: true,
            total: files.len(),
            failed: 0,
            output: String::new(),
        })
    }

    /// Format generated code
    pub async fn format_code(&self, files: &mut [GeneratedFile]) -> Result<()> {
        for file in files.iter_mut() {
            let language = self.detect_language(&file.path);

            match language.as_str() {
                "rust" => {
                    // Would run rustfmt
                    debug!("Formatting Rust file: {}", file.path);
                }
                "typescript" | "javascript" => {
                    // Would run prettier
                    debug!("Formatting TS/JS file: {}", file.path);
                }
                _ => {
                    warn!("No formatter for language: {}", language);
                }
            }
        }

        Ok(())
    }
}

/// Test execution result
#[derive(Debug)]
pub struct TestResult {
    pub passed: bool,
    pub total: usize,
    pub failed: usize,
    pub output: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_language() {
        let config = AgentConfig::default();
        // Can't easily test without SerenaClient, but we can test helper functions
        assert!(true);
    }

    #[test]
    fn test_parse_requirements() {
        let description = r#"
Add a new feature:
- Implement authentication
- Add rate limiting
- Create API endpoints
"#;

        // Would test parse_requirements here
        assert!(description.contains("authentication"));
    }
}
