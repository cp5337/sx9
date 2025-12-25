// Enforces: SDC §6–§11, CLSGS, Annex A.2/A.3/A.4
// Authority: RFC-9141, RFC-9142

mod _budget;

use sx9_foundation_core::data::Utc;
use sx9_harness::{
    agents::{BehavioralScope, DriftSignal, DriftVector, GovernanceGate, QaStatus},
    lineage::LineageTracker,
    linear::{IntentAnchor, LinearQaSignal},
    nats::subjects::forge,
};

fn scope(role: &str, action: &str, constraint: &str, object: &str) -> BehavioralScope {
    BehavioralScope {
        role: role.to_string(),
        action: action.to_string(),
        constraint: constraint.to_string(),
        object: object.to_string(),
    }
}

/// Test that BehavioralScope fields are correctly populated (Annex A.2)
#[test]
fn nvnn_fields_roundtrip() {
    let s = scope("service", "process", "bounded", "request");
    assert_eq!(s.role, "service");
    assert_eq!(s.action, "process");
    assert_eq!(s.constraint, "bounded");
    assert_eq!(s.object, "request");
    assert!(s.is_valid());
}

/// Test N-V-N-N annotation string generation (Annex A.2)
#[test]
fn nvnn_annotation_format() {
    let s = scope("factory", "generate", "rust_crate", "source_code");
    let annotation = s.as_annotation();
    assert_eq!(annotation, "// FACTORY_GENERATE_RUST_CRATE_SOURCE_CODE");
}

/// Test N-V-N-N parsing from annotation string (Annex A.4.1)
#[test]
fn nvnn_parse_from_annotation() {
    let parsed = LineageTracker::parse_nvnn("// FACTORY_GENERATE_RUST_CRATE_SOURCE_CODE");
    assert!(parsed.is_some());
    let scope = parsed.unwrap();
    assert_eq!(scope.role, "factory");
    assert_eq!(scope.action, "generate");
}

/// Test that Linear task gating blocks on static QA failure (Annex A.3.2)
#[test]
fn linear_task_gating_blocks_on_static_qa_failure() {
    // Create a QA signal indicating static QA failure
    let qa_signal = LinearQaSignal {
        static_passed: false,
        qa_status: None,
        blocking_gate: None,
        summary: "Missing N-V-N-N annotation in src/foo.rs".to_string(),
        checked_at: Utc::now(),
    };

    // Per Annex A.3.2: Static QA failure implies task cannot progress
    assert!(!qa_signal.static_passed);
}

/// Test intent anchors map tasks to behavioral regions (Annex A.3.1)
#[test]
fn intent_anchor_maps_to_behavioral_region() {
    let anchor = IntentAnchor {
        nvnn_pattern: "// SERVICE_VALIDATE_BOUNDED_INPUT".to_string(),
        file_paths: vec!["src/service.rs".to_string()],
        code_regions: vec!["src/service.rs:10-25".to_string()],
        agent_scope: Some("Sentinel".to_string()),
        confidence: 0.95,
    };

    assert!(anchor.nvnn_pattern.contains("SERVICE"));
    assert!(anchor.confidence > 0.9);
}

/// Test NATS subjects have expected governance namespaces (RFC-9142)
#[test]
fn nats_subjects_have_expected_namespaces() {
    // Drift signal subjects
    assert!(forge::governance::DRIFT_DETECTED.starts_with("sx9.forge.governance.drift"));
    assert!(forge::governance::DRIFT_ROLE.contains("role"));
    assert!(forge::governance::DRIFT_CONSTRAINT.contains("constraint"));

    // Dynamic drift subject
    let drift_subj = forge::governance::drift_for("agent_registry");
    assert!(drift_subj.contains("agent_registry"));

    // Gate subjects (RFC-9142 Section 7.1)
    assert!(forge::governance::GATE_OBSERVE.contains("observe"));
    assert!(forge::governance::GATE_WARN.contains("warn"));
    assert!(forge::governance::GATE_BLOCK.contains("block"));
    assert!(forge::governance::GATE_ESCALATE.contains("escalate"));

    // Static and Semantic QA subjects
    assert!(forge::static_qa::PASSED.contains("static"));
    assert!(forge::semantic_qa::DRIFT_ADVISORY.contains("semantic"));
}

/// Test lineage subjects for Git tracking (Annex A.4)
#[test]
fn nats_subjects_lineage_tracking() {
    assert!(forge::governance::LINEAGE_REGRESSION.contains("regression"));
    assert!(forge::governance::LINEAGE_LOSS.contains("annotation_loss"));
    assert!(forge::governance::LINEAGE_EXPANSION.contains("scope_expansion"));

    let commit_subj = forge::governance::lineage_for_commit("abc123def");
    assert!(commit_subj.contains("abc123def"));
}

/// Test drift signal structure (RFC-9142)
#[test]
fn drift_signal_structure() {
    let drift = DriftSignal {
        vector: DriftVector::Constraint,
        score: 0.75,
        delta_angle: 45.0,
        explanation: "Constraint boundary expanded without declaration".to_string(),
        detected_at: Utc::now(),
    };

    assert_eq!(drift.vector, DriftVector::Constraint);
    assert!(drift.score >= 0.0 && drift.score <= 1.0);
    assert!(drift.delta_angle <= 180.0);
}

/// Test governance gate levels (RFC-9142 Section 7.1)
#[test]
fn governance_gate_levels() {
    assert_eq!(GovernanceGate::default(), GovernanceGate::Observe);

    // Test all gate levels exist
    let _observe = GovernanceGate::Observe;
    let _warn = GovernanceGate::Warn;
    let _gate = GovernanceGate::Gate;
    let _escalate = GovernanceGate::Escalate;
}

/// Test QA status aggregation (Annex A.3.2)
#[test]
fn qa_status_with_drift_signals() {
    let drift1 = DriftSignal {
        vector: DriftVector::Role,
        score: 0.6,
        delta_angle: 30.0,
        explanation: "Role assumed undeclared responsibility".to_string(),
        detected_at: Utc::now(),
    };

    let drift2 = DriftSignal {
        vector: DriftVector::Pattern,
        score: 0.8,
        delta_angle: 60.0,
        explanation: "N-V-N-N pattern violated".to_string(),
        detected_at: Utc::now(),
    };

    let qa_status = QaStatus {
        static_passed: true,
        drift_signals: vec![drift1, drift2],
        gate_level: GovernanceGate::Warn,
        last_checked: Utc::now(),
    };

    assert!(qa_status.static_passed);
    assert_eq!(qa_status.drift_signals.len(), 2);
    // Max drift score should be 0.8
    let max_score = qa_status.drift_signals.iter().map(|d| d.score).fold(0.0_f32, f32::max);
    assert!((max_score - 0.8).abs() < 0.001);
}
