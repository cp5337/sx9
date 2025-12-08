//! Caldera ability execution and management

// use anyhow::Result; // Removed unused import
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub ability_id: String,
    pub name: String,
    pub description: String,
    pub tactic: String,
    pub technique_id: String,
    pub technique_name: String,
    pub executors: Vec<Executor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Executor {
    pub name: String,
    pub platform: String,
    pub command: String,
    pub language: Option<String>,
    pub cleanup: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityExecution {
    pub ability_id: String,
    pub paw: String,
    pub facts: Option<HashMap<String, String>>,
    pub executor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub link_id: String,
    pub operation_id: String,
    pub paw: String,
    pub ability_id: String,
    pub status: String,
    pub output: String,
    pub pid: Option<u32>,
}
