//! CTAS Unified Progress and Error Monitoring System
//! 
//! Provides consistent progress tracking, error monitoring, and operational feedback
//! across all CTAS crates for uniform user experience and operational intelligence.

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::{info, warn, error, debug, trace};

/// Progress tracking state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgressState {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Progress event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgressEvent {
    Started { message: String },
    Updated { message: String, progress: f64 },
    Completed { message: String, duration: Duration },
    Failed { message: String, error: String },
    Cancelled { message: String },
}

/// Progress tracker for individual operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressTracker {
    pub id: Uuid,
    pub name: String,
    pub state: ProgressState,
    pub progress: f64, // 0.0 to 1.0
    pub message: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub events: Vec<ProgressEvent>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ProgressTracker {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            state: ProgressState::Pending,
            progress: 0.0,
            message: String::new(),
            start_time: Utc::now(),
            end_time: None,
            events: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn start(&mut self, message: String) {
        self.state = ProgressState::Running;
        self.message = message.clone();
        self.start_time = Utc::now();
        self.events.push(ProgressEvent::Started { message });
        info!("🚀 Started: {}", self.name);
    }

    pub fn update(&mut self, progress: f64, message: String) {
        self.progress = progress.max(0.0).min(1.0);
        self.message = message.clone();
        self.events.push(ProgressEvent::Updated { 
            message, 
            progress: self.progress 
        });
        
        let percentage = (self.progress * 100.0) as u32;
        debug!("📊 {}: {}% - {}", self.name, percentage, self.message);
    }

    pub fn complete(&mut self, message: String) {
        self.state = ProgressState::Completed;
        self.progress = 1.0;
        self.message = message.clone();
        self.end_time = Some(Utc::now());
        
        let duration = (self.end_time.unwrap() - self.start_time).to_std().unwrap_or_default();
        self.events.push(ProgressEvent::Completed { 
            message, 
            duration 
        });
        
        info!("✅ Completed: {} ({:?})", self.name, duration);
    }

    pub fn fail(&mut self, message: String, error: String) {
        self.state = ProgressState::Failed;
        self.message = message.clone();
        self.end_time = Some(Utc::now());
        self.events.push(ProgressEvent::Failed { message, error: error.clone() });
        
        error!("❌ Failed: {} - {}", self.name, error);
    }

    pub fn cancel(&mut self, message: String) {
        self.state = ProgressState::Cancelled;
        self.message = message.clone();
        self.end_time = Some(Utc::now());
        self.events.push(ProgressEvent::Cancelled { message });
        
        warn!("⏹️  Cancelled: {}", self.name);
    }

    pub fn add_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }

    pub fn duration(&self) -> Option<Duration> {
        self.end_time.map(|end| (end - self.start_time).to_std().unwrap_or_default())
    }

    pub fn is_active(&self) -> bool {
        matches!(self.state, ProgressState::Running)
    }
}

/// Multi-stage progress tracker for complex operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiStageProgress {
    pub id: Uuid,
    pub name: String,
    pub stages: Vec<ProgressTracker>,
    pub current_stage: usize,
    pub overall_progress: f64,
    pub state: ProgressState,
}

impl MultiStageProgress {
    pub fn new(name: String, stages: Vec<String>) -> Self {
        let stage_trackers = stages.into_iter()
            .map(|stage_name| ProgressTracker::new(stage_name))
            .collect();
            
        Self {
            id: Uuid::new_v4(),
            name,
            stages: stage_trackers,
            current_stage: 0,
            overall_progress: 0.0,
            state: ProgressState::Pending,
        }
    }

    pub fn start_stage(&mut self, stage_index: usize, message: String) {
        if stage_index < self.stages.len() {
            self.current_stage = stage_index;
            self.stages[stage_index].start(message);
            self.state = ProgressState::Running;
            info!("🎯 Multi-stage: {} - Starting stage {}", self.name, stage_index + 1);
        }
    }

    pub fn update_stage(&mut self, stage_index: usize, progress: f64, message: String) {
        if stage_index < self.stages.len() {
            self.stages[stage_index].update(progress, message);
            self.update_overall_progress();
        }
    }

    pub fn complete_stage(&mut self, stage_index: usize, message: String) {
        if stage_index < self.stages.len() {
            self.stages[stage_index].complete(message);
            self.update_overall_progress();
            
            if stage_index == self.stages.len() - 1 {
                self.state = ProgressState::Completed;
                info!("🎉 Multi-stage completed: {}", self.name);
            }
        }
    }

    pub fn fail_stage(&mut self, stage_index: usize, message: String, error: String) {
        if stage_index < self.stages.len() {
            self.stages[stage_index].fail(message, error);
            self.state = ProgressState::Failed;
            error!("💥 Multi-stage failed: {} at stage {}", self.name, stage_index + 1);
        }
    }

    fn update_overall_progress(&mut self) {
        let total_stages = self.stages.len() as f64;
        let completed_stages = self.stages.iter()
            .filter(|stage| matches!(stage.state, ProgressState::Completed))
            .count() as f64;
        
        let current_stage_progress = if self.current_stage < self.stages.len() {
            self.stages[self.current_stage].progress
        } else {
            0.0
        };
        
        self.overall_progress = (completed_stages + current_stage_progress) / total_stages;
    }

    pub fn current_stage_tracker(&mut self) -> Option<&mut ProgressTracker> {
        self.stages.get_mut(self.current_stage)
    }
}

/// Global progress manager for system-wide monitoring
pub struct ProgressManager {
    trackers: Arc<Mutex<HashMap<Uuid, ProgressTracker>>>,
    multi_stages: Arc<Mutex<HashMap<Uuid, MultiStageProgress>>>,
}

impl ProgressManager {
    pub fn new() -> Self {
        Self {
            trackers: Arc::new(Mutex::new(HashMap::new())),
            multi_stages: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_tracker(&self, name: String) -> Uuid {
        let tracker = ProgressTracker::new(name);
        let id = tracker.id;
        
        if let Ok(mut trackers) = self.trackers.lock() {
            trackers.insert(id, tracker);
        }
        
        id
    }

    pub fn create_multi_stage(&self, name: String, stages: Vec<String>) -> Uuid {
        let multi_stage = MultiStageProgress::new(name, stages);
        let id = multi_stage.id;
        
        if let Ok(mut multi_stages) = self.multi_stages.lock() {
            multi_stages.insert(id, multi_stage);
        }
        
        id
    }

    pub fn get_tracker(&self, id: Uuid) -> Option<ProgressTracker> {
        if let Ok(trackers) = self.trackers.lock() {
            trackers.get(&id).cloned()
        } else {
            None
        }
    }

    pub fn get_multi_stage(&self, id: Uuid) -> Option<MultiStageProgress> {
        if let Ok(multi_stages) = self.multi_stages.lock() {
            multi_stages.get(&id).cloned()
        } else {
            None
        }
    }

    pub fn update_tracker(&self, id: Uuid, progress: f64, message: String) {
        if let Ok(mut trackers) = self.trackers.lock() {
            if let Some(tracker) = trackers.get_mut(&id) {
                tracker.update(progress, message);
            }
        }
    }

    pub fn complete_tracker(&self, id: Uuid, message: String) {
        if let Ok(mut trackers) = self.trackers.lock() {
            if let Some(tracker) = trackers.get_mut(&id) {
                tracker.complete(message);
            }
        }
    }

    pub fn fail_tracker(&self, id: Uuid, message: String, error: String) {
        if let Ok(mut trackers) = self.trackers.lock() {
            if let Some(tracker) = trackers.get_mut(&id) {
                tracker.fail(message, error);
            }
        }
    }

    pub fn get_active_trackers(&self) -> Vec<ProgressTracker> {
        if let Ok(trackers) = self.trackers.lock() {
            trackers.values()
                .filter(|tracker| tracker.is_active())
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_system_status(&self) -> SystemStatus {
        if let Ok(trackers) = self.trackers.lock() {
            let total = trackers.len();
            let active = trackers.values().filter(|t| t.is_active()).count();
            let completed = trackers.values()
                .filter(|t| matches!(t.state, ProgressState::Completed))
                .count();
            let failed = trackers.values()
                .filter(|t| matches!(t.state, ProgressState::Failed))
                .count();
            
            SystemStatus {
                total_operations: total,
                active_operations: active,
                completed_operations: completed,
                failed_operations: failed,
                success_rate: if total > 0 { completed as f64 / total as f64 } else { 0.0 },
            }
        } else {
            SystemStatus::default()
        }
    }
}

/// System-wide status summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub total_operations: usize,
    pub active_operations: usize,
    pub completed_operations: usize,
    pub failed_operations: usize,
    pub success_rate: f64,
}

impl Default for SystemStatus {
    fn default() -> Self {
        Self {
            total_operations: 0,
            active_operations: 0,
            completed_operations: 0,
            failed_operations: 0,
            success_rate: 0.0,
        }
    }
}

/// Global progress manager instance
lazy_static::lazy_static! {
    pub static ref PROGRESS_MANAGER: ProgressManager = ProgressManager::new();
}

/// Convenience macros for progress tracking
#[macro_export]
macro_rules! progress_track {
    ($name:expr, $block:expr) => {{
        let tracker_id = $crate::progress::PROGRESS_MANAGER.create_tracker($name.to_string());
        $crate::progress::PROGRESS_MANAGER.update_tracker(tracker_id, 0.0, "Starting operation".to_string());
        
        let result = $block;
        
        match &result {
            Ok(_) => {
                $crate::progress::PROGRESS_MANAGER.complete_tracker(tracker_id, "Operation completed successfully".to_string());
            }
            Err(e) => {
                $crate::progress::PROGRESS_MANAGER.fail_tracker(tracker_id, "Operation failed".to_string(), e.to_string());
            }
        }
        
        result
    }};
}

#[macro_export]
macro_rules! progress_update {
    ($progress:expr, $message:expr) => {
        // This would need to be implemented with thread-local storage for current tracker
        trace!("Progress: {}% - {}", ($progress * 100.0) as u32, $message);
    };
}
