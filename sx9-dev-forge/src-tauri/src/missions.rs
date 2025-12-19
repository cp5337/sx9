//! Mission management and local storage for SX9 Dev Forge

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum MissionError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Mission not found: {0}")]
    NotFound(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MissionStatus {
    Drafted,
    Active,
    Checkpoint,
    Blocked,
    Completed,
    Failed,
    RolledBack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub id: String,
    pub title: String,
    pub prompt_type: String,
    pub persona: String,
    pub harness: String,
    pub status: MissionStatus,
    pub phase: String,
    pub priority: String,
    
    // Configuration
    pub objective: String,
    pub hard_constraints: Vec<String>,
    pub soft_constraints: Vec<String>,
    pub deliverables: Vec<String>,
    
    // Integration
    pub linear_issue_id: Option<String>,
    pub linear_issue_url: Option<String>,
    pub rfcs: Vec<String>,
    pub project_path: Option<PathBuf>,
    pub ide: Option<String>,
    
    // Tracking
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    
    // History
    pub checkpoints: Vec<Checkpoint>,
    pub artifacts: Vec<Artifact>,
    pub violations: Vec<Violation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub message: String,
    pub phase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub timestamp: DateTime<Utc>,
    pub constraint: String,
    pub description: String,
    pub severity: String,
}

impl Mission {
    pub fn new(
        title: String,
        prompt_type: String,
        persona: String,
        harness: String,
    ) -> Self {
        let now = Utc::now();
        let id = format!(
            "MSN-{}-{}",
            now.format("%Y%m%d"),
            &Uuid::new_v4().to_string()[..8]
        );

        Self {
            id,
            title,
            prompt_type,
            persona,
            harness,
            status: MissionStatus::Drafted,
            phase: "PLAN".to_string(),
            priority: "P2".to_string(),
            objective: String::new(),
            hard_constraints: vec![],
            soft_constraints: vec![],
            deliverables: vec![],
            linear_issue_id: None,
            linear_issue_url: None,
            rfcs: vec![],
            project_path: None,
            ide: None,
            created_at: now,
            updated_at: now,
            started_at: None,
            completed_at: None,
            checkpoints: vec![],
            artifacts: vec![],
            violations: vec![],
        }
    }

    pub fn start(&mut self) {
        self.status = MissionStatus::Active;
        self.started_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn checkpoint(&mut self, message: String) {
        let checkpoint = Checkpoint {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            message,
            phase: self.phase.clone(),
        };
        self.checkpoints.push(checkpoint);
        self.status = MissionStatus::Checkpoint;
        self.updated_at = Utc::now();
    }

    pub fn complete(&mut self) {
        self.status = MissionStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn fail(&mut self, reason: String) {
        self.status = MissionStatus::Failed;
        self.violations.push(Violation {
            timestamp: Utc::now(),
            constraint: "MISSION_FAILURE".to_string(),
            description: reason,
            severity: "CRITICAL".to_string(),
        });
        self.updated_at = Utc::now();
    }

    pub fn add_artifact(&mut self, name: String, path: PathBuf) {
        self.artifacts.push(Artifact {
            id: Uuid::new_v4().to_string(),
            name,
            path,
            created_at: Utc::now(),
        });
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MissionStore {
    missions: HashMap<String, Mission>,
    #[serde(skip)]
    store_path: PathBuf,
}

impl MissionStore {
    pub fn new(store_path: PathBuf) -> Self {
        Self {
            missions: HashMap::new(),
            store_path,
        }
    }

    pub fn load(store_path: PathBuf) -> Result<Self, MissionError> {
        if store_path.exists() {
            let content = std::fs::read_to_string(&store_path)?;
            let mut store: MissionStore = serde_json::from_str(&content)?;
            store.store_path = store_path;
            Ok(store)
        } else {
            Ok(Self::new(store_path))
        }
    }

    pub fn save(&self) -> Result<(), MissionError> {
        if let Some(parent) = self.store_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(&self)?;
        std::fs::write(&self.store_path, content)?;
        Ok(())
    }

    pub fn create(&mut self, mission: Mission) -> Result<String, MissionError> {
        let id = mission.id.clone();
        self.missions.insert(id.clone(), mission);
        self.save()?;
        Ok(id)
    }

    pub fn get(&self, id: &str) -> Option<&Mission> {
        self.missions.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Mission> {
        self.missions.get_mut(id)
    }

    pub fn update(&mut self, mission: Mission) -> Result<(), MissionError> {
        if !self.missions.contains_key(&mission.id) {
            return Err(MissionError::NotFound(mission.id));
        }
        self.missions.insert(mission.id.clone(), mission);
        self.save()?;
        Ok(())
    }

    pub fn delete(&mut self, id: &str) -> Result<(), MissionError> {
        if self.missions.remove(id).is_none() {
            return Err(MissionError::NotFound(id.to_string()));
        }
        self.save()?;
        Ok(())
    }

    pub fn list(&self) -> Vec<&Mission> {
        let mut missions: Vec<_> = self.missions.values().collect();
        missions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        missions
    }

    pub fn list_by_status(&self, status: MissionStatus) -> Vec<&Mission> {
        self.missions
            .values()
            .filter(|m| m.status == status)
            .collect()
    }

    pub fn active(&self) -> Vec<&Mission> {
        self.list_by_status(MissionStatus::Active)
    }
}

/// Get the default store path for missions
pub fn default_store_path() -> PathBuf {
    directories::ProjectDirs::from("com", "sx9", "dev-forge")
        .map(|dirs| dirs.data_dir().join("missions.json"))
        .unwrap_or_else(|| PathBuf::from("missions.json"))
}
