//! Core data types for CTAS-7 agent system

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Agent identifier with collision-resistant properties
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(pub Uuid);

impl AgentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AgentId {
    fn default() -> Self {
        Self::new()
    }
}

/// Message identifier for routing and correlation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(pub Uuid);

impl MessageId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Task identifier for work coordination
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub Uuid);

impl TaskId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Session identifier for agent interactions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Priority levels for message and task routing
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}

/// Message payload for agent communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: MessageId,
    pub from: AgentId,
    pub to: Option<AgentId>,
    pub content: String,
    pub priority: Priority,
    pub timestamp: DateTime<Utc>,
    pub session_id: Option<SessionId>,
}

/// Task definition for agent execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub agent_id: AgentId,
    pub description: String,
    pub priority: Priority,
    pub created_at: DateTime<Utc>,
    pub deadline: Option<DateTime<Utc>>,
    pub session_id: Option<SessionId>,
}

/// Task execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: TaskId,
    pub success: bool,
    pub output: String,
    pub completed_at: DateTime<Utc>,
    pub execution_time_ms: u64,
}

/// Agent status for health monitoring
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentStatus {
    Online,
    Offline,
    Busy,
    Error,
}

/// Agent metadata for registration and discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    pub id: AgentId,
    pub name: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub status: AgentStatus,
    pub last_seen: DateTime<Utc>,
}

/// USIM Transaction for Blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UsimTransaction {
    KeyRegistration {
        key_ref: KeyReference,
        registrar: String,
        timestamp: u64,
        signature: String,
    },
    BuildVerification {
        verifier: String,
        timestamp: u64,
        outcome: String,
    },
    KeyRevocation {
        key_fingerprint: String,
        revoker: String,
        timestamp: u64,
        reason: String,
    },
    TrustEndorsement {
        endorser: String,
        endorsed: String,
        trust_level: u8,
        timestamp: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyReference {
    pub fingerprint: String,
    pub algorithm: String,
}

/// Mining Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningResult {
    pub success: bool,
    pub hash: String,
    pub nonce: u32,
    pub mining_time_ms: u64,
    pub difficulty: u8,
}

/// Processed Document for Ingestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedDocument {
    pub id: String, // Trivariate Hash
    pub path: String,
    pub content_type: String,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRecord {
    pub id: String,
    pub status: String,
}

impl MiningResult {
    pub fn success(hash: String, nonce: u32, mining_time_ms: u64, difficulty: u8) -> Self {
        Self {
            success: true,
            hash,
            nonce,
            mining_time_ms,
            difficulty,
        }
    }

    pub fn failure() -> Self {
        Self {
            success: false,
            hash: "".to_string(),
            nonce: 0,
            mining_time_ms: 0,
            difficulty: 0,
        }
    }
}