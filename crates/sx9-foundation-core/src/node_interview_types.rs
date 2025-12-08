//! Node Interview Types - Automated Capability Analysis
//! 
//! This module defines types for automated node interviews
//! to assess capabilities and operational readiness.

use serde::{Deserialize, Serialize};

/// Node interview result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInterviewResult {
    pub node_id: String,
    pub capabilities: Vec<NodeCapability>,
    pub readiness_score: f64,
    pub recommendations: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Node capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapability {
    pub name: String,
    pub category: CapabilityCategory,
    pub status: CapabilityStatus,
    pub version: String,
    pub description: String,
}

/// Capability categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapabilityCategory {
    Network,
    Security,
    Intelligence,
    Operations,
    Infrastructure,
}

/// Capability status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapabilityStatus {
    Available,
    Unavailable,
    Degraded,
    Unknown,
}
