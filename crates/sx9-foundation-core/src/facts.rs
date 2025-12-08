//! Caldera facts management

// use anyhow::Result; // Removed unused import
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fact {
    pub id: String,
    pub name: String,
    pub value: String,
    pub source: String,
    pub score: Option<i32>,
    pub r#trait: String,
    pub created: Option<String>,
    pub updated: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactSource {
    pub id: String,
    pub name: String,
    pub facts: Vec<Fact>,
    pub adjustments: Vec<FactAdjustment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactAdjustment {
    pub ability_id: String,
    pub r#trait: String,
    pub value: String,
    pub operator: String,
}
