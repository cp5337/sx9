// Enforces: SDC §6–§11, CLSGS, Annex A.2/A.3/A.4
// Authority: RFC-9141, RFC-9142

//! Criterion benchmarks for CLSGS Annex operations
//!
//! Run with: cargo bench -p sx9-harness

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sx9_harness::{
    agents::BehavioralScope,
    lineage::{LineageTracker, LineageMarker},
    nats::subjects::forge,
};

fn bench_nvnn_parse(c: &mut Criterion) {
    c.bench_function("nvnn_parse", |b| {
        b.iter(|| {
            let annotation = black_box("// FACTORY_GENERATE_RUST_CRATE_SOURCE_CODE");
            LineageTracker::parse_nvnn(annotation)
        })
    });
}

fn bench_behavioral_scope_annotation(c: &mut Criterion) {
    let scope = BehavioralScope {
        role: "factory".into(),
        action: "generate".into(),
        constraint: "rust_crate".into(),
        object: "source_code".into(),
    };

    c.bench_function("scope_as_annotation", |b| {
        b.iter(|| black_box(&scope).as_annotation())
    });
}

fn bench_nats_subject_build(c: &mut Criterion) {
    c.bench_function("nats_drift_subject", |b| {
        b.iter(|| forge::governance::drift_for(black_box("component_name")))
    });

    c.bench_function("nats_gate_subject", |b| {
        b.iter(|| forge::governance::gate_for_pr(black_box("pr-123")))
    });

    c.bench_function("nats_lineage_subject", |b| {
        b.iter(|| forge::governance::lineage_for_commit(black_box("abc123def")))
    });
}

fn bench_lineage_compare(c: &mut Criterion) {
    let tracker = LineageTracker::new("/tmp");

    // Create 500 markers for realistic benchmark
    let markers: Vec<LineageMarker> = (0..500)
        .map(|i| LineageMarker {
            annotation: format!("// WORKER_PROCESS_BOUNDED_TASK_{}", i),
            behavioral_scope: Some(BehavioralScope {
                role: "worker".into(),
                action: "process".into(),
                constraint: "bounded".into(),
                object: format!("task_{}", i),
            }),
            file_path: format!("src/worker_{}.rs", i),
            line_number: i as u32 + 1,
            introduced_in: "commit_a".into(),
            last_modified_in: None,
            present_in_head: true,
        })
        .collect();

    c.bench_function("lineage_compare_500_markers", |b| {
        b.iter(|| {
            tracker.compare_commits(
                black_box(&markers),
                black_box(&[]),
                "commit_a",
                "commit_b",
            )
        })
    });
}

criterion_group!(
    benches,
    bench_nvnn_parse,
    bench_behavioral_scope_annotation,
    bench_nats_subject_build,
    bench_lineage_compare
);
criterion_main!(benches);
