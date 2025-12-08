//! Node types and components for SlotGraph ECS architecture

use bevy::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Core SlotGraph node component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct SlotGraphNode {
    pub id: Uuid,
    pub position: Vec3, // Scaled to real-world distance
    pub metadata: HashMap<String, String>,
    pub state: NodeState,
    pub node_type: SlotNodeType,
}

/// Node state tracking
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct NodeState {
    pub status: NodeStatus,
    pub last_update: f32,
    pub health_score: f32,
    pub execution_count: u32,
    pub failure_count: u32,
}

/// Node status enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeStatus {
    Inactive,
    Ready,
    Executing,
    Completed,
    Failed,
    Critical,
    Maintenance,
}

/// SlotGraph node types mapping to CTAS Universal Cognigraph atoms
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SlotNodeType {
    // Universal Node Types
    Source,
    Sink, 
    Transformer,
    Router,
    Buffer,
    Gate,
    Monitor,
    Catalyst,
    Inhibitor,
    Relay,
    
    // CTAS Node Types
    People,
    Object,
    Location,
    Event,
    Relationship,
    Task,
    Asset,
    Agent,
    System,
    Intelligence,
}

/// Task metadata component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct TaskMetadata {
    pub task_name: String,
    pub task_id: Uuid,
    pub hd4_phase: HD4Phase,
    pub priority: TaskPriority,
    pub estimated_duration: f32,
    pub required_capabilities: Vec<String>,
    pub dependencies: Vec<Uuid>,
    pub hash_id: Option<String>,
}

/// HD4 operational phases
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HD4Phase {
    Hunt,
    Detect,
    Disrupt,
    Disable,
    Dominate,
}

/// Task priority levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskPriority {
    Critical,
    High,
    Medium,
    Low,
    Background,
}

/// Geospatial component for nodes with location data
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct GeospatialData {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub coordinate_system: CoordinateSystem,
}

/// Coordinate system types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CoordinateSystem {
    WGS84,
    NAD83,
    ETRS89,
    MGRS(String), // Grid zone designator
}

/// Network connectivity component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct NetworkNode {
    pub ip_address: Option<String>,
    pub port_range: Option<(u16, u16)>,
    pub protocols: Vec<String>,
    pub connectivity_status: ConnectivityStatus,
}

/// Network connectivity status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectivityStatus {
    Online,
    Offline,
    Intermittent,
    Restricted,
    Unknown,
}

/// Intelligence data component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceData {
    pub intelligence_type: IntelligenceType,
    pub confidence_level: f32, // 0.0 to 1.0
    pub source_reliability: f32, // 0.0 to 1.0
    pub collection_time: f64,
    pub eei_priority: EEIPriority,
}

/// Intelligence types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IntelligenceType {
    HUMINT, // Human Intelligence
    SIGINT, // Signals Intelligence
    GEOINT, // Geospatial Intelligence
    OSINT,  // Open Source Intelligence
    TECHINT, // Technical Intelligence
    FININT,  // Financial Intelligence
    CYBINT,  // Cyber Intelligence
}

/// Essential Elements of Information priority
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EEIPriority {
    Priority1, // Critical
    Priority2, // Essential
    Priority3, // Desirable
    Priority4, // Background,
}

impl SlotGraphNode {
    /// Create a new SlotGraph node
    pub fn new(
        node_type: SlotNodeType,
        position: Vec3,
        metadata: HashMap<String, String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            metadata,
            state: NodeState::default(),
            node_type,
        }
    }
    
    /// Create a system control node
    pub fn system_node(name: String, position: Vec3) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("name".to_string(), name);
        metadata.insert("role".to_string(), "system".to_string());
        
        Self::new(SlotNodeType::System, position, metadata)
    }
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            status: NodeStatus::Inactive,
            last_update: 0.0,
            health_score: 1.0,
            execution_count: 0,
            failure_count: 0,
        }
    }
}

impl TaskMetadata {
    /// Check if task can be executed based on dependencies and state
    pub fn can_execute(&self) -> bool {
        // TODO: Implement dependency checking logic
        // For now, simple check based on priority
        matches!(self.priority, TaskPriority::Critical | TaskPriority::High)
    }
}

/// Bundle for spawning complete SlotGraph nodes
#[derive(Bundle)]
pub struct SlotGraphBundle {
    pub node: SlotGraphNode,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl SlotGraphBundle {
    /// Create a basic node bundle
    pub fn new(node: SlotGraphNode) -> Self {
        let transform = Transform::from_translation(node.position);
        
        Self {
            node,
            transform,
            global_transform: GlobalTransform::default(),
        }
    }
    
    /// Create a system node bundle
    pub fn system_node(name: String, position: Vec3) -> Self {
        let node = SlotGraphNode::system_node(name, position);
        Self::new(node)
    }
    
    /// Create a task node bundle
    pub fn task_node(
        task_name: String,
        hd4_phase: HD4Phase,
        position: Vec3,
    ) -> (Self, TaskMetadata) {
        let mut metadata = HashMap::new();
        metadata.insert("task_name".to_string(), task_name.clone());
        metadata.insert("hd4_phase".to_string(), format!("{:?}", hd4_phase));
        
        let node = SlotGraphNode::new(SlotNodeType::Task, position, metadata);
        let task_metadata = TaskMetadata {
            task_name,
            task_id: node.id,
            hd4_phase,
            priority: TaskPriority::Medium,
            estimated_duration: 1.0,
            required_capabilities: vec![],
            dependencies: vec![],
            hash_id: None,
        };
        
        (Self::new(node), task_metadata)
    }
}
