// Enforces: SDC §6–§11, CLSGS, Annex A.2/A.3/A.4
// Authority: RFC-9141, RFC-9142

//! Performance smoke tests with budget enforcement
//!
//! These tests verify CLSGS operations meet performance budgets
//! without hard-coding threshold values. Set SX9_PERF_BUDGETS_PATH
//! environment variable to a JSON file with budget values.

mod _budget;

use sx9_harness::{
    agents::BehavioralScope,
    lineage::{LineageTracker, LineageMarker},
    nats::subjects::forge,
};

#[test]
fn perf_smoke_budgeted() {
    // Load performance budgets from environment
    let budgets = match _budget::load_perf_budgets() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Skipping perf smoke: {e}");
            return;
        }
    };

    // N-V-N-N parse budget
    let parse_ns = measure_mean_ns(|| {
        let _ = LineageTracker::parse_nvnn("// FACTORY_GENERATE_RUST_CRATE_SOURCE_CODE");
    });
    assert!(
        parse_ns <= budgets.nvnn_parse_mean_ns,
        "nvnn parse mean_ns={parse_ns} exceeds budget={}",
        budgets.nvnn_parse_mean_ns
    );

    // BehavioralScope annotation budget
    let scope_ns = measure_mean_ns(|| {
        let scope = BehavioralScope {
            role: "factory".into(),
            action: "generate".into(),
            constraint: "rust_crate".into(),
            object: "source_code".into(),
        };
        let _ = scope.as_annotation();
    });
    assert!(
        scope_ns <= budgets.nvnn_parse_mean_ns * 2,
        "scope annotation mean_ns={scope_ns} exceeds 2x parse budget"
    );

    // NATS subject build budget
    let subject_ns = measure_mean_ns(|| {
        let _ = forge::governance::drift_for("component_name");
        let _ = forge::governance::gate_for_pr("pr-123");
        let _ = forge::governance::lineage_for_commit("abc123");
    });
    assert!(
        subject_ns <= budgets.nats_subject_build_mean_ns,
        "nats subject build mean_ns={subject_ns} exceeds budget={}",
        budgets.nats_subject_build_mean_ns
    );
}

#[test]
fn perf_lineage_compare_scaling() {
    // Load performance budgets from environment
    let budgets = match _budget::load_perf_budgets() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Skipping lineage perf: {e}");
            return;
        }
    };

    let tracker = LineageTracker::new("/tmp");

    // Create 100 markers
    let markers: Vec<LineageMarker> = (0..100)
        .map(|i| LineageMarker {
            annotation: format!("// WORKER_PROCESS_BOUNDED_TASK_{}", i),
            behavioral_scope: Some(BehavioralScope {
                role: "worker".into(),
                action: "process".into(),
                constraint: "bounded".into(),
                object: format!("task_{}", i),
            }),
            file_path: format!("src/worker_{}.rs", i),
            line_number: 10,
            introduced_in: "commit_a".into(),
            last_modified_in: None,
            present_in_head: true,
        })
        .collect();

    let analyze_ns = measure_mean_ns(|| {
        let _ = tracker.compare_commits(&markers, &[], "commit_a", "commit_b");
    });

    let per_marker = analyze_ns / 100;
    assert!(
        per_marker <= budgets.lineage_scan_per_marker_mean_ns,
        "lineage per_marker mean_ns={per_marker} exceeds budget={}",
        budgets.lineage_scan_per_marker_mean_ns
    );
}

/// Measure mean execution time in nanoseconds
fn measure_mean_ns<F: Fn()>(f: F) -> u64 {
    let iters: u64 = 10_000;
    let start = std::time::Instant::now();
    for _ in 0..iters {
        f();
    }
    let elapsed = start.elapsed();
    (elapsed.as_nanos() as u64) / iters
}
