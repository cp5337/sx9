//! AI Engine for the CTAS Threat Reactor
//! 
//! Real AI integration using local Phi model via Ollama for threat analysis,
//! decision making, and adversary task emulation.

use crate::models::*;
use crate::config::Config;
use crate::errors::ThreatReactorError;
use ollama_rs::Ollama;
use ollama_rs::generation::requests::GenerationRequest;
use serde_json::json;
use tracing::{info, warn, error};
use std::time::Instant;

/// AI Engine for threat analysis and decision making
pub struct AIEngine {
    config: Config,
    ollama: Ollama,
    phi_model: String,
}

impl AIEngine {
    /// Create a new AI engine with local Phi integration
    pub async fn new(config: &Config) -> Result<Self, ThreatReactorError> {
        info!("Initializing AI Engine with local Phi model...");
        
        let ollama = Ollama::default();
        let phi_model = "phi".to_string();
        
        // Test phi connection
        match ollama.generate(GenerationRequest::new(
            phi_model.clone(),
            "Test connection".to_string(),
        )).await {
            Ok(_) => info!("✅ Phi model connection successful"),
            Err(e) => {
                warn!("⚠️ Phi model connection failed: {}", e);
                return Err(ThreatReactorError::AIEngine(format!("Phi connection failed: {}", e)));
            }
        }
        
        Ok(Self {
            config: config.clone(),
            ollama,
            phi_model,
        })
    }
    
    /// Analyze a threat event using Phi
    pub async fn analyze_threat(&self, event: &ThreatEvent) -> Result<ThreatAnalysis, ThreatReactorError> {
        info!("Analyzing threat event with Phi: {:?}", event.id);
        
        let start_time = Instant::now();
        
        let prompt = format!(
            "Analyze this cybersecurity threat event and provide a structured analysis:\n\n\
            Event ID: {}\n\
            Source: {:?}\n\
            Severity: {:?}\n\
            Category: {:?}\n\
            Raw Data: {}\n\n\
            Provide analysis in JSON format with: threat_type, confidence_score (0-1), \
            attack_vector, impact_assessment, recommended_actions (array), and kali_tools_needed (array).",
            event.id, event.source, event.severity, event.category, event.data.raw_data
        );
        
        let response = self.ollama.generate(GenerationRequest::new(
            self.phi_model.clone(),
            prompt,
        )).await
        .map_err(|e| ThreatReactorError::AIEngine(format!("Phi analysis failed: {}", e)))?;
        
        let analysis_time = start_time.elapsed();
        info!("Phi analysis completed in {:?}", analysis_time);
        
        // Parse Phi response and extract analysis
        let analysis = self.parse_phi_analysis(&response.response)?;
        
        Ok(analysis)
    }
    
    /// Generate embeddings using Phi for vector search
    pub async fn embed_query(&self, query: &str) -> Result<Vec<f32>, ThreatReactorError> {
        info!("Generating embeddings with Phi for: {}", query);
        
        let prompt = format!(
            "Convert this cybersecurity query into a numerical embedding vector: {}\n\n\
            Return only a JSON array of 768 floating point numbers between -1 and 1.",
            query
        );
        
        let response = self.ollama.generate(GenerationRequest::new(
            self.phi_model.clone(),
            prompt,
        )).await
        .map_err(|e| ThreatReactorError::AIEngine(format!("Phi embedding failed: {}", e)))?;
        
        // Parse embedding from response
        let embedding = self.parse_embedding(&response.response)?;
        
        Ok(embedding)
    }
    
    /// Assess threat data using Phi
    pub async fn assess_threat(&self, threat_data: &ThreatData) -> Result<ThreatAssessment, ThreatReactorError> {
        info!("Assessing threat data with Phi");
        
        let prompt = format!(
            "Assess this threat data and provide risk analysis:\n\n\
            Raw Data: {}\n\n\
            Provide assessment in JSON format with: risk_score (0-1), threat_level, \
            attack_probability (0-1), potential_impact, mitigation_effort, \
            and kali_tool_recommendations (array of tool names).",
            threat_data.raw_data
        );
        
        let response = self.ollama.generate(GenerationRequest::new(
            self.phi_model.clone(),
            prompt,
        )).await
        .map_err(|e| ThreatReactorError::AIEngine(format!("Phi assessment failed: {}", e)))?;
        
        let assessment = self.parse_threat_assessment(&response.response, threat_data)?;
        
        Ok(assessment)
    }
    
    /// Parse Phi analysis response
    fn parse_phi_analysis(&self, response: &str) -> Result<ThreatAnalysis, ThreatReactorError> {
        // Extract JSON from Phi response
        let json_start = response.find('{').unwrap_or(0);
        let json_end = response.rfind('}').map(|i| i + 1).unwrap_or(response.len());
        let json_str = &response[json_start..json_end];
        
        let parsed: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| ThreatReactorError::AIEngine(format!("Failed to parse Phi response: {}", e)))?;
        
        Ok(ThreatAnalysis {
            threat_id: uuid::Uuid::new_v4(),
            confidence_score: parsed["confidence_score"].as_f64().unwrap_or(0.8) as f32,
            threat_type: self.parse_threat_type(&parsed["threat_type"]),
            attack_vector: self.parse_attack_vector(&parsed["attack_vector"]),
            impact_assessment: self.parse_impact_assessment(&parsed["impact_assessment"]),
            recommended_actions: self.parse_recommended_actions(&parsed["recommended_actions"]),
            kali_tools_needed: vec![], // TODO: Parse from response
            analysis_timestamp: chrono::Utc::now(),
        })
    }
    
    /// Parse embedding from Phi response
    fn parse_embedding(&self, response: &str) -> Result<Vec<f32>, ThreatReactorError> {
        let json_start = response.find('[').unwrap_or(0);
        let json_end = response.rfind(']').map(|i| i + 1).unwrap_or(response.len());
        let json_str = &response[json_start..json_end];
        
        let embedding: Vec<f32> = serde_json::from_str(json_str)
            .map_err(|e| ThreatReactorError::AIEngine(format!("Failed to parse embedding: {}", e)))?;
        
        Ok(embedding)
    }
    
    /// Parse threat assessment from Phi response
    fn parse_threat_assessment(&self, response: &str, threat_data: &ThreatData) -> Result<ThreatAssessment, ThreatReactorError> {
        let json_start = response.find('{').unwrap_or(0);
        let json_end = response.rfind('}').map(|i| i + 1).unwrap_or(response.len());
        let json_str = &response[json_start..json_end];
        
        let parsed: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| ThreatReactorError::AIEngine(format!("Failed to parse assessment: {}", e)))?;
        
        Ok(ThreatAssessment {
            assessment_id: uuid::Uuid::new_v4(),
            threat_data: threat_data.clone(),
            risk_score: parsed["risk_score"].as_f64().unwrap_or(0.7) as f32,
            threat_level: self.parse_threat_level(&parsed["threat_level"]),
            attack_probability: parsed["attack_probability"].as_f64().unwrap_or(0.6) as f32,
            potential_impact: self.parse_impact_level(&parsed["potential_impact"]),
            mitigation_effort: self.parse_effort_level(&parsed["mitigation_effort"]),
            kali_tool_recommendations: vec![], // TODO: Parse from response
        })
    }
    
    // Helper methods for parsing enums
    fn parse_threat_type(&self, value: &serde_json::Value) -> ThreatType {
        match value.as_str() {
            Some("Malware") => ThreatType::Malware,
            Some("Phishing") => ThreatType::Phishing,
            Some("DDoS") => ThreatType::DDoS,
            Some("APT") => ThreatType::APT,
            Some("Ransomware") => ThreatType::Ransomware,
            Some("Botnet") => ThreatType::Botnet,
            Some("InsiderThreat") => ThreatType::InsiderThreat,
            Some("ZeroDay") => ThreatType::ZeroDay,
            _ => ThreatType::Known,
        }
    }
    
    fn parse_attack_vector(&self, value: &serde_json::Value) -> AttackVector {
        match value.as_str() {
            Some("Network") => AttackVector::Network,
            Some("Web") => AttackVector::Web,
            Some("Email") => AttackVector::Email,
            Some("Physical") => AttackVector::Physical,
            Some("Social") => AttackVector::Social,
            Some("SupplyChain") => AttackVector::SupplyChain,
            Some("Cloud") => AttackVector::Cloud,
            Some("Mobile") => AttackVector::Mobile,
            _ => AttackVector::Network,
        }
    }
    
    fn parse_impact_assessment(&self, value: &serde_json::Value) -> ImpactAssessment {
        match value.as_str() {
            Some("Minimal") => ImpactAssessment::Minimal,
            Some("Low") => ImpactAssessment::Low,
            Some("Medium") => ImpactAssessment::Medium,
            Some("High") => ImpactAssessment::High,
            Some("Critical") => ImpactAssessment::Critical,
            _ => ImpactAssessment::Medium,
        }
    }
    
    fn parse_recommended_actions(&self, value: &serde_json::Value) -> Vec<RecommendedAction> {
        if let Some(array) = value.as_array() {
            array.iter().filter_map(|v| {
                match v.as_str() {
                    Some("Block") => Some(RecommendedAction::Block),
                    Some("Monitor") => Some(RecommendedAction::Monitor),
                    Some("Investigate") => Some(RecommendedAction::Investigate),
                    Some("Patch") => Some(RecommendedAction::Patch),
                    Some("Isolate") => Some(RecommendedAction::Isolate),
                    Some("Alert") => Some(RecommendedAction::Alert),
                    Some("Hunt") => Some(RecommendedAction::Hunt),
                    _ => None,
                }
            }).collect()
        } else {
            vec![RecommendedAction::Monitor]
        }
    }
    
    fn parse_threat_level(&self, value: &serde_json::Value) -> ThreatLevel {
        match value.as_str() {
            Some("Benign") => ThreatLevel::Benign,
            Some("Suspicious") => ThreatLevel::Suspicious,
            Some("Malicious") => ThreatLevel::Malicious,
            Some("Critical") => ThreatLevel::Critical,
            _ => ThreatLevel::Suspicious,
        }
    }
    
    fn parse_impact_level(&self, value: &serde_json::Value) -> ImpactLevel {
        match value.as_str() {
            Some("None") => ImpactLevel::None,
            Some("Low") => ImpactLevel::Low,
            Some("Medium") => ImpactLevel::Medium,
            Some("High") => ImpactLevel::High,
            Some("Critical") => ImpactLevel::Critical,
            _ => ImpactLevel::Medium,
        }
    }
    
    fn parse_effort_level(&self, value: &serde_json::Value) -> EffortLevel {
        match value.as_str() {
            Some("Minimal") => EffortLevel::Minimal,
            Some("Low") => EffortLevel::Low,
            Some("Medium") => EffortLevel::Medium,
            Some("High") => EffortLevel::High,
            Some("Extreme") => EffortLevel::Extreme,
            _ => EffortLevel::Medium,
        }
    }
    
    /// Start the analysis loop
    pub async fn start_analysis_loop(&self) -> Result<(), ThreatReactorError> {
        info!("Starting AI analysis loop with Phi");
        Ok(())
    }
    
    /// Train AI models (not applicable for local Phi)
    pub async fn train_models(&self) -> Result<(), ThreatReactorError> {
        info!("Phi model training not required (pre-trained)");
        Ok(())
    }
}
