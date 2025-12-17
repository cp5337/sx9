//! CTAS-7 Authoritative Bloat Monitor
//! Real-time line count monitoring with automated fix suggestions

use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

/// Authoritative line count metrics
#[derive(Debug, Serialize, Deserialize)]
struct LineCountMetrics {
    total_files: usize,
    total_lines: usize,
    code_lines: usize,
    comment_lines: usize,
    empty_lines: usize,
    function_count: usize,
    complexity_points: usize,
    bloat_score: f64,
}

/// File-level bloat analysis
#[derive(Debug, Serialize, Deserialize)]
struct FileBloatAnalysis {
    path: String,
    lines: usize,
    functions: usize,
    complexity: usize,
    bloat_level: BloatLevel,
    suggested_fixes: Vec<String>,
}

/// Bloat severity levels
#[derive(Debug, Serialize, Deserialize)]
enum BloatLevel {
    Clean,      // <100 lines
    Acceptable, // 100-300 lines
    Warning,    // 300-500 lines
    Critical,   // 500-1000 lines
    Emergency,  // >1000 lines
}

/// Mid-composition analysis hook
#[derive(Debug)]
struct CompositionHook {
    threshold_lines: usize,
    threshold_functions: usize,
    auto_fix_enabled: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📏 CTAS-7 AUTHORITATIVE BLOAT MONITOR");
    println!("====================================");

    let target = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let auto_fix = std::env::args().nth(2).map(|s| s == "--auto-fix").unwrap_or(false);

    // Analyze current state
    let metrics = analyze_codebase(&target)?;
    let file_analysis = analyze_individual_files(&target)?;

    print_metrics(&metrics);
    print_file_analysis(&file_analysis);

    // Check for bloat patterns
    let bloat_patterns = detect_bloat_patterns(&file_analysis);
    if !bloat_patterns.is_empty() {
        print_bloat_patterns(&bloat_patterns);

        if auto_fix {
            apply_automated_fixes(&bloat_patterns)?;
        }
    }

    // Set up mid-composition monitoring
    setup_composition_hooks(&target)?;

    Ok(())
}

fn analyze_codebase(root: &str) -> Result<LineCountMetrics, Box<dyn std::error::Error>> {
    let mut metrics = LineCountMetrics {
        total_files: 0,
        total_lines: 0,
        code_lines: 0,
        comment_lines: 0,
        empty_lines: 0,
        function_count: 0,
        complexity_points: 0,
        bloat_score: 0.0,
    };

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        .filter(|e| !e.path().to_string_lossy().contains("target/"))
        .filter(|e| !e.path().to_string_lossy().contains("analysis-archive/"))
        .filter(|e| !e.path().to_string_lossy().contains(".sandbox/"))
    {
        let content = fs::read_to_string(entry.path())?;
        analyze_file_content(&content, &mut metrics);
        metrics.total_files += 1;
    }

    // Calculate bloat score (higher = more bloated)
    metrics.bloat_score = calculate_bloat_score(&metrics);

    Ok(metrics)
}

fn analyze_file_content(content: &str, metrics: &mut LineCountMetrics) {
    for line in content.lines() {
        let trimmed = line.trim();
        metrics.total_lines += 1;

        if trimmed.is_empty() {
            metrics.empty_lines += 1;
        } else if trimmed.starts_with("//") || trimmed.starts_with("/*") {
            metrics.comment_lines += 1;
        } else {
            metrics.code_lines += 1;

            // Function detection
            if (trimmed.contains("fn ") || trimmed.contains("async fn ")) &&
               !trimmed.starts_with("//") && trimmed.contains("(") {
                metrics.function_count += 1;
            }

            // Complexity indicators
            if trimmed.contains("if ") || trimmed.contains("match ") ||
               trimmed.contains("for ") || trimmed.contains("while ") ||
               trimmed.contains("loop ") {
                metrics.complexity_points += 1;
            }
        }
    }
}

fn analyze_individual_files(root: &str) -> Result<Vec<FileBloatAnalysis>, Box<dyn std::error::Error>> {
    let mut analyses = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        .filter(|e| !e.path().to_string_lossy().contains("target/"))
        .filter(|e| !e.path().to_string_lossy().contains("analysis-archive/"))
        .filter(|e| !e.path().to_string_lossy().contains(".sandbox/"))
    {
        let content = fs::read_to_string(entry.path())?;
        let analysis = analyze_single_file(&content, entry.path().to_string_lossy().as_ref());
        analyses.push(analysis);
    }

    // Sort by bloat level (most bloated first)
    analyses.sort_by(|a, b| b.lines.cmp(&a.lines));

    Ok(analyses)
}

fn analyze_single_file(content: &str, path: &str) -> FileBloatAnalysis {
    let lines = content.lines().count();
    let mut functions = 0;
    let mut complexity = 0;

    for line in content.lines() {
        let trimmed = line.trim();

        if (trimmed.contains("fn ") || trimmed.contains("async fn ")) &&
           !trimmed.starts_with("//") {
            functions += 1;
        }

        if trimmed.contains("if ") || trimmed.contains("match ") ||
           trimmed.contains("for ") || trimmed.contains("while ") {
            complexity += 1;
        }
    }

    let bloat_level = match lines {
        0..=100 => BloatLevel::Clean,
        101..=300 => BloatLevel::Acceptable,
        301..=500 => BloatLevel::Warning,
        501..=1000 => BloatLevel::Critical,
        _ => BloatLevel::Emergency,
    };

    let suggested_fixes = generate_fix_suggestions(lines, functions, complexity, path);

    FileBloatAnalysis {
        path: path.to_string(),
        lines,
        functions,
        complexity,
        bloat_level,
        suggested_fixes,
    }
}

fn generate_fix_suggestions(lines: usize, functions: usize, complexity: usize, path: &str) -> Vec<String> {
    let mut fixes = Vec::new();

    if lines > 500 {
        fixes.push("CRITICAL: Split into multiple modules".to_string());
    }
    if lines > 300 {
        fixes.push("Extract utility functions to separate module".to_string());
    }
    if functions > 20 {
        fixes.push(format!("Too many functions ({}), consider grouping", functions));
    }
    if complexity > 50 {
        fixes.push(format!("High complexity ({}), refactor control flow", complexity));
    }
    if path.contains("test") && lines > 200 {
        fixes.push("Test file too large, split into multiple test files".to_string());
    }
    if lines > 100 && complexity < 5 {
        fixes.push("Mostly boilerplate, consider code generation".to_string());
    }

    // Pattern-specific fixes
    if path.contains("bin/") && lines > 300 {
        fixes.push("Binary should be thin wrapper, move logic to lib".to_string());
    }

    fixes
}

fn calculate_bloat_score(metrics: &LineCountMetrics) -> f64 {
    let mut score = 0.0;

    // Penalize large total line count
    score += (metrics.total_lines as f64 / 1000.0) * 10.0;

    // Penalize low comment ratio
    let comment_ratio = if metrics.total_lines > 0 {
        metrics.comment_lines as f64 / metrics.total_lines as f64
    } else {
        0.0
    };
    if comment_ratio < 0.1 {
        score += 20.0;
    }

    // Penalize high complexity
    let complexity_ratio = if metrics.code_lines > 0 {
        metrics.complexity_points as f64 / metrics.code_lines as f64
    } else {
        0.0
    };
    score += complexity_ratio * 50.0;

    // Penalize many functions per file
    let functions_per_file = if metrics.total_files > 0 {
        metrics.function_count as f64 / metrics.total_files as f64
    } else {
        0.0
    };
    if functions_per_file > 15.0 {
        score += 15.0;
    }

    score
}

fn detect_bloat_patterns(analyses: &[FileBloatAnalysis]) -> Vec<String> {
    let mut patterns = Vec::new();

    // Check for emergency files
    let emergency_files: Vec<_> = analyses.iter()
        .filter(|a| matches!(a.bloat_level, BloatLevel::Emergency))
        .collect();

    if !emergency_files.is_empty() {
        patterns.push(format!("EMERGENCY: {} files >1000 lines", emergency_files.len()));
    }

    // Check for test bloat
    let bloated_tests: Vec<_> = analyses.iter()
        .filter(|a| a.path.contains("test") && a.lines > 200)
        .collect();

    if !bloated_tests.is_empty() {
        patterns.push(format!("Test bloat: {} test files >200 lines", bloated_tests.len()));
    }

    // Check for bin bloat
    let bloated_bins: Vec<_> = analyses.iter()
        .filter(|a| a.path.contains("bin/") && a.lines > 300)
        .collect();

    if !bloated_bins.is_empty() {
        patterns.push(format!("Binary bloat: {} bin files >300 lines", bloated_bins.len()));
    }

    patterns
}

fn apply_automated_fixes(patterns: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 AUTOMATED FIXES:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    for pattern in patterns {
        if pattern.contains("EMERGENCY:") {
            println!("🚨 EMERGENCY PATTERN: {}", pattern);
            println!("   → Create refactor plan for >1000 line files");
            println!("   → Split into logical modules");
            println!("   → Extract common utilities");
        }

        if pattern.contains("Test bloat:") {
            println!("🧪 TEST BLOAT: {}", pattern);
            println!("   → Split integration tests by feature");
            println!("   → Extract test utilities to common module");
            println!("   → Use test macros for repetitive patterns");
        }

        if pattern.contains("Binary bloat:") {
            println!("📦 BINARY BLOAT: {}", pattern);
            println!("   → Move business logic to lib.rs");
            println!("   → Keep main.rs as thin CLI wrapper");
            println!("   → Extract configuration parsing");
        }
    }

    // Generate refactoring script
    let refactor_script = r#"#!/bin/bash
# CTAS-7 Automated Refactoring Helper
# Generated from bloat analysis

echo "🔧 REFACTORING ASSISTANT"
echo "======================="

# Find emergency files
echo "Emergency files (>1000 lines):"
find . -name "*.rs" -not -path "./target/*" -exec wc -l {} \; | awk '$1 > 1000 {print $2 " (" $1 " lines)"}' | sort -rn

echo ""
echo "Suggested actions:"
echo "1. Split large files into logical modules"
echo "2. Extract utility functions to common crate"
echo "3. Move test helpers to test_utils module"
echo "4. Use macro generation for boilerplate"

echo ""
echo "Tesla-grade targets:"
echo "- Source files: <300 lines"
echo "- Test files: <200 lines"
echo "- Binary files: <100 lines"
echo "- Functions: <50 lines"
"#;

    fs::write("./refactor_helper.sh", refactor_script)?;
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata("./refactor_helper.sh")?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions("./refactor_helper.sh", perms)?;

    println!("✅ Generated: ./refactor_helper.sh");
    println!("🎯 Tesla-grade targets: src <300, tests <200, bins <100 lines");

    Ok(())
}

fn setup_composition_hooks(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 MID-COMPOSITION MONITORING:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Create composition monitoring config
    let hook_config = CompositionHook {
        threshold_lines: 300,
        threshold_functions: 15,
        auto_fix_enabled: false,
    };

    // Write monitoring script
    let monitor_script = format!(r#"#!/bin/bash
# CTAS-7 Mid-Composition Bloat Monitor
# Runs every 30 seconds during development

while true; do
    clear
    echo "🔍 REAL-TIME BLOAT MONITOR"
    echo "=========================="

    # Quick analysis of modified files
    git diff --name-only --cached | grep '\.rs$' | while read file; do
        if [ -f "$file" ]; then
            lines=$(wc -l < "$file")
            if [ $lines -gt {} ]; then
                echo "⚠️  $file: $lines lines (THRESHOLD EXCEEDED)"
            else
                echo "✅ $file: $lines lines"
            fi
        fi
    done

    sleep 30
done
"#, hook_config.threshold_lines);

    let script_path = format!("{}/monitor_composition.sh", target);
    fs::write(&script_path, monitor_script)?;

    // Make executable
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(&script_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script_path, perms)?;

    println!("✅ Monitor script: {}", script_path);
    println!("✅ Threshold: {} lines, {} functions",
        hook_config.threshold_lines, hook_config.threshold_functions);
    println!("Run './monitor_composition.sh' during development");

    Ok(())
}

fn print_metrics(metrics: &LineCountMetrics) {
    println!("\n📊 AUTHORITATIVE METRICS:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Files: {}", metrics.total_files);
    println!("  Total Lines: {}", metrics.total_lines);
    println!("  Code Lines: {}", metrics.code_lines);
    println!("  Comment Lines: {} ({:.1}%)",
        metrics.comment_lines,
        metrics.comment_lines as f64 / metrics.total_lines as f64 * 100.0
    );
    println!("  Empty Lines: {}", metrics.empty_lines);
    println!("  Functions: {}", metrics.function_count);
    println!("  Complexity Points: {}", metrics.complexity_points);
    println!("  Bloat Score: {:.1}", metrics.bloat_score);

    let health = if metrics.bloat_score < 20.0 {
        "🟢 HEALTHY"
    } else if metrics.bloat_score < 50.0 {
        "🟡 WARNING"
    } else {
        "🔴 CRITICAL"
    };
    println!("  Health: {}", health);
}

fn print_file_analysis(analyses: &[FileBloatAnalysis]) {
    println!("\n📋 FILE-LEVEL ANALYSIS:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    for analysis in analyses.iter().take(10) {
        let level_emoji = match analysis.bloat_level {
            BloatLevel::Clean => "🟢",
            BloatLevel::Acceptable => "🟡",
            BloatLevel::Warning => "🟠",
            BloatLevel::Critical => "🔴",
            BloatLevel::Emergency => "🚨",
        };

        println!("{} {} ({} lines, {} functions)",
            level_emoji,
            analysis.path.replace("src/", ""),
            analysis.lines,
            analysis.functions
        );

        if !analysis.suggested_fixes.is_empty() {
            for fix in &analysis.suggested_fixes {
                println!("    💡 {}", fix);
            }
        }
    }
}

fn print_bloat_patterns(patterns: &[String]) {
    println!("\n🚨 BLOAT PATTERNS DETECTED:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    for pattern in patterns {
        println!("⚠️  {}", pattern);
    }
}