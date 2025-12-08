//! File metadata and enums

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub path: String,
    pub size_bytes: u64,
    pub modified_at: DateTime<Utc>,
    pub file_type: FileType,
    pub priority: PriorityLevel,
    pub content_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileType {
    Code,
    Documentation,
    Configuration,
    Data,
    Binary,
    Archive,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PriorityLevel {
    Critical,
    High,
    Medium,
    Low,
    Archive,
}
