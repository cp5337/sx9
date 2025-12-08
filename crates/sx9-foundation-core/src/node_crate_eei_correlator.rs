//! Node-Crate-EEI Correlator - Intelligence Correlation Engine
//! 
//! This module correlates Essential Elements of Information (EEI)
//! with node and crate capabilities for operational intelligence.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

/// EEI correlation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EEICorrelationResult {
    pub correlation_id: String,
    pub mission_type: String,
    pub eeis: Vec<EEI>,
    pub correlated_crates: Vec<String>,
    pub correlated_nodes: Vec<String>,
    pub confidence_score: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Essential Element of Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EEI {
    pub eei_id: String,
    pub category: EEICategory,
    pub priority: u8,
    pub description: String,
    pub required_capabilities: Vec<String>,
    pub source_nodes: Vec<String>,
    pub target_crates: Vec<String>,
}

/// EEI categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EEICategory {
    ThreatIntelligence,
    NetworkSecurity,
    SystemSecurity,
    OperationalSecurity,
    PhysicalSecurity,
    PersonnelSecurity,
}

/// Node-Crate-EEI Correlator
pub struct NodeCrateEEICorrelator {
    eeis: HashMap<String, EEI>,
    node_capabilities: HashMap<String, Vec<String>>,
    crate_capabilities: HashMap<String, Vec<String>>,
    correlation_history: Vec<EEICorrelationResult>,
}

impl NodeCrateEEICorrelator {
    /// Create a new node-crate-EEI correlator
    pub fn new() -> Self {
        Self {
            eeis: HashMap::new(),
            node_capabilities: HashMap::new(),
            crate_capabilities: HashMap::new(),
            correlation_history: Vec::new(),
        }
    }

    /// Add EEI for correlation
    pub fn add_eei(&mut self, eei: EEI) {
        info!("📊 Adding EEI: {}", eei.eei_id);
        self.eeis.insert(eei.eei_id.clone(), eei);
    }

    /// Correlate EEI for a mission
    pub async fn correlate_mission_eei(&mut self, mission_type: &str) -> Result<EEICorrelationResult, Box<dyn std::error::Error>> {
        let correlation_id = format!("correlation-{}-{}", mission_type, chrono::Utc::now().timestamp());
        let mut relevant_eeis = Vec::new();
        let mut correlated_crates = Vec::new();
        let mut correlated_nodes = Vec::new();

        // Find relevant EEIs for mission type
        for eei in self.eeis.values() {
            if self.is_eei_relevant_for_mission(eei, mission_type) {
                relevant_eeis.push(eei.clone());
                
                // Correlate with crates and nodes
                for capability in &eei.required_capabilities {
                    if let Some(crates) = self.find_crates_with_capability(capability) {
                        correlated_crates.extend(crates);
                    }
                    if let Some(nodes) = self.find_nodes_with_capability(capability) {
                        correlated_nodes.extend(nodes);
                    }
                }
            }
        }

        // Remove duplicates
        correlated_crates.sort();
        correlated_crates.dedup();
        correlated_nodes.sort();
        correlated_nodes.dedup();

        let confidence_score = self.calculate_correlation_confidence(&relevant_eeis, &correlated_crates, &correlated_nodes);

        let eeis_count = relevant_eeis.len();
        let result = EEICorrelationResult {
            correlation_id,
            mission_type: mission_type.to_string(),
            eeis: relevant_eeis,
            correlated_crates,
            correlated_nodes,
            confidence_score,
            timestamp: chrono::Utc::now(),
        };

        self.correlation_history.push(result.clone());
        info!("🔗 Correlated {} EEIs for mission: {}", eeis_count, mission_type);

        Ok(result)
    }

    /// Check if EEI is relevant for mission type
    fn is_eei_relevant_for_mission(&self, eei: &EEI, mission_type: &str) -> bool {
        match mission_type {
            "portscan" => matches!(eei.category, EEICategory::NetworkSecurity),
            "osint" => matches!(eei.category, EEICategory::ThreatIntelligence),
            "threat_hunting" => matches!(eei.category, EEICategory::ThreatIntelligence | EEICategory::SystemSecurity),
            "incident_response" => matches!(eei.category, EEICategory::OperationalSecurity | EEICategory::SystemSecurity),
            _ => true, // Default to include all EEIs
        }
    }

    /// Find crates with specific capability
    fn find_crates_with_capability(&self, capability: &str) -> Option<Vec<String>> {
        let mut matching_crates = Vec::new();
        
        for (crate_name, capabilities) in &self.crate_capabilities {
            if capabilities.iter().any(|cap| cap.contains(capability)) {
                matching_crates.push(crate_name.clone());
            }
        }
        
        if matching_crates.is_empty() {
            None
        } else {
            Some(matching_crates)
        }
    }

    /// Find nodes with specific capability
    fn find_nodes_with_capability(&self, capability: &str) -> Option<Vec<String>> {
        let mut matching_nodes = Vec::new();
        
        for (node_name, capabilities) in &self.node_capabilities {
            if capabilities.iter().any(|cap| cap.contains(capability)) {
                matching_nodes.push(node_name.clone());
            }
        }
        
        if matching_nodes.is_empty() {
            None
        } else {
            Some(matching_nodes)
        }
    }

    /// Calculate correlation confidence score
    fn calculate_correlation_confidence(&self, eeis: &[EEI], crates: &[String], nodes: &[String]) -> f64 {
        if eeis.is_empty() {
            return 0.0;
        }

        let mut score = 0.0;
        let total_eeis = eeis.len() as f64;

        for eei in eeis {
            let mut eei_score = 0.0;
            
            // Score based on crate coverage
            let crate_coverage = eei.required_capabilities.iter()
                .filter(|cap| crates.iter().any(|c| c.contains(cap.as_str())))
                .count() as f64 / eei.required_capabilities.len() as f64;
            eei_score += crate_coverage * 0.6;

            // Score based on node coverage
            let node_coverage = eei.required_capabilities.iter()
                .filter(|cap| nodes.iter().any(|n| n.contains(cap.as_str())))
                .count() as f64 / eei.required_capabilities.len() as f64;
            eei_score += node_coverage * 0.4;

            score += eei_score;
        }

        score / total_eeis
    }

    /// Add node capabilities
    pub fn add_node_capabilities(&mut self, node_id: String, capabilities: Vec<String>) {
        self.node_capabilities.insert(node_id, capabilities);
    }

    /// Add crate capabilities
    pub fn add_crate_capabilities(&mut self, crate_name: String, capabilities: Vec<String>) {
        self.crate_capabilities.insert(crate_name, capabilities);
    }

    /// Get correlation history
    pub fn get_correlation_history(&self) -> &[EEICorrelationResult] {
        &self.correlation_history
    }
}

impl Default for NodeCrateEEICorrelator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correlator_creation() {
        let correlator = NodeCrateEEICorrelator::new();
        assert_eq!(correlator.eeis.len(), 0);
    }

    #[test]
    fn test_eei_relevance() {
        let correlator = NodeCrateEEICorrelator::new();
        let eei = EEI {
            eei_id: "test-eei".to_string(),
            category: EEICategory::NetworkSecurity,
            priority: 1,
            description: "Test EEI".to_string(),
            required_capabilities: vec!["network_scanning".to_string()],
            source_nodes: vec![],
            target_crates: vec![],
        };

        assert!(correlator.is_eei_relevant_for_mission(&eei, "portscan"));
        assert!(!correlator.is_eei_relevant_for_mission(&eei, "osint"));
    }
}
