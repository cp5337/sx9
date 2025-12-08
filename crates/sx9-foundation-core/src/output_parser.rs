use anyhow::Result;
use regex::Regex;
use tracing::info;
use crate::types::ProcessedDocument;
use super::persona_manager::ElitePersona;
use super::analysis_engine::*;

#[derive(Debug, Clone)]
pub struct OutputParser;

impl OutputParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse repo-prompt output into structured analysis
    pub fn parse_repo_prompt_output(
        &self,
        raw_output: &str,
        document: &ProcessedDocument,
        persona: &ElitePersona,
    ) -> Result<RepoPromptAnalysis> {
        info!("📝 Parsing repo-prompt analysis output");

        // Extract priority recommendation
        let priority_recommendation = self.extract_priority_recommendation(raw_output)?;

        // Extract key insights
        let key_insights = self.extract_key_insights(raw_output);

        // Extract recommendations
        let recommendations = self.extract_recommendations(raw_output)?;

        // Extract integration points
        let integration_points = self.extract_integration_points(raw_output);

        // Extract threat indicators
        let threat_indicators = self.extract_threat_indicators(raw_output);

        // Calculate confidence score based on content analysis
        let confidence_score = self.calculate_confidence_score(raw_output, document);

        let analysis = RepoPromptAnalysis {
            analysis_id: uuid::Uuid::new_v4().to_string(),
            document_hash: document.record.id,
            assigned_persona: persona,
            confidence_score,
            priority_recommendation,
            key_insights,
            recommendations,
            integration_points,
            threat_indicators,
            analysis_timestamp: chrono::Utc::now(),
            processing_time_ms: 0, // Will be set by caller
            raw_output: raw_output.to_string(),
        };

        Ok(analysis)
    }

    /// Extract priority recommendation from analysis
    pub fn extract_priority_recommendation(&self, output: &str) -> Result<PriorityRecommendation> {
        let output_lower = output.to_lowercase();

        if output_lower.contains("critical") {
            Ok(PriorityRecommendation::Critical {
                reason: "Identified as critical priority by AI analysis".to_string(),
                immediate_actions: vec!["Review immediately".to_string()],
            })
        } else if output_lower.contains("high") {
            Ok(PriorityRecommendation::High {
                reason: "Identified as high priority by AI analysis".to_string(),
                review_within_hours: 24,
            })
        } else if output_lower.contains("low") {
            Ok(PriorityRecommendation::Low {
                reason: "Identified as low priority by AI analysis".to_string(),
                archive_suggested: false,
            })
        } else {
            Ok(PriorityRecommendation::Medium {
                reason: "Default medium priority assignment".to_string(),
                review_within_days: 7,
            })
        }
    }

    /// Extract key insights from analysis
    pub fn extract_key_insights(&self, output: &str) -> Vec<String> {
        let mut insights = Vec::with_capacity(15);

        // Look for bullet points or numbered lists
        let bullet_regex = Regex::new(r"(?m)^[•\-\*]\s*(.+)$").unwrap();
        for caps in bullet_regex.captures_iter(output) {
            if let Some(insight) = caps.get(1) {
                insights.push(insight.as_str().trim().to_string());
            }
        }

        // If no bullet points found, look for lines containing "insight"
        if insights.is_empty() {
            let insight_regex = Regex::new(r"(?i).*insight.*:?\s*(.+)").unwrap();
            for line in output.lines() {
                if let Some(caps) = insight_regex.captures(line) {
                    if let Some(insight) = caps.get(1) {
                        insights.push(insight.as_str().trim().to_string());
                    }
                }
            }
        }

        // Default insights if none found
        if insights.is_empty() {
            insights.push("Document requires detailed analysis".to_string());
            insights.push("Integration opportunities exist with CTAS systems".to_string());
        }

        insights
    }

    /// Extract recommendations from analysis
    pub fn extract_recommendations(&self, output: &str) -> Result<Vec<Recommendation>> {
        let mut recommendations = Vec::with_capacity(15);

        // Simple recommendation extraction - look for recommendation patterns
        let recommendation_patterns = [
            "should",
            "recommend",
            "suggest",
            "consider",
            "implement",
        ];

        for line in output.lines() {
            let line_lower = line.to_lowercase();
            for pattern in &recommendation_patterns {
                if line_lower.contains(pattern) {
                    recommendations.push(Recommendation {
                        category: RecommendationCategory::SystemIntegration,
                        title: "AI-Generated Recommendation".to_string(),
                        description: line.trim().to_string(),
                        confidence: 0.7,
                        implementation_effort: ImplementationEffort::Medium,
                        benefits: vec!["Improved system integration".to_string()],
                    });
                    break;
                }
            }
        }

        // Default recommendation if none found
        if recommendations.is_empty() {
            recommendations.push(Recommendation {
                category: RecommendationCategory::ProcessOptimization,
                title: "Manual Review Required".to_string(),
                description: "Document should be reviewed manually for detailed recommendations".to_string(),
                confidence: 1.0,
                implementation_effort: ImplementationEffort::Low,
                benefits: vec!["Ensure proper document handling".to_string()],
            });
        }

        Ok(recommendations)
    }

    /// Extract integration points from analysis
    pub fn extract_integration_points(&self, output: &str) -> Vec<IntegrationPoint> {
        let mut integration_points = Vec::with_capacity(30);

        // Look for mentions of system components
        let systems = [
            ("document manager", IntegrationType::DocumentManager),
            ("hook system", IntegrationType::HookSystem),
            ("communication", IntegrationType::CommunicationProtocol),
            ("database", IntegrationType::DatabaseSystem),
            ("api", IntegrationType::ExternalAPI),
            ("interface", IntegrationType::UserInterface),
        ];

        let output_lower = output.to_lowercase();
        for (system_name, integration_type) in systems {
            if output_lower.contains(system_name) {
                integration_points.push(IntegrationPoint {
                    system_name: system_name.to_string(),
                    integration_type,
                    complexity: 0.5,
                    benefits: vec!["Enhanced system integration".to_string()],
                    requirements: vec!["System compatibility check".to_string()],
                });
            }
        }

        integration_points
    }

    /// Extract threat indicators from analysis
    pub fn extract_threat_indicators(&self, output: &str) -> Vec<ThreatIndicator> {
        let mut threats = Vec::with_capacity(15);

        let threat_keywords = [
            ("vulnerability", ThreatType::SecurityVulnerability),
            ("exposure", ThreatType::DataExposure),
            ("weakness", ThreatType::SystemWeakness),
            ("gap", ThreatType::ProcessGap),
            ("compliance", ThreatType::ComplianceIssue),
        ];

        let output_lower = output.to_lowercase();
        for (keyword, threat_type) in threat_keywords {
            if output_lower.contains(keyword) {
                threats.push(ThreatIndicator {
                    indicator_type: threat_type,
                    severity: ThreatSeverity::Medium,
                    description: format!("Potential {} identified in analysis", keyword),
                    context: "Identified through AI analysis".to_string(),
                    mitigation_suggestions: vec!["Detailed review recommended".to_string()],
                });
            }
        }

        threats
    }

    /// Calculate confidence score for analysis
    pub fn calculate_confidence_score(&self, output: &str, _document: &ProcessedDocument) -> f64 {
        // Simple confidence calculation based on output length and content
        let output_len = output.len();
        let has_structured_sections = output.contains("RECOMMENDATION") || 
                                     output.contains("INSIGHTS") ||
                                     output.contains("ANALYSIS");

        let base_score = if has_structured_sections { 0.8 } else { 0.6 };
        let length_bonus = (output_len as f64 / 1000.0).min(0.2);

        (base_score + length_bonus).min(1.0)
    }
} 