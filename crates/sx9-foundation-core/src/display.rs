//! CTAS-7 Line Analyzer Display Module
//! Display and output formatting functions
//! Follows CTAS-7 standards: ≤200 LOC

#[cfg(feature = "embedded-firefly")]
use alloc::{
    string::{String, ToString},
    vec::Vec,
    format,
    boxed::Box,
};

use super::core::LineAnalysis;
use super::metrics::RealTimeMetrics;

/// Get scanning emoji based on line analysis
pub fn get_scanning_emoji(_analysis: &LineAnalysis, line_num: usize) -> &'static str {
    // Vary emoji based on what's being found
    if line_num % 20 == 0 { "🔥" }         // Performance check
    else if line_num % 15 == 0 { "🧠" }   // Complexity check
    else if line_num % 10 == 0 { "🎯" }   // Quality check
    else if line_num % 7 == 0 { "🔒" }    // Security check
    else if line_num % 5 == 0 { "⚡" }    // Speed check
    else { "📊" }                         // General analysis
}

/// Show branded summary with Unicode assembly
#[cfg(not(feature = "embedded-firefly"))]
pub fn show_branded_summary(analysis: &LineAnalysis, metrics: &RealTimeMetrics, _filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📈 REAL-TIME ANALYSIS SUMMARY");
    println!("═══════════════════════════════════════");

    // Show branded Unicode assembly style output
    let grade = metrics.get_quality_grade();
    let unicode_grade = match grade {
        "A+" | "A" => "💎🚀",
        "B+" | "B" => "🎯✅",
        "C+" | "C" => "📊⚡",
        _ => "🔧⚠️"
    };

    println!("🎯 Quality Grade: {} {}", grade, unicode_grade);
    println!("📊 Real-time Score: {:.1}/100", metrics.quality_score);
    println!("🔍 Functions Found: {}", metrics.functions_found);
    println!("🔀 Control Structures: {}", metrics.control_structures);
    println!("➡️  Assignments: {}", metrics.assignments);

    if metrics.warnings > 0 {
        println!("⚠️  Warnings: {} detected", metrics.warnings);
    } else {
        println!("✅ No warnings detected");
    }

    // Show Unicode compression preview
    let compressed_preview = format!("{}⭐🎨🔒#{:.0}#{:.0}",
        unicode_grade,
        metrics.quality_score,
        analysis.complexity_total as f64
    );
    println!("🗜️  Unicode Assembly: {}", compressed_preview);

    // Show code structure breakdown
    println!("\n📋 CODE STRUCTURE BREAKDOWN:");
    println!("   📏 Total Lines: {}", analysis.total_lines);
    println!("   🧮 Logical Lines: {} ({:.1}%)",
        analysis.logical_lines,
        if analysis.total_lines > 0 {
            (analysis.logical_lines as f64 / analysis.total_lines as f64) * 100.0
        } else { 0.0 }
    );
    println!("   💬 Comments: {} ({:.1}%)",
        analysis.comment_lines,
        if analysis.total_lines > 0 {
            (analysis.comment_lines as f64 / analysis.total_lines as f64) * 100.0
        } else { 0.0 }
    );
    println!("   ⬜ Empty Lines: {}", analysis.empty_lines);

    println!("\n🔧 COMPLEXITY METRICS:");
    let avg_complexity = if analysis.logical_lines > 0 {
        analysis.complexity_total as f64 / analysis.logical_lines as f64
    } else { 0.0 };
    println!("   🎲 Total Complexity: {}", analysis.complexity_total);
    println!("   📊 Average per Line: {:.2}", avg_complexity);

    let complexity_rating = if avg_complexity <= 1.0 { "🎨 CLEAN" }
        else if avg_complexity <= 2.0 { "⚡ GOOD" }
        else { "🔥 COMPLEX" };
    println!("   🏆 Rating: {}", complexity_rating);

    // Show top operators and operands like the strip reporter
    println!("\n🛠️  OPERATOR ANALYSIS:");
    let mut top_ops: Vec<_> = analysis.operators.iter().collect();
    top_ops.sort_by(|a, b| b.1.cmp(a.1));
    for (op, count) in top_ops.iter().take(5) {
        println!("   '{}': {} occurrences", op, count);
    }

    // Tesla engineering phase
    println!("\n🏭 TESLA ENGINEERING PHASE: {}", metrics.get_tesla_phase());
    if metrics.meets_tesla_standards() {
        println!("✅ Meets Tesla/SpaceX Standards");
    } else {
        println!("❌ Below Tesla/SpaceX Standards");
    }

    Ok(())
}

/// Print cumulative results
#[cfg(not(feature = "embedded-firefly"))]
pub fn print_cumulative_results(analysis: &LineAnalysis) {
    println!("📊 Line-by-Line Totals:");
    println!("   Total Lines: {}", analysis.total_lines);
    println!("   Empty Lines: {}", analysis.empty_lines);
    println!("   Comment Lines: {}", analysis.comment_lines);
    println!("   Logical Lines: {}", analysis.logical_lines);
    println!("   Total Complexity: {}", analysis.complexity_total);

    println!("\n🔧 Operator Summary:");
    let mut op_vec: Vec<_> = analysis.operators.iter().collect();
    op_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (op, count) in op_vec.iter().take(10) {
        println!("   '{}': {} occurrences", op, count);
    }

    println!("\n📊 Most Common Operands:");
    let mut operand_vec: Vec<_> = analysis.operands.iter().collect();
    operand_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (operand, count) in operand_vec.iter().take(10) {
        println!("   '{}': {} occurrences", operand, count);
    }

    println!("\n📈 Derived Metrics:");
    let code_density = if analysis.total_lines > 0 {
        (analysis.logical_lines as f64 / analysis.total_lines as f64) * 100.0
    } else { 0.0 };

    let comment_ratio = if analysis.total_lines > 0 {
        (analysis.comment_lines as f64 / analysis.total_lines as f64) * 100.0
    } else { 0.0 };

    let avg_complexity = if analysis.logical_lines > 0 {
        analysis.complexity_total as f64 / analysis.logical_lines as f64
    } else { 0.0 };

    println!("   Code Density: {:.1}%", code_density);
    println!("   Comment Ratio: {:.1}%", comment_ratio);
    println!("   Avg Complexity per Line: {:.2}", avg_complexity);
    println!("   Distinct Operators: {}", analysis.operators.len());
    println!("   Distinct Operands: {}", analysis.operands.len());
}