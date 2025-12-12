// 🚀 CTAS-7 Performance Validation Runner
// Execute performance test harness and validate data flow speeds

use std::env;
use tokio;

// Import our performance test harness
use sx9_foundation_daemon::testing::performance_test_harness::{
    PerformanceTestHarness, TestResults,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("🎯 CTAS-7 Performance Validation Suite");
    println!("=====================================");
    println!("");

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let test_mode = args.get(1).map(|s| s.as_str()).unwrap_or("full");

    match test_mode {
        "hash" => run_hash_performance_test().await?,
        "routing" => run_routing_latency_test().await?,
        "services" => run_service_response_test().await?,
        "glaf" => run_glaf_intelligence_test().await?,
        "quick" => run_quick_validation().await?,
        "full" | _ => run_full_test_suite().await?,
    }

    Ok(())
}

/// Run full comprehensive test suite
async fn run_full_test_suite() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Running Full Performance Test Suite...");

    let mut harness = PerformanceTestHarness::new();
    let results = harness.run_full_test_suite().await?;

    // Display results summary
    println!("\n📊 PERFORMANCE VALIDATION RESULTS");
    println!("==================================");

    // Hash Performance Results
    println!("\n🔐 Hash Performance (MurmurHash3):");
    println!(
        "   Target: {:.0} MB/s",
        harness.test_config.hash_performance_target
    );
    println!(
        "   Actual: {:.0} MB/s",
        results.hash_performance.throughput_mb_per_sec
    );
    println!(
        "   Status: {}",
        if results.hash_performance.meets_target {
            "✅ PASS"
        } else {
            "❌ FAIL"
        }
    );
    println!(
        "   Avg Latency: {:.1}ns",
        results.hash_performance.average_latency_ns
    );
    println!(
        "   Operations/sec: {}",
        results.hash_performance.operations_per_second
    );

    // Routing Latency Results
    println!("\n⚡ Routing Latency (HFT):");
    println!(
        "   Target: <{:.0}ns",
        harness.test_config.routing_latency_target
    );
    println!(
        "   Actual: {:.1}ns",
        results.routing_latency.average_latency_ns
    );
    println!(
        "   Status: {}",
        if results.routing_latency.meets_hft_target {
            "✅ PASS"
        } else {
            "❌ FAIL"
        }
    );
    println!("   P99: {:.1}ns", results.routing_latency.p99_latency_ns);
    println!("   P95: {:.1}ns", results.routing_latency.p95_latency_ns);

    // Service Response Results
    println!("\n🌐 Service Response Times:");
    println!(
        "   Target: <{:.0}ms",
        harness.test_config.service_response_target
    );
    println!(
        "   Actual: {:.1}ms",
        results.service_response.average_response_time_ms
    );
    println!(
        "   Discovery: {:.1}ms",
        results.service_response.discovery_latency_ms
    );
    println!(
        "   Coordination: {:.1}ms",
        results.service_response.coordination_latency_ms
    );

    // GLAF Intelligence Results
    println!("\n🧠 GLAF Intelligence Performance:");
    println!(
        "   Intelligence Processing: {:.1}ms",
        results.glaf_integration.intelligence_processing_ms
    );
    println!(
        "   Threat Correlation: {:.1}ms",
        results.glaf_integration.threat_correlation_ms
    );
    println!(
        "   Alert Generation: {:.1}ms",
        results.glaf_integration.alert_generation_ms
    );
    println!(
        "   Total Pipeline: {:.1}ms",
        results.glaf_integration.total_pipeline_latency_ms
    );
    println!(
        "   Accuracy: {:.1}%",
        results.glaf_integration.intelligence_accuracy
    );

    // Throughput Results
    println!("\n📈 System Throughput:");
    println!(
        "   Target: {} ops/sec",
        harness.test_config.throughput_target
    );
    println!(
        "   Actual: {} ops/sec",
        results.throughput.operations_per_second
    );
    println!(
        "   Data Processed: {:.1} MB",
        results.throughput.data_processed_mb
    );
    println!(
        "   Concurrent Streams: {}",
        results.throughput.concurrent_streams
    );

    // Overall Score
    println!("\n🎯 OVERALL PERFORMANCE SCORE");
    println!("============================");
    println!("   Score: {:.1}/100", results.overall_score);

    let grade = if results.overall_score >= 90.0 {
        "🏆 EXCELLENT"
    } else if results.overall_score >= 80.0 {
        "✅ GOOD"
    } else if results.overall_score >= 70.0 {
        "⚠️ ACCEPTABLE"
    } else {
        "❌ NEEDS IMPROVEMENT"
    };

    println!("   Grade: {}", grade);

    // Service Status Summary
    println!("\n🔧 Service Status Summary:");
    for (service_name, service_result) in &results.service_response.service_tests {
        let status_icon = if service_result.success_rate > 95.0 {
            "✅"
        } else {
            "⚠️"
        };
        println!(
            "   {} {}: {:.1}ms ({:.1}% success)",
            status_icon, service_name, service_result.response_time_ms, service_result.success_rate
        );
    }

    // Save results to file
    save_results_to_file(&results).await?;

    println!("\n🎯 Performance validation complete!");
    if results.overall_score >= 80.0 {
        println!("✅ System ready for production deployment");
    } else {
        println!("⚠️ Performance optimization recommended");
    }

    Ok(())
}

/// Run quick validation (essential metrics only)
async fn run_quick_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Running Quick Performance Validation...");

    let mut harness = PerformanceTestHarness::new();

    // Run critical tests only
    let hash_results = harness.test_hash_performance().await?;
    let routing_results = harness.test_routing_latency().await?;

    println!("\n📊 QUICK VALIDATION RESULTS");
    println!("===========================");

    // Critical metrics
    println!(
        "🔐 Hash Performance: {:.0} MB/s ({})",
        hash_results.throughput_mb_per_sec,
        if hash_results.meets_target {
            "✅ PASS"
        } else {
            "❌ FAIL"
        }
    );

    println!(
        "⚡ Routing Latency: {:.1}ns ({})",
        routing_results.average_latency_ns,
        if routing_results.meets_hft_target {
            "✅ PASS"
        } else {
            "❌ FAIL"
        }
    );

    // Quick score calculation
    let quick_score = if hash_results.meets_target && routing_results.meets_hft_target {
        100.0
    } else if hash_results.meets_target || routing_results.meets_hft_target {
        50.0
    } else {
        0.0
    };

    println!("\n🎯 Quick Score: {:.0}/100", quick_score);

    Ok(())
}

/// Run hash performance test only
async fn run_hash_performance_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 Testing Hash Performance (MurmurHash3)...");

    let harness = PerformanceTestHarness::new();
    let results = harness.test_hash_performance().await?;

    println!("\n📊 HASH PERFORMANCE RESULTS");
    println!("===========================");
    println!("Algorithm: {}", results.algorithm);
    println!("Throughput: {:.0} MB/s", results.throughput_mb_per_sec);
    println!("Operations/sec: {}", results.operations_per_second);
    println!("Avg Latency: {:.1}ns", results.average_latency_ns);
    println!(
        "Target Met: {}",
        if results.meets_target {
            "✅ YES"
        } else {
            "❌ NO"
        }
    );

    println!("\nPayload Size Breakdown:");
    for payload_result in &results.payload_size_tests {
        println!(
            "  {} bytes: {:.0} MB/s ({:.1}ns latency)",
            payload_result.size_bytes,
            payload_result.throughput_mb_per_sec,
            payload_result.latency_ns
        );
    }

    Ok(())
}

/// Run routing latency test only
async fn run_routing_latency_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Testing Routing Latency (HFT)...");

    let harness = PerformanceTestHarness::new();
    let results = harness.test_routing_latency().await?;

    println!("\n📊 ROUTING LATENCY RESULTS");
    println!("==========================");
    println!("Average: {:.1}ns", results.average_latency_ns);
    println!("P95: {:.1}ns", results.p95_latency_ns);
    println!("P99: {:.1}ns", results.p99_latency_ns);
    println!("Min: {:.1}ns", results.min_latency_ns);
    println!("Max: {:.1}ns", results.max_latency_ns);
    println!(
        "HFT Target Met: {}",
        if results.meets_hft_target {
            "✅ YES"
        } else {
            "❌ NO"
        }
    );

    println!("\nRouting Hop Breakdown:");
    for hop_result in &results.routing_hops {
        println!(
            "  {}: {:.1}ns ({:.2}% success)",
            hop_result.hop_name, hop_result.latency_ns, hop_result.success_rate
        );
    }

    Ok(())
}

/// Run service response test only
async fn run_service_response_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Testing Service Response Times...");

    let harness = PerformanceTestHarness::new();
    let results = harness.test_service_responses().await?;

    println!("\n📊 SERVICE RESPONSE RESULTS");
    println!("===========================");
    println!(
        "Average Response: {:.1}ms",
        results.average_response_time_ms
    );
    println!("Discovery Latency: {:.1}ms", results.discovery_latency_ms);
    println!(
        "Coordination Latency: {:.1}ms",
        results.coordination_latency_ms
    );

    println!("\nIndividual Service Results:");
    for (service_name, service_result) in &results.service_tests {
        println!(
            "  {}: {:.1}ms ({:.1}% success, {} ops/s)",
            service_name,
            service_result.response_time_ms,
            service_result.success_rate,
            service_result.throughput_ops_per_sec
        );
    }

    Ok(())
}

/// Run GLAF intelligence test only
async fn run_glaf_intelligence_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧠 Testing GLAF Intelligence Performance...");

    let harness = PerformanceTestHarness::new();
    let results = harness.test_glaf_performance().await?;

    println!("\n📊 GLAF INTELLIGENCE RESULTS");
    println!("============================");
    println!(
        "Intelligence Processing: {:.1}ms",
        results.intelligence_processing_ms
    );
    println!("Threat Correlation: {:.1}ms", results.threat_correlation_ms);
    println!("Alert Generation: {:.1}ms", results.alert_generation_ms);
    println!("PLASMA Integration: {:.1}ms", results.plasma_integration_ms);
    println!("Total Pipeline: {:.1}ms", results.total_pipeline_latency_ms);
    println!(
        "Intelligence Accuracy: {:.1}%",
        results.intelligence_accuracy
    );

    let pipeline_acceptable = results.total_pipeline_latency_ms <= 1000.0;
    let accuracy_acceptable = results.intelligence_accuracy >= 90.0;

    println!(
        "\nGLAF Status: {}",
        if pipeline_acceptable && accuracy_acceptable {
            "✅ OPERATIONAL"
        } else {
            "⚠️ NEEDS ATTENTION"
        }
    );

    Ok(())
}

/// Save results to JSON file for analysis
async fn save_results_to_file(results: &TestResults) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::fs;

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("performance_results_{}.json", timestamp);

    let json_results = serde_json::to_string_pretty(results)?;
    fs::write(&filename, json_results).await?;

    println!("\n💾 Results saved to: {}", filename);

    Ok(())
}

/// Display usage information
fn display_usage() {
    println!("🚀 CTAS-7 Performance Validator");
    println!("");
    println!("Usage:");
    println!("  cargo run --bin performance_validator [mode]");
    println!("");
    println!("Modes:");
    println!("  full     - Run complete test suite (default)");
    println!("  quick    - Run essential tests only");
    println!("  hash     - Test hash performance only");
    println!("  routing  - Test routing latency only");
    println!("  services - Test service responses only");
    println!("  glaf     - Test GLAF intelligence only");
    println!("");
    println!("Examples:");
    println!("  cargo run --bin performance_validator");
    println!("  cargo run --bin performance_validator quick");
    println!("  cargo run --bin performance_validator hash");
}
