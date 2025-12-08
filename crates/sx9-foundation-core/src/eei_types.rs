//! EEI Types - Essential Elements of Information
//! 
//! This module defines types for Essential Elements of Information
//! used in intelligence gathering and operational planning.

use serde::{Deserialize, Serialize};

/// Essential Element of Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EEI {
    pub id: String,
    pub category: EEICategory,
    pub priority: EEIPriority,
    pub description: String,
    pub requirements: Vec<String>,
    pub sources: Vec<String>,
    pub status: EEIStatus,
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

/// EEI priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EEIPriority {
    Critical,
    High,
    Medium,
    Low,
}

/// EEI status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EEIStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}
