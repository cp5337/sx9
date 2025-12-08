use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use tracing::{info, warn};
use crate::types::ProcessedDocument;
use super::persona_manager::ElitePersona;

/// Result of repo-prompt analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoPromptAnalysis {
    pub analysis_id: String,
    pub document_hash: String,
    pub assigned_persona: ElitePersona,
    pub confidence_score: f64,
    pub priority_recommendation: PriorityRecommendation,
    pub key_insights: Vec<String>,
    pub recommendations: Vec<Recommendation>,
    pub integration_points: Vec<IntegrationPoint>,
    pub threat_indicators: Vec<ThreatIndicator>,
    pub analysis_timestamp: chrono::DateTime<chrono::Utc>,
    pub processing_time_ms: u64,
    pub raw_output: String,
}

/// Priority recommendation from AI analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriorityRecommendation {
    Critical {
        reason: String,
        immediate_actions: Vec<String>,
    },
    High {
        reason: String,
        review_within_hours: u32,
    },
    Medium {
        reason: String,
        review_within_days: u32,
    },
    Low {
        reason: String,
        archive_suggested: bool,
    },
}

/// AI-generated recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub category: RecommendationCategory,
    pub title: String,
    pub description: String,
    pub confidence: f64,
    pub implementation_effort: ImplementationEffort,
    pub benefits: Vec<String>,
}

/// Categories of recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationCategory {
    SystemIntegration,
    CodeRefactoring,
    DocumentationImprovement,
    SecurityEnhancement,
    ProcessOptimization,
    ArchitecturalChange,
}

/// Implementation effort levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationEffort {
    Low,    // < 1 hour
    Medium, // 1-8 hours
    High,   // 1-5 days
    Epic,   // > 5 days
}

/// Integration points identified by AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationPoint {
    pub system_name: String,
    pub integration_type: IntegrationType,
    pub complexity: f64,
    pub benefits: Vec<String>,
    pub requirements: Vec<String>,
}

/// Types of system integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationType {
    DocumentManager,
    HookSystem,
    CommunicationProtocol,
    DatabaseSystem,
    ExternalAPI,
    UserInterface,
}

/// Threat indicators found in documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    pub indicator_type: ThreatType,
    pub severity: ThreatSeverity,
    pub description: String,
    pub context: String,
    pub mitigation_suggestions: Vec<String>,
}

/// Types of threats that can be identified
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatType {
    SecurityVulnerability,
    DataExposure,
    SystemWeakness,
    ProcessGap,
    ComplianceIssue,
}

/// Threat severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Analysis engine for repo-prompt processing
#[derive(Debug, Clone)]
pub struct AnalysisEngine {
    repo_prompt_path: String,
}

impl AnalysisEngine {
    pub fn new(repo_prompt_path: String) -> Self {
        Self { repo_prompt_path }
    }

    /// Create analysis prompt for repo-prompt system
    pub fn create_analysis_prompt(&self, document: &ProcessedDocument, persona: &ElitePersona) -> Result<String> {
        let prompt = format!(r#"
CTAS DOCUMENT INTELLIGENCE ANALYSIS REQUEST

Document ID: {}
Document Type: {:?}
File Type: {}
Size: {} bytes
Content Hash: {}

ASSIGNED ANALYST: {}
SPECIALTY: {}
ANALYSIS FOCUS: {:?}

DOCUMENT CONTENT:
---BEGIN DOCUMENT---
{}
---END DOCUMENT---

ANALYSIS REQUIREMENTS:
1. Provide PRIORITY RECOMMENDATION (Critical/High/Medium/Low) with reasoning
2. Identify KEY INSIGHTS (3-5 bullet points)
3. List ACTIONABLE RECOMMENDATIONS with implementation effort estimates
4. Identify SYSTEM INTEGRATION opportunities with existing CTAS infrastructure
5. Note any THREAT INDICATORS or security concerns
6. Assess DOCUMENT VALUE for the CTAS ecosystem

RESPONSE FORMAT:
Please provide structured analysis that can be parsed programmatically.
Use clear sections and bullet points.
Include confidence scores (0.0-1.0) for recommendations.

CONTEXT: This is part of the CTAS v6.6 Document Intelligence System.
The analysis should consider integration with:
- Hook folder system
- Document manager
- Communication protocols (UDP/RPC/IRC/iMessage)
- Elite team coordination
- Multi-modal intelligence processing
"#,
            document.record.id,
            document.record.metadata.file_type,
            document.record.metadata.file_type,
            document.record.metadata.size,
            document.record.id,
            persona.name.clone(),
            persona.specialty,
            persona.analysis_type,
            document.record.content
        );

        Ok(prompt)
    }

    /// Execute repo-prompt analysis
    pub async fn execute_repo_prompt_analysis(&self, prompt: &str) -> Result<String> {
        info!("🚀 Executing repo-prompt analysis");

        // Try to execute the repo-prompt binary
        let mut output = Command::new(&self.repo_prompt_path)
            .arg("--mode")
            .arg("document-analysis")
            .arg("--input")
            .arg("-") // Read from stdin
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // Write prompt to stdin
        if let Some(mut stdin) = output.stdin.take() {
            use std::io::Write;
            stdin.write_all(prompt.as_bytes())?;
        }

        let result = output.wait_with_output()?;

        if result.status.success() {
            Ok(String::from_utf8_lossy(&result.stdout).to_string())
        } else {
            let error = String::from_utf8_lossy(&result.stderr);
            warn!("⚠️ Repo-prompt execution failed: {}", error);
            
            // Fallback: Generate basic analysis
            self.generate_fallback_analysis(prompt)
        }
    }

    /// Generate fallback analysis if repo-prompt fails
    fn generate_fallback_analysis(&self, prompt: &str) -> Result<String> {
        warn!("🔄 Generating fallback analysis");

        // Extract document content for basic analysis
        let content = if let Some(start) = prompt.find("---BEGIN DOCUMENT---") {
            if let Some(end) = prompt.find("---END DOCUMENT---") {
                &prompt[start + 20..end]
            } else {
                ""
            }
        } else {
            ""
        };

        let analysis = format!(r#"
FALLBACK ANALYSIS GENERATED

PRIORITY RECOMMENDATION: Medium
REASON: Document processed through fallback analysis system

KEY INSIGHTS:
• Document contains {} characters
• Requires human review for detailed analysis
• Should be processed when repo-prompt system is available

RECOMMENDATIONS:
• Review document manually (Confidence: 1.0, Effort: Medium)
• Retry with full repo-prompt analysis when system is available
• Consider document type and content for appropriate routing

INTEGRATION OPPORTUNITIES:
• Document manager integration for storage and retrieval
• Hook system integration for automated processing
• Elite team assignment based on content analysis

THREAT INDICATORS:
• None identified in fallback analysis
• Full security analysis requires repo-prompt system

DOCUMENT VALUE: Medium - Requires detailed analysis to determine full value
"#, content.len());

        Ok(analysis)
    }
} 