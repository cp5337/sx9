//! Graph Relationship Analysis
//! Node mapping and tactical relationship modeling

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use petgraph::{Graph, Directed};
use petgraph::graph::NodeIndex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeRelationship {
    pub relationship_id: String,
    pub source_node: String,
    pub target_node: String,
    pub relationship_type: RelationshipType,
    pub strength: f64,
    pub directionality: Directionality,
    pub tactical_context: String,
    pub metadata: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    DependsOn,
    CommunicatesWith,
    ControlsAccess,
    SharesData,
    MonitorsStatus,
    ProvidesService,
    RequiresResource,
    TacticalSupport,
    CommandStructure,
    LogisticalSupport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Directionality {
    Unidirectional,
    Bidirectional,
    Conditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TacticalNode {
    pub node_id: String,
    pub node_type: NodeType,
    pub capabilities: Vec<String>,
    pub operational_status: String,
    pub geographic_location: Option<(f64, f64)>,
    pub security_classification: String,
    pub tactical_attributes: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    SystemComponent,
    TacticalUnit,
    CommunicationHub,
    DataSource,
    DecisionNode,
    ResourceProvider,
    SensorPlatform,
    WeaponSystem,
    CommandPost,
    LogisticsNode,
}

pub type TacticalGraph = Graph<TacticalNode, NodeRelationship, Directed>;

pub fn build_graph_relationships(interview_results: &Value) -> Vec<NodeRelationship> {
    let mut relationships = Vec::new();

    // Extract and analyze relationship patterns from interview data
    if let Some(dependency_data) = interview_results.get("dependency_graph") {
        relationships.extend(extract_dependency_relationships(dependency_data));
    }

    if let Some(communication_data) = interview_results.get("communication_paths") {
        relationships.extend(extract_communication_relationships(communication_data));
    }

    if let Some(tactical_data) = interview_results.get("tactical_connections") {
        relationships.extend(extract_tactical_relationships(tactical_data));
    }

    // Add derived relationships based on analysis
    relationships.extend(infer_tactical_relationships(&interview_results));

    relationships
}

fn extract_dependency_relationships(dependency_data: &Value) -> Vec<NodeRelationship> {
    let mut relationships = Vec::new();

    // Parse dependency chains and create relationship objects
    if let Some(dependencies) = dependency_data.get("critical_paths") {
        if let Some(paths) = dependencies.as_array() {
            for (i, path) in paths.iter().enumerate() {
                if let Some(path_str) = path.as_str() {
                    let relationship = NodeRelationship {
                        relationship_id: format!("DEP-{:03}", i),
                        source_node: "system_core".to_string(),
                        target_node: path_str.to_string(),
                        relationship_type: RelationshipType::DependsOn,
                        strength: 0.8,
                        directionality: Directionality::Unidirectional,
                        tactical_context: "Critical operational dependency".to_string(),
                        metadata: {
                            let mut map = HashMap::new();
                            map.insert("criticality".to_string(), json!("HIGH"));
                            map.insert("failure_impact".to_string(), json!("MISSION_CRITICAL"));
                            map
                        },
                    };
                    relationships.push(relationship);
                }
            }
        }
    }

    relationships
}

fn extract_communication_relationships(communication_data: &Value) -> Vec<NodeRelationship> {
    let mut relationships = Vec::new();

    // Parse communication pathways
    if let Some(channels) = communication_data.get("primary_channels") {
        if let Some(channel_array) = channels.as_array() {
            for (i, channel) in channel_array.iter().enumerate() {
                if let Some(channel_str) = channel.as_str() {
                    let relationship = NodeRelationship {
                        relationship_id: format!("COMM-{:03}", i),
                        source_node: "command_center".to_string(),
                        target_node: format!("{}_endpoint", channel_str),
                        relationship_type: RelationshipType::CommunicatesWith,
                        strength: 0.9,
                        directionality: Directionality::Bidirectional,
                        tactical_context: "Primary communication channel".to_string(),
                        metadata: {
                            let mut map = HashMap::new();
                            map.insert("channel_type".to_string(), json!(channel_str));
                            map.insert("reliability".to_string(), json!(0.95));
                            map.insert("encryption".to_string(), json!("AES-256"));
                            map
                        },
                    };
                    relationships.push(relationship);
                }
            }
        }
    }

    relationships
}

fn extract_tactical_relationships(tactical_data: &Value) -> Vec<NodeRelationship> {
    let mut relationships = Vec::new();

    // Parse tactical unit relationships
    if let Some(units) = tactical_data.get("friendly_units") {
        if let Some(unit_array) = units.as_array() {
            for (i, unit) in unit_array.iter().enumerate() {
                let relationship = NodeRelationship {
                    relationship_id: format!("TAC-{:03}", i),
                    source_node: "tactical_command".to_string(),
                    target_node: format!("unit_{}", i),
                    relationship_type: RelationshipType::CommandStructure,
                    strength: 0.85,
                    directionality: Directionality::Unidirectional,
                    tactical_context: "Command and control relationship".to_string(),
                    metadata: {
                        let mut map = HashMap::new();
                        map.insert("command_level".to_string(), json!("OPERATIONAL"));
                        map.insert("authority_type".to_string(), json!("DIRECT"));
                        map
                    },
                };
                relationships.push(relationship);
            }
        }
    }

    relationships
}

fn infer_tactical_relationships(interview_results: &Value) -> Vec<NodeRelationship> {
    let mut relationships = Vec::new();

    // Infer relationships based on tactical patterns

    // Intelligence sharing relationships
    let intel_relationship = NodeRelationship {
        relationship_id: "INFERRED-INTEL-001".to_string(),
        source_node: "intelligence_fusion_center".to_string(),
        target_node: "tactical_operations_center".to_string(),
        relationship_type: RelationshipType::SharesData,
        strength: 0.7,
        directionality: Directionality::Unidirectional,
        tactical_context: "Intelligence data flow".to_string(),
        metadata: {
            let mut map = HashMap::new();
            map.insert("data_type".to_string(), json!("TACTICAL_INTELLIGENCE"));
            map.insert("update_frequency".to_string(), json!("REAL_TIME"));
            map.insert("classification".to_string(), json!("SECRET"));
            map
        },
    };
    relationships.push(intel_relationship);

    // Logistics support relationships
    let logistics_relationship = NodeRelationship {
        relationship_id: "INFERRED-LOG-001".to_string(),
        source_node: "logistics_support_area".to_string(),
        target_node: "forward_operating_base".to_string(),
        relationship_type: RelationshipType::LogisticalSupport,
        strength: 0.8,
        directionality: Directionality::Bidirectional,
        tactical_context: "Supply chain relationship".to_string(),
        metadata: {
            let mut map = HashMap::new();
            map.insert("supply_categories".to_string(), json!(["ammunition", "fuel", "medical", "food"]));
            map.insert("resupply_schedule".to_string(), json!("DAILY"));
            map
        },
    };
    relationships.push(logistics_relationship);

    // Sensor-to-decision relationships
    let sensor_relationship = NodeRelationship {
        relationship_id: "INFERRED-SENSOR-001".to_string(),
        source_node: "isr_platform".to_string(),
        target_node: "targeting_cell".to_string(),
        relationship_type: RelationshipType::ProvidesService,
        strength: 0.9,
        directionality: Directionality::Unidirectional,
        tactical_context: "Target acquisition data flow".to_string(),
        metadata: {
            let mut map = HashMap::new();
            map.insert("sensor_type".to_string(), json!("MULTI_SPECTRAL"));
            map.insert("coverage_area".to_string(), json!("AREA_OF_OPERATIONS"));
            map.insert("resolution".to_string(), json!("HIGH"));
            map
        },
    };
    relationships.push(sensor_relationship);

    relationships
}

pub fn analyze_relationship_patterns(relationships: &[NodeRelationship]) -> Value {
    let mut pattern_analysis = HashMap::new();

    // Analyze relationship type distribution
    let mut type_counts = HashMap::new();
    for rel in relationships {
        let count = type_counts.entry(format!("{:?}", rel.relationship_type)).or_insert(0);
        *count += 1;
    }
    pattern_analysis.insert("relationship_type_distribution".to_string(), json!(type_counts));

    // Analyze strength distribution
    let strengths: Vec<f64> = relationships.iter().map(|r| r.strength).collect();
    let avg_strength = strengths.iter().sum::<f64>() / strengths.len() as f64;
    pattern_analysis.insert("average_relationship_strength".to_string(), json!(avg_strength));

    // Analyze directionality patterns
    let mut directionality_counts = HashMap::new();
    for rel in relationships {
        let count = directionality_counts.entry(format!("{:?}", rel.directionality)).or_insert(0);
        *count += 1;
    }
    pattern_analysis.insert("directionality_distribution".to_string(), json!(directionality_counts));

    // Identify critical relationships
    let critical_relationships: Vec<&NodeRelationship> = relationships
        .iter()
        .filter(|r| r.strength > 0.8)
        .collect();
    pattern_analysis.insert("critical_relationship_count".to_string(), json!(critical_relationships.len()));

    json!(pattern_analysis)
}

pub fn build_tactical_graph(relationships: &[NodeRelationship]) -> TacticalGraph {
    let mut graph = Graph::new();
    let mut node_indices = HashMap::new();

    // Add nodes to graph
    for relationship in relationships {
        // Add source node if not exists
        if !node_indices.contains_key(&relationship.source_node) {
            let node = create_tactical_node(&relationship.source_node);
            let idx = graph.add_node(node);
            node_indices.insert(relationship.source_node.clone(), idx);
        }

        // Add target node if not exists
        if !node_indices.contains_key(&relationship.target_node) {
            let node = create_tactical_node(&relationship.target_node);
            let idx = graph.add_node(node);
            node_indices.insert(relationship.target_node.clone(), idx);
        }

        // Add edge
        let source_idx = node_indices[&relationship.source_node];
        let target_idx = node_indices[&relationship.target_node];
        graph.add_edge(source_idx, target_idx, relationship.clone());
    }

    graph
}

fn create_tactical_node(node_id: &str) -> TacticalNode {
    let (node_type, capabilities) = match node_id {
        id if id.contains("command") => (NodeType::CommandPost, vec!["command_control".to_string(), "decision_making".to_string()]),
        id if id.contains("sensor") || id.contains("isr") => (NodeType::SensorPlatform, vec!["surveillance".to_string(), "reconnaissance".to_string()]),
        id if id.contains("comm") => (NodeType::CommunicationHub, vec!["data_relay".to_string(), "network_management".to_string()]),
        id if id.contains("logistics") => (NodeType::LogisticsNode, vec!["supply_management".to_string(), "transport".to_string()]),
        id if id.contains("intel") => (NodeType::DataSource, vec!["intelligence_analysis".to_string(), "data_fusion".to_string()]),
        _ => (NodeType::SystemComponent, vec!["basic_operations".to_string()]),
    };

    TacticalNode {
        node_id: node_id.to_string(),
        node_type,
        capabilities,
        operational_status: "OPERATIONAL".to_string(),
        geographic_location: None,
        security_classification: "UNCLASSIFIED".to_string(),
        tactical_attributes: HashMap::new(),
    }
}

pub fn calculate_network_metrics(graph: &TacticalGraph) -> Value {
    json!({
        "node_count": graph.node_count(),
        "edge_count": graph.edge_count(),
        "density": calculate_graph_density(graph),
        "clustering_coefficient": calculate_clustering_coefficient(graph),
        "critical_nodes": identify_critical_nodes(graph),
        "network_resilience": assess_network_resilience(graph)
    })
}

fn calculate_graph_density(graph: &TacticalGraph) -> f64 {
    let nodes = graph.node_count() as f64;
    let edges = graph.edge_count() as f64;

    if nodes <= 1.0 {
        return 0.0;
    }

    edges / (nodes * (nodes - 1.0))
}

fn calculate_clustering_coefficient(_graph: &TacticalGraph) -> f64 {
    // Simplified clustering coefficient calculation
    0.65 // Placeholder
}

fn identify_critical_nodes(graph: &TacticalGraph) -> Vec<String> {
    let mut critical_nodes = Vec::new();

    for node_idx in graph.node_indices() {
        let in_degree = graph.neighbors_directed(node_idx, petgraph::Direction::Incoming).count();
        let out_degree = graph.neighbors_directed(node_idx, petgraph::Direction::Outgoing).count();

        if in_degree + out_degree > 3 { // Threshold for criticality
            if let Some(node) = graph.node_weight(node_idx) {
                critical_nodes.push(node.node_id.clone());
            }
        }
    }

    critical_nodes
}

fn assess_network_resilience(_graph: &TacticalGraph) -> f64 {
    // Simplified resilience assessment
    0.78 // Placeholder
}