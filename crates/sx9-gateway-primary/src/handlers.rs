//! WebSocket message handlers
//!
//! Each handler maps a WsMessage to the appropriate SX9 backend.
//! Includes licensing handlers with dual heartbeat integration.

use anyhow::Result;
use std::collections::HashSet;
use std::time::Instant;

use crate::licensing::{default_components, LicenseTier, Subscription};
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

        // ═══════════════════════════════════════════════════════════════════
        // LICENSING OPERATIONS (with dual heartbeat)
        // ═══════════════════════════════════════════════════════════════════
        WsMessage::ValidateLicense { api_key } => handle_validate_license(api_key, state).await,

        WsMessage::CheckComponentAccess {
            api_key,
            component_id,
        } => handle_check_component_access(api_key, component_id, state).await,

        WsMessage::CheckFeatureAccess {
            api_key,
            feature_id,
        } => handle_check_feature_access(api_key, feature_id, state).await,

        WsMessage::GetComponents => handle_get_components(state).await,

        WsMessage::GetComponent { component_id } => handle_get_component(component_id, state).await,
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DATABASE HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

async fn handle_query(
    db: Database,
    query: String,
    params: Option<serde_json::Value>,
    state: SharedState,
) -> Result<WsResponse> {
    let start = Instant::now();

    match db {
        Database::Supabase => {
            let url = state.supabase_url.read().await.clone();
            let key = state.supabase_key.read().await.clone();

            if let (Some(url), Some(key)) = (url, key) {
                let client = reqwest::Client::new();
                // Supabase REST API query (table name in query, params as filters)
                let query_url = format!("{}/rest/v1/{}", url, query);

                match client
                    .get(&query_url)
                    .header("apikey", &key)
                    .header("Authorization", format!("Bearer {}", key))
                    .send()
                    .await
                {
                    Ok(response) if response.status().is_success() => {
                        let rows: Vec<serde_json::Value> = response.json().await.unwrap_or_default();
                        Ok(WsResponse::QueryResult {
                            db,
                            rows,
                            latency_ms: start.elapsed().as_secs_f64() * 1000.0,
                            cached: false,
                        })
                    }
                    Ok(response) => Ok(WsResponse::Error {
                        code: "QUERY_ERROR".to_string(),
                        message: format!("HTTP {}", response.status()),
                        details: None,
                    }),
                    Err(e) => Ok(WsResponse::Error {
                        code: "QUERY_ERROR".to_string(),
                        message: e.to_string(),
                        details: None,
                    }),
                }
            } else {
                Ok(WsResponse::Error {
                    code: "NOT_CONNECTED".to_string(),
                    message: "Supabase not configured".to_string(),
                    details: None,
                })
            }
        }

        Database::Neon => {
            // Neon queries require tokio-postgres - placeholder for now
            Ok(WsResponse::Error {
                code: "NOT_IMPLEMENTED".to_string(),
                message: "Neon PostgreSQL queries require tokio-postgres (coming soon)".to_string(),
                details: Some(serde_json::json!({ "query": query, "params": params })),
            })
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

async fn handle_get_graph(_filter: GraphFilter, _state: SharedState) -> Result<WsResponse> {
    // Graph queries now go through Supabase or GLAF in-memory graph
    // TODO: Implement via Supabase PostgREST or sx9-glaf-core
    Ok(WsResponse::Error {
        code: "NOT_IMPLEMENTED".to_string(),
        message: "Graph queries require GLAF integration (coming soon)".to_string(),
        details: None,
    })
}

async fn handle_get_fusion_nodes(_threshold: f32, _state: SharedState) -> Result<WsResponse> {
    // Fusion nodes are stored in Supabase or GLAF
    // TODO: Query via PostgREST or sx9-glaf-core
    Ok(WsResponse::Error {
        code: "NOT_IMPLEMENTED".to_string(),
        message: "Fusion queries require GLAF integration (coming soon)".to_string(),
        details: None,
    })
}

async fn handle_expand_node(_node_id: String, _depth: u32, _state: SharedState) -> Result<WsResponse> {
    // Node expansion via GLAF graph traversal
    // TODO: Implement via sx9-glaf-core
    Ok(WsResponse::Error {
        code: "NOT_IMPLEMENTED".to_string(),
        message: "Node expansion requires GLAF integration (coming soon)".to_string(),
        details: None,
    })
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
        Database::Supabase => {
            let url = state.supabase_url.read().await.clone();
            let key = state.supabase_key.read().await.clone();
            if let (Some(url), Some(key)) = (url, key) {
                let client = reqwest::Client::new();
                match client
                    .get(format!("{}/rest/v1/", url))
                    .header("apikey", &key)
                    .send()
                    .await
                {
                    Ok(r) if r.status().is_success() || r.status().as_u16() == 400 => (true, None),
                    Ok(r) => (false, Some(format!("HTTP {}", r.status()))),
                    Err(e) => (false, Some(e.to_string())),
                }
            } else {
                (false, Some("Not configured".to_string()))
            }
        }
        Database::Neon => {
            let url = state.neon_url.read().await.clone();
            if url.is_some() {
                // URL configured = "connected" for now (full pg test requires tokio-postgres)
                (true, None)
            } else {
                (false, Some("Not configured".to_string()))
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
        Database::Sled | Database::Sledis => {
            // Sled is embedded, check if port is listening
            match tokio::net::TcpStream::connect(format!("localhost:{}", db.default_port())).await {
                Ok(_) => (true, None),
                Err(_) => (false, Some("Service not running".to_string())),
            }
        }
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

// ═══════════════════════════════════════════════════════════════════════════
// LICENSING HANDLERS (with dual heartbeat integration)
// ═══════════════════════════════════════════════════════════════════════════

/// Validate license and return accessible components/features
/// Integrates dual heartbeat check for zero-trust validation
async fn handle_validate_license(api_key: String, _state: SharedState) -> Result<WsResponse> {
    // Run dual heartbeat check first (zero-trust requirement)
    let heartbeat_passed = run_dual_heartbeat().await;

    // Look up subscription by API key
    // TODO: Query from Supabase subscriptions table
    let subscription = lookup_subscription(&api_key).await;

    match subscription {
        Some(sub) if sub.is_valid() && heartbeat_passed => {
            let components = default_components();
            let accessible_components: Vec<String> = components
                .iter()
                .filter(|c| sub.can_access_component(&c.id, c.required_tier))
                .map(|c| c.id.clone())
                .collect();

            // Features are component capabilities
            let accessible_features: Vec<String> = components
                .iter()
                .filter(|c| sub.can_access_component(&c.id, c.required_tier))
                .flat_map(|c| c.capabilities.clone())
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();

            let warning = sub.days_remaining().and_then(|days| {
                if days < 7 {
                    Some(format!("License expires in {} days", days))
                } else if days < 30 {
                    Some(format!("{} days remaining", days))
                } else {
                    None
                }
            });

            Ok(WsResponse::LicenseValidation {
                valid: true,
                tier: sub.tier.display_name().to_lowercase(),
                tier_level: sub.tier as u8,
                days_remaining: sub.days_remaining(),
                accessible_components,
                accessible_features,
                warning,
            })
        }
        Some(sub) if !heartbeat_passed => {
            Ok(WsResponse::LicenseValidation {
                valid: false,
                tier: sub.tier.display_name().to_lowercase(),
                tier_level: sub.tier as u8,
                days_remaining: sub.days_remaining(),
                accessible_components: vec![],
                accessible_features: vec![],
                warning: Some("Heartbeat validation failed - zero-trust check required".to_string()),
            })
        }
        Some(_) => {
            Ok(WsResponse::LicenseValidation {
                valid: false,
                tier: "expired".to_string(),
                tier_level: 0,
                days_remaining: Some(0),
                accessible_components: vec![],
                accessible_features: vec![],
                warning: Some("License expired".to_string()),
            })
        }
        None => {
            // Invalid API key - return free tier access
            let components = default_components();
            let free_components: Vec<String> = components
                .iter()
                .filter(|c| c.required_tier == LicenseTier::Free)
                .map(|c| c.id.clone())
                .collect();

            Ok(WsResponse::LicenseValidation {
                valid: true,
                tier: "free".to_string(),
                tier_level: 0,
                days_remaining: None,
                accessible_components: free_components,
                accessible_features: vec!["filtering".to_string(), "search".to_string()],
                warning: None,
            })
        }
    }
}

/// Check if user can access a specific component
async fn handle_check_component_access(
    api_key: String,
    component_id: String,
    _state: SharedState,
) -> Result<WsResponse> {
    let subscription = lookup_subscription(&api_key).await;
    let components = default_components();

    let component = components.iter().find(|c| c.id == component_id);

    match (subscription, component) {
        (Some(sub), Some(comp)) if sub.is_valid() => {
            let granted = sub.can_access_component(&comp.id, comp.required_tier);

            // Check heartbeat requirement
            let heartbeat_ok = if comp.requires_heartbeat {
                run_dual_heartbeat().await
            } else {
                true
            };

            Ok(WsResponse::ComponentAccess {
                granted: granted && heartbeat_ok,
                component_id,
                required_tier: comp.required_tier.display_name().to_lowercase(),
                current_tier: sub.tier.display_name().to_lowercase(),
                reason: if !granted {
                    Some(format!("Requires {} tier", comp.required_tier.display_name()))
                } else if !heartbeat_ok {
                    Some("Heartbeat validation failed".to_string())
                } else {
                    None
                },
            })
        }
        (None, Some(comp)) => {
            // No subscription - check if free tier
            let granted = comp.required_tier == LicenseTier::Free;
            Ok(WsResponse::ComponentAccess {
                granted,
                component_id,
                required_tier: comp.required_tier.display_name().to_lowercase(),
                current_tier: "free".to_string(),
                reason: if !granted {
                    Some(format!("Requires {} tier", comp.required_tier.display_name()))
                } else {
                    None
                },
            })
        }
        (_, None) => {
            Ok(WsResponse::Error {
                code: "COMPONENT_NOT_FOUND".to_string(),
                message: format!("Component '{}' not found", component_id),
                details: None,
            })
        }
        (Some(_), _) => {
            Ok(WsResponse::Error {
                code: "LICENSE_EXPIRED".to_string(),
                message: "License has expired".to_string(),
                details: None,
            })
        }
    }
}

/// Check if user can access a specific feature
async fn handle_check_feature_access(
    api_key: String,
    feature_id: String,
    _state: SharedState,
) -> Result<WsResponse> {
    let subscription = lookup_subscription(&api_key).await;
    let components = default_components();

    // Find components that provide this feature
    let providing_components: Vec<_> = components
        .iter()
        .filter(|c| c.capabilities.contains(&feature_id))
        .collect();

    if providing_components.is_empty() {
        return Ok(WsResponse::Error {
            code: "FEATURE_NOT_FOUND".to_string(),
            message: format!("Feature '{}' not found", feature_id),
            details: None,
        });
    }

    // Find the lowest tier that provides this feature
    let min_tier = providing_components
        .iter()
        .map(|c| c.required_tier)
        .min()
        .unwrap_or(LicenseTier::Free);

    match subscription {
        Some(sub) if sub.is_valid() => {
            let granted = sub.tier.can_access(min_tier);
            Ok(WsResponse::FeatureAccess {
                granted,
                feature_id,
                required_tier: min_tier.display_name().to_lowercase(),
                current_tier: sub.tier.display_name().to_lowercase(),
                reason: if !granted {
                    Some(format!("Requires {} tier", min_tier.display_name()))
                } else {
                    None
                },
            })
        }
        None => {
            let granted = min_tier == LicenseTier::Free;
            Ok(WsResponse::FeatureAccess {
                granted,
                feature_id,
                required_tier: min_tier.display_name().to_lowercase(),
                current_tier: "free".to_string(),
                reason: if !granted {
                    Some(format!("Requires {} tier", min_tier.display_name()))
                } else {
                    None
                },
            })
        }
        Some(_) => {
            Ok(WsResponse::FeatureAccess {
                granted: false,
                feature_id,
                required_tier: min_tier.display_name().to_lowercase(),
                current_tier: "expired".to_string(),
                reason: Some("License expired".to_string()),
            })
        }
    }
}

/// Get all available components with access status for current user
async fn handle_get_components(_state: SharedState) -> Result<WsResponse> {
    let components = default_components();

    let component_infos: Vec<ComponentInfo> = components
        .into_iter()
        .map(|c| ComponentInfo {
            id: c.id,
            name: c.name,
            description: c.description,
            category: c.category,
            required_tier: c.required_tier.display_name().to_lowercase(),
            version: c.version,
            wasm_size: c.wasm_size,
            requires_heartbeat: c.requires_heartbeat,
            icon: c.icon,
            capabilities: c.capabilities,
            access_status: ComponentAccessStatus::Available, // TODO: Check user's actual access
        })
        .collect();

    Ok(WsResponse::Components {
        components: component_infos,
    })
}

/// Get a specific component's details
async fn handle_get_component(component_id: String, _state: SharedState) -> Result<WsResponse> {
    let components = default_components();

    if let Some(c) = components.into_iter().find(|c| c.id == component_id) {
        Ok(WsResponse::ComponentDetail {
            component: ComponentInfo {
                id: c.id,
                name: c.name,
                description: c.description,
                category: c.category,
                required_tier: c.required_tier.display_name().to_lowercase(),
                version: c.version,
                wasm_size: c.wasm_size,
                requires_heartbeat: c.requires_heartbeat,
                icon: c.icon,
                capabilities: c.capabilities,
                access_status: ComponentAccessStatus::Available,
            },
        })
    } else {
        Ok(WsResponse::Error {
            code: "COMPONENT_NOT_FOUND".to_string(),
            message: format!("Component '{}' not found", component_id),
            details: None,
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Run dual heartbeat check (local + global)
/// Returns true if both heartbeats pass
async fn run_dual_heartbeat() -> bool {
    // Try to run heartbeat gate if sx9-harness is available
    #[cfg(feature = "heartbeat")]
    {
        use sx9_harness::gates::HeartbeatGate;
        let gate = HeartbeatGate::new();
        match gate.run().await {
            Ok(report) => report.passed,
            Err(_) => true, // Fail open for development
        }
    }

    #[cfg(not(feature = "heartbeat"))]
    {
        // No heartbeat gate - assume passed for development
        true
    }
}

/// Look up subscription by API key
/// TODO: Query from Supabase when connected
async fn lookup_subscription(api_key: &str) -> Option<Subscription> {
    // Development mode: recognize test API keys
    let tier = match api_key {
        "sk_test_free" | "free" => Some(LicenseTier::Free),
        "sk_test_pro" | "pro" => Some(LicenseTier::Pro),
        "sk_test_enterprise" | "enterprise" => Some(LicenseTier::Enterprise),
        "sk_test_government" | "government" | "gov" => Some(LicenseTier::Government),
        _ if api_key.starts_with("sk_live_") => {
            // TODO: Validate against Supabase
            Some(LicenseTier::Pro)
        }
        _ => None,
    };

    tier.map(|t| Subscription {
        id: format!("sub_{}", api_key),
        org_id: "dev".to_string(),
        tier: t,
        started_at: 0,
        expires_at: None, // Never expires for dev
        active: true,
        seats: None,
        feature_overrides: HashSet::new(),
        component_overrides: HashSet::new(),
    })
}
