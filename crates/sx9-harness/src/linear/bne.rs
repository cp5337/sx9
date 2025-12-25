//! Bar Napkin Engineering (BNE) Workflow
//!
//! Transforms voice ideation into structured Linear issues with scholarly references.
//!
//! ## Flow (RFC-9141)
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                         BNE WORKFLOW                                     │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │                                                                          │
//! │  Voice Input ──► Transcription ──► Intent Classification                │
//! │                                            │                             │
//! │                                            ▼                             │
//! │                             ┌──────────────────────────┐                │
//! │                             │   Scholarly Research     │                │
//! │                             │   (2 references min)     │                │
//! │                             └────────────┬─────────────┘                │
//! │                                          │                               │
//! │                                          ▼                               │
//! │                             ┌──────────────────────────┐                │
//! │                             │   PoC Test Generation    │                │
//! │                             └────────────┬─────────────┘                │
//! │                                          │                               │
//! │                                          ▼                               │
//! │                             ┌──────────────────────────┐                │
//! │                             │   SDLC Gate Decision     │                │
//! │                             │   ┌───────┬──────────┐   │                │
//! │                             │   │Approve│ Defer    │   │                │
//! │                             │   └───────┴──────────┘   │                │
//! │                             └────────────┬─────────────┘                │
//! │                                          │                               │
//! │                                          ▼                               │
//! │                               Structured Linear Issue                    │
//! │                                                                          │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use sx9_foundation_core::data::{DateTime, Utc, Uuid};

use super::{LinearIssue, LinearPriority, LinearState};

/// Intent classification for BNE ideation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeationIntent {
    /// Primary intent category
    pub category: IntentCategory,

    /// Extracted keywords for scholarly search
    pub keywords: Vec<String>,

    /// Suggested priority based on urgency language
    pub suggested_priority: LinearPriority,

    /// Confidence score (0.0-1.0)
    pub confidence: f32,

    /// Raw ideation text
    pub raw_text: String,
}

/// Categories of development intent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentCategory {
    /// New feature request
    Feature,

    /// Bug fix
    BugFix,

    /// Performance optimization
    Optimization,

    /// Security improvement
    Security,

    /// Documentation
    Documentation,

    /// Refactoring
    Refactor,

    /// Infrastructure/DevOps
    Infrastructure,

    /// Research/Exploration
    Research,
}

impl IntentCategory {
    pub fn as_label(&self) -> &'static str {
        match self {
            IntentCategory::Feature => "feature",
            IntentCategory::BugFix => "bug",
            IntentCategory::Optimization => "performance",
            IntentCategory::Security => "security",
            IntentCategory::Documentation => "docs",
            IntentCategory::Refactor => "refactor",
            IntentCategory::Infrastructure => "infra",
            IntentCategory::Research => "research",
        }
    }
}

/// Scholarly reference from Zotero or web
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScholarlyReference {
    /// Reference title
    pub title: String,

    /// Authors
    pub authors: Vec<String>,

    /// Year published
    pub year: Option<i32>,

    /// DOI or URL
    pub identifier: String,

    /// Short abstract or description
    pub abstract_text: Option<String>,

    /// Source (Zotero, Web, Manual)
    pub source: ReferenceSource,

    /// Relevance score (0.0-1.0)
    pub relevance: f32,
}

/// Source of scholarly reference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferenceSource {
    /// Local Zotero library
    Zotero,

    /// Web search (Google Scholar, arXiv, etc.)
    Web,

    /// Manually provided by user
    Manual,
}

/// Proof of Concept test specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PocTest {
    /// Test name
    pub name: String,

    /// What the test verifies
    pub description: String,

    /// Suggested test type
    pub test_type: PocTestType,

    /// Acceptance criteria
    pub acceptance_criteria: Vec<String>,

    /// Estimated complexity (1-5)
    pub complexity: u8,
}

/// Type of PoC test
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PocTestType {
    /// Unit test
    Unit,

    /// Integration test
    Integration,

    /// End-to-end test
    E2E,

    /// Performance benchmark
    Benchmark,

    /// Security scan
    Security,

    /// Manual verification
    Manual,
}

impl PocTestType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PocTestType::Unit => "unit",
            PocTestType::Integration => "integration",
            PocTestType::E2E => "e2e",
            PocTestType::Benchmark => "benchmark",
            PocTestType::Security => "security",
            PocTestType::Manual => "manual",
        }
    }
}

/// SDLC gate decision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SdlcGateDecision {
    /// Approved for SDLC entry
    Approved,

    /// Deferred pending more info
    Deferred,

    /// Rejected (not aligned with goals)
    Rejected,

    /// Pending human review
    PendingReview,
}

/// Complete BNE artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BneArtifact {
    /// Unique artifact ID
    pub id: Uuid,

    /// Original ideation intent
    pub intent: IdeationIntent,

    /// Scholarly references (min 2)
    pub references: Vec<ScholarlyReference>,

    /// PoC test specification
    pub poc_test: PocTest,

    /// SDLC gate decision
    pub gate_decision: SdlcGateDecision,

    /// Structured issue draft
    pub issue_draft: BneIssueDraft,

    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

/// Draft issue from BNE process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BneIssueDraft {
    /// Issue title
    pub title: String,

    /// Structured description
    pub description: String,

    /// Suggested labels
    pub labels: Vec<String>,

    /// Suggested priority
    pub priority: LinearPriority,

    /// Behavioral scope (N-V-N-N)
    pub behavioral_scope: BehavioralScope,
}

/// N-V-N-N behavioral scope annotation (RFC-9141)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralScope {
    /// Role (Factory, Analyst, etc.)
    pub role: String,

    /// Action (generate, analyze, etc.)
    pub action: String,

    /// Constraint (rust_crate, algorithm, etc.)
    pub constraint: String,

    /// Object (source_code, computation, etc.)
    pub object: String,
}

impl BehavioralScope {
    pub fn to_annotation(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.role, self.action, self.constraint, self.object
        )
    }
}

/// BNE Workflow processor
pub struct BneWorkflow {
    /// Minimum scholarly references required
    min_references: usize,

    /// Auto-approve confidence threshold
    auto_approve_threshold: f32,
}

impl Default for BneWorkflow {
    fn default() -> Self {
        Self {
            min_references: 2,
            auto_approve_threshold: 0.8,
        }
    }
}

impl BneWorkflow {
    /// Create new BNE workflow
    pub fn new() -> Self {
        Self::default()
    }

    /// Set minimum references required
    pub fn with_min_references(mut self, count: usize) -> Self {
        self.min_references = count;
        self
    }

    /// Set auto-approve threshold
    pub fn with_auto_approve_threshold(mut self, threshold: f32) -> Self {
        self.auto_approve_threshold = threshold;
        self
    }

    /// Classify intent from raw ideation text
    pub fn classify_intent(&self, text: &str) -> IdeationIntent {
        let text_lower = text.to_lowercase();

        // Classify category
        let category = if text_lower.contains("bug") || text_lower.contains("fix") || text_lower.contains("broken") {
            IntentCategory::BugFix
        } else if text_lower.contains("security") || text_lower.contains("vulnerability") {
            IntentCategory::Security
        } else if text_lower.contains("faster") || text_lower.contains("performance") || text_lower.contains("optimize") {
            IntentCategory::Optimization
        } else if text_lower.contains("document") || text_lower.contains("readme") {
            IntentCategory::Documentation
        } else if text_lower.contains("refactor") || text_lower.contains("cleanup") {
            IntentCategory::Refactor
        } else if text_lower.contains("deploy") || text_lower.contains("ci") || text_lower.contains("infrastructure") {
            IntentCategory::Infrastructure
        } else if text_lower.contains("research") || text_lower.contains("explore") || text_lower.contains("investigate") {
            IntentCategory::Research
        } else {
            IntentCategory::Feature
        };

        // Extract keywords (simple word extraction)
        let keywords: Vec<String> = text
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .filter(|w| !["the", "and", "for", "with", "this", "that", "would", "could", "should"].contains(&w.to_lowercase().as_str()))
            .take(5)
            .map(|w| w.to_lowercase())
            .collect();

        // Suggest priority based on urgency
        let suggested_priority = if text_lower.contains("urgent") || text_lower.contains("critical") || text_lower.contains("asap") {
            LinearPriority::Urgent
        } else if text_lower.contains("important") || text_lower.contains("soon") {
            LinearPriority::High
        } else {
            LinearPriority::Medium
        };

        IdeationIntent {
            category,
            keywords,
            suggested_priority,
            confidence: 0.7, // Default confidence
            raw_text: text.to_string(),
        }
    }

    /// Generate PoC test from intent
    pub fn generate_poc_test(&self, intent: &IdeationIntent) -> PocTest {
        let test_type = match intent.category {
            IntentCategory::BugFix => PocTestType::Unit,
            IntentCategory::Feature => PocTestType::Integration,
            IntentCategory::Security => PocTestType::Security,
            IntentCategory::Optimization => PocTestType::Benchmark,
            _ => PocTestType::Manual,
        };

        let complexity = match intent.category {
            IntentCategory::BugFix => 2,
            IntentCategory::Feature => 3,
            IntentCategory::Optimization => 4,
            IntentCategory::Security => 4,
            _ => 2,
        };

        PocTest {
            name: format!("test_{}", intent.category.as_label()),
            description: format!("Verify {} implementation", intent.category.as_label()),
            test_type,
            acceptance_criteria: vec![
                "Compiles without errors".to_string(),
                "All tests pass".to_string(),
                "No regressions introduced".to_string(),
            ],
            complexity,
        }
    }

    /// Make SDLC gate decision
    pub fn gate_decision(
        &self,
        intent: &IdeationIntent,
        references: &[ScholarlyReference],
    ) -> SdlcGateDecision {
        // Require minimum references
        if references.len() < self.min_references {
            return SdlcGateDecision::Deferred;
        }

        // Auto-approve high confidence
        if intent.confidence >= self.auto_approve_threshold {
            return SdlcGateDecision::Approved;
        }

        // Research needs human review
        if intent.category == IntentCategory::Research {
            return SdlcGateDecision::PendingReview;
        }

        SdlcGateDecision::Approved
    }

    /// Generate issue draft from BNE artifact
    pub fn generate_issue_draft(&self, intent: &IdeationIntent, references: &[ScholarlyReference], poc: &PocTest) -> BneIssueDraft {
        let title = self.generate_title(intent);
        let description = self.format_description(intent, references, poc);

        let behavioral_scope = BehavioralScope {
            role: match intent.category {
                IntentCategory::Feature | IntentCategory::BugFix => "Factory".to_string(),
                IntentCategory::Security => "Guardian".to_string(),
                IntentCategory::Documentation => "Scribe".to_string(),
                _ => "Analyst".to_string(),
            },
            action: match intent.category {
                IntentCategory::Feature => "generate".to_string(),
                IntentCategory::BugFix => "repair".to_string(),
                IntentCategory::Optimization => "optimize".to_string(),
                IntentCategory::Security => "harden".to_string(),
                IntentCategory::Documentation => "document".to_string(),
                _ => "analyze".to_string(),
            },
            constraint: "rust_crate".to_string(),
            object: "source_code".to_string(),
        };

        BneIssueDraft {
            title,
            description,
            labels: vec![
                intent.category.as_label().to_string(),
                "bne".to_string(),
            ],
            priority: intent.suggested_priority,
            behavioral_scope,
        }
    }

    /// Generate issue title from intent
    fn generate_title(&self, intent: &IdeationIntent) -> String {
        // Simple title generation - take first sentence or 10 words
        let words: Vec<&str> = intent.raw_text.split_whitespace().take(10).collect();
        let title = words.join(" ");

        // Capitalize first letter
        let mut chars = title.chars();
        match chars.next() {
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            None => title,
        }
    }

    /// Format issue description with references
    fn format_description(&self, intent: &IdeationIntent, references: &[ScholarlyReference], poc: &PocTest) -> String {
        let mut desc = String::new();

        // Summary
        desc.push_str("## Summary\n\n");
        desc.push_str(&intent.raw_text);
        desc.push_str("\n\n");

        // Behavioral scope
        desc.push_str("## Behavioral Scope\n\n");
        desc.push_str("```\n");
        desc.push_str(&format!("Role: {}\n", "Factory"));
        desc.push_str(&format!("Action: {}\n", intent.category.as_label()));
        desc.push_str(&format!("Constraint: rust_crate\n"));
        desc.push_str(&format!("Object: source_code\n"));
        desc.push_str("```\n\n");

        // Scholarly references
        if !references.is_empty() {
            desc.push_str("## Scholarly References\n\n");
            for (i, r) in references.iter().enumerate() {
                desc.push_str(&format!(
                    "{}. **{}** ({})\n   - {}\n   - Relevance: {:.0}%\n\n",
                    i + 1,
                    r.title,
                    r.year.map(|y| y.to_string()).unwrap_or_else(|| "n.d.".to_string()),
                    r.identifier,
                    r.relevance * 100.0
                ));
            }
        }

        // PoC Test
        desc.push_str("## PoC Test\n\n");
        desc.push_str(&format!("**Type:** {}\n", poc.test_type.as_str()));
        desc.push_str(&format!("**Complexity:** {}/5\n\n", poc.complexity));
        desc.push_str("### Acceptance Criteria\n\n");
        for criterion in &poc.acceptance_criteria {
            desc.push_str(&format!("- [ ] {}\n", criterion));
        }

        desc
    }

    /// Create full BNE artifact from ideation text
    pub fn process_ideation(&self, text: &str, references: Vec<ScholarlyReference>) -> BneArtifact {
        let intent = self.classify_intent(text);
        let poc_test = self.generate_poc_test(&intent);
        let gate_decision = self.gate_decision(&intent, &references);
        let issue_draft = self.generate_issue_draft(&intent, &references, &poc_test);

        BneArtifact {
            id: Uuid::new_v4(),
            intent,
            references,
            poc_test,
            gate_decision,
            issue_draft,
            created_at: Utc::now(),
        }
    }

    /// Convert BNE artifact to Linear issue
    pub fn to_linear_issue(&self, artifact: &BneArtifact) -> LinearIssue {
        LinearIssue {
            id: artifact.id.to_string(),
            identifier: format!("BNE-{}", &artifact.id.to_string()[..8]),
            title: artifact.issue_draft.title.clone(),
            description: Some(artifact.issue_draft.description.clone()),
            state: match artifact.gate_decision {
                SdlcGateDecision::Approved => LinearState::Todo,
                SdlcGateDecision::Deferred | SdlcGateDecision::PendingReview => LinearState::Backlog,
                SdlcGateDecision::Rejected => LinearState::Canceled,
            },
            priority: artifact.issue_draft.priority,
            team_id: String::new(),
            project_id: None,
            cycle_id: None,
            label_ids: artifact.issue_draft.labels.clone(),
            assignee_id: None,
            creator_id: "bne-workflow".to_string(),
            url: String::new(),
            created_at: artifact.created_at,
            updated_at: artifact.created_at,
            intent_anchors: Vec::new(),
            qa_signal: None,
            can_progress: artifact.gate_decision == SdlcGateDecision::Approved,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_intent_feature() {
        let workflow = BneWorkflow::new();
        let intent = workflow.classify_intent("I want to add a new export feature for CSV files");

        assert_eq!(intent.category, IntentCategory::Feature);
        assert!(!intent.keywords.is_empty());
    }

    #[test]
    fn test_classify_intent_bug() {
        let workflow = BneWorkflow::new();
        let intent = workflow.classify_intent("The login button is broken and needs to be fixed urgently");

        assert_eq!(intent.category, IntentCategory::BugFix);
        assert_eq!(intent.suggested_priority, LinearPriority::Urgent);
    }

    #[test]
    fn test_generate_poc_test() {
        let workflow = BneWorkflow::new();
        let intent = workflow.classify_intent("Security vulnerability in auth module");
        let poc = workflow.generate_poc_test(&intent);

        assert_eq!(poc.test_type, PocTestType::Security);
        assert!(poc.complexity >= 3);
    }

    #[test]
    fn test_gate_decision_needs_references() {
        let workflow = BneWorkflow::new().with_min_references(2);
        let intent = workflow.classify_intent("Add feature X");

        let decision = workflow.gate_decision(&intent, &[]);
        assert_eq!(decision, SdlcGateDecision::Deferred);
    }

    #[test]
    fn test_process_ideation() {
        let workflow = BneWorkflow::new().with_min_references(0);
        let artifact = workflow.process_ideation(
            "We need to add WebSocket support for real-time updates",
            vec![],
        );

        assert_eq!(artifact.intent.category, IntentCategory::Feature);
        assert!(!artifact.issue_draft.title.is_empty());
        assert!(artifact.issue_draft.description.contains("Summary"));
    }

    #[test]
    fn test_behavioral_scope_annotation() {
        let scope = BehavioralScope {
            role: "Factory".to_string(),
            action: "generate".to_string(),
            constraint: "rust_crate".to_string(),
            object: "source_code".to_string(),
        };

        assert_eq!(scope.to_annotation(), "Factory-generate-rust_crate-source_code");
    }
}
