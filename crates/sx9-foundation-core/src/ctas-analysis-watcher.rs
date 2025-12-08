#!/usr/bin/env rust-script
//! CTAS-7 Analysis Drop Zone Watcher
//! Monitors directories for new crates, auto-unzips, and triggers analysis

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, SystemTime};
use std::collections::HashMap;

const ANALYZER_PATH: &str = "./target/release/ctas7-phd-analyzer";
const POST_STATS_PATH: &str = "./target/release/post_stats";
const CDN_ENDPOINT: &str = "http://localhost:18108/analysis/ingest";
const WATCH_INTERVAL: Duration = Duration::from_secs(2);

struct AnalysisWatcher {
    drop_zones: Vec<PathBuf>,
    processed_files: HashMap<PathBuf, SystemTime>,
    auto_unzip: bool,
}

impl AnalysisWatcher {
    fn new(auto_unzip: bool) -> Self {
        Self {
            drop_zones: vec![
                PathBuf::from("./drop-zone"),
                PathBuf::from("../analysis-drop"),
                PathBuf::from("./git-hooks/analysis-queue"),
            ],
            processed_files: HashMap::new(),
            auto_unzip,
        }
    }

    fn setup_drop_zones(&self) -> std::io::Result<()> {
        for zone in &self.drop_zones {
            fs::create_dir_all(zone)?;
            println!("📂 Drop zone ready: {}", zone.display());
        }

        // Create README for each drop zone
        for zone in &self.drop_zones {
            let readme = zone.join("README.md");
            if !readme.exists() {
                fs::write(&readme, format!(r#"# CTAS-7 Analysis Drop Zone

This directory automatically monitors for:
- `.zip` files containing Rust crates (auto-extracted)
- Rust project directories
- Individual `.rs` files

## Usage:
1. **Drop zip files**: Will be auto-extracted and analyzed
2. **Drop directories**: Will be recursively analyzed
3. **Drop .rs files**: Will be analyzed individually

## Analysis Output:
- Cyclomatic complexity
- Halstead metrics
- Maintainability Index
- Documentation density
- Automatic posting to Statistical CDN: {}

## Auto-processed: {}

Drop zone: `{}`
"#, CDN_ENDPOINT, self.auto_unzip, zone.display()))?;
            }
        }
        Ok(())
    }

    fn watch(&mut self) -> std::io::Result<()> {
        println!("🔍 Starting CTAS-7 analysis watcher...");
        println!("📊 CDN endpoint: {}", CDN_ENDPOINT);
        println!("🔄 Auto-unzip: {}", self.auto_unzip);

        self.setup_drop_zones()?;

        loop {
            for zone in &self.drop_zones.clone() {
                if let Err(e) = self.scan_zone(zone) {
                    eprintln!("⚠️ Error scanning {}: {}", zone.display(), e);
                }
            }
            thread::sleep(WATCH_INTERVAL);
        }
    }

    fn scan_zone(&mut self, zone: &Path) -> std::io::Result<()> {
        if !zone.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(zone)? {
            let entry = entry?;
            let path = entry.path();

            // Skip README and hidden files
            if path.file_name().unwrap_or_default().to_string_lossy().starts_with('.') ||
               path.file_name().unwrap_or_default() == "README.md" {
                continue;
            }

            let modified = entry.metadata()?.modified()?;

            // Check if this file is new or modified
            match self.processed_files.get(&path) {
                Some(last_modified) if *last_modified >= modified => continue,
                _ => {}
            }

            println!("🆕 New item detected: {}", path.display());

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    match ext.to_string_lossy().as_ref() {
                        "zip" => {
                            if self.auto_unzip {
                                if let Err(e) = self.unzip_and_analyze(&path) {
                                    eprintln!("❌ Failed to unzip {}: {}", path.display(), e);
                                }
                            } else {
                                println!("📦 Zip file detected but auto-unzip disabled: {}", path.display());
                            }
                        },
                        "rs" => {
                            if let Err(e) = self.analyze_file(&path) {
                                eprintln!("❌ Failed to analyze {}: {}", path.display(), e);
                            }
                        },
                        _ => {
                            println!("⏭️ Skipping unsupported file: {}", path.display());
                        }
                    }
                }
            } else if path.is_dir() {
                if let Err(e) = self.analyze_directory(&path) {
                    eprintln!("❌ Failed to analyze directory {}: {}", path.display(), e);
                }
            }

            self.processed_files.insert(path, modified);
        }

        Ok(())
    }

    fn unzip_and_analyze(&self, zip_path: &Path) -> std::io::Result<()> {
        println!("📦 Extracting: {}", zip_path.display());

        let extract_dir = zip_path.parent().unwrap().join(
            zip_path.file_stem().unwrap()
        );

        // Create extraction directory
        fs::create_dir_all(&extract_dir)?;

        // Extract zip
        let output = Command::new("unzip")
            .args(&["-q", "-o"])
            .arg(zip_path)
            .arg("-d")
            .arg(&extract_dir)
            .output()?;

        if !output.status.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Unzip failed: {}", String::from_utf8_lossy(&output.stderr))
            ));
        }

        println!("✅ Extracted to: {}", extract_dir.display());

        // Analyze the extracted content
        self.analyze_directory(&extract_dir)?;

        // Clean up zip file
        fs::remove_file(zip_path)?;
        println!("🗑️ Cleaned up zip: {}", zip_path.display());

        Ok(())
    }

    fn analyze_file(&self, file_path: &Path) -> std::io::Result<()> {
        println!("🔬 Analyzing file: {}", file_path.display());
        self.run_analysis(file_path, "single-file")
    }

    fn analyze_directory(&self, dir_path: &Path) -> std::io::Result<()> {
        println!("🔬 Analyzing directory: {}", dir_path.display());
        self.run_analysis(dir_path, "directory")
    }

    fn run_analysis(&self, target: &Path, analysis_type: &str) -> std::io::Result<()> {
        // Generate unique tag for this analysis
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let tag = format!("{}-{}", analysis_type, timestamp);

        println!("📊 Running PhD-level analysis...");

        // Run analyzer and pipe to CDN poster
        let analyzer = Command::new(ANALYZER_PATH)
            .args(&[target.to_string_lossy().as_ref(), "--json"])
            .stdout(Stdio::piped())
            .spawn()?;

        let cdn_poster = Command::new(POST_STATS_PATH)
            .args(&[
                "--url", CDN_ENDPOINT,
                "--source", "ctas7-drop-zone",
                "--tag", &tag
            ])
            .stdin(analyzer.stdout.unwrap())
            .output()?;

        if cdn_poster.status.success() {
            println!("✅ Analysis complete and posted to CDN");
            println!("🏷️ Tag: {}", tag);
        } else {
            eprintln!("⚠️ CDN posting failed: {}", String::from_utf8_lossy(&cdn_poster.stderr));
        }

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let auto_unzip = std::env::args().any(|arg| arg == "--auto-unzip");

    println!("🎯 CTAS-7 Analysis Drop Zone Watcher");
    println!("═══════════════════════════════════");

    let mut watcher = AnalysisWatcher::new(auto_unzip);
    watcher.watch()
}