use anyhow::Result;
use serde::{Deserialize, Serialize};
use regex::Regex;

/// Elite team personas for specialized analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElitePersona {
    pub name: String,
    pub specialty: String,
    pub keywords: Vec<String>,
    pub analysis_type: AnalysisType,
}

/// Types of analysis available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisType {
    TechnicalArchitecture,
    SecurityThreatAnalysis,
    FinancialIntelligence,
    CulturalIntelligence,
    AISystemAnalysis,
    InfrastructureAnalysis,
}

/// Persona manager for elite team routing
#[derive(Debug, Clone)]
pub struct PersonaManager {
    available_personas: Vec<ElitePersona>,
}

impl PersonaManager {
    pub fn new() -> Self {
        Self {
            available_personas: Self::initialize_personas(),
        }
    }

    /// Initialize elite team personas
    fn initialize_personas() -> Vec<ElitePersona> {
        vec![
            ElitePersona {
                name: "Commander Hayes".to_string(),
                specialty: "Strategic Operations & Leadership".to_string(),
                keywords: vec!["strategy", "operations", "leadership", "command"].iter().map(|s| s.to_string()).collect(),
                analysis_type: AnalysisType::TechnicalArchitecture,
            },
            ElitePersona {
                name: "Dmitri Kozlov".to_string(),
                specialty: "APT & Advanced Threat Analysis".to_string(),
                keywords: vec!["apt", "threat", "security", "attack", "vulnerability"].iter().map(|s| s.to_string()).collect(),
                analysis_type: AnalysisType::SecurityThreatAnalysis,
            },
            ElitePersona {
                name: "James Sterling".to_string(),
                specialty: "Financial Intelligence & Analysis".to_string(),
                keywords: vec!["financial", "money", "blockchain", "transaction", "economic"].iter().map(|s| s.to_string()).collect(),
                analysis_type: AnalysisType::FinancialIntelligence,
            },
            ElitePersona {
                name: "Omar Al-Rashid".to_string(),
                specialty: "MENA Cultural & Linguistic Intelligence".to_string(),
                keywords: vec!["cultural", "linguistic", "mena", "social", "regional"].iter().map(|s| s.to_string()).collect(),
                analysis_type: AnalysisType::CulturalIntelligence,
            },
            ElitePersona {
                name: "Natasha Volkov".to_string(),
                specialty: "AI & Neural Systems Analysis".to_string(),
                keywords: vec!["ai", "neural", "machine learning", "algorithm", "automation"].iter().map(|s| s.to_string()).collect(),
                analysis_type: AnalysisType::AISystemAnalysis,
            },
            ElitePersona {
                name: "Emily Chen".to_string(),
                specialty: "Infrastructure & Systems Engineering".to_string(),
                keywords: vec!["infrastructure", "system", "network", "deployment", "architecture"].iter().map(|s| s.to_string()).collect(),
                analysis_type: AnalysisType::InfrastructureAnalysis,
            },
        ]
    }

    /// Select the best persona for analyzing this document
    pub fn select_persona(&self, content: &str) -> Result<ElitePersona> {
        let content_lower = content.to_lowercase();
        let mut best_match = (0, &self.available_personas[0]);

        for persona in &self.available_personas {
            let mut score = 0;
            
            // Count keyword matches
            for keyword in &persona.keywords {
                let keyword_regex = Regex::new(&format!(r"\b{}\b", keyword.to_lowercase()))?;
                score += keyword_regex.find_iter(&content_lower).count();
            }

            if score > best_match.0 {
                best_match = (score, persona);
            }
        }

        // If no clear winner, analyze content type
        if best_match.0 == 0 {
            if content_lower.contains("architecture") || content_lower.contains("system") {
                return Ok(self.available_personas.iter()
                    .find(|p| p.name == "Commander Hayes")
                    .unwrap()
                    .clone());
            } else if content_lower.contains("threat") || content_lower.contains("security") {
                return Ok(self.available_personas.iter()
                    .find(|p| p.name == "Dmitri Kozlov")
                    .unwrap()
                    .clone());
            }
        }

        Ok(best_match.1)
    }

    /// Get the number of available personas
    pub fn get_persona_count(&self) -> u32 {
        self.available_personas.len() as u32
    }
} 