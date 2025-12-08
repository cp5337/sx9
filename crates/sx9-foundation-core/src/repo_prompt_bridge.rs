/*
// ┌─────────────────────────────────────────────────────────────┐
// │ █████████████████ CTAS USIM HEADER ███████████████████████ │
// ├─────────────────────────────────────────────────────────────┤
// │ 🔖 hash_id      : USIM-REPO-PROMPT-BRIDGE-0001             │
// │ 📁 domain       : AI Analysis, Document Intelligence       │
// │ 🧠 description  : Bridge to CTAS repo-prompt system for    │
// │                  intelligent document analysis and review  │
// │ 🕸️ hash_type    : UUID → CTAS-repo-prompt integration      │
// │ 🔄 parent_node  : NODE_DOCUMENT_INTELLIGENCE                │
// │ 🧩 dependencies : tokio, serde_json, regex, anyhow         │
// │ 🔧 tool_usage   : AI analysis, recommendations, triage     │
// │ 📡 input_type   : Documents, code, markdown content        │
// │ 🧪 test_status  : development                               │
// │ 🧠 cognitive_fn : AI analysis, pattern recognition         │
// │ ⌛ TTL Policy   : 6.5 Persistent                            │
// └─────────────────────────────────────────────────────────────┘
*/

use crate::types::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::collections::HashMap;
use regex::Regex;
use tracing::{info, warn, error};
use std::path::Path;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Import the new sub-modules
use crate::persona_manager::*;
use crate::analysis_engine::*;
use crate::output_parser::*;
use crate::cache_manager::*;

#[derive(Debug, Clone)]
pub struct RepoPromptBridge {
    /// Path to the repo-prompt executable
    repo_prompt_path: String,
    /// Persona manager for elite team routing
    persona_manager: PersonaManager,
    /// Analysis engine for processing
    analysis_engine: AnalysisEngine,
    /// Output parser for structured results
    output_parser: OutputParser,
    /// Cache manager for performance
    cache_manager: CacheManager,
}

impl RepoPromptBridge {
    /// Create new repo-prompt bridge
    pub fn new() -> Self {
        let repo_prompt_path = std::env::var("CTAS_REPO_PROMPT_PATH")
            .unwrap_or_else(|_| "../ctas-repo-prompt/target/release/ctas-repo-prompt".to_string());

        Self {
            repo_prompt_path: repo_prompt_path,
            persona_manager: PersonaManager::new(),
            analysis_engine: AnalysisEngine::new(repo_prompt_path),
            output_parser: OutputParser::new(),
            cache_manager: CacheManager::new(),
        }
    }

    /// Analyze document with repo-prompt system
    pub async fn analyze_document(&mut self, document: &ProcessedDocument) -> Result<RepoPromptAnalysis> {
        let start_time = std::time::Instant::now();

        info!("🧠 Starting repo-prompt analysis for document: {}", document.record.id);

        // Check cache first
        if let Some(cached) = self.cache_manager.get_cached_analysis(&document.record.id) {
            info!("📋 Using cached analysis for document");
            return Ok(cached);
        }

        // Select best persona for this document
        let assigned_persona = self.persona_manager.select_persona(&document.record.content)?;

        // Prepare analysis prompt
        let analysis_prompt = self.analysis_engine.create_analysis_prompt(document, &assigned_persona)?;

        // Execute repo-prompt analysis
        let raw_output = self.analysis_engine.execute_repo_prompt_analysis(&analysis_prompt).await?;

        // Parse and structure the output
        let mut analysis = self.output_parser.parse_repo_prompt_output(
            &raw_output, 
            document, 
            &assigned_persona
        )?;

        analysis.processing_time_ms = start_time.elapsed().as_millis() as u64;
        analysis.analysis_timestamp = chrono::Utc::now();

        // Cache the result
        self.cache_manager.cache_analysis(&document.record.id, &analysis);

        info!("✅ Completed repo-prompt analysis in {}ms", analysis.processing_time_ms);

        Ok(analysis)
    }

    /// Get analysis statistics
    pub fn get_analysis_stats(&self) -> AnalysisStats {
        AnalysisStats {
            total_analyses: self.cache_manager.get_total_analyses(),
            cached_analyses: self.cache_manager.get_cached_count(),
            available_personas: self.persona_manager.get_persona_count(),
            repo_prompt_path: self.repo_prompt_path,
        }
    }
}

/// Statistics for analysis system
#[derive(Debug, Serialize)]
pub struct AnalysisStats {
    pub total_analyses: u64,
    pub cached_analyses: u64,
    pub available_personas: u32,
    pub repo_prompt_path: String,
}

// Re-export main types from sub-modules
pub use persona_manager::{ElitePersona, AnalysisType};
pub use analysis_engine::{RepoPromptAnalysis, PriorityRecommendation, Recommendation, RecommendationCategory, ImplementationEffort, IntegrationPoint, IntegrationType, ThreatIndicator, ThreatType, ThreatSeverity};
pub use output_parser::*;
pub use cache_manager::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{DocumentType, ProcessedDocument};

    #[test]
    fn test_persona_selection() {
        let bridge = RepoPromptBridge::new();
        
        let content = "This document discusses financial blockchain analysis and transaction monitoring";
        let persona = bridge.persona_manager.select_persona(content).unwrap();
        
        assert_eq!(persona.name.clone(), "James Sterling");
    }

    #[test]
    fn test_priority_extraction() {
        let bridge = RepoPromptBridge::new();
        
        let output = "PRIORITY RECOMMENDATION: Critical\nThis is a critical security issue";
        let priority = bridge.output_parser.extract_priority_recommendation(output).unwrap();
        
        match priority {
            PriorityRecommendation::Critical { .. } => {},
            _ => panic!("Expected critical priority"),
        }
    }

    #[test]
    fn test_insights_extraction() {
        let bridge = RepoPromptBridge::new();
        
        let output = "KEY INSIGHTS:\n• First insight\n• Second insight\n• Third insight";
        let insights = bridge.output_parser.extract_key_insights(output);
        
        assert_eq!(insights.len(), 3);
        assert_eq!(insights[0], "First insight");
    }
}
