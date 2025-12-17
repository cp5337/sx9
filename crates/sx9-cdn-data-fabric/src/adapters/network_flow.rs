//! Network Flow Analyzer Adapter
//!
//! Integrates with Forge workflow system and SYNAPTIX9 event orchestrator
//! for real-time network flow analysis and visualization.
//!
//! Key capabilities:
//! - TAPS buffer monitoring (message throughput)
//! - Workflow execution tracking
//! - Neural Mux connectivity status
//! - Cognitive atom interaction forces
//! - Service orchestration health

use crate::registry::DatabaseInfo;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

// ============================================================================
// Network Flow Types
// ============================================================================

/// Network flow event from SYNAPTIX9/Forge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkFlowEvent {
    pub id: String,
    pub timestamp: String,
    pub event_type: NetworkFlowEventType,
    pub source: FlowEndpoint,
    pub destination: FlowEndpoint,
    pub payload: FlowPayload,
    pub metadata: FlowMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkFlowEventType {
    // Workflow events
    WorkflowStarted,
    WorkflowCompleted,
    WorkflowFailed,
    WorkflowProgress,

    // TAPS buffer events
    TapsBufferWrite,
    TapsBufferRead,
    TapsBufferOverflow,

    // Neural Mux events
    NeuralMuxConnect,
    NeuralMuxDisconnect,
    NeuralMuxTranslate,

    // Cognitive atom events
    CognitiveAtomCreate,
    CognitiveAtomInteract,
    CognitiveAtomDecay,

    // Service events
    ServiceStartup,
    ServiceHealthCheck,
    ServiceEscalation,

    // Data flow events
    DataIngestion,
    DataTransformation,
    DataExport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowEndpoint {
    pub service_id: String,
    pub service_type: String,
    pub port: u16,
    pub protocol: String,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowPayload {
    pub size_bytes: usize,
    pub message_count: usize,
    pub compression: Option<String>,
    pub encryption: Option<String>,
    pub content_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowMetadata {
    pub workflow_id: Option<String>,
    pub workflow_type: Option<String>,
    pub priority: Option<u8>,
    pub hd4_phase: Option<String>,
    pub cognitive_load: Option<f64>,
    pub latency_ms: Option<f64>,
    pub throughput_mps: Option<f64>,
}

// ============================================================================
// Workflow State
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    pub id: String,
    pub name: String,
    pub workflow_type: WorkflowType,
    pub status: WorkflowStatus,
    pub priority: u8,
    pub progress: f64,
    pub started_at: String,
    pub steps: Vec<WorkflowStep>,
    pub connections: Vec<WorkflowConnection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkflowType {
    Osint,
    Threat,
    Media,
    Stream,
    Geospatial,
    Semantic,
    Hd4,
    MonteCarlo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkflowStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub name: String,
    pub step_type: String,
    pub status: WorkflowStatus,
    pub input_sources: Vec<String>,
    pub output_targets: Vec<String>,
    pub execution_time_ms: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConnection {
    pub from_step: String,
    pub to_step: String,
    pub data_type: String,
    pub throughput: f64,
    pub active: bool,
}

// ============================================================================
// TAPS Buffer State
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TapsBufferState {
    pub buffer_id: String,
    pub capacity: usize,
    pub usage: usize,
    pub usage_percent: f64,
    pub throughput_mps: f64,
    pub peak_throughput_mps: f64,
    pub messages_processed: u64,
    pub messages_dropped: u64,
    pub partitions: Vec<TapsPartition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TapsPartition {
    pub partition_id: String,
    pub topic: String,
    pub offset: u64,
    pub lag: u64,
    pub consumer_group: String,
}

// ============================================================================
// Neural Mux State
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralMuxState {
    pub mux_status: String,
    pub connected_systems: Vec<ConnectedSystem>,
    pub translation_layers: Vec<TranslationLayer>,
    pub throughput_mbps: f64,
    pub latency_ms: f64,
    pub error_rate: f64,
    pub adaptation_speed: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedSystem {
    pub system_id: String,
    pub system_type: String,
    pub connection_status: String,
    pub protocol: String,
    pub data_rate: f64,
    pub last_heartbeat: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationLayer {
    pub source_protocol: String,
    pub target_protocol: String,
    pub efficiency_rating: f64,
    pub translations_count: u64,
}

// ============================================================================
// Service Orchestrator State
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOrchestratorState {
    pub services: Vec<ServiceState>,
    pub dependencies: Vec<ServiceDependency>,
    pub escalation_log: Vec<EscalationEvent>,
    pub summary: ServiceSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceState {
    pub name: String,
    pub port: u16,
    pub service_type: String,
    pub status: String,
    pub healthy: bool,
    pub failure_count: u32,
    pub start_time: Option<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDependency {
    pub from_service: String,
    pub to_service: String,
    pub dependency_type: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationEvent {
    pub timestamp: String,
    pub service: String,
    pub level: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSummary {
    pub total: usize,
    pub running: usize,
    pub failed: usize,
    pub stopped: usize,
}

// ============================================================================
// Adapter Implementation
// ============================================================================

/// Get network flow events from SX9 orchestrator
pub async fn get_flow_events(
    info: &DatabaseInfo,
    limit: usize,
    event_type: Option<&str>,
) -> anyhow::Result<Vec<NetworkFlowEvent>> {
    let url = format!("http://{}:{}/api/flow/events", info.host, info.port);

    let client = reqwest::Client::new();
    let mut params = vec![("limit", limit.to_string())];
    if let Some(et) = event_type {
        params.push(("event_type", et.to_string()));
    }

    let response = client.get(&url).query(&params).send().await?;

    if response.status().is_success() {
        let events: Vec<NetworkFlowEvent> = response.json().await?;
        Ok(events)
    } else {
        // Return mock data for development
        Ok(generate_mock_flow_events(limit))
    }
}

/// Get current workflow states
pub async fn get_workflow_states(info: &DatabaseInfo) -> anyhow::Result<Vec<WorkflowState>> {
    let url = format!("http://{}:{}/api/workflows", info.host, info.port);

    match reqwest::get(&url).await {
        Ok(response) if response.status().is_success() => {
            let workflows: Vec<WorkflowState> = response.json().await?;
            Ok(workflows)
        }
        _ => Ok(generate_mock_workflows()),
    }
}

/// Get TAPS buffer state
pub async fn get_taps_buffer_state(info: &DatabaseInfo) -> anyhow::Result<TapsBufferState> {
    let url = format!("http://{}:{}/api/taps/status", info.host, info.port);

    match reqwest::get(&url).await {
        Ok(response) if response.status().is_success() => {
            let state: TapsBufferState = response.json().await?;
            Ok(state)
        }
        _ => Ok(generate_mock_taps_state()),
    }
}

/// Get Neural Mux state
pub async fn get_neural_mux_state(info: &DatabaseInfo) -> anyhow::Result<NeuralMuxState> {
    let url = format!("http://{}:{}/api/neural-mux/status", info.host, info.port);

    match reqwest::get(&url).await {
        Ok(response) if response.status().is_success() => {
            let state: NeuralMuxState = response.json().await?;
            Ok(state)
        }
        _ => Ok(generate_mock_neural_mux_state()),
    }
}

/// Get service orchestrator state
pub async fn get_orchestrator_state(
    info: &DatabaseInfo,
) -> anyhow::Result<ServiceOrchestratorState> {
    let url = format!("http://{}:{}/api/orchestrator/status", info.host, info.port);

    match reqwest::get(&url).await {
        Ok(response) if response.status().is_success() => {
            let state: ServiceOrchestratorState = response.json().await?;
            Ok(state)
        }
        _ => Ok(generate_mock_orchestrator_state()),
    }
}

/// Convert network flow data to graph format
pub fn flow_to_graph(
    events: &[NetworkFlowEvent],
    workflows: &[WorkflowState],
    services: &ServiceOrchestratorState,
) -> Value {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut node_ids = std::collections::HashSet::new();

    // Add service nodes
    for (idx, service) in services.services.iter().enumerate() {
        let node_id = format!("service:{}", service.name);
        if !node_ids.contains(&node_id) {
            node_ids.insert(node_id.clone());
            nodes.push(json!({
                "id": node_id,
                "labels": ["Service", &service.service_type],
                "properties": {
                    "name": service.name,
                    "port": service.port,
                    "status": service.status,
                    "healthy": service.healthy,
                    "type": service.service_type
                },
                "x": (idx % 5) as f64 * 150.0,
                "y": (idx / 5) as f64 * 100.0,
                "size": if service.healthy { 30.0 } else { 20.0 },
                "color": if service.healthy { "#22c55e" } else { "#ef4444" }
            }));
        }
    }

    // Add service dependency edges
    for dep in &services.dependencies {
        edges.push(json!({
            "id": format!("dep:{}:{}", dep.from_service, dep.to_service),
            "source": format!("service:{}", dep.from_service),
            "target": format!("service:{}", dep.to_service),
            "type": "DEPENDS_ON",
            "properties": {
                "dependency_type": dep.dependency_type,
                "required": dep.required
            },
            "style": if dep.required { "solid" } else { "dashed" }
        }));
    }

    // Add workflow nodes
    for (idx, workflow) in workflows.iter().enumerate() {
        let node_id = format!("workflow:{}", workflow.id);
        if !node_ids.contains(&node_id) {
            node_ids.insert(node_id.clone());
            nodes.push(json!({
                "id": node_id,
                "labels": ["Workflow", format!("{:?}", workflow.workflow_type)],
                "properties": {
                    "name": workflow.name,
                    "status": format!("{:?}", workflow.status),
                    "progress": workflow.progress,
                    "priority": workflow.priority
                },
                "x": 600.0 + (idx % 3) as f64 * 150.0,
                "y": (idx / 3) as f64 * 100.0,
                "size": 25.0 + workflow.priority as f64 * 5.0,
                "color": match workflow.status {
                    WorkflowStatus::Running => "#3b82f6",
                    WorkflowStatus::Completed => "#22c55e",
                    WorkflowStatus::Failed => "#ef4444",
                    _ => "#6b7280"
                }
            }));
        }

        // Add workflow step nodes and connections
        for step in &workflow.steps {
            let step_id = format!("step:{}:{}", workflow.id, step.id);
            if !node_ids.contains(&step_id) {
                node_ids.insert(step_id.clone());
                nodes.push(json!({
                    "id": step_id,
                    "labels": ["WorkflowStep"],
                    "properties": step,
                    "size": 15.0
                }));
            }
        }

        for conn in &workflow.connections {
            edges.push(json!({
                "id": format!("flow:{}:{}:{}", workflow.id, conn.from_step, conn.to_step),
                "source": format!("step:{}:{}", workflow.id, conn.from_step),
                "target": format!("step:{}:{}", workflow.id, conn.to_step),
                "type": "DATA_FLOW",
                "properties": {
                    "data_type": conn.data_type,
                    "throughput": conn.throughput,
                    "active": conn.active
                },
                "width": conn.throughput / 100.0
            }));
        }
    }

    // Add flow event edges
    for event in events {
        let source_id = format!("service:{}", event.source.service_id);
        let dest_id = format!("service:{}", event.destination.service_id);

        if node_ids.contains(&source_id) && node_ids.contains(&dest_id) {
            edges.push(json!({
                "id": format!("event:{}", event.id),
                "source": source_id,
                "target": dest_id,
                "type": format!("{:?}", event.event_type),
                "properties": {
                    "timestamp": event.timestamp,
                    "size_bytes": event.payload.size_bytes,
                    "message_count": event.payload.message_count,
                    "latency_ms": event.metadata.latency_ms
                },
                "animated": true
            }));
        }
    }

    json!({
        "nodes": nodes,
        "edges": edges,
        "stats": {
            "nodeCount": nodes.len(),
            "edgeCount": edges.len(),
            "serviceCount": services.services.len(),
            "workflowCount": workflows.len(),
            "eventCount": events.len()
        }
    })
}

// ============================================================================
// Mock Data Generators (for development)
// ============================================================================

fn generate_mock_flow_events(limit: usize) -> Vec<NetworkFlowEvent> {
    (0..limit.min(10))
        .map(|i| NetworkFlowEvent {
            id: format!("event-{}", i),
            timestamp: chrono::Utc::now().to_rfc3339(),
            event_type: match i % 4 {
                0 => NetworkFlowEventType::WorkflowProgress,
                1 => NetworkFlowEventType::TapsBufferWrite,
                2 => NetworkFlowEventType::DataIngestion,
                _ => NetworkFlowEventType::ServiceHealthCheck,
            },
            source: FlowEndpoint {
                service_id: "glaf-core".to_string(),
                service_type: "surrealdb".to_string(),
                port: 18019,
                protocol: "websocket".to_string(),
                host: "localhost".to_string(),
            },
            destination: FlowEndpoint {
                service_id: "cdn-fabric".to_string(),
                service_type: "aggregator".to_string(),
                port: 18100,
                protocol: "http".to_string(),
                host: "localhost".to_string(),
            },
            payload: FlowPayload {
                size_bytes: 1024 + i * 100,
                message_count: 10 + i,
                compression: Some("gzip".to_string()),
                encryption: Some("tls".to_string()),
                content_type: "application/json".to_string(),
            },
            metadata: FlowMetadata {
                workflow_id: Some(format!("wf-{}", i)),
                workflow_type: Some("OSINT".to_string()),
                priority: Some((i % 5 + 1) as u8),
                hd4_phase: Some("Detect".to_string()),
                cognitive_load: Some(0.5 + (i as f64 * 0.05)),
                latency_ms: Some(10.0 + (i as f64 * 2.0)),
                throughput_mps: Some(450.0 + (i as f64 * 10.0)),
            },
        })
        .collect()
}

fn generate_mock_workflows() -> Vec<WorkflowState> {
    vec![
        WorkflowState {
            id: "wf-osint-alpha".to_string(),
            name: "Operation: Deep Dive".to_string(),
            workflow_type: WorkflowType::Osint,
            status: WorkflowStatus::Running,
            priority: 5,
            progress: 67.0,
            started_at: "2 mins ago".to_string(),
            steps: vec![
                WorkflowStep {
                    id: "step-1".to_string(),
                    name: "Data Ingestion".to_string(),
                    step_type: "source".to_string(),
                    status: WorkflowStatus::Completed,
                    input_sources: vec![],
                    output_targets: vec!["step-2".to_string()],
                    execution_time_ms: Some(150.0),
                },
                WorkflowStep {
                    id: "step-2".to_string(),
                    name: "Analysis".to_string(),
                    step_type: "transformer".to_string(),
                    status: WorkflowStatus::Running,
                    input_sources: vec!["step-1".to_string()],
                    output_targets: vec!["step-3".to_string()],
                    execution_time_ms: None,
                },
            ],
            connections: vec![WorkflowConnection {
                from_step: "step-1".to_string(),
                to_step: "step-2".to_string(),
                data_type: "json".to_string(),
                throughput: 450.0,
                active: true,
            }],
        },
        WorkflowState {
            id: "wf-threat-bravo".to_string(),
            name: "Threat Scan: Sector 7".to_string(),
            workflow_type: WorkflowType::Threat,
            status: WorkflowStatus::Running,
            priority: 4,
            progress: 32.0,
            started_at: "5 mins ago".to_string(),
            steps: vec![],
            connections: vec![],
        },
    ]
}

fn generate_mock_taps_state() -> TapsBufferState {
    TapsBufferState {
        buffer_id: "taps-main".to_string(),
        capacity: 10000,
        usage: 243,
        usage_percent: 2.43,
        throughput_mps: 450.0,
        peak_throughput_mps: 1200.0,
        messages_processed: 1458000,
        messages_dropped: 3,
        partitions: vec![TapsPartition {
            partition_id: "p0".to_string(),
            topic: "osint-events".to_string(),
            offset: 12345,
            lag: 5,
            consumer_group: "ctas7-analyzers".to_string(),
        }],
    }
}

fn generate_mock_neural_mux_state() -> NeuralMuxState {
    NeuralMuxState {
        mux_status: "active".to_string(),
        connected_systems: vec![
            ConnectedSystem {
                system_id: "glaf-core".to_string(),
                system_type: "database".to_string(),
                connection_status: "connected".to_string(),
                protocol: "websocket".to_string(),
                data_rate: 1000.0,
                last_heartbeat: chrono::Utc::now().to_rfc3339(),
            },
            ConnectedSystem {
                system_id: "neo4j-viz".to_string(),
                system_type: "visualization".to_string(),
                connection_status: "connected".to_string(),
                protocol: "bolt".to_string(),
                data_rate: 500.0,
                last_heartbeat: chrono::Utc::now().to_rfc3339(),
            },
        ],
        translation_layers: vec![TranslationLayer {
            source_protocol: "surql".to_string(),
            target_protocol: "cypher".to_string(),
            efficiency_rating: 0.95,
            translations_count: 15000,
        }],
        throughput_mbps: 1000.0,
        latency_ms: 50.0,
        error_rate: 0.001,
        adaptation_speed: 0.8,
    }
}

fn generate_mock_orchestrator_state() -> ServiceOrchestratorState {
    ServiceOrchestratorState {
        services: vec![
            ServiceState {
                name: "surrealdb".to_string(),
                port: 8000,
                service_type: "database".to_string(),
                status: "running".to_string(),
                healthy: true,
                failure_count: 0,
                start_time: Some(chrono::Utc::now().to_rfc3339()),
                dependencies: vec![],
            },
            ServiceState {
                name: "glaf-core".to_string(),
                port: 18019,
                service_type: "database".to_string(),
                status: "running".to_string(),
                healthy: true,
                failure_count: 0,
                start_time: Some(chrono::Utc::now().to_rfc3339()),
                dependencies: vec!["surrealdb".to_string()],
            },
            ServiceState {
                name: "cdn-fabric".to_string(),
                port: 18100,
                service_type: "aggregator".to_string(),
                status: "running".to_string(),
                healthy: true,
                failure_count: 0,
                start_time: Some(chrono::Utc::now().to_rfc3339()),
                dependencies: vec!["glaf-core".to_string()],
            },
        ],
        dependencies: vec![
            ServiceDependency {
                from_service: "glaf-core".to_string(),
                to_service: "surrealdb".to_string(),
                dependency_type: "database".to_string(),
                required: true,
            },
            ServiceDependency {
                from_service: "cdn-fabric".to_string(),
                to_service: "glaf-core".to_string(),
                dependency_type: "data_source".to_string(),
                required: true,
            },
        ],
        escalation_log: vec![],
        summary: ServiceSummary {
            total: 3,
            running: 3,
            failed: 0,
            stopped: 0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_to_graph() {
        let events = generate_mock_flow_events(5);
        let workflows = generate_mock_workflows();
        let services = generate_mock_orchestrator_state();

        let graph = flow_to_graph(&events, &workflows, &services);

        assert!(graph.get("nodes").unwrap().as_array().unwrap().len() > 0);
        assert!(graph.get("edges").unwrap().as_array().unwrap().len() > 0);
    }
}
