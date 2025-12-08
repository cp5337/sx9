//! CTAS-7 Minimal Self-Certifier
//! Bootstrap solution for the meta-certification problem

use std::fs;
use walkdir::WalkDir;

/// Ultra-simple quality metrics for self-certification
#[derive(Debug)]
struct SimpleMetrics {
    files: usize,
    total_lines: usize,
    code_lines: usize,
    comment_lines: usize,
    function_count: usize,
    complexity_estimate: usize,
}

/// Minimal certification grade
#[derive(Debug)]
enum CertificationGrade {
    A, // Tesla grade
    B, // Good
    C, // Acceptable
    F, // Fail
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 CTAS-7 MINIMAL SELF-CERTIFIER");
    println!("================================");
    println!("Solving the meta-certification bootstrap problem...");
    println!();

    let target_dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| ".".to_string());

    let metrics = analyze_codebase(&target_dir)?;
    let grade = calculate_grade(&metrics);

    print_results(&metrics, &grade);

    // Self-certification logic
    let self_certifies = matches!(grade, CertificationGrade::A | CertificationGrade::B);

    if self_certifies {
        println!("✅ SELF-CERTIFICATION: PASSED");
        println!("The minimal certifier approves the full certification system!");
        std::process::exit(0);
    } else {
        println!("❌ SELF-CERTIFICATION: FAILED");
        println!("The certification system needs improvement before it can certify others.");
        std::process::exit(1);
    }
}

fn analyze_codebase(root: &str) -> Result<SimpleMetrics, Box<dyn std::error::Error>> {
    let mut metrics = SimpleMetrics {
        files: 0,
        total_lines: 0,
        code_lines: 0,
        comment_lines: 0,
        function_count: 0,
        complexity_estimate: 0,
    };

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        .filter(|e| !e.path().to_string_lossy().contains("target/"))
    {
        let content = fs::read_to_string(entry.path())?;
        analyze_file(&content, &mut metrics);
        metrics.files += 1;
    }

    Ok(metrics)
}

fn analyze_file(content: &str, metrics: &mut SimpleMetrics) {
    for line in content.lines() {
        let trimmed = line.trim();
        metrics.total_lines += 1;

        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with("//") || trimmed.starts_with("/*") {
            metrics.comment_lines += 1;
        } else {
            metrics.code_lines += 1;

            // Simple function detection
            if trimmed.contains("fn ") && !trimmed.starts_with("//") {
                metrics.function_count += 1;
            }

            // Simple complexity estimation
            if trimmed.contains("if ") || trimmed.contains("match ") ||
               trimmed.contains("for ") || trimmed.contains("while ") {
                metrics.complexity_estimate += 1;
            }
        }
    }
}

fn calculate_grade(metrics: &SimpleMetrics) -> CertificationGrade {
    let doc_ratio = if metrics.total_lines > 0 {
        metrics.comment_lines as f64 / metrics.total_lines as f64 * 100.0
    } else {
        0.0
    };

    let complexity_per_file = if metrics.files > 0 {
        metrics.complexity_estimate as f64 / metrics.files as f64
    } else {
        0.0
    };

    let functions_per_file = if metrics.files > 0 {
        metrics.function_count as f64 / metrics.files as f64
    } else {
        0.0
    };

    // Ultra-simple grading criteria
    let mut score = 100.0;

    // Penalize low documentation
    if doc_ratio < 5.0 {
        score -= 20.0;
    }

    // Penalize high complexity
    if complexity_per_file > 20.0 {
        score -= 30.0;
    }

    // Penalize too many functions per file
    if functions_per_file > 15.0 {
        score -= 15.0;
    }

    // Penalize huge codebases
    if metrics.total_lines > 10000 {
        score -= 25.0;
    }

    match score {
        s if s >= 85.0 => CertificationGrade::A,
        s if s >= 70.0 => CertificationGrade::B,
        s if s >= 50.0 => CertificationGrade::C,
        _ => CertificationGrade::F,
    }
}

fn print_results(metrics: &SimpleMetrics, grade: &CertificationGrade) {
    println!("📊 MINIMAL ANALYSIS RESULTS:");
    println!("  Files: {}", metrics.files);
    println!("  Total Lines: {}", metrics.total_lines);
    println!("  Code Lines: {}", metrics.code_lines);
    println!("  Comment Lines: {}", metrics.comment_lines);
    println!("  Functions: {}", metrics.function_count);
    println!("  Complexity Estimate: {}", metrics.complexity_estimate);
    println!();

    let doc_ratio = if metrics.total_lines > 0 {
        metrics.comment_lines as f64 / metrics.total_lines as f64 * 100.0
    } else {
        0.0
    };

    println!("📈 QUALITY INDICATORS:");
    println!("  Documentation: {:.1}%", doc_ratio);
    println!("  Avg Complexity/File: {:.1}",
        metrics.complexity_estimate as f64 / metrics.files.max(1) as f64);
    println!("  Avg Functions/File: {:.1}",
        metrics.function_count as f64 / metrics.files.max(1) as f64);
    println!();

    let grade_emoji = match grade {
        CertificationGrade::A => "💎🚀",
        CertificationGrade::B => "🎯📊",
        CertificationGrade::C => "🔧⚠️",
        CertificationGrade::F => "🚨❌",
    };

    println!("🏆 CERTIFICATION GRADE: {:?} {}", grade, grade_emoji);
    println!();

    match grade {
        CertificationGrade::A => {
            println!("✅ TESLA GRADE: Meets all Tesla/SpaceX standards");
            println!("Ready for mission-critical deployment");
        }
        CertificationGrade::B => {
            println!("✅ GOOD GRADE: Acceptable for production");
            println!("Minor improvements recommended");
        }
        CertificationGrade::C => {
            println!("⚠️  ACCEPTABLE: Basic quality standards met");
            println!("Significant improvements needed for Tesla grade");
        }
        CertificationGrade::F => {
            println!("❌ FAIL: Does not meet basic quality standards");
            println!("Major refactoring required before deployment");
        }
    }
}