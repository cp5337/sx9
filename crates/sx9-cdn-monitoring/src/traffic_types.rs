//! Traffic Analysis Types
//!
//! Core data structures for traffic analysis and threat detection.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::cyber_operations::ThreatLevel;

/// Suspicious Activity Detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousActivity {
    pub activity_id: String,
    pub activity_type: ActivityType,
    pub source_ip: String,
    pub country: String,
    pub timestamp: DateTime<Utc>,
    pub severity: ThreatLevel,
    pub description: String,
    pub indicators: Vec<String>,
    pub response_taken: Option<String>,
}

/// Activity Types for Classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ActivityType {
    PortScanning,
    VulnerabilityProbing,
    SQLInjection,
    XSSAttempt,
    DirectoryTraversal,
    BruteForceLogin,
    DDoSAttempt,
    BotnetActivity,
    UnknownUserAgent,
    SuspiciousGeolocation,
    RateLimitExceeded,
    MaliciousPayload,
}

/// Request Data for Analysis
#[derive(Debug, Clone)]
pub struct RequestData {
    pub source_ip: String,
    pub user_agent: String,
    pub path: String,
    pub payload: String,
    pub country: String,
    pub timestamp: DateTime<Utc>,
}

/// Analysis Result
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub threat_score: f64,
    pub indicators: Vec<String>,
    pub response_action: ResponseAction,
    pub confidence: f64,
    pub analysis_time: DateTime<Utc>,
}

/// Response Actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseAction {
    Log,
    Alert,
    Block,
    RateLimit,
    Redirect,
    Honeypot,
    Analyze,
    Quarantine,
}
