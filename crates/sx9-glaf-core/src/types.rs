use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Unique identifier for nodes and edges
pub type NodeId = String;
pub type EdgeId = String;

/// XY Position
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct XYPosition {
    pub x: f64,
    pub y: f64,
}

/// Node Dimensions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
}

/// User-facing Node structure (matches graph-db pattern)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub r#type: String, // "type" is reserved in Rust
    pub position: XYPosition,
    pub data: HashMap<String, Value>,

    // Optional flags
    #[serde(default)]
    pub draggable: bool,
    #[serde(default)]
    pub selectable: bool,
    #[serde(default)]
    pub connectable: bool,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub selected: bool,

    // Read-only system fields (optional in user input)
    pub measured: Option<Dimensions>,
}

/// Internal Node structure (System computed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalNode {
    #[serde(flatten)]
    pub node: Node,
    pub internals: NodeInternals,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInternals {
    pub position_absolute: XYPosition,
    pub z: i32,
}

/// Edge structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: EdgeId,
    pub source: NodeId,
    pub target: NodeId,
    pub r#type: Option<String>,
    pub data: Option<HashMap<String, Value>>,
    #[serde(default)]
    pub selected: bool,
    #[serde(default)]
    pub animated: bool,
}

/// Node Change Discriminated Union
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum NodeChange {
    #[serde(rename = "position")]
    Position {
        id: NodeId,
        position: Option<XYPosition>,
        dragging: Option<bool>,
    },
    #[serde(rename = "dimensions")]
    Dimensions {
        id: NodeId,
        dimensions: Dimensions,
        resizing: Option<bool>,
    },
    #[serde(rename = "select")]
    Selection { id: NodeId, selected: bool },
    #[serde(rename = "remove")]
    Remove { id: NodeId },
    #[serde(rename = "add")]
    Add { item: Node, index: Option<usize> },
    #[serde(rename = "replace")]
    Replace { id: NodeId, item: Node },
    #[serde(rename = "upsert")]
    Upsert { item: Node },
}

/// Apply node changes logic (Pure function equivalent)
pub fn apply_node_changes(changes: Vec<NodeChange>, nodes: &mut Vec<Node>) {
    for change in changes {
        match change {
            NodeChange::Position { id, position, .. } => {
                if let Some(pos) = position {
                    if let Some(node) = nodes.iter_mut().find(|n| n.id == id) {
                        node.position = pos;
                    }
                }
            }
            NodeChange::Dimensions { id, dimensions, .. } => {
                if let Some(node) = nodes.iter_mut().find(|n| n.id == id) {
                    node.measured = Some(dimensions);
                }
            }
            NodeChange::Selection { id, selected } => {
                if let Some(node) = nodes.iter_mut().find(|n| n.id == id) {
                    node.selected = selected;
                }
            }
            NodeChange::Remove { id } => {
                nodes.retain(|n| n.id != id);
            }
            NodeChange::Add { item, index } => match index {
                Some(i) => {
                    if i <= nodes.len() {
                        nodes.insert(i, item);
                    } else {
                        nodes.push(item);
                    }
                }
                None => nodes.push(item),
            },
            NodeChange::Replace { id, item } => {
                if let Some(idx) = nodes.iter().position(|n| n.id == id) {
                    nodes[idx] = item;
                }
            }
            NodeChange::Upsert { item } => {
                if let Some(idx) = nodes.iter().position(|n| n.id == item.id) {
                    nodes[idx] = item;
                } else {
                    nodes.push(item);
                }
            }
        }
    }
}
