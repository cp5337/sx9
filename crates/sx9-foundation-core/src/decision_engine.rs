//! Decision Engine for the CTAS Threat Reactor
//! 
//! Real AI-powered decision making using local Phi model for threat response
//! and adversary task emulation decisions.

use crate::models::*;
use crate::config::Config;
use crate::errors::ThreatReactorError;
use ollama_rs::Ollama;
use ollama_rs::generation::requests::GenerationRequest;
use tracing::{info, warn};
use std::time::Instant;

/// Decision Engine for AI-powered threat response decisions
pub struct DecisionEngine {
    config: Config,
    ollama: Ollama,
    phi_model: String,
}

impl DecisionEngine {
    /// Create a new decision engine with Phi integration
    pub async fn new(config: &Config) -> Result<Self, ThreatReactorError> {
        info!("Initializing Decision Engine with Phi...");
        
        let ollama = Ollama::default();
        let phi_model = "phi".to_string();
        
        Ok(Self {
            config: config.clone(),
            ollama,
            phi_model,
        })
    }
    
    /// Make AI-driven decision for threat response
    pub async fn make_decision(
        &self,
        analysis: &ThreatAnalysis,
        similar_threats: &[ThreatIntelligence],
    ) -> Result<ThreatResponse, ThreatReactorError> {
        info!("Making AI-driven decision with Phi");
        
        let start_time = Instant::now();
        
        let prompt = format!(
            "Make a cybersecurity response decision based on this threat analysis:\n\n\
            Threat Type: {:?}\n\
            Confidence Score: {}\n\
            Attack Vector: {:?}\n\
            Impact Assessment: {:?}\n\
            Recommended Actions: {:?}\n\n\
            Similar Threats Found: {}\n\n\
            Provide response decision in JSON format with: response_type, actions_taken (array), \
            success_rate (0-1), confidence_score (0-1), and kali_operations (array of tool names).",
            analysis.threat_type,
            analysis.confidence_score,
            analysis.attack_vector,
            analysis.impact_assessment,
            analysis.recommended_actions,
            similar_threats.len()
        );
        
        let response = self.ollama.generate(GenerationRequest::new(
            self.phi_model.clone(),
            prompt,
        )).await
        .map_err(|e| ThreatReactorError::AIEngine(format!("Phi decision failed: {}", e)))?;
        
        let decision_time = start_time.elapsed();
        info!("Phi decision completed in {:?}", decision_time);
        
        let threat_response = self.parse_decision_response(&response.response, analysis)?;
        
        Ok(threat_response)
    }
    
    /// Parse decision response from Phi
    fn parse_decision_response(&self, response: &str, analysis: &ThreatAnalysis) -> Result<ThreatResponse, ThreatReactorError> {
        let json_start = response.find('{').unwrap_or(0);
        let json_end = response.rfind('}').map(|i| i + 1).unwrap_or(response.len());
        let json_str = &response[json_start..json_end];
        
        let parsed: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| ThreatReactorError::AIEngine(format!("Failed to parse decision: {}", e)))?;
        
        Ok(ThreatResponse {
            response_id: uuid::Uuid::new_v4(),
            threat_event_id: analysis.threat_id,
            response_type: self.parse_response_type(&parsed["response_type"]),
            actions_taken: self.parse_response_actions(&parsed["actions_taken"]),
            kali_operations: vec![], // TODO: Parse from response
            success_rate: parsed["success_rate"].as_f64().unwrap_or(0.8) as f32,
            confidence_score: parsed["confidence_score"].as_f64().unwrap_or(0.7) as f32,
            response_timestamp: chrono::Utc::now(),
        })
    }
    
    /// Parse response type from Phi response
    fn parse_response_type(&self, value: &serde_json::Value) -> ResponseType {
        match value.as_str() {
            Some("Automated") => ResponseType::Automated,
            Some("Manual") => ResponseType::Manual,
            Some("Hybrid") => ResponseType::Hybrid,
            Some("Escalated") => ResponseType::Escalated,
            _ => ResponseType::Automated,
        }
    }
    
    /// Parse response actions from Phi response
    fn parse_response_actions(&self, value: &serde_json::Value) -> Vec<ResponseAction> {
        if let Some(array) = value.as_array() {
            array.iter().filter_map(|v| {
                match v.as_str() {
                    Some("NetworkBlock") => Some(ResponseAction::NetworkBlock),
                    Some("ProcessKill") => Some(ResponseAction::ProcessKill),
                    Some("FileQuarantine") => Some(ResponseAction::FileQuarantine),
                    Some("UserLockout") => Some(ResponseAction::UserLockout),
                    Some("SystemIsolation") => Some(ResponseAction::SystemIsolation),
                    Some("AlertGeneration") => Some(ResponseAction::AlertGeneration),
                    Some("LogAnalysis") => Some(ResponseAction::LogAnalysis),
                    Some("ThreatHunting") => Some(ResponseAction::ThreatHunting),
                    _ => None,
                }
            }).collect()
        } else {
            vec![ResponseAction::AlertGeneration]
        }
    }
}
