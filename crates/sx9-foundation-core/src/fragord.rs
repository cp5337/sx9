//! Fragmentary Order (FRAGORD) Management

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FRAGORD {
    pub id: String,
    pub parent_opord_id: String,
    pub sequence_number: u32,
    pub dtg: DateTime<Utc>,
    pub changes: Vec<OrderChange>,
    pub new_tasks: Vec<String>,
    pub status: FRAGORDStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderChange {
    pub section: String,
    pub original_text: String,
    pub new_text: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FRAGORDStatus {
    Draft,
    Approved,
    Disseminated,
    Executed,
    Superseded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFRAGORDRequest {
    pub parent_opord_id: String,
    pub changes: Vec<OrderChange>,
    pub new_tasks: Vec<String>,
}

impl FRAGORD {
    pub fn new(
        parent_opord_id: String,
        changes: Vec<OrderChange>,
        new_tasks: Vec<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            parent_opord_id,
            sequence_number: 1,
            dtg: Utc::now(),
            changes,
            new_tasks,
            status: FRAGORDStatus::Draft,
            created_at: Utc::now(),
        }
    }

    pub fn add_change(&mut self, section: String, original: String,
                     new: String, reason: String) {
        self.changes.push(OrderChange {
            section,
            original_text: original,
            new_text: new,
            reason,
        });
    }

    pub fn approve(&mut self) {
        self.status = FRAGORDStatus::Approved;
    }

    pub fn disseminate(&mut self) {
        self.status = FRAGORDStatus::Disseminated;
    }
}