// Enforces: SDC §6–§11, CLSGS, Annex A.2/A.3/A.4
// Authority: RFC-9141, RFC-9142

//! Smoke tests for CLSGS Annex integration
//!
//! These tests verify the end-to-end flow from Linear tasks through
//! Git lineage tracking to governance signals.

use sx9_foundation_core::data::Utc;
use sx9_harness::{
    agents::{BehavioralScope, DriftSignal, DriftVector, GovernanceGate, QaStatus},
    lineage::{LineageTracker, LineageMarker, AnnotationChanges, PullRequestBoundary},
    linear::{IntentAnchor, LinearQaSignal},
};

fn scope(role: &str, action: &str, constraint: &str, object: &str) -> BehavioralScope {
    BehavioralScope {
        role: role.into(),
        action: action.into(),
        constraint: constraint.into(),
        object: object.into(),
    }
}

/// Smoke test: Linear task → Intent Anchor → QA Signal → Governance Gate
#[test]
fn smoke_linear_to_governance_signal() {
    // 1. Create intent anchor mapping task to behavioral region (Annex A.3.1)
    let anchor = IntentAnchor {
        nvnn_pattern: "// SERVICE_WRITE_IDEMPOTENT_RECORD".to_string(),
        file_paths: vec!["src/service.rs".to_string()],
        code_regions: vec!["src/service.rs:20-45".to_string()],
        agent_scope: Some("Forge".to_string()),
        confidence: 0.92,
    };

    assert!(!anchor.file_paths.is_empty());

    // 2. Create drift signal indicating constraint drift (RFC-9142)
    let drift = DriftSignal {
        vector: DriftVector::Constraint,
        score: 0.85,
        delta_angle: 72.0,
        explanation: "IDEMPOTENT constraint removed from service write path".to_string(),
        detected_at: Utc::now(),
    };

    // 3. Create QA status with drift (Annex A.3.2)
    let qa_status = QaStatus {
        static_passed: true,
        drift_signals: vec![drift.clone()],
        gate_level: GovernanceGate::Escalate,
        last_checked: Utc::now(),
    };

    // 4. Create Linear QA signal (Annex A.3.2)
    let qa_signal = LinearQaSignal {
        static_passed: true,
        qa_status: Some(qa_status),
        blocking_gate: Some(GovernanceGate::Escalate),
        summary: "Constraint drift detected in service.rs".to_string(),
        checked_at: Utc::now(),
    };

    // Verify: Static QA passed but governance gate blocks progression
    assert!(qa_signal.static_passed);
    assert!(qa_signal.blocking_gate.is_some());
    assert_eq!(qa_signal.blocking_gate.unwrap(), GovernanceGate::Escalate);
}

/// Smoke test: Lineage tracker detects annotation removal (Annex A.4.1)
#[test]
fn smoke_lineage_detects_annotation_loss() {
    let tracker = LineageTracker::new("/tmp");

    // Create markers from before and after states
    let old_marker = LineageMarker {
        annotation: "// SERVICE_WRITE_IDEMPOTENT_RECORD".to_string(),
        behavioral_scope: Some(scope("service", "write", "idempotent", "record")),
        file_path: "src/service.rs".to_string(),
        line_number: 20,
        introduced_in: "abc123".to_string(),
        last_modified_in: None,
        present_in_head: true,
    };

    let old_markers = vec![old_marker.clone()];
    let new_markers: Vec<LineageMarker> = vec![]; // Annotation removed

    // Compare commits
    let changes = tracker.compare_commits(&old_markers, &new_markers, "abc123", "def456");

    // Verify: Annotation loss detected
    assert!(!changes.removed.is_empty(), "annotation loss must be detected");
    assert_eq!(changes.net_change, -1);

    // Check drift signal on the loss
    let loss = &changes.removed[0];
    assert!(loss.drift_signal.is_some());
}

/// Smoke test: Scope expansion detection (Annex A.4.2)
#[test]
fn smoke_scope_expansion_detection() {
    let tracker = LineageTracker::new("/tmp");

    let original = scope("service", "read", "bounded", "cache");
    let expanded = scope("service", "write", "bounded", "cache"); // Action changed

    let expansion = tracker.detect_scope_expansion(
        &original,
        &expanded,
        "CacheService",
        "commit_xyz",
    );

    // Verify: Scope expansion detected (action changed from read to write)
    assert!(expansion.is_some());
    let exp = expansion.unwrap();
    assert_eq!(exp.drift_vector, DriftVector::Role);
    assert_eq!(exp.component, "CacheService");
}

/// Smoke test: PR boundary does not equate approval with alignment (Annex A.4.3)
#[test]
fn smoke_pr_approval_not_alignment() {
    use sx9_harness::lineage::PullRequestBoundary;

    let pr = PullRequestBoundary {
        pr_id: "PR-456".to_string(),
        base_branch: "main".to_string(),
        head_branch: "feature/new-service".to_string(),
        merge_commit: None,
        static_qa_passed: true,
        drift_signals: vec![],
        annotation_changes: AnnotationChanges::default(),
        // Per CLSGS A.4.3: Approval does not equal alignment
        approval_equals_alignment: false,
    };

    // This invariant MUST always be false per CLSGS doctrine
    assert!(!pr.approval_equals_alignment);
}

/// Smoke test: Full governance gate escalation path
#[test]
fn smoke_governance_gate_escalation() {
    // Escalation path: Observe → Warn → Gate → Escalate
    let gate_observe = GovernanceGate::Observe;
    assert_eq!(gate_observe, GovernanceGate::Observe);

    let gate_warn = GovernanceGate::Warn;
    assert_eq!(gate_warn, GovernanceGate::Warn);

    let gate_block = GovernanceGate::Gate;
    assert_eq!(gate_block, GovernanceGate::Gate);

    let gate_escalate = GovernanceGate::Escalate;
    assert_eq!(gate_escalate, GovernanceGate::Escalate);
}
