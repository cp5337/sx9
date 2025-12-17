//! Neo4j Adapter
//!
//! Handles communication with Neo4j instances (for visualization only).
//! Note: GLAF is the primary data store, Neo4j is optional visualization layer.

use crate::registry::DatabaseInfo;
use serde_json::{json, Value};

/// Execute a Cypher query
///
/// Note: This requires neo4rs crate or HTTP API.
/// For now, we'll use the HTTP API for simplicity.
pub async fn execute(info: &DatabaseInfo, query: &str) -> anyhow::Result<Vec<Value>> {
    let url = format!("http://{}:{}/db/neo4j/tx/commit", info.host, 7474);

    let client = reqwest::Client::new();

    let body = json!({
        "statements": [{
            "statement": query,
            "resultDataContents": ["row", "graph"]
        }]
    });

    let response = client
        .post(&url)
        .basic_auth("neo4j", Some("password"))
        .json(&body)
        .send()
        .await?;

    let result: Value = response.json().await?;

    // Extract results
    let results = result
        .get("results")
        .and_then(|r| r.as_array())
        .map(|arr| {
            arr.iter()
                .flat_map(|r| {
                    r.get("data")
                        .and_then(|d| d.as_array())
                        .map(|rows| {
                            rows.iter()
                                .filter_map(|row| row.get("row").cloned())
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default()
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(results)
}

/// Get schema from Neo4j
pub async fn get_schema(info: &DatabaseInfo) -> anyhow::Result<Value> {
    // Get labels
    let labels_query = "CALL db.labels()";
    let labels = execute(info, labels_query).await.unwrap_or_default();

    // Get relationship types
    let rels_query = "CALL db.relationshipTypes()";
    let relationships = execute(info, rels_query).await.unwrap_or_default();

    Ok(json!({
        "db_type": "neo4j",
        "labels": labels,
        "relationship_types": relationships
    }))
}

/// Check health of Neo4j instance
pub async fn health_check(info: &DatabaseInfo) -> bool {
    let url = format!("http://{}:{}", info.host, 7474);

    match reqwest::get(&url).await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

/// Convert SurrealDB graph data to Neo4j import format
pub fn to_neo4j_import(nodes: &[Value], edges: &[Value]) -> Value {
    json!({
        "nodes": nodes.iter().map(|n| {
            json!({
                "id": n.get("id"),
                "labels": n.get("labels"),
                "properties": n.get("properties")
            })
        }).collect::<Vec<_>>(),
        "relationships": edges.iter().map(|e| {
            json!({
                "id": e.get("id"),
                "type": e.get("type"),
                "startNode": e.get("source"),
                "endNode": e.get("target"),
                "properties": e.get("properties")
            })
        }).collect::<Vec<_>>()
    })
}
