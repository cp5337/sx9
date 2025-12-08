//! QA0: Code Census & Baseline Analysis
//! 
//! Simple initial code census: file count, line count, and genetic hash assignment.
//! Focused on basic metrics without complex dependencies.

use anyhow::Result;
use std::collections::HashMap;
use tokio::process::Command;
use tracing::info;
use crate::metrics::CensusMetrics;
use crate::hash_engine::Hasher;

/// Code census system for QA0 baseline analysis
pub struct CodeCensus;

impl CodeCensus {
    pub fn new() -> Self {
        Self
    }

    /// Run initial census: count files, lines, and assign genetic hash
    pub async fn run_initial_census(&self, repo_path: &str) -> Result<CensusMetrics> {
        info!("🔍 QA0: Starting initial census for {}", repo_path);
        
        let rust_files = self.count_files_by_extension(repo_path, "rs").await?;
        let python_files = self.count_files_by_extension(repo_path, "py").await?;
        let ts_files = self.count_files_by_extension(repo_path, "ts").await?;
        let total_files = rust_files + python_files + ts_files;
        let total_lines = self.count_total_lines(repo_path).await?;
        
        let mut language_distribution = HashMap::new();
        language_distribution.insert("Rust".to_string(), rust_files);
        language_distribution.insert("Python".to_string(), python_files);
        language_distribution.insert("TypeScript".to_string(), ts_files);
        
        // Generate genetic hash from census data
        let genetic_hash = self.generate_genetic_hash(&language_distribution, total_files, total_lines);
        
        let census = CensusMetrics {
            file_count: total_files,
            line_count: total_lines,
            language_distribution,
            complexity_score: 0.0, // Not calculated in initial census
            debt_ratio: 0.0,
            test_coverage: 0.0,
        };
        
        info!("✅ QA0: Census complete - Files: {}, Lines: {}, Hash: {}", 
              total_files, total_lines, genetic_hash);
        
        Ok(census)
    }

    async fn count_files_by_extension(&self, repo_path: &str, extension: &str) -> Result<u64> {
        let output = Command::new("find")
            .args(&[repo_path, "-name", &format!("*.{}", extension), "-type", "f"])
            .output()
            .await?;
        let files = String::from_utf8_lossy(&output.stdout);
        Ok(files.lines().filter(|line| !line.is_empty()).count() as u64)
    }

    async fn count_total_lines(&self, repo_path: &str) -> Result<u64> {
        let output = Command::new("find")
            .args(&[repo_path, "-name", "*.rs", "-o", "-name", "*.py", "-o", "-name", "*.ts"])
            .output()
            .await?;
        let files = String::from_utf8_lossy(&output.stdout);
        let file_list: Vec<&str> = files.lines().filter(|line| !line.is_empty()).collect();
        let mut total_lines = 0u64;
        for file_path in file_list {
            if let Ok(content) = tokio::fs::read_to_string(file_path).await {
                total_lines += content.lines().count() as u64;
            }
        }
        Ok(total_lines)
    }

    fn generate_genetic_hash(&self, language_dist: &HashMap<String, u64>, files: u64, lines: u64) -> String {
        let mut hasher = Hasher::new();
        hasher.update(format!("{}:{}:{}", files, lines, language_dist.len()).as_bytes());
        hasher.finalize().to_hex()[..16].to_string()
    }
}
