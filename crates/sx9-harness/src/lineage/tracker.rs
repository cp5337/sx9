//! Git Lineage Tracker
//!
//! Tracks behavioral declarations (N-V-N-N annotations) across Git history.
//!
//! CLSGS Annex A.4: Semantic QA evaluates drift longitudinally by comparing
//! historical annotations to current behavior.

use std::collections::HashMap;
use std::path::Path;

use sx9_foundation_core::data::{DateTime, Utc};

use crate::agents::{BehavioralScope, DriftSignal, DriftVector};
use super::types::*;

/// N-V-N-N annotation pattern regex
const NVNN_PATTERN: &str = r"//\s*([A-Z_]+)_([A-Z_]+)_([A-Z_]+)_([A-Z_]+)";

/// Git lineage tracker for semantic history analysis.
///
/// Per CLSGS Annex A.4: Git is treated as a semantic history carrier.
/// This tracker analyzes N-V-N-N annotations across commits to detect drift.
#[derive(Debug)]
pub struct LineageTracker {
    /// Repository root path
    repo_path: std::path::PathBuf,

    /// Cache of lineage markers by commit SHA
    marker_cache: HashMap<String, Vec<LineageMarker>>,

    /// Current HEAD commit SHA
    head_sha: Option<String>,
}

impl LineageTracker {
    /// Create a new lineage tracker for a repository.
    pub fn new<P: AsRef<Path>>(repo_path: P) -> Self {
        Self {
            repo_path: repo_path.as_ref().to_path_buf(),
            marker_cache: HashMap::new(),
            head_sha: None,
        }
    }

    /// Parse N-V-N-N annotation from a comment string.
    ///
    /// Format: `// ROLE_ACTION_CONSTRAINT_OBJECT`
    pub fn parse_nvnn(annotation: &str) -> Option<BehavioralScope> {
        // Simple parsing - look for pattern like // WORD_WORD_WORD_WORD
        let trimmed = annotation.trim();
        if !trimmed.starts_with("//") {
            return None;
        }

        let content = trimmed.trim_start_matches("//").trim();
        let parts: Vec<&str> = content.split('_').collect();

        if parts.len() >= 4 {
            Some(BehavioralScope {
                role: parts[0].to_lowercase(),
                action: parts[1].to_lowercase(),
                constraint: parts[2].to_lowercase(),
                object: parts[3].to_lowercase(),
            })
        } else {
            None
        }
    }

    /// Scan a file for N-V-N-N annotations.
    pub fn scan_file(&self, file_path: &Path, commit_sha: &str) -> Vec<LineageMarker> {
        let mut markers = Vec::new();

        // Read file content (placeholder - would use git show in real impl)
        let content = match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => return markers,
        };

        for (line_num, line) in content.lines().enumerate() {
            if line.contains("//") && line.to_uppercase().contains('_') {
                // Check for N-V-N-N pattern
                if let Some(scope) = Self::parse_nvnn(line) {
                    markers.push(LineageMarker {
                        annotation: line.trim().to_string(),
                        behavioral_scope: Some(scope),
                        file_path: file_path.to_string_lossy().to_string(),
                        line_number: (line_num + 1) as u32,
                        introduced_in: commit_sha.to_string(),
                        last_modified_in: None,
                        present_in_head: true,
                    });
                }
            }
        }

        markers
    }

    /// Compare annotations between two commits to detect drift.
    pub fn compare_commits(
        &self,
        old_markers: &[LineageMarker],
        new_markers: &[LineageMarker],
        old_sha: &str,
        new_sha: &str,
    ) -> AnnotationChanges {
        let mut changes = AnnotationChanges::default();

        // Build lookup maps
        let old_by_key: HashMap<String, &LineageMarker> = old_markers
            .iter()
            .map(|m| (format!("{}:{}", m.file_path, m.annotation), m))
            .collect();

        let new_by_key: HashMap<String, &LineageMarker> = new_markers
            .iter()
            .map(|m| (format!("{}:{}", m.file_path, m.annotation), m))
            .collect();

        // Find removed annotations
        for (key, old_marker) in &old_by_key {
            if !new_by_key.contains_key(key) {
                changes.removed.push(AnnotationLoss {
                    annotation: old_marker.annotation.clone(),
                    file_path: old_marker.file_path.clone(),
                    present_in: old_sha.to_string(),
                    lost_in: new_sha.to_string(),
                    explicitly_revised: false, // Would check commit message
                    drift_signal: Some(DriftSignal {
                        vector: DriftVector::Pattern,
                        score: 0.6,
                        delta_angle: 90.0,
                        explanation: format!(
                            "N-V-N-N annotation removed from {}",
                            old_marker.file_path
                        ),
                        detected_at: Utc::now(),
                    }),
                });
            }
        }

        // Find added annotations
        for (key, new_marker) in &new_by_key {
            if !old_by_key.contains_key(key) {
                changes.added.push((*new_marker).clone());
            }
        }

        changes.net_change = changes.added.len() as i32 - changes.removed.len() as i32;
        changes
    }

    /// Analyze a commit for lineage changes.
    pub fn analyze_commit(
        &self,
        commit_sha: &str,
        parent_sha: Option<&str>,
        changes: AnnotationChanges,
    ) -> LineageAnalysis {
        let drift_signals: Vec<DriftSignal> = changes
            .removed
            .iter()
            .filter_map(|loss| loss.drift_signal.clone())
            .collect();

        let semantic_regression = changes
            .removed
            .iter()
            .any(|loss| !loss.explicitly_revised);

        LineageAnalysis {
            operation: GitOperation::Commit,
            commit_sha: commit_sha.to_string(),
            parent_shas: parent_sha.map(|s| vec![s.to_string()]).unwrap_or_default(),
            changes,
            semantic_regression,
            drift_signals,
            analyzed_at: Utc::now(),
        }
    }

    /// Create a pull request boundary analysis.
    pub fn analyze_pr(
        &self,
        pr_id: &str,
        base_branch: &str,
        head_branch: &str,
        static_qa_passed: bool,
        changes: AnnotationChanges,
    ) -> PullRequestBoundary {
        let drift_signals: Vec<DriftSignal> = changes
            .removed
            .iter()
            .filter_map(|loss| loss.drift_signal.clone())
            .collect();

        PullRequestBoundary {
            pr_id: pr_id.to_string(),
            base_branch: base_branch.to_string(),
            head_branch: head_branch.to_string(),
            merge_commit: None,
            static_qa_passed,
            drift_signals,
            annotation_changes: changes,
            // Per CLSGS A.4.3: Approval does not equal alignment
            approval_equals_alignment: false,
        }
    }

    /// Detect scope expansion (responsibility creep) between two behavioral scopes.
    pub fn detect_scope_expansion(
        &self,
        original: &BehavioralScope,
        current: &BehavioralScope,
        component: &str,
        commit_sha: &str,
    ) -> Option<ScopeExpansion> {
        // Check for role expansion
        if original.role != current.role || original.action != current.action {
            return Some(ScopeExpansion {
                component: component.to_string(),
                original_scope: original.clone(),
                expanded_scope: current.clone(),
                detected_in: commit_sha.to_string(),
                drift_vector: DriftVector::Role,
            });
        }

        // Check for constraint erosion
        if original.constraint != current.constraint {
            return Some(ScopeExpansion {
                component: component.to_string(),
                original_scope: original.clone(),
                expanded_scope: current.clone(),
                detected_in: commit_sha.to_string(),
                drift_vector: DriftVector::Constraint,
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nvnn() {
        let annotation = "// FACTORY_GENERATE_RUST_CRATE_SOURCE_CODE";
        let scope = LineageTracker::parse_nvnn(annotation).unwrap();

        assert_eq!(scope.role, "factory");
        assert_eq!(scope.action, "generate");
        assert_eq!(scope.constraint, "rust");
        assert_eq!(scope.object, "crate");
    }

    #[test]
    fn test_parse_nvnn_invalid() {
        assert!(LineageTracker::parse_nvnn("// not valid").is_none());
        assert!(LineageTracker::parse_nvnn("regular comment").is_none());
    }

    #[test]
    fn test_compare_commits_detects_removal() {
        let tracker = LineageTracker::new("/tmp");

        let old_markers = vec![LineageMarker {
            annotation: "// TEST_ACTION_CONSTRAINT_OBJECT".to_string(),
            behavioral_scope: Some(BehavioralScope {
                role: "test".to_string(),
                action: "action".to_string(),
                constraint: "constraint".to_string(),
                object: "object".to_string(),
            }),
            file_path: "test.rs".to_string(),
            line_number: 1,
            introduced_in: "abc123".to_string(),
            last_modified_in: None,
            present_in_head: true,
        }];

        let new_markers: Vec<LineageMarker> = vec![];

        let changes = tracker.compare_commits(&old_markers, &new_markers, "abc123", "def456");

        assert_eq!(changes.removed.len(), 1);
        assert_eq!(changes.net_change, -1);
    }
}
