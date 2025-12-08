/*!
Graph Neural Network (GNN) Integration with Legion ECS
Professional network analysis and pattern recognition
*/

use legion::prelude::*;
use petgraph::{Graph, Directed, NodeIndex, EdgeIndex};
use petgraph::algo::{dijkstra, connected_components};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn, error};
use uuid::Uuid;
use ndarray::{Array1, Array2};

/// GNN Node Component for Legion entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GNNNode {
    pub entity_id: Uuid,
    pub node_type: NodeType,
    pub features: Vec<f32>,
    pub embeddings: Option<Vec<f32>>,
    pub graph_id: Uuid,
}

/// GNN Edge Component representing relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GNNEdge {
    pub edge_id: Uuid,
    pub source_entity: Uuid,
    pub target_entity: Uuid,
    pub edge_type: EdgeType,
    pub weight: f32,
    pub features: Vec<f32>,
}

/// Types of nodes in the professional network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Task,
    Agent,
    Resource,
    Threat,
    Asset,
    Communication,
    Intelligence,
    System,
}

/// Types of edges/relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeType {
    DependsOn,
    Communicates,
    Controls,
    Monitors,
    Threatens,
    Supports,
    Coordinates,
    Analyzes,
}

/// GNN Analysis Results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GNNAnalysisResult {
    pub graph_id: Uuid,
    pub node_count: usize,
    pub edge_count: usize,
    pub connected_components: usize,
    pub critical_nodes: Vec<Uuid>,
    pub network_density: f32,
    pub clustering_coefficient: f32,
    pub shortest_paths: HashMap<(Uuid, Uuid), f32>,
    pub node_centrality: HashMap<Uuid, f32>,
    pub community_structure: Vec<Vec<Uuid>>,
    pub analysis_timestamp: chrono::DateTime<chrono::Utc>,
}

/// Professional Graph Neural Network Manager
pub struct GNNManager {
    graphs: HashMap<Uuid, Graph<GNNNode, GNNEdge, Directed>>,
    node_mappings: HashMap<Uuid, (Uuid, NodeIndex)>, // entity_id -> (graph_id, node_index)
    edge_mappings: HashMap<Uuid, (Uuid, EdgeIndex)>, // edge_id -> (graph_id, edge_index)
    analysis_cache: HashMap<Uuid, GNNAnalysisResult>,
}

impl GNNManager {
    pub fn new() -> Self {
        info!("🕸️ Initializing Graph Neural Network manager for Legion ECS");
        
        Self {
            graphs: HashMap::new(),
            node_mappings: HashMap::new(),
            edge_mappings: HashMap::new(),
            analysis_cache: HashMap::new(),
        }
    }
    
    pub fn create_graph(&mut self, graph_id: Uuid) -> Uuid {
        let graph = Graph::new();
        self.graphs.insert(graph_id, graph);
        info!("📊 Created new GNN graph: {}", graph_id);
        graph_id
    }
    
    pub fn add_node(&mut self, graph_id: Uuid, node: GNNNode) -> Result<NodeIndex, String> {
        if let Some(graph) = self.graphs.get_mut(&graph_id) {
            let node_index = graph.add_node(node.clone());
            self.node_mappings.insert(node.entity_id, (graph_id, node_index));
            
            info!("🔵 Added GNN node: {} (type: {:?})", node.entity_id, node.node_type);
            Ok(node_index)
        } else {
            Err("Graph not found".to_string())
        }
    }
    
    pub fn add_edge(
        &mut self, 
        graph_id: Uuid, 
        edge: GNNEdge
    ) -> Result<EdgeIndex, String> {
        if let Some(graph) = self.graphs.get_mut(&graph_id) {
            // Find node indices
            let source_idx = self.node_mappings.get(&edge.source_entity)
                .and_then(|(gid, idx)| if *gid == graph_id { Some(*idx) } else { None })
                .ok_or("Source node not found")?;
                
            let target_idx = self.node_mappings.get(&edge.target_entity)
                .and_then(|(gid, idx)| if *gid == graph_id { Some(*idx) } else { None })
                .ok_or("Target node not found")?;
            
            let edge_index = graph.add_edge(source_idx, target_idx, edge.clone());
            self.edge_mappings.insert(edge.edge_id, (graph_id, edge_index));
            
            info!("🔗 Added GNN edge: {} -> {} (type: {:?})", 
                  edge.source_entity, edge.target_entity, edge.edge_type);
            Ok(edge_index)
        } else {
            Err("Graph not found".to_string())
        }
    }
    
    pub fn analyze_network(&mut self, graph_id: Uuid) -> Result<GNNAnalysisResult, String> {
        if let Some(graph) = self.graphs.get(&graph_id) {
            info!("🔍 Analyzing GNN graph: {}", graph_id);
            
            let node_count = graph.node_count();
            let edge_count = graph.edge_count();
            
            // Connected components analysis
            let components = connected_components(graph);
            
            // Calculate network density
            let max_edges = if node_count > 1 { 
                node_count * (node_count - 1) 
            } else { 
                1 
            };
            let network_density = edge_count as f32 / max_edges as f32;
            
            // Find critical nodes (high degree centrality)
            let mut critical_nodes = Vec::new();
            let mut node_centrality = HashMap::new();
            
            for node_idx in graph.node_indices() {
                let degree = graph.neighbors(node_idx).count() as f32;
                let centrality = degree / (node_count - 1) as f32;
                
                if let Some(node_data) = graph.node_weight(node_idx) {
                    node_centrality.insert(node_data.entity_id, centrality);
                    
                    // Nodes with centrality > 0.3 are considered critical
                    if centrality > 0.3 {
                        critical_nodes.push(node_data.entity_id);
                    }
                }
            }
            
            // Calculate shortest paths using Dijkstra's algorithm
            let mut shortest_paths = HashMap::new();
            for start_idx in graph.node_indices() {
                if let Some(start_node) = graph.node_weight(start_idx) {
                    let paths = dijkstra(&*graph, start_idx, None, |edge| edge.weight);
                    
                    for (end_idx, distance) in paths {
                        if let Some(end_node) = graph.node_weight(end_idx) {
                            if start_idx != end_idx {
                                shortest_paths.insert(
                                    (start_node.entity_id, end_node.entity_id), 
                                    distance
                                );
                            }
                        }
                    }
                }
            }
            
            // Simulate community detection (in real implementation, use proper algorithms)
            let community_structure = self.simulate_community_detection(graph);
            
            // Simulate clustering coefficient
            let clustering_coefficient = self.calculate_clustering_coefficient(graph);
            
            let result = GNNAnalysisResult {
                graph_id,
                node_count,
                edge_count,
                connected_components: components,
                critical_nodes,
                network_density,
                clustering_coefficient,
                shortest_paths,
                node_centrality,
                community_structure,
                analysis_timestamp: chrono::Utc::now(),
            };
            
            // Cache the analysis
            self.analysis_cache.insert(graph_id, result.clone());
            
            info!("✅ GNN analysis complete: {} nodes, {} edges, {} components", 
                  node_count, edge_count, components);
            
            Ok(result)
        } else {
            Err("Graph not found".to_string())
        }
    }
    
    fn simulate_community_detection(
        &self, 
        graph: &Graph<GNNNode, GNNEdge, Directed>
    ) -> Vec<Vec<Uuid>> {
        // Simplified community detection simulation
        // In production, use proper algorithms like Louvain or Leiden
        let mut communities = Vec::new();
        let mut current_community = Vec::new();
        
        for (idx, node) in graph.node_weights().enumerate() {
            current_community.push(node.entity_id);
            
            // Create new community every 3-5 nodes (simulated)
            if idx % 4 == 3 {
                communities.push(current_community.clone());
                current_community.clear();
            }
        }
        
        if !current_community.is_empty() {
            communities.push(current_community);
        }
        
        communities
    }
    
    fn calculate_clustering_coefficient(
        &self, 
        graph: &Graph<GNNNode, GNNEdge, Directed>
    ) -> f32 {
        // Simplified clustering coefficient calculation
        // In production, implement proper local clustering coefficient
        if graph.node_count() < 3 {
            return 0.0;
        }
        
        let total_triangles = graph.node_count() / 3; // Simulated
        let possible_triangles = graph.node_count() * (graph.node_count() - 1) / 2;
        
        if possible_triangles == 0 {
            0.0
        } else {
            total_triangles as f32 / possible_triangles as f32
        }
    }
    
    pub fn get_graph_metrics(&self, graph_id: Uuid) -> Option<&GNNAnalysisResult> {
        self.analysis_cache.get(&graph_id)
    }
    
    pub fn get_all_graphs(&self) -> Vec<Uuid> {
        self.graphs.keys().cloned().collect()
    }
}

/// Legion ECS System for GNN Processing
pub fn gnn_analysis_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("gnn_analysis_system")
        .write_resource::<GNNManager>()
        .with_query(<(Entity, &GNNNode)>::query())
        .build(|_, world, resources, query| {
            let mut gnn_manager = resources.get_mut::<GNNManager>().unwrap();
            
            // Collect all unique graph IDs
            let mut graph_ids = std::collections::HashSet::new();
            for (_, gnn_node) in query.iter(world) {
                graph_ids.insert(gnn_node.graph_id);
            }
            
            // Analyze each graph
            for graph_id in graph_ids {
                if let Ok(analysis) = gnn_manager.analyze_network(graph_id) {
                    info!("📊 GNN Analysis for graph {}: {} critical nodes, {:.2} density", 
                          graph_id, analysis.critical_nodes.len(), analysis.network_density);
                }
            }
        })
}

/// System for GNN-driven task coordination
pub fn gnn_task_coordination_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("gnn_task_coordination")
        .read_resource::<GNNManager>()
        .with_query(<(Entity, &mut crate::Task, &GNNNode)>::query())
        .build(|_, world, resources, query| {
            let gnn_manager = resources.get::<GNNManager>().unwrap();
            
            for (entity, mut task, gnn_node) in query.iter_mut(world) {
                if let Some(analysis) = gnn_manager.get_graph_metrics(gnn_node.graph_id) {
                    // GNN-driven task priority adjustment
                    if analysis.critical_nodes.contains(&gnn_node.entity_id) {
                        match task.priority {
                            crate::TaskPriority::Low => {
                                task.priority = crate::TaskPriority::Medium;
                                info!("🕸️ GNN elevated critical node task '{}' to Medium priority", task.name);
                            },
                            crate::TaskPriority::Medium => {
                                task.priority = crate::TaskPriority::High;
                                info!("🕸️ GNN elevated critical node task '{}' to High priority", task.name);
                            },
                            _ => {}
                        }
                    }
                    
                    // Coordinate tasks based on network connectivity
                    if analysis.network_density > 0.7 {
                        info!("🕸️ High network density detected - coordinating task execution");
                        // In production: implement sophisticated task coordination
                    }
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gnn_manager_creation() {
        let manager = GNNManager::new();
        assert_eq!(manager.graphs.len(), 0);
    }
    
    #[test]
    fn test_graph_creation() {
        let mut manager = GNNManager::new();
        let graph_id = Uuid::new_v4();
        
        let result = manager.create_graph(graph_id);
        assert_eq!(result, graph_id);
        assert!(manager.graphs.contains_key(&graph_id));
    }
    
    #[test]
    fn test_node_addition() {
        let mut manager = GNNManager::new();
        let graph_id = Uuid::new_v4();
        manager.create_graph(graph_id);
        
        let node = GNNNode {
            entity_id: Uuid::new_v4(),
            node_type: NodeType::Task,
            features: vec![1.0, 2.0, 3.0],
            embeddings: None,
            graph_id,
        };
        
        let result = manager.add_node(graph_id, node.clone());
        assert!(result.is_ok());
        assert!(manager.node_mappings.contains_key(&node.entity_id));
    }
    
    #[test]
    fn test_network_analysis() {
        let mut manager = GNNManager::new();
        let graph_id = Uuid::new_v4();
        manager.create_graph(graph_id);
        
        // Add some nodes
        let node1 = GNNNode {
            entity_id: Uuid::new_v4(),
            node_type: NodeType::Task,
            features: vec![1.0],
            embeddings: None,
            graph_id,
        };
        
        let node2 = GNNNode {
            entity_id: Uuid::new_v4(),
            node_type: NodeType::Agent,
            features: vec![2.0],
            embeddings: None,
            graph_id,
        };
        
        manager.add_node(graph_id, node1.clone()).unwrap();
        manager.add_node(graph_id, node2.clone()).unwrap();
        
        // Add edge
        let edge = GNNEdge {
            edge_id: Uuid::new_v4(),
            source_entity: node1.entity_id,
            target_entity: node2.entity_id,
            edge_type: EdgeType::Coordinates,
            weight: 1.0,
            features: vec![0.5],
        };
        
        manager.add_edge(graph_id, edge).unwrap();
        
        // Analyze network
        let analysis = manager.analyze_network(graph_id).unwrap();
        assert_eq!(analysis.node_count, 2);
        assert_eq!(analysis.edge_count, 1);
    }
}