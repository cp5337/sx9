//! CTAS-7 Performance Metrics
//! 
//! Performance metrics collection and analysis.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Performance Metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub metric_id: String,
    pub metric_name: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: DateTime<Utc>,
    pub confidence_interval: (f64, f64),
    pub standard_deviation: f64,
}

/// Performance Summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub summary_id: String,
    pub total_requests: u64,
    pub requests_per_second: f64,
    pub average_response_time: f64,
    pub p95_response_time: f64,
    pub p99_response_time: f64,
    pub error_rate: f64,
    pub throughput_mbps: f64,
    pub timestamp: DateTime<Utc>,
}

/// Performance Comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceComparison {
    pub comparison_id: String,
    pub baseline_metric: PerformanceMetric,
    pub comparison_metric: PerformanceMetric,
    pub improvement_ratio: f64,
    pub statistical_significance: bool,
    pub p_value: f64,
    pub created_at: DateTime<Utc>,
}

