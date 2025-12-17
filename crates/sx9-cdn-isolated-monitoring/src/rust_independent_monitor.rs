// Cargo.toml
[package]
name = "independent-monitor"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
chrono = { version = "0.4", features = ["serde"] }
regex = "1.0"
walkdir = "2.0"
sha2 = "0.10"
hex = "0.4"
rusqlite = { version = "0.29", features = ["bundled"] }
clap = { version = "4.0", features = ["derive"] }
flate2 = "1.0"
indicatif = "0.17"
tabled = "0.15"
colored = "2.0"

// src/main.rs
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::Parser;
use colored::*;
use std::path::PathBuf;

mod monitor;
mod context;
mod database;
mod violations;
mod leaderboard;

use monitor::IndependentMonitor;

#[derive(Parser)]
#[command(name = "independent-monitor")]
#[command(about = "Independent code quality monitor - catches eager agents")]
struct Args {
    /// Workspace root directory
    #[arg(short, long, default_value = ".")]
    workspace: PathBuf,
    
    /// Output directory for reports
    #[arg(short, long, default_value = "quality-reports")]
    output: PathBuf,
    
    /// Generate leaderboard
    #[arg(short, long)]
    leaderboard: bool,
    
    /// Compact context
    #[arg(short, long)]
    compact: bool,
    
    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("{}", "🚀 Independent Quality Monitor".bright_blue().bold());
    println!("{}", "================================".bright_blue());
    
    let mut monitor = IndependentMonitor::new(args.workspace.clone()).await?;
    
    // Scan workspace for violations
    println!("\n{}", "🔍 Scanning workspace for violations...".yellow());
    let metrics = monitor.scan_workspace().await?;
    
    // Generate reports
    let report_dir = args.output;
    std::fs::create_dir_all(&report_dir).context("Failed to create report directory")?;
    
    if args.leaderboard {
        println!("\n{}", "🏆 Generating leaderboard...".yellow());
        let leaderboard = monitor.generate_leaderboard(&metrics).await?;
        leaderboard.save_to_file(&report_dir.join("leaderboard.json"))?;
        leaderboard.print_summary();
    }
    
    if args.compact {
        println!("\n{}", "🗜️  Compacting context...".yellow());
        let context_manager = context::ContextManager::new(args.workspace.clone());
        let compaction_report = context_manager.compact_context().await?;
        println!("✅ Context compaction complete. Ratio: {:.2}x", compaction_report.compression_ratio);
    }
    
    // Print summary
    let total_violations: usize = metrics.iter().map(|m| m.violations.len()).sum();
    let total_loc: usize = metrics.iter().map(|m| m.total_loc).sum();
    
    println!("\n{}", "📊 Scan Summary".green().bold());
    println!("Crates scanned: {}", metrics.len().to_string().bright_white());
    println!("Total LOC: {}", total_loc.to_string().bright_white());
    println!("Total violations: {}", total_violations.to_string().bright_red());
    println!("Average quality score: {:.1}", 
        metrics.iter().map(|m| m.quality_score).sum::<f64>() / metrics.len() as f64);
    
    Ok(())
}

// src/monitor.rs
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use sha2::{Sha256, Digest};

use crate::database::Database;
use crate::violations::{QualityViolation, ViolationType};
use crate::leaderboard::Leaderboard;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CrateMetrics {
    pub name: String,
    pub total_loc: usize,
    pub rust_files: usize,
    pub last_write: DateTime<Utc>,
    pub violations: Vec<QualityViolation>,
    
    // Eagerness indicators
    pub functions_over_200_lines: usize,
    pub lines_over_90_cols: usize,
    pub max_nesting_depth: usize,
    pub nested_violations: usize,
    
    // Code quality issues
    pub duplicate_blocks: usize,
    pub verbose_names: usize,
    pub non_canonical_paths: usize,
    pub model_generation_score: f64,
    
    // Trend data
    pub loc_trend_7d: i32,
    pub violation_trend_7d: i32,
    pub quality_score: f64,
}

pub struct IndependentMonitor {
    workspace_root: PathBuf,
    database: Database,
    eagerness_patterns: HashMap<String, Regex>,
}

impl IndependentMonitor {
    pub async fn new(workspace_root: PathBuf) -> Result<Self> {
        let database = Database::new(&workspace_root.join(".quality_monitor.db")).await?;
        
        let mut eagerness_patterns = HashMap::new();
        
        // Patterns that indicate model "eagerness"
        eagerness_patterns.insert(
            "verbose_function_names".to_string(),
            Regex::new(r"fn\s+(\w{25,})").unwrap()
        );
        eagerness_patterns.insert(
            "excessive_parameters".to_string(),
            Regex::new(r"fn\s+\w+\([^)]{100,}\)").unwrap()
        );
        eagerness_patterns.insert(
            "deep_nesting".to_string(),
            Regex::new(r"(\s{16,})(?:if|match|for|while)").unwrap()
        );
        eagerness_patterns.insert(
            "repetitive_cloning".to_string(),
            Regex::new(r"(\.clone\(\)[^;]*){3,}").unwrap()
        );
        eagerness_patterns.insert(
            "verbose_variables".to_string(),
            Regex::new(r"let\s+(\w{20,})\s*=").unwrap()
        );
        eagerness_patterns.insert(
            "non_canonical_paths".to_string(),
            Regex::new(r"\w+::\w+::\w+::\w+::\w+").unwrap()
        );
        eagerness_patterns.insert(
            "excessive_unwraps".to_string(),
            Regex::new(r"(\.unwrap\(\)[^;]*){2,}").unwrap()
        );
        eagerness_patterns.insert(
            "println_spam".to_string(),
            Regex::new(r"(println!\([^)]*\)[^;]*){3,}").unwrap()
        );
        
        Ok(Self {
            workspace_root,
            database,
            eagerness_patterns,
        })
    }
    
    pub async fn scan_workspace(&mut self) -> Result<Vec<CrateMetrics>> {
        let mut metrics = Vec::new();
        
        // Find all Cargo.toml files (crates)
        for entry in WalkDir::new(&self.workspace_root)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| !e.path().to_string_lossy().contains("target/"))
        {
            let entry = entry.context("Failed to read directory entry")?;
            
            if entry.file_name() == "Cargo.toml" {
                let crate_dir = entry.path().parent().unwrap();
                let crate_name = crate_dir.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                
                println!("  📦 Analyzing crate: {}", crate_name);
                
                let crate_metrics = self.analyze_crate(crate_dir, &crate_name).await?;
                
                // Store metrics in database
                self.database.store_metrics(&crate_metrics).await?;
                
                metrics.push(crate_metrics);
            }
        }
        
        Ok(metrics)
    }
    
    async fn analyze_crate(&mut self, crate_dir: &Path, crate_name: &str) -> Result<CrateMetrics> {
        let mut violations = Vec::new();
        let mut total_loc = 0;
        let mut functions_over_200 = 0;
        let mut lines_over_90 = 0;
        let mut max_nesting = 0;
        let mut nested_violations = 0;
        let mut duplicate_blocks = 0;
        let mut verbose_names = 0;
        let mut non_canonical_paths = 0;
        let mut model_generation_scores = Vec::new();
        
        // Find all Rust files
        let rust_files: Vec<PathBuf> = WalkDir::new(crate_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
            .map(|e| e.path().to_path_buf())
            .collect();
        
        for rust_file in &rust_files {
            let content = fs::read_to_string(rust_file)
                .with_context(|| format!("Failed to read file: {}", rust_file.display()))?;
            
            let lines: Vec<&str> = content.lines().collect();
            total_loc += lines.len();
            
            // Analyze each line
            for (line_num, line) in lines.iter().enumerate() {
                let line_number = line_num + 1;
                
                // Check line length violations
                if line.len() > 90 {
                    lines_over_90 += 1;
                    violations.push(QualityViolation {
                        crate_name: crate_name.to_string(),
                        file_path: rust_file.strip_prefix(&self.workspace_root)
                            .unwrap_or(rust_file)
                            .to_string_lossy()
                            .to_string(),
                        violation_type: ViolationType::LineTooLong,
                        line_number,
                        severity: "minor".to_string(),
                        details: format!("Line length: {} chars", line.len()),
                        detected_at: Utc::now(),
                    });
                }
                
                // Check eagerness patterns
                for (pattern_name, regex) in &self.eagerness_patterns {
                    if regex.is_match(line) {
                        let severity = match pattern_name.as_str() {
                            "deep_nesting" | "verbose_function_names" => "major",
                            _ => "minor",
                        };
                        
                        violations.push(QualityViolation {
                            crate_name: crate_name.to_string(),
                            file_path: rust_file.strip_prefix(&self.workspace_root)
                                .unwrap_or(rust_file)
                                .to_string_lossy()
                                .to_string(),
                            violation_type: ViolationType::from_str(pattern_name),
                            line_number,
                            severity: severity.to_string(),
                            details: format!("Pattern: {}", pattern_name),
                            detected_at: Utc::now(),
                        });
                        
                        match pattern_name.as_str() {
                            "verbose_function_names" => verbose_names += 1,
                            "non_canonical_paths" => non_canonical_paths += 1,
                            "deep_nesting" => nested_violations += 1,
                            _ => {}
                        }
                    }
                }
            }
            
            // Analyze functions
            let function_violations = self.analyze_functions(&content, rust_file, crate_name)?;
            functions_over_200 += function_violations.iter()
                .filter(|v| matches!(v.violation_type, ViolationType::FunctionTooLong))
                .count();
            violations.extend(function_violations);
            
            // Calculate nesting depth
            let nesting_depth = self.calculate_max_nesting(&content);
            max_nesting = max_nesting.max(nesting_depth);
            
            // Detect duplicates
            let duplicates = self.detect_duplicates_in_file(&content);
            duplicate_blocks += duplicates;
            
            // Model generation detection
            let model_score = self.detect_model_generation(&content);
            model_generation_scores.push(model_score);
            
            // Track file write
            self.track_file_write(crate_name, rust_file, lines.len(), model_score > 0.7).await?;
        }
        
        // Calculate trends
        let loc_trend = self.database.calculate_loc_trend(crate_name, 7).await?;
        let violation_trend = self.database.calculate_violation_trend(crate_name, 7).await?;
        
        // Calculate quality score
        let quality_score = self.calculate_quality_score(
            total_loc,
            violations.len(),
            functions_over_200,
            lines_over_90,
            nested_violations,
            duplicate_blocks,
        );
        
        let avg_model_score = if model_generation_scores.is_empty() {
            0.0
        } else {
            model_generation_scores.iter().sum::<f64>() / model_generation_scores.len() as f64
        };
        
        Ok(CrateMetrics {
            name: crate_name.to_string(),
            total_loc,
            rust_files: rust_files.len(),
            last_write: self.database.get_last_write_time(crate_name).await?,
            violations,
            functions_over_200_lines: functions_over_200,
            lines_over_90_cols: lines_over_90,
            max_nesting_depth: max_nesting,
            nested_violations,
            duplicate_blocks,
            verbose_names,
            non_canonical_paths,
            model_generation_score: avg_model_score,
            loc_trend_7d: loc_trend,
            violation_trend_7d: violation_trend,
            quality_score,
        })
    }
    
    fn analyze_functions(&self, content: &str, file_path: &Path, crate_name: &str) -> Result<Vec<QualityViolation>> {
        let mut violations = Vec::new();
        
        // Regex to match function definitions and their bodies
        let func_regex = Regex::new(
            r"(?m)^(?:\s*pub\s+)?(?:async\s+)?fn\s+(\w+).*?\{((?:[^{}]|{[^}]*})*)\}"
        ).unwrap();
        
        for cap in func_regex.captures_iter(content) {
            let func_name = &cap[1];
            let func_body = &cap[2];
            let func_lines = func_body.lines().count();
            
            if func_lines > 200 {
                // Find the line number of the function start
                let func_start = cap.get(0).unwrap().start();
                let line_number = content[..func_start].lines().count() + 1;
                
                violations.push(QualityViolation {
                    crate_name: crate_name.to_string(),
                    file_path: file_path.strip_prefix(&self.workspace_root)
                        .unwrap_or(file_path)
                        .to_string_lossy()
                        .to_string(),
                    violation_type: ViolationType::FunctionTooLong,
                    line_number,
                    severity: "major".to_string(),
                    details: format!("Function '{}' is {} lines (limit: 200)", func_name, func_lines),
                    detected_at: Utc::now(),
                });
            }
        }
        
        Ok(violations)
    }
    
    fn calculate_max_nesting(&self, content: &str) -> usize {
        let mut max_depth = 0;
        let mut current_depth = 0;
        
        for line in content.lines() {
            for ch in line.chars() {
                match ch {
                    '{' => {
                        current_depth += 1;
                        max_depth = max_depth.max(current_depth);
                    }
                    '}' => {
                        current_depth = current_depth.saturating_sub(1);
                    }
                    _ => {}
                }
            }
        }
        
        max_depth
    }
    
    fn detect_model_generation(&self, content: &str) -> f64 {
        let mut score = 0.0;
        
        // Model generation indicator patterns
        let model_patterns = [
            (r"// This function", 0.1),
            (r"// Here we", 0.1),
            (r"// First, let's", 0.1),
            (r"// Now we need to", 0.1),
            (r#"\.expect\(".*"\)"#, 0.05),
            (r#"panic!\(".*should never happen.*"\)"#, 0.2),
            (r#"todo!\(".*implement.*"\)"#, 0.1),
            (r#"println!\(".*Debug.*"\)"#, 0.1),
        ];
        
        for (pattern, weight) in &model_patterns {
            let regex = Regex::new(pattern).unwrap();
            let matches = regex.find_iter(content).count() as f64;
            score += matches * weight;
        }
        
        // Structural indicators
        let clone_count = Regex::new(r"\.clone\(\)").unwrap().find_iter(content).count();
        if clone_count > 5 {
            score += 0.2;
        }
        
        let unwrap_count = Regex::new(r"\.unwrap\(\)").unwrap().find_iter(content).count();
        if unwrap_count > 3 {
            score += 0.2;
        }
        
        // Verbose naming patterns
        let verbose_names = Regex::new(r"\b\w{20,}\b").unwrap().find_iter(content).count();
        if verbose_names > 10 {
            score += 0.3;
        }
        
        score.min(1.0)
    }
    
    fn detect_duplicates_in_file(&self, content: &str) -> usize {
        let lines: Vec<&str> = content.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with("//"))
            .collect();
        
        let mut duplicates = 0;
        let mut seen_blocks = HashMap::new();
        
        // Check for duplicate blocks of 3+ lines
        for window in lines.windows(3) {
            let block = window.join("\n");
            let hash = {
                let mut hasher = Sha256::new();
                hasher.update(block.as_bytes());
                hex::encode(hasher.finalize())
            };
            
            *seen_blocks.entry(hash).or_insert(0) += 1;
        }
        
        // Count blocks that appear more than once
        for count in seen_blocks.values() {
            if *count > 1 {
                duplicates += 1;
            }
        }
        
        duplicates
    }
    
    fn calculate_quality_score(
        &self,
        total_loc: usize,
        total_violations: usize,
        functions_over_200: usize,
        lines_over_90: usize,
        nested_violations: usize,
        duplicate_blocks: usize,
    ) -> f64 {
        let mut score = 100.0;
        
        // Penalize violations
        score -= functions_over_200 as f64 * 5.0;    // -5 per long function
        score -= lines_over_90 as f64 * 0.1;         // -0.1 per wide line
        score -= nested_violations as f64 * 3.0;     // -3 per deeply nested function
        score -= duplicate_blocks as f64 * 2.0;      // -2 per duplicate block
        
        // Don't go below 0
        score.max(0.0)
    }
    
    async fn track_file_write(
        &mut self,
        crate_name: &str,
        file_path: &Path,
        loc_count: usize,
        likely_model_generated: bool,
    ) -> Result<()> {
        let author_type = if likely_model_generated { "model" } else { "unknown" };
        
        self.database.track_file_write(
            crate_name,
            &file_path.to_string_lossy(),
            loc_count,
            author_type,
        ).await?;
        
        Ok(())
    }
    
    pub async fn generate_leaderboard(&mut self, metrics: &[CrateMetrics]) -> Result<Leaderboard> {
        Leaderboard::generate(metrics, &mut self.database).await
    }
}

// src/violations.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityViolation {
    pub crate_name: String,
    pub file_path: String,
    pub violation_type: ViolationType,
    pub line_number: usize,
    pub severity: String,
    pub details: String,
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationType {
    LineTooLong,
    FunctionTooLong,
    VerboseFunctionNames,
    ExcessiveParameters,
    DeepNesting,
    RepetitiveCloning,
    VerboseVariables,
    NonCanonicalPaths,
    ExcessiveUnwraps,
    PrintlnSpam,
    DuplicateCode,
}

impl ViolationType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "line_too_long" => ViolationType::LineTooLong,
            "function_too_long" => ViolationType::FunctionTooLong,
            "verbose_function_names" => ViolationType::VerboseFunctionNames,
            "excessive_parameters" => ViolationType::ExcessiveParameters,
            "deep_nesting" => ViolationType::DeepNesting,
            "repetitive_cloning" => ViolationType::RepetitiveCloning,
            "verbose_variables" => ViolationType::VerboseVariables,
            "non_canonical_paths" => ViolationType::NonCanonicalPaths,
            "excessive_unwraps" => ViolationType::ExcessiveUnwraps,
            "println_spam" => ViolationType::PrintlnSpam,
            _ => ViolationType::DuplicateCode,
        }
    }
}

// src/database.rs
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params, OptionalExtension};
use std::path::Path;
use tokio::task;

use crate::monitor::CrateMetrics;
use crate::violations::QualityViolation;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub async fn new(db_path: &Path) -> Result<Self> {
        let connection = Connection::open(db_path)
            .context("Failed to open SQLite database")?;
        
        let db = Self { connection };
        db.init_tables().await?;
        Ok(db)
    }
    
    async fn init_tables(&self) -> Result<()> {
        task::block_in_place(|| {
            self.connection.execute(
                r#"
                CREATE TABLE IF NOT EXISTS crate_metrics (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    crate_name TEXT NOT NULL,
                    timestamp TEXT NOT NULL,
                    total_loc INTEGER,
                    rust_files INTEGER,
                    violations INTEGER,
                    quality_score REAL,
                    metrics_json TEXT
                )
                "#,
                [],
            )?;
            
            self.connection.execute(
                r#"
                CREATE TABLE IF NOT EXISTS violations (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    crate_name TEXT NOT NULL,
                    file_path TEXT NOT NULL,
                    violation_type TEXT NOT NULL,
                    line_number INTEGER,
                    severity TEXT,
                    details TEXT,
                    detected_at TEXT
                )
                "#,
                [],
            )?;
            
            self.connection.execute(
                r#"
                CREATE TABLE IF NOT EXISTS file_writes (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    crate_name TEXT NOT NULL,
                    file_path TEXT NOT NULL,
                    loc_count INTEGER,
                    write_timestamp TEXT,
                    author_type TEXT
                )
                "#,
                [],
            )?;
            
            Ok::<(), anyhow::Error>(())
        })?;
        
        Ok(())
    }
    
    pub async fn store_metrics(&self, metrics: &CrateMetrics) -> Result<()> {
        let metrics_json = serde_json::to_string(metrics)?;
        let timestamp = Utc::now().to_rfc3339();
        
        task::block_in_place(|| {
            self.connection.execute(
                r#"
                INSERT INTO crate_metrics (
                    crate_name, timestamp, total_loc, rust_files, 
                    violations, quality_score, metrics_json
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                "#,
                params![
                    metrics.name,
                    timestamp,
                    metrics.total_loc as i64,
                    metrics.rust_files as i64,
                    metrics.violations.len() as i64,
                    metrics.quality_score,
                    metrics_json
                ],
            )?;
            
            // Store individual violations
            for violation in &metrics.violations {
                self.connection.execute(
                    r#"
                    INSERT INTO violations (
                        crate_name, file_path, violation_type, line_number,
                        severity, details, detected_at
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                    "#,
                    params![
                        violation.crate_name,
                        violation.file_path,
                        format!("{:?}", violation.violation_type),
                        violation.line_number as i64,
                        violation.severity,
                        violation.details,
                        violation.detected_at.to_rfc3339()
                    ],
                )?;
            }
            
            Ok::<(), anyhow::Error>(())
        })?;
        
        Ok(())
    }
    
    pub async fn calculate_loc_trend(&self, crate_name: &str, days: i64) -> Result<i32> {
        task::block_in_place(|| {
            let cutoff = (Utc::now() - chrono::Duration::days(days)).to_rfc3339();
            
            let result: Option<(i64, i64)> = self.connection.query_row(
                r#"
                SELECT 
                    (SELECT total_loc FROM crate_metrics 
                     WHERE crate_name = ?1 
                     ORDER BY timestamp DESC LIMIT 1) as current_loc,
                    (SELECT total_loc FROM crate_metrics 
                     WHERE crate_name = ?1 AND timestamp < ?2
                     ORDER BY timestamp DESC LIMIT 1) as old_loc
                "#,
                params![crate_name, cutoff],
                |row| Ok((row.get(0)?, row.get(1)?)),
            ).optional()?;
            
            match result {
                Some((current, old)) => Ok((current - old) as i32),
                None => Ok(0),
            }
        })
    }
    
    pub async fn calculate_violation_trend(&self, crate_name: &str, days: i64) -> Result<i32> {
        task::block_in_place(|| {
            let cutoff = (Utc::now() - chrono::Duration::days(days)).to_rfc3339();
            
            let result: Option<(i64, i64)> = self.connection.query_row(
                r#"
                SELECT 
                    (SELECT violations FROM crate_metrics 
                     WHERE crate_name = ?1 
                     ORDER BY timestamp DESC LIMIT 1) as current_violations,
                    (SELECT violations FROM crate_metrics 
                     WHERE crate_name = ?1 AND timestamp < ?2
                     ORDER BY timestamp DESC LIMIT 1) as old_violations
                "#,
                params![crate_name, cutoff],
                |row| Ok((row.get(0)?, row.get(1)?)),
            ).optional()?;
            
            match result {
                Some((current, old)) => Ok((current - old) as i32),
                None => Ok(0),
            }
        })
    }
    
    pub async fn get_last_write_time(&self, crate_name: &str) -> Result<DateTime<Utc>> {
        task::block_in_place(|| {
            let timestamp: Option<String> = self.connection.query_row(
                "SELECT write_timestamp FROM file_writes WHERE crate_name = ?1 ORDER BY write_timestamp DESC LIMIT 1",
                params![crate_name],
                |row| row.get(0),
            ).optional()?;
            
            match timestamp {
                Some(ts) => DateTime::parse_from_rfc3339(&ts)
                    .map(|dt| dt.with_timezone(&Utc))
                    .context("Failed to parse timestamp"),
                None => Ok(Utc::now()),
            }
        })
    }
    
    pub async fn track_file_write(
        &self,
        crate_name: &str,
        file_path: &str,
        loc_count: usize,
        author_type: &str,
    ) -> Result<()> {
        let timestamp = Utc::now().to_rfc3339();
        
        task::block_in_place(|| {
            self.connection.execute(
                r#"
                INSERT INTO file_writes (
                    crate_name, file_path, loc_count, write_timestamp, author_type
                ) VALUES (?1, ?2, ?3, ?4, ?5)
                "#,
                params![crate_name, file_path, loc_count as i64, timestamp, author_type],
            )?;
            
            Ok::<(), anyhow::Error>(())
        })?;
        
        Ok(())
    }
}

// src/leaderboard.rs
use anyhow::Result;
use colored::*;
use std::collections::HashMap;
use std::path::Path;
use tabled::{Table, Tabled};

use crate::database::Database;
use crate::monitor::CrateMetrics;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Leaderboard {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub total_crates: usize,
    pub hall_of_shame: Vec<ShameEntry>,
    pub quality_leaders: Vec<LeaderEntry>,
    pub trends: TrendData,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Tabled)]
pub struct ShameEntry {
    #[tabled(rename = "Crate")]
    pub crate_name: String,
    #[tabled(rename = "Score")]
    pub quality_score: String,
    #[tabled(rename = "LOC")]
    pub total_loc: String,
    #[tabled(rename = "Violations")]
    pub violation_count: String,
    #[tabled(rename = "Model Score")]
    pub model_generation_score: String,
    #[tabled(rename = "Trend")]
    pub trend: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Tabled)]
pub struct LeaderEntry {
    #[tabled(rename = "Crate")]
    pub crate_name: String,
    #[tabled(rename = "Score")]
    pub quality_score: String,
    #[tabled(rename = "LOC")]
    pub total_loc: String,
    #[tabled(rename = "Violations")]
    pub violation_count: String,
    #[tabled(rename = "Trend")]
    pub trend: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TrendData {
    pub total_loc: usize,
    pub total_violations: usize,
    pub avg_quality_score: f64,
}

impl Leaderboard {
    pub async fn generate(metrics: &[CrateMetrics], _database: &mut Database) -> Result<Self> {
        let mut sorted_metrics = metrics.to_vec();
        sorted_metrics.sort_by(|a, b| a.quality_score.partial_cmp(&b.quality_score).unwrap());
        
        // Hall of Shame (worst performers)
        let hall_of_shame: Vec<ShameEntry> = sorted_metrics
            .iter()
            .filter(|m| m.quality_score < 50.0)
            .take(10)
            .map(|m| ShameEntry {
                crate_name: m.name.clone(),
                quality_score: format!("{:.1}", m.quality_score),
                total_loc: m.total_loc.to_string(),
                violation_count: m.violations.len().to_string(),
                model_generation_score: format!("{:.2}", m.model_generation_score),
                trend: if m.violation_trend_7d > 0 { "📈".to_string() } 
                       else if m.violation_trend_7d < 0 { "📉".to_string() } 
                       else { "➡️".to_string() },
            })
            .collect();
        
        // Quality Leaders (best performers)
        let quality_leaders: Vec<LeaderEntry> = sorted_metrics
            .iter()
            .rev()
            .filter(|m| m.quality_score > 80.0)
            .take(5)
            .map(|m| LeaderEntry {
                crate_name: m.name.clone(),
                quality_score: format!("{:.1}", m.quality_score),
                total_loc: m.total_loc.to_string(),
                violation_count: m.violations.len().to_string(),
                trend: if m.loc_trend_7d > 0 { "📈".to_string() } 
                       else if m.loc_trend_7d < 0 { "📉".to_string() } 
                       else { "➡️".to_string() },
            })
            .collect();
        
        let trends = TrendData {
            total_loc: metrics.iter().map(|m| m.total_loc).sum(),
            total_violations: metrics.iter().map(|m| m.violations.len()).sum(),
            avg_quality_score: metrics.iter().map(|m| m.quality_score).sum::<f64>() / metrics.len() as f64,
        };
        
        Ok(Leaderboard {
            timestamp: chrono::Utc::now(),
            total_crates: metrics.len(),
            hall_of_shame,
            quality_leaders,
            trends,
        })
    }
    
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
    
    pub fn print_summary(&self) {
        println!("\n{}", "🏆 QUALITY LEADERBOARD".bright_yellow().bold());
        println!("{}", "=".repeat(50).bright_yellow());
        
        if !self.hall_of_shame.is_empty() {
            println!("\n{}", "😱 HALL OF SHAME (Needs Immediate Attention)".bright_red().bold());
            let table = Table::new(&self.hall_of_shame);
            println!("{}", table.to_string().red());
        }
        
        if !self.quality_leaders.is_empty() {
            println!("\n{}", "⭐ QUALITY LEADERS".bright_green().bold());
            let table = Table::new(&self.quality_leaders);
            println!("{}", table.to_string().green());
        }
        
        println!("\n{}", "📊 OVERALL TRENDS".bright_blue().bold());
        println!("Total LOC: {}", self.trends.total_loc.to_string().bright_white());
        println!("Total Violations: {}", self.trends.total_violations.to_string().bright_red());
        println!("Average Quality Score: {:.1}", self.trends.avg_quality_score);
    }
}

// src/context.rs
use anyhow::{Context as AnyhowContext, Result};
use chrono::{DateTime, Utc};
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextSnapshot {
    pub timestamp: DateTime<Utc>,
    pub snapshot_id: String,
    pub original_size: usize,
    pub compressed_size: usize,
    pub compression_ratio: f64,
    pub context_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceContext {
    pub workspace_info: WorkspaceInfo,
    pub crates: HashMap<String, CrateInfo>,
    pub files: HashMap<String, FileInfo>,
    pub configurations: HashMap<String, ConfigInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    pub root: String,
    pub scan_time: DateTime<Utc>,
    pub total_files: usize,
    pub git_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrateInfo {
    pub path: String,
    pub rust_files: usize,
    pub total_loc: usize,
    pub last_modified: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub size: u64,
    pub modified: f64,
    pub hash: String,
    pub crate_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigInfo {
    pub hash: String,
    pub modified: f64,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompactionReport {
    pub timestamp: DateTime<Utc>,
    pub original_size: usize,
    pub compressed_size: usize,
    pub compression_ratio: f64,
    pub files_processed: usize,
    pub crates_processed: usize,
}

pub struct ContextManager {
    workspace_root: PathBuf,
    folder_path: PathBuf,
}

impl ContextManager {
    pub fn new(workspace_root: PathBuf) -> Self {
        let folder_path = workspace_root.join(".all-llm-folder");
        Self {
            workspace_root,
            folder_path,
        }
    }
    
    pub async fn compact_context(&self) -> Result<CompactionReport> {
        self.ensure_structure()?;
        
        // Gather current workspace context
        let context = self.gather_workspace_context().await?;
        
        // Create compressed context
        let original_json = serde_json::to_string_pretty(&context)?;
        let original_size = original_json.len();
        
        // Compress and save
        let context_file = self.folder_path.join("context").join("current.json.gz");
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(original_json.as_bytes())?;
        let compressed = encoder.finish()?;
        let compressed_size = compressed.len();
        
        fs::write(&context_file, &compressed)?;
        
        // Create snapshot
        let snapshot = self.create_snapshot(&context, original_size, compressed_size)?;
        
        // Generate summary
        let summary = self.generate_summary(&context)?;
        let summary_file = self.folder_path.join("context").join("summary.md");
        fs::write(summary_file, summary)?;
        
        // Update standards
        self.update_standards()?;
        
        Ok(CompactionReport {
            timestamp: Utc::now(),
            original_size,
            compressed_size,
            compression_ratio: original_size as f64 / compressed_size as f64,
            files_processed: context.files.len(),
            crates_processed: context.crates.len(),
        })
    }
    
    fn ensure_structure(&self) -> Result<()> {
        let subdirs = ["context", "systems", "standards", "snapshots", "metadata"];
        
        for subdir in &subdirs {
            fs::create_dir_all(self.folder_path.join(subdir))?;
        }
        
        // Create manifest if it doesn't exist
        let manifest_path = self.folder_path.join("MANIFEST.json");
        if !manifest_path.exists() {
            let manifest = serde_json::json!({
                "version": "1.0.0",
                "created": Utc::now(),
                "description": "Universal LLM context folder",
                "schema_version": "2024.1",
                "compaction_strategy": "hierarchical_lossless"
            });
            fs::write(manifest_path, serde_json::to_string_pretty(&manifest)?)?;
        }
        
        Ok(())
    }
    
    async fn gather_workspace_context(&self) -> Result<WorkspaceContext> {
        let mut crates = HashMap::new();
        let mut files = HashMap::new();
        let mut configurations = HashMap::new();
        
        // Scan for Cargo.toml files (crates)
        for entry in WalkDir::new(&self.workspace_root)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| !e.path().to_string_lossy().contains("target/"))
            .filter_entry(|e| !e.path().to_string_lossy().contains(".all-llm-folder"))
        {
            let entry = entry?;
            
            if entry.file_name() == "Cargo.toml" {
                let crate_dir = entry.path().parent().unwrap();
                let crate_name = crate_dir.file_name().unwrap().to_string_lossy().to_string();
                
                let rust_files: Vec<_> = WalkDir::new(crate_dir)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
                    .collect();
                
                let total_loc = rust_files
                    .iter()
                    .map(|f| {
                        fs::read_to_string(f.path())
                            .map(|content| content.lines().count())
                            .unwrap_or(0)
                    })
                    .sum();
                
                let last_modified = rust_files
                    .iter()
                    .map(|f| f.metadata().unwrap().modified().unwrap())
                    .max()
                    .map(|time| time.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64())
                    .unwrap_or(0.0);
                
                crates.insert(crate_name.clone(), CrateInfo {
                    path: crate_dir.strip_prefix(&self.workspace_root)?.to_string_lossy().to_string(),
                    rust_files: rust_files.len(),
                    total_loc,
                    last_modified,
                });
                
                // Add individual files
                for rust_file in rust_files {
                    let metadata = rust_file.metadata()?;
                    let content = fs::read_to_string(rust_file.path())?;
                    let hash = {
                        let mut hasher = Sha256::new();
                        hasher.update(content.as_bytes());
                        hex::encode(hasher.finalize())
                    };
                    
                    files.insert(
                        rust_file.path().strip_prefix(&self.workspace_root)?.to_string_lossy().to_string(),
                        FileInfo {
                            size: metadata.len(),
                            modified: metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs_f64(),
                            hash,
                            crate_name: crate_name.clone(),
                        }
                    );
                }
            }
        }
        
        // Scan for configuration files
        let config_patterns = ["*.toml", "*.yaml", "*.json"];
        for pattern in &config_patterns {
            for entry in glob::glob(&format!("{}/**/{}", self.workspace_root.display(), pattern))? {
                let path = entry?;
                if path.to_string_lossy().contains(".all-llm-folder") || 
                   path.to_string_lossy().contains("target/") {
                    continue;
                }
                
                let metadata = path.metadata()?;
                let content = fs::read_to_string(&path)?;
                let hash = {
                    let mut hasher = Sha256::new();
                    hasher.update(content.as_bytes());
                    hex::encode(hasher.finalize())
                };
                
                configurations.insert(
                    path.strip_prefix(&self.workspace_root)?.to_string_lossy().to_string(),
                    ConfigInfo {
                        hash,
                        modified: metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs_f64(),
                        size: metadata.len(),
                    }
                );
            }
        }
        
        Ok(WorkspaceContext {
            workspace_info: WorkspaceInfo {
                root: self.workspace_root.to_string_lossy().to_string(),
                scan_time: Utc::now(),
                total_files: files.len() + configurations.len(),
                git_hash: self.get_git_hash(),
            },
            crates,
            files,
            configurations,
        })
    }
    
    fn get_git_hash(&self) -> Option<String> {
        std::process::Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .current_dir(&self.workspace_root)
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
                } else {
                    None
                }
            })
    }
    
    fn create_snapshot(&self, context: &WorkspaceContext, original_size: usize, compressed_size: usize) -> Result<ContextSnapshot> {
        let snapshot_id = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let context_json = serde_json::to_string(context)?;
        let context_hash = {
            let mut hasher = Sha256::new();
            hasher.update(context_json.as_bytes());
            hex::encode(hasher.finalize())
        };
        
        let snapshot = ContextSnapshot {
            timestamp: Utc::now(),
            snapshot_id: snapshot_id.clone(),
            original_size,
            compressed_size,
            compression_ratio: original_size as f64 / compressed_size as f64,
            context_hash,
        };
        
        // Save snapshot metadata
        let metadata_file = self.folder_path.join("metadata").join(format!("snapshot_{}.json", snapshot_id));
        fs::write(metadata_file, serde_json::to_string_pretty(&snapshot)?)?;
        
        Ok(snapshot)
    }
    
    fn generate_summary(&self, context: &WorkspaceContext) -> Result<String> {
        let mut summary = format!(
            r#"# Workspace Context Summary

Generated: {}

## Overview
- **Total Crates:** {}
- **Total Files:** {}
- **Total LOC:** {}
- **Git Hash:** {}

## Crates
"#,
            Utc::now().format("%Y-%m-%d %H:%M:%S"),
            context.crates.len(),
            context.workspace_info.total_files,
            context.crates.values().map(|c| c.total_loc).sum::<usize>(),
            context.workspace_info.git_hash.as_deref().unwrap_or("N/A")
        );
        
        for (crate_name, crate_info) in &context.crates {
            summary.push_str(&format!(
                "- **{}**: {:,} LOC, {} files\n",
                crate_name, crate_info.total_loc, crate_info.rust_files
            ));
        }
        
        Ok(summary)
    }
    
    fn update_standards(&self) -> Result<()> {
        let standards = serde_json::json!({
            "rust_standards": {
                "version": "1.0.0",
                "updated": Utc::now(),
                "rules": [
                    {
                        "name": "function_length_limit",
                        "description": "Functions must not exceed 200 lines",
                        "severity": "error"
                    },
                    {
                        "name": "line_length_limit",
                        "description": "Lines must not exceed 90 characters",
                        "severity": "warning"
                    },
                    {
                        "name": "canonical_naming",
                        "description": "Use short, canonical names",
                        "severity": "warning"
                    },
                    {
                        "name": "avoid_deep_nesting",
                        "description": "Avoid nesting deeper than 4 levels",
                        "severity": "error"
                    }
                ]
            }
        });
        
        let standards_file = self.folder_path.join("standards").join("rust.json");
        fs::write(standards_file, serde_json::to_string_pretty(&standards)?)?;
        
        Ok(())
    }
}
