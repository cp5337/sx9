use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub type NodeId = Uuid;
pub type EdgeId = Uuid;

/// Nonagon Method as defined in RFC-9302/9305
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonagonNode {
    pub id: NodeId,
    pub label: String,
    pub node_type: NodeType,
    /// Semantic Trivariate (Alpha): Context, Meaning, Intent
    pub alpha: [f64; 3], 
    /// Operational Trivariate (Beta): Phase, Intensity, Duration
    pub beta: [f64; 3],  
    /// Temporal Trivariate (Gamma): Historical, Current, Predictive
    pub gamma: [f64; 3], 
    pub properties: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub center_mass: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    Task, Actor, Object, Event, Attribute, Tool, Mission, Threat, Indicator
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum HD4Phase {
    Hunt = 0,
    Detect = 1,
    Disrupt = 2,
    Disable = 3,
    Dominate = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TethEdge {
    pub id: EdgeId,
    pub source: NodeId,
    pub target: NodeId,
    pub relationship: String,
    pub entropy_bits: f64, // TETH Metric
    pub confidence: f64,
}

impl TethEdge {
    pub fn validate_entropy(&self) -> bool {
        self.entropy_bits >= 2.5
    }
}

impl NonagonNode {
    pub fn new(label: String, node_type: NodeType) -> Self {
        Self {
            id: Uuid::now_v7(),
            label,
            node_type,
            alpha: [0.0; 3],
            beta: [0.0; 3],
            gamma: [0.0; 3],
            properties: HashMap::new(),
            created_at: Utc::now(),
            center_mass: 0.0,
        }
    }
    
    pub fn calculate_center(&mut self) {
        let sum_alpha: f64 = self.alpha.iter().sum();
        let sum_beta: f64 = self.beta.iter().sum();
        let sum_gamma: f64 = self.gamma.iter().sum();
        self.center_mass = (sum_alpha + sum_beta + sum_gamma) / 9.0;
    }
}
