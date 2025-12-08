use legion::*;
use petgraph::{graph::NodeIndex, Graph};
use std::collections::HashMap;

use crate::components::{Position, TriptyxId};

pub struct SlotGraphView {
    pub graph: Graph<usize, f64>,
    pub ent_to_idx: HashMap<Entity, NodeIndex>,
}

impl SlotGraphView {
    pub fn build(world: &World) -> Self {
        let mut graph = Graph::<usize, f64>::new();
        let mut ent_to_idx = HashMap::new();
        let mut query = <(Entity, Read<Position>)>::query();
        for (entity, _pos) in query.iter(world) {
            let idx = graph.add_node(entity.index());
            ent_to_idx.insert(*entity, idx);
        }
        Self { graph, ent_to_idx }
    }
    pub fn degree_norm(&self, idx: NodeIndex, scale: f64) -> f64 {
        let deg = self.graph.neighbors_undirected(idx).count() as f64;
        (deg / scale).clamp(0.0, 1.0)
    }
}
