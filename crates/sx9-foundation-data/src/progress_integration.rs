//! CTAS-7 Progress Integration
//! 
//! Integration with the progress monitor system for quality gates and progress tracking.

use std::collections::HashMap;
use tracing::{info, warn, error};
use chrono::Utc;
use reqwest::Client;

/// Progress Integration
#[derive(Debug)]
pub struct ProgressIntegration {
    pub progress_data: HashMap<String, ProgressData>,
    pub quality_gates: HashMap<String, QualityGate>,
    pub client: Client,
    pub progress_system_url: String,
}

/// Progress Data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProgressData {
    pub progress_id: String,
    pub stage_name: String,
    pub progress_percentage: f64,
    pub status: ProgressStatus,
    pub timestamp: chrono::DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

/// Quality Gate
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QualityGate {
    pub gate_id: String,
    pub gate_name: String,
    pub status: GateStatus,
    pub requirements: Vec<String>,
    pub passed_requirements: Vec<String>,
    pub timestamp: chrono::DateTime<Utc>,
}

/// Progress Status
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProgressStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
    Paused,
}

/// Gate Status
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GateStatus {
    Open,
    Closed,
    Warning,
    Critical,
}

impl ProgressIntegration {
    pub fn new() -> Self {
        Self {
            progress_data: HashMap::new(),
            quality_gates: HashMap::new(),
            client: Client::new(),
            progress_system_url: "standalone".to_string(),
        }
    }
    
    pub async fn sync_progress_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("🔄 Using internal progress data (standalone mode)");
        
        // In standalone mode, we use our own progress data
        // No external sync needed - progress is managed internally
        info!("✅ Progress system running in standalone mode");
        
        Ok(())
    }
    
    pub fn get_progress_summary(&self) -> ProgressSummary {
        let total_stages = self.progress_data.len();
        let completed_stages = self.progress_data
            .values()
            .filter(|data| data.status == ProgressStatus::Completed)
            .count();
        
        let in_progress_stages = self.progress_data
            .values()
            .filter(|data| data.status == ProgressStatus::InProgress)
            .count();
        
        let failed_stages = self.progress_data
            .values()
            .filter(|data| data.status == ProgressStatus::Failed)
            .count();
        
        let overall_progress = if total_stages > 0 {
            (completed_stages as f64 / total_stages as f64) * 100.0
        } else {
            0.0
        };
        
        ProgressSummary {
            total_stages,
            completed_stages,
            in_progress_stages,
            failed_stages,
            overall_progress,
            quality_gates_open: self.quality_gates
                .values()
                .filter(|gate| gate.status == GateStatus::Open)
                .count(),
            quality_gates_closed: self.quality_gates
                .values()
                .filter(|gate| gate.status == GateStatus::Closed)
                .count(),
        }
    }
    
    pub fn get_progress_data(&self) -> Vec<&ProgressData> {
        self.progress_data.values().collect()
    }
    
    pub fn get_quality_gates(&self) -> Vec<&QualityGate> {
        self.quality_gates.values().collect()
    }
    
    pub fn get_stage_progress(&self, stage_name: &str) -> Option<&ProgressData> {
        self.progress_data
            .values()
            .find(|data| data.stage_name == stage_name)
    }
    
    pub fn add_progress_data(&mut self, progress_data: ProgressData) {
        self.progress_data.insert(progress_data.progress_id.clone(), progress_data);
    }
    
    pub fn update_progress(&mut self, progress_id: &str, percentage: f64, status: ProgressStatus) {
        if let Some(data) = self.progress_data.get_mut(progress_id) {
            data.progress_percentage = percentage;
            data.status = status;
            data.timestamp = Utc::now();
        }
    }
    
    pub fn add_quality_gate(&mut self, quality_gate: QualityGate) {
        self.quality_gates.insert(quality_gate.gate_id.clone(), quality_gate);
    }
}

/// Progress Summary
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProgressSummary {
    pub total_stages: usize,
    pub completed_stages: usize,
    pub in_progress_stages: usize,
    pub failed_stages: usize,
    pub overall_progress: f64,
    pub quality_gates_open: usize,
    pub quality_gates_closed: usize,
}
