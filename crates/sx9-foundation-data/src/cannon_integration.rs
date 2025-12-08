//! Cannon Plug Integration - Connect streaming to Cannon Plug system

use serde::{Deserialize, Serialize};
use crate::StreamEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CannonPlugConfig {
    pub endpoint: String,
    pub api_key: String,
    pub enabled: bool,
    pub protocols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CannonPlugPayload {
    pub stream_id: String,
    pub usim_header: crate::USIMHeader,
    pub data: serde_json::Value,
    pub metadata: CannonMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CannonMetadata {
    pub source_station: String,
    pub protocol: String,
    pub hash_flow: String,
    pub priority: u8,
    pub ops_classification: String,
}

pub async fn cannon_integration_endpoint(
    axum::Json(request): axum::Json<serde_json::Value>,
) -> axum::Json<serde_json::Value> {
    let stream_data = &request["stream_data"];
    let cannon_config = &request["cannon_config"];
    
    // Create Cannon Plug payload
    let payload = CannonPlugPayload {
        stream_id: uuid::Uuid::new_v4().to_string(),
        usim_header: serde_json::from_value(request["usim_header"].clone()).unwrap_or_default(),
        data: stream_data.clone(),
        metadata: CannonMetadata {
            source_station: "STATION_5".to_string(),
            protocol: "USIM".to_string(),
            hash_flow: "blake3_cannon_flow".to_string(),
            priority: 255,
            ops_classification: "TACTICAL".to_string(),
        },
    };
    
    axum::Json(serde_json::json!({
        "status": "integrated",
        "cannon_stream_id": payload.stream_id,
        "metadata": payload.metadata
    }))
}

impl Default for crate::USIMHeader {
    fn default() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            hash_chain: String::new(),
            lisp_metadata: String::new(),
            protocol: String::new(),
            timestamp: chrono::Utc::now(),
            signature: String::new(),
        }
    }
}