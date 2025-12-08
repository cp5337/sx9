//! Stream Blocks - Port allocation and mirror blocks following CTAS pattern

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamBlock {
    pub block_id: String,
    pub primary_port: u16,
    pub mirror_ports: Vec<u16>,
    pub protocol_support: Vec<StreamProtocol>,
    pub hash_flows: Vec<HashFlow>,
    pub ops_enabled: bool,
    pub deception_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamProtocol {
    HTTP,
    HTTPS,
    WebSocket,
    TCP,
    UDP,
    QUIC,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashFlow {
    pub flow_id: String,
    pub algorithm: HashAlgorithm,
    pub source_stations: Vec<String>,
    pub destination_cdns: Vec<String>,
    pub cannon_plug_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashAlgorithm {
    Blake3,
    SHA256,
    XXHash,
}

pub async fn start_workflow_endpoint(
    axum::Json(request): axum::Json<serde_json::Value>,
) -> axum::Json<serde_json::Value> {
    let workflow_type = request["workflow_type"].as_str().unwrap_or("default");
    let trigger_data = &request["trigger_data"];
    
    axum::Json(serde_json::json!({
        "status": "started",
        "workflow_id": uuid::Uuid::new_v4().to_string(),
        "workflow_type": workflow_type,
        "estimated_duration": "30s"
    }))
}