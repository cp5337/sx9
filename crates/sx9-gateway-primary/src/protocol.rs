//! WebSocket protocol definitions
//!
//! This module defines the message types that flow between the UI and the gateway.
//! Every message maps directly to existing SX9 infrastructure.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Target database for queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Database {
    Supabase,
    Surrealdb,
    Sled,
    Sledis,
    Nats,
}

impl Database {
    pub fn default_port(&self) -> u16 {
        match self {
            Database::Supabase => 18000,
            Database::Surrealdb => 18010,
            Database::Sled => 18400,
            Database::Sledis => 18401,
            Database::Nats => 18020,
        }
    }
    
    pub fn brand_color(&self) -> &'static str {
        match self {
            Database::Supabase => "#3ecf8e",
            Database::Surrealdb => "#ff00a0",
            Database::Sled => "#ff6b35",
            Database::Sledis => "#dc382d",
            Database::Nats => "#4222ff",
        }
    }
}

/// Graph filter for node/edge queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphFilter {
    /// Filter by node type
    pub node_type: Option<String>,
    /// Filter by relationship type
    pub edge_type: Option<String>,
    /// Maximum depth for traversal
    pub max_depth: Option<u32>,
    /// Only return fusion nodes
    pub fusion_only: bool,
    /// Minimum fusion score threshold
    pub min_fusion_score: Option<f32>,
    /// Filter by trivariate hash prefix
    pub hash_prefix: Option<String>,
}

impl Default for GraphFilter {
    fn default() -> Self {
        Self {
            node_type: None,
            edge_type: None,
            max_depth: Some(3),
            fusion_only: false,
            min_fusion_score: None,
            hash_prefix: None,
        }
    }
}

/// Workflow identifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowId(pub Uuid);

/// Messages from UI → Gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsMessage {
    // ═══════════════════════════════════════════════════════════════════
    // DATABASE OPERATIONS
    // Maps to: Supabase, SurrealDB, Sled, Sledis drivers
    // ═══════════════════════════════════════════════════════════════════
    
    /// Execute a query against a specific database
    Query {
        db: Database,
        query: String,
        /// Optional parameters for prepared statements
        params: Option<serde_json::Value>,
    },
    
    /// Subscribe to real-time changes on a table/collection
    Subscribe {
        db: Database,
        table: String,
        /// Optional filter condition
        filter: Option<String>,
    },
    
    /// Unsubscribe from a table
    Unsubscribe {
        db: Database,
        table: String,
    },
    
    // ═══════════════════════════════════════════════════════════════════
    // GRAPH OPERATIONS
    // Maps to: SurrealDB graph queries + GLAF correlation
    // ═══════════════════════════════════════════════════════════════════
    
    /// Get graph data with optional filtering
    GetGraph {
        filter: GraphFilter,
    },
    
    /// Get fusion nodes (cross-database correlations)
    GetFusionNodes {
        /// Minimum confidence threshold (0.0-1.0)
        threshold: f32,
    },
    
    /// Expand a node to show its neighbors
    ExpandNode {
        node_id: String,
        depth: u32,
    },
    
    /// Run GLAF correlation analysis
    RunCorrelation {
        /// Source nodes to correlate
        source_ids: Vec<String>,
    },
    
    // ═══════════════════════════════════════════════════════════════════
    // WORKFLOW OPERATIONS
    // Maps to: sx9-atlas-bus, Forge Engine
    // ═══════════════════════════════════════════════════════════════════
    
    /// List all workflows
    GetWorkflows,
    
    /// Get a specific workflow's state
    GetWorkflow {
        id: WorkflowId,
    },
    
    /// Start a workflow
    StartWorkflow {
        id: WorkflowId,
        /// Optional input parameters
        input: Option<serde_json::Value>,
    },
    
    /// Stop a running workflow
    StopWorkflow {
        id: WorkflowId,
    },
    
    /// Get current PlasmaState (delta_angle, entropy, SDT gate)
    GetPlasmaState,
    
    /// Subscribe to PlasmaState changes
    SubscribePlasma,
    
    // ═══════════════════════════════════════════════════════════════════
    // HEALTH & CONNECTION
    // Maps to: NATS health subjects
    // ═══════════════════════════════════════════════════════════════════
    
    /// Get connection status for all databases
    GetConnections,
    
    /// Test connection to a specific database
    TestConnection {
        db: Database,
    },
    
    /// Ping (keepalive)
    Ping,
}

/// Messages from Gateway → UI
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsResponse {
    // ═══════════════════════════════════════════════════════════════════
    // SUCCESS RESPONSES
    // ═══════════════════════════════════════════════════════════════════
    
    /// Query result
    QueryResult {
        db: Database,
        rows: Vec<serde_json::Value>,
        latency_ms: f64,
        cached: bool,
    },
    
    /// Real-time update from subscription
    SubscriptionUpdate {
        db: Database,
        table: String,
        event: SubscriptionEvent,
        data: serde_json::Value,
    },
    
    /// Graph data
    GraphData {
        nodes: Vec<GraphNode>,
        edges: Vec<GraphEdge>,
    },
    
    /// Fusion nodes
    FusionNodes {
        nodes: Vec<FusionNode>,
    },
    
    /// Workflow list
    Workflows {
        workflows: Vec<WorkflowSummary>,
    },
    
    /// Workflow detail
    WorkflowDetail {
        workflow: WorkflowDetail,
    },
    
    /// PlasmaState snapshot
    PlasmaState {
        delta_angle: u16,
        entropy: u32,
        excited: bool,
        sdt_state: String,
        ring_strength: f64,
        delta_class: String,
    },
    
    /// Connection status for all databases
    Connections {
        statuses: Vec<ConnectionStatus>,
    },
    
    /// Pong response
    Pong {
        server_time: u64,
    },
    
    // ═══════════════════════════════════════════════════════════════════
    // ERROR RESPONSES
    // ═══════════════════════════════════════════════════════════════════
    
    /// Error response
    Error {
        code: String,
        message: String,
        details: Option<serde_json::Value>,
    },
}

/// Subscription event types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionEvent {
    Insert,
    Update,
    Delete,
}

/// Graph node for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub node_type: String,
    /// Shape: "circle", "square", "diamond", "nonagon" (fusion)
    pub shape: String,
    pub color: String,
    pub size: f32,
    /// Trivariate hash if available
    pub trivariate_hash: Option<String>,
    /// Source database
    pub source_db: Database,
    /// Additional properties
    pub properties: serde_json::Value,
}

/// Graph edge for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub edge_type: String,
    pub label: Option<String>,
    pub weight: Option<f32>,
    pub color: String,
}

/// Fusion node (cross-database correlation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionNode {
    pub id: String,
    pub trivariate_hash: String,
    pub fusion_score: f32,
    pub fusion_method: String,
    pub sources: Vec<FusionSource>,
    pub created_at: u64,
    pub last_correlated: u64,
}

/// Source reference for a fusion node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionSource {
    pub db: Database,
    pub table: String,
    pub id: String,
    pub last_sync: u64,
}

/// Workflow summary for list view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSummary {
    pub id: Uuid,
    pub name: String,
    pub status: WorkflowStatus,
    pub node_count: u32,
    pub last_run: Option<u64>,
}

/// Workflow status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkflowStatus {
    Idle,
    Running,
    Paused,
    Error,
    Completed,
}

/// Detailed workflow information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDetail {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: WorkflowStatus,
    pub nodes: Vec<WorkflowNode>,
    pub edges: Vec<WorkflowEdge>,
    pub plasma_state: Option<serde_json::Value>,
    pub created_at: u64,
    pub updated_at: u64,
}

/// Workflow node (for React Flow)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
    pub id: String,
    pub node_type: String,
    pub position: Position,
    pub data: serde_json::Value,
}

/// Position for workflow nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Workflow edge (for React Flow)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub source_handle: Option<String>,
    pub target_handle: Option<String>,
}

/// Connection status for a database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub db: Database,
    pub connected: bool,
    pub latency_ms: Option<f64>,
    pub last_check: u64,
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let msg = WsMessage::Query {
            db: Database::Surrealdb,
            query: "SELECT * FROM entity".to_string(),
            params: None,
        };
        
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"Query\""));
        assert!(json.contains("\"db\":\"surrealdb\""));
        
        let parsed: WsMessage = serde_json::from_str(&json).unwrap();
        match parsed {
            WsMessage::Query { db, query, .. } => {
                assert_eq!(db, Database::Surrealdb);
                assert_eq!(query, "SELECT * FROM entity");
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_response_serialization() {
        let resp = WsResponse::PlasmaState {
            delta_angle: 180,
            entropy: 42,
            excited: true,
            sdt_state: "Conducting".to_string(),
            ring_strength: 0.95,
            delta_class: "Micro".to_string(),
        };
        
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"type\":\"PlasmaState\""));
        assert!(json.contains("\"delta_angle\":180"));
    }
}

