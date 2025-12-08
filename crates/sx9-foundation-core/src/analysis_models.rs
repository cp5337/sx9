//! CTAS-7 Statistical Analysis Models
//! 
//! Data models for statistical analysis, ML models, and performance metrics.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Statistical Analysis Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisRequest {
    pub analysis_name: String,
    pub analysis_type: AnalysisType,
    pub data_source: String,
    pub parameters: HashMap<String, Value>,
}

/// Statistical Analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalAnalysis {
    pub analysis_id: String,
    pub analysis_name: String,
    pub analysis_type: AnalysisType,
    pub data_source: String,
    pub parameters: HashMap<String, Value>,
    pub results: AnalysisResults,
    pub created_at: DateTime<Utc>,
    pub status: AnalysisStatus,
}

/// Analysis Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisType {
    PerformanceComparison,
    HashAlgorithmAnalysis,
    AnomalyDetection,
    BehavioralAnalysis,
    ThreatClassification,
    GeolocationAnalysis,
    UserAgentProfiling,
    Custom(String),
}

/// Analysis Results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResults {
    pub mean: f64,
    pub median: f64,
    pub standard_deviation: f64,
    pub confidence_interval: (f64, f64),
    pub p_value: f64,
    pub t_statistic: f64,
    pub sample_size: usize,
}

/// Analysis Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

/// ML Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLModel {
    pub model_id: String,
    pub model_name: String,
    pub model_type: MLModelType,
    pub accuracy: f64,
    pub last_trained: DateTime<Utc>,
    pub active: bool,
}

/// ML Model Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MLModelType {
    AnomalyDetection,
    BehavioralAnalysis,
    ThreatClassification,
    GeolocationAnalysis,
    UserAgentProfiling,
}

/// Hash Comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashComparison {
    pub comparison_id: String,
    pub algorithm_a: String,
    pub algorithm_b: String,
    pub performance_ratio: f64,
    pub t_statistic: f64,
    pub p_value: f64,
    pub confidence_interval: (f64, f64),
    pub sample_size: usize,
    pub statistical_power: f64,
    pub significance_level: f64,
    pub created_at: DateTime<Utc>,
}

/// Statistical Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalReport {
    pub report_id: String,
    pub report_name: String,
    pub study_type: String,
    pub sample_size: usize,
    pub statistical_power: f64,
    pub significance_level: f64,
    pub confidence_interval: u8,
    pub key_findings: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// Statistical Error
#[derive(Debug, thiserror::Error)]
pub enum StatisticalError {
    #[error("Analysis not found: {0}")]
    AnalysisNotFound(String),
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),
    #[error("ML model error: {0}")]
    MLModelError(String),
    #[error("Statistical computation error: {0}")]
    ComputationError(String),
}

use serde_json::Value;
use std::collections::HashMap;

