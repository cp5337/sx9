use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnnContext {
    pub entropy: f32,
    pub latency_score: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnnAdvisory {
    pub confidence: f32,
    pub recommendation: String,
    pub reason_trace: Vec<String>,
    pub timestamp: DateTime<Utc>,
}
