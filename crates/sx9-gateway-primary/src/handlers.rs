//! WebSocket message handlers
//!
//! Each handler maps a WsMessage to the appropriate SX9 backend.

use anyhow::Result;
use std::time::Instant;

use crate::protocol::*;
use crate::state::{ports, SharedState};

/// Handle incoming WebSocket messages
pub async fn handle_message(msg: WsMessage, state: SharedState) -> Result<WsResponse> {
    match msg {
        // ═══════════════════════════════════════════════════════════════════
        // DATABASE OPERATIONS
        // ═══════════════════════════════════════════════════════════════════
        WsMessage::Query { db, query, params } => handle_query(db, query, params, state).await,

        WsMessage::Subscribe { db, table, filter } => {
            handle_subscribe(db, table, filter, state).await
        }

        WsMessage::Unsubscribe { db, table } => handle_unsubscribe(db, table, state).await,

        // ═══════════════════════════════════════════════════════════════════
        // GRAPH OPERATIONS
        // ═══════════════════════════════════════════════════════════════════
        WsMessage::GetGraph { filter } => handle_get_graph(filter, state).await,

        WsMessage::GetFusionNodes { threshold } => handle_get_fusion_nodes(threshold, state).await,

        WsMessage::ExpandNode { node_id, depth } => handle_expand_node(node_id, depth, state).await,

        WsMessage::RunCorrelation { source_ids } => handle_run_correlation(source_ids, state).await,

        // ═══════════════════════════════════════════════════════════════════
        // WORKFLOW OPERATIONS
        // ═══════════════════════════════════════════════════════════════════
        WsMessage::GetWorkflows => handle_get_workflows(state).await,

        WsMessage::GetWorkflow { id } => handle_get_workflow(id, state).await,

        WsMessage::StartWorkflow { id, input } => handle_start_workflow(id, input, state).await,

        WsMessage::StopWorkflow { id } => handle_stop_workflow(id, state).await,

        WsMessage::GetPlasmaState => handle_get_plasma_state(state).await,

        WsMessage::SubscribePlasma => handle_subscribe_plasma(state).await,

        // ═══════════════════════════════════════════════════════════════════
        // HEALTH & CONNECTION
        // ═══════════════════════════════════════════════════════════════════
        WsMessage::GetConnections => handle_get_connections(state).await,

        WsMessage::TestConnection { db } => handle_test_connection(db, state).await,

        WsMessage::Ping => Ok(WsResponse::Pong {
            server_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }),
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DATABASE HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

async fn handle_query(
    db: Database,
    query: String,
    _params: Option<serde_json::Value>,
    state: SharedState,
) -> Result<WsResponse> {
    let start = Instant::now();

    match db {
        Database::Surrealdb => {
            let surreal_guard = state.surrealdb.read().await;
            if let Some(ref surreal) = *surreal_guard {
                match surreal.query(&query).await {
                    Ok(mut response) => {
                        let rows: Vec<serde_json::Value> = response.take(0).unwrap_or_default();
                        Ok(WsResponse::QueryResult {
                            db,
                            rows,
                            latency_ms: start.elapsed().as_secs_f64() * 1000.0,
                            cached: false,
                        })
                    }
                    Err(e) => Ok(WsResponse::Error {
                        code: "QUERY_ERROR".to_string(),
                        message: e.to_string(),
                        details: None,
                    }),
                }
            } else {
                Ok(WsResponse::Error {
                    code: "NOT_CONNECTED".to_string(),
                    message: "SurrealDB not connected".to_string(),
                    details: None,
                })
            }
        }

        // TODO: Implement other database handlers
        _ => Ok(WsResponse::Error {
            code: "NOT_IMPLEMENTED".to_string(),
            message: format!("{:?} queries not yet implemented", db),
            details: None,
        }),
    }
}

async fn handle_subscribe(
    _db: Database,
    _table: String,
    _filter: Option<String>,
    _state: SharedState,
) -> Result<WsResponse> {
    // TODO: Implement real-time subscriptions via NATS
    Ok(WsResponse::Error {
        code: "NOT_IMPLEMENTED".to_string(),
        message: "Subscriptions not yet implemented".to_string(),
        details: None,
    })
}

async fn handle_unsubscribe(
    _db: Database,
    _table: String,
    _state: SharedState,
) -> Result<WsResponse> {
    Ok(WsResponse::Error {
        code: "NOT_IMPLEMENTED".to_string(),
        message: "Unsubscribe not yet implemented".to_string(),
        details: None,
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// GRAPH HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

async fn handle_get_graph(filter: GraphFilter, state: SharedState) -> Result<WsResponse> {
    // Build SurrealQL query based on filter
    let mut query = String::from("SELECT * FROM entity");

    if let Some(ref node_type) = filter.node_type {
        query.push_str(&format!(" WHERE type = '{}'", node_type));
    }

    if filter.fusion_only {
        query.push_str(" WHERE fusion_score IS NOT NULL");
    }

    query.push_str(" FETCH edges");

    let surreal_guard = state.surrealdb.read().await;
    if let Some(surreal) = surreal_guard.as_ref() {
        match surreal.query(&query).await {
            Ok(mut response) => {
                let entities: Vec<serde_json::Value> = response.take(0).unwrap_or_default();

                // Transform to GraphNode/GraphEdge format
                let nodes: Vec<GraphNode> = entities
                    .iter()
                    .filter_map(|e| {
                        Some(GraphNode {
                            id: e.get("id")?.as_str()?.to_string(),
                            label: e
                                .get("name")
                                .and_then(|n| n.as_str())
                                .unwrap_or("")
                                .to_string(),
                            node_type: e
                                .get("type")
                                .and_then(|t| t.as_str())
                                .unwrap_or("default")
                                .to_string(),
                            shape: if e.get("fusion_score").is_some() {
                                "nonagon"
                            } else {
                                "circle"
                            }
                            .to_string(),
                            color: if e.get("fusion_score").is_some() {
                                "#00ffff"
                            } else {
                                Database::Surrealdb.brand_color()
                            }
                            .to_string(),
                            size: 1.0,
                            trivariate_hash: e
                                .get("trivariate_hash")
                                .and_then(|h| h.as_str())
                                .map(String::from),
                            source_db: Database::Surrealdb,
                            properties: e.clone(),
                        })
                    })
                    .collect();

                // Extract edges from the fetched data
                let edges: Vec<GraphEdge> = vec![]; // TODO: Parse edges from response

                Ok(WsResponse::GraphData { nodes, edges })
            }
            Err(e) => Ok(WsResponse::Error {
                code: "GRAPH_ERROR".to_string(),
                message: e.to_string(),
                details: None,
            }),
        }
    } else {
        Ok(WsResponse::Error {
            code: "NOT_CONNECTED".to_string(),
            message: "SurrealDB not connected".to_string(),
            details: None,
        })
    }
}

async fn handle_get_fusion_nodes(threshold: f32, state: SharedState) -> Result<WsResponse> {
    let query = format!(
        "SELECT * FROM fusion_node WHERE fusion_score >= {} ORDER BY fusion_score DESC",
        threshold
    );

    let surreal_guard = state.surrealdb.read().await;
    if let Some(surreal) = surreal_guard.as_ref() {
        match surreal.query(&query).await {
            Ok(mut response) => {
                let raw: Vec<serde_json::Value> = response.take(0).unwrap_or_default();

                let nodes: Vec<FusionNode> = raw
                    .iter()
                    .filter_map(|n| {
                        Some(FusionNode {
                            id: n.get("id")?.as_str()?.to_string(),
                            trivariate_hash: n.get("trivariate_hash")?.as_str()?.to_string(),
                            fusion_score: n.get("fusion_score")?.as_f64()? as f32,
                            fusion_method: n
                                .get("fusion_method")
                                .and_then(|m| m.as_str())
                                .unwrap_or("hash")
                                .to_string(),
                            sources: vec![], // TODO: Parse sources
                            created_at: n.get("created_at").and_then(|t| t.as_u64()).unwrap_or(0),
                            last_correlated: n
                                .get("last_correlated")
                                .and_then(|t| t.as_u64())
                                .unwrap_or(0),
                        })
                    })
                    .collect();

                Ok(WsResponse::FusionNodes { nodes })
            }
            Err(e) => Ok(WsResponse::Error {
                code: "FUSION_ERROR".to_string(),
                message: e.to_string(),
                details: None,
            }),
        }
    } else {
        Ok(WsResponse::Error {
            code: "NOT_CONNECTED".to_string(),
            message: "SurrealDB not connected".to_string(),
            details: None,
        })
    }
}

async fn handle_expand_node(node_id: String, depth: u32, state: SharedState) -> Result<WsResponse> {
    let query = format!(
        "SELECT ->relates_to->(entity WHERE true LIMIT {}) AS neighbors FROM entity:{}",
        depth * 10,
        node_id
    );

    let surreal_guard = state.surrealdb.read().await;
    if let Some(surreal) = surreal_guard.as_ref() {
        match surreal.query(&query).await {
            Ok(mut response) => {
                let _raw: Vec<serde_json::Value> = response.take(0).unwrap_or_default();
                // TODO: Transform to graph format
                Ok(WsResponse::GraphData {
                    nodes: vec![],
                    edges: vec![],
                })
            }
            Err(e) => Ok(WsResponse::Error {
                code: "EXPAND_ERROR".to_string(),
                message: e.to_string(),
                details: None,
            }),
        }
    } else {
        Ok(WsResponse::Error {
            code: "NOT_CONNECTED".to_string(),
            message: "SurrealDB not connected".to_string(),
            details: None,
        })
    }
}

async fn handle_run_correlation(
    _source_ids: Vec<String>,
    _state: SharedState,
) -> Result<WsResponse> {
    // TODO: Implement GLAF correlation via sx9-atlas-bus
    Ok(WsResponse::Error {
        code: "NOT_IMPLEMENTED".to_string(),
        message: "Correlation not yet implemented".to_string(),
        details: None,
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// WORKFLOW HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

async fn handle_get_workflows(_state: SharedState) -> Result<WsResponse> {
    // TODO: Query Forge engine for workflows
    Ok(WsResponse::Workflows {
        workflows: vec![WorkflowSummary {
            id: uuid::Uuid::new_v4(),
            name: "Example Workflow".to_string(),
            status: WorkflowStatus::Idle,
            node_count: 5,
            last_run: None,
        }],
    })
}

async fn handle_get_workflow(_id: WorkflowId, _state: SharedState) -> Result<WsResponse> {
    // TODO: Query Forge engine for specific workflow
    Ok(WsResponse::Error {
        code: "NOT_IMPLEMENTED".to_string(),
        message: "Workflow detail not yet implemented".to_string(),
        details: None,
    })
}

async fn handle_start_workflow(
    _id: WorkflowId,
    _input: Option<serde_json::Value>,
    _state: SharedState,
) -> Result<WsResponse> {
    // TODO: Start workflow via sx9-atlas-bus
    Ok(WsResponse::Error {
        code: "NOT_IMPLEMENTED".to_string(),
        message: "Workflow start not yet implemented".to_string(),
        details: None,
    })
}

async fn handle_stop_workflow(_id: WorkflowId, _state: SharedState) -> Result<WsResponse> {
    // TODO: Stop workflow via sx9-atlas-bus
    Ok(WsResponse::Error {
        code: "NOT_IMPLEMENTED".to_string(),
        message: "Workflow stop not yet implemented".to_string(),
        details: None,
    })
}

async fn handle_get_plasma_state(state: SharedState) -> Result<WsResponse> {
    if let Some(plasma) = state.get_plasma().await {
        Ok(WsResponse::PlasmaState {
            delta_angle: plasma.delta_angle,
            entropy: plasma.entropy,
            excited: plasma.excited,
            sdt_state: format!("{:?}", plasma.sdt_state),
            ring_strength: 0.0,                 // TODO: Get from polycrystal
            delta_class: "Unknown".to_string(), // TODO: Calculate
        })
    } else {
        Ok(WsResponse::Error {
            code: "NO_PLASMA".to_string(),
            message: "PlasmaState not available".to_string(),
            details: None,
        })
    }
}

async fn handle_subscribe_plasma(_state: SharedState) -> Result<WsResponse> {
    // TODO: Subscribe to PlasmaState changes via NATS
    Ok(WsResponse::Error {
        code: "NOT_IMPLEMENTED".to_string(),
        message: "Plasma subscription not yet implemented".to_string(),
        details: None,
    })
}

// ═══════════════════════════════════════════════════════════════════════════
// HEALTH HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

async fn handle_get_connections(state: SharedState) -> Result<WsResponse> {
    let statuses = state.get_connection_statuses().await;
    Ok(WsResponse::Connections { statuses })
}

async fn handle_test_connection(db: Database, state: SharedState) -> Result<WsResponse> {
    let start = Instant::now();

    let (connected, error) = match db {
        Database::Surrealdb => {
            let surreal_guard = state.surrealdb.read().await;
            if let Some(surreal) = surreal_guard.as_ref() {
                match surreal.query("INFO FOR DB").await {
                    Ok(_) => (true, None),
                    Err(e) => (false, Some(e.to_string())),
                }
            } else {
                (false, Some("Not connected".to_string()))
            }
        }
        Database::Nats => {
            let nats_guard = state.nats.read().await;
            if nats_guard.is_some() {
                (true, None)
            } else {
                (false, Some("Not connected".to_string()))
            }
        }
        _ => (false, Some("Not implemented".to_string())),
    };

    Ok(WsResponse::Connections {
        statuses: vec![ConnectionStatus {
            db,
            connected,
            latency_ms: Some(start.elapsed().as_secs_f64() * 1000.0),
            last_check: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            error,
        }],
    })
}
