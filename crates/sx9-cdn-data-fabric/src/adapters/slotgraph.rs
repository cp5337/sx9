//! SlotGraph ECS Adapter
//!
//! Handles communication with the SlotGraph ECS server.
//! This connects to the Legion ECS world for real-time entity data.

use crate::registry::DatabaseInfo;
use serde_json::{json, Value};

/// Query entities from SlotGraph ECS
pub async fn execute(info: &DatabaseInfo, query: &str) -> anyhow::Result<Vec<Value>> {
    let url = format!("http://{}:{}/api/query", info.host, info.port);

    let client = reqwest::Client::new();

    let body = json!({
        "query": query
    });

    let response = client.post(&url).json(&body).send().await?;

    if response.status().is_success() {
        let result: Value = response.json().await?;
        let entities = result
            .get("entities")
            .and_then(|e| e.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(entities)
    } else {
        Err(anyhow::anyhow!("SlotGraph query failed"))
    }
}

/// Get all entities with their components
pub async fn get_all_entities(info: &DatabaseInfo) -> anyhow::Result<Vec<Value>> {
    let url = format!("http://{}:{}/api/entities", info.host, info.port);

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let result: Value = response.json().await?;
        let entities = result
            .get("entities")
            .and_then(|e| e.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(entities)
    } else {
        Err(anyhow::anyhow!("Failed to get entities"))
    }
}

/// Get graph topology from SlotGraph
pub async fn get_graph(info: &DatabaseInfo) -> anyhow::Result<Value> {
    let url = format!("http://{}:{}/api/graph", info.host, info.port);

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let graph: Value = response.json().await?;
        Ok(graph)
    } else {
        Err(anyhow::anyhow!("Failed to get graph"))
    }
}

/// Get schema (component types) from SlotGraph
pub async fn get_schema(info: &DatabaseInfo) -> anyhow::Result<Value> {
    let url = format!("http://{}:{}/api/schema", info.host, info.port);

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let schema: Value = response.json().await?;
        Ok(json!({
            "db_type": "slotgraph",
            "components": schema
        }))
    } else {
        // Return default schema based on known components
        Ok(json!({
            "db_type": "slotgraph",
            "components": [
                {"name": "TriptyxId", "fields": ["uuid"]},
                {"name": "Position", "fields": ["x", "y", "z"]},
                {"name": "OodaPhase", "fields": ["observe", "orient", "decide", "act"]},
                {"name": "ActivityState", "fields": ["active", "dormant", "transitioning"]},
                {"name": "Convergence", "fields": ["h1", "h2", "combined"]},
                {"name": "Timestamp", "fields": ["created", "updated"]},
                {"name": "NodeKind", "fields": ["kind"]},
                {"name": "NodeMeta", "fields": ["label", "description", "tags"]}
            ]
        }))
    }
}

/// Check health of SlotGraph instance
pub async fn health_check(info: &DatabaseInfo) -> bool {
    let url = format!("http://{}:{}/health", info.host, info.port);

    match reqwest::get(&url).await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

/// Convert ECS entities to graph format
pub fn entities_to_graph(entities: &[Value]) -> Value {
    let nodes: Vec<Value> = entities
        .iter()
        .enumerate()
        .map(|(idx, entity)| {
            let id = entity
                .get("id")
                .and_then(|v| v.as_str())
                .map(String::from)
                .unwrap_or_else(|| format!("entity_{}", idx));

            let labels = entity
                .get("components")
                .and_then(|c| c.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|c| c.get("type").and_then(|t| t.as_str()))
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_else(|| vec!["Entity".to_string()]);

            json!({
                "id": id,
                "labels": labels,
                "properties": entity
            })
        })
        .collect();

    json!({
        "nodes": nodes,
        "edges": [],
        "stats": {
            "nodeCount": nodes.len(),
            "edgeCount": 0
        }
    })
}
