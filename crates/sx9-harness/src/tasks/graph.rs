//! Task Dependency Graph
//!
//! DAG structure for task execution ordering and narrative generation.

use super::core::{Task, TaskId, TASKS};
use std::collections::{HashMap, HashSet, VecDeque};

/// Task graph for dependency resolution
pub struct TaskGraph {
    /// Adjacency list: task -> tasks it enables
    edges: HashMap<TaskId, Vec<TaskId>>,
    /// Reverse adjacency: task -> tasks it requires
    reverse: HashMap<TaskId, Vec<TaskId>>,
}

impl TaskGraph {
    /// Build graph from task definitions
    pub fn new() -> Self {
        let mut edges: HashMap<TaskId, Vec<TaskId>> = HashMap::new();
        let mut reverse: HashMap<TaskId, Vec<TaskId>> = HashMap::new();

        for task in TASKS {
            edges.entry(task.id).or_default();
            reverse.entry(task.id).or_default();

            for &req in task.requires {
                edges.entry(req).or_default().push(task.id);
                reverse.entry(task.id).or_default().push(req);
            }
        }

        Self { edges, reverse }
    }

    /// Get tasks that can run after this task completes
    pub fn enables(&self, id: TaskId) -> &[TaskId] {
        self.edges.get(&id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Get tasks required before this task can run
    pub fn requires(&self, id: TaskId) -> &[TaskId] {
        self.reverse.get(&id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Get root tasks (no dependencies)
    pub fn roots(&self) -> Vec<TaskId> {
        TASKS
            .iter()
            .filter(|t| t.requires.is_empty())
            .map(|t| t.id)
            .collect()
    }

    /// Get leaf tasks (nothing depends on them)
    pub fn leaves(&self) -> Vec<TaskId> {
        TASKS
            .iter()
            .filter(|t| self.enables(t.id).is_empty())
            .map(|t| t.id)
            .collect()
    }

    /// Topological sort for execution order
    pub fn topo_sort(&self) -> Vec<TaskId> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut temp = HashSet::new();

        fn visit(
            id: TaskId,
            graph: &TaskGraph,
            visited: &mut HashSet<TaskId>,
            temp: &mut HashSet<TaskId>,
            result: &mut Vec<TaskId>,
        ) {
            if visited.contains(&id) {
                return;
            }
            if temp.contains(&id) {
                panic!("Cycle detected at {:?}", id);
            }
            temp.insert(id);
            for &next in graph.enables(id) {
                visit(next, graph, visited, temp, result);
            }
            temp.remove(&id);
            visited.insert(id);
            result.push(id);
        }

        for task in TASKS {
            visit(task.id, self, &mut visited, &mut temp, &mut result);
        }

        result.reverse();
        result
    }

    /// Get execution layers (tasks that can run in parallel)
    pub fn layers(&self) -> Vec<Vec<TaskId>> {
        let mut layers = Vec::new();
        let mut remaining: HashSet<TaskId> = TASKS.iter().map(|t| t.id).collect();
        let mut completed: HashSet<TaskId> = HashSet::new();

        while !remaining.is_empty() {
            let layer: Vec<TaskId> = remaining
                .iter()
                .filter(|&&id| {
                    self.requires(id).iter().all(|r| completed.contains(r))
                })
                .copied()
                .collect();

            if layer.is_empty() {
                panic!("Cycle detected in task graph");
            }

            for &id in &layer {
                remaining.remove(&id);
                completed.insert(id);
            }

            layers.push(layer);
        }

        layers
    }

    /// Find path between two tasks
    pub fn path(&self, from: TaskId, to: TaskId) -> Option<Vec<TaskId>> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent: HashMap<TaskId, TaskId> = HashMap::new();

        queue.push_back(from);
        visited.insert(from);

        while let Some(current) = queue.pop_front() {
            if current == to {
                let mut path = vec![to];
                let mut node = to;
                while let Some(&p) = parent.get(&node) {
                    path.push(p);
                    node = p;
                }
                path.reverse();
                return Some(path);
            }

            for &next in self.enables(current) {
                if !visited.contains(&next) {
                    visited.insert(next);
                    parent.insert(next, current);
                    queue.push_back(next);
                }
            }
        }

        None
    }

    /// Export as DOT format for visualization
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph TaskGraph {\n");
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("  node [shape=box];\n\n");

        // Group by domain
        let domains = [
            ("SESSION", vec![TaskId::StartSession, TaskId::EndSession, TaskId::CheckpointSession]),
            ("MEMORY", vec![TaskId::ReadKnowledge, TaskId::WriteKnowledge, TaskId::ReadProgress, TaskId::WriteProgress, TaskId::ReadFocus, TaskId::WriteFocus]),
            ("CONTEXT", vec![TaskId::GatherContext, TaskId::SummarizeContext, TaskId::PruneContext]),
            ("LINEAR", vec![TaskId::FetchIssue, TaskId::UpdateIssue, TaskId::CreateIssue, TaskId::CloseIssue]),
            ("GIT", vec![TaskId::ReadStatus, TaskId::CreateBranch, TaskId::StageChanges, TaskId::CreateCommit, TaskId::CreateCheckpoint, TaskId::PushBranch]),
            ("CODE", vec![TaskId::ReadFile, TaskId::WriteFile, TaskId::EditFile, TaskId::DeleteFile, TaskId::GenerateCode, TaskId::RefactorCode]),
            ("QA", vec![TaskId::RunTests, TaskId::RunStaticGate, TaskId::RunArchGate, TaskId::RunPatternGate, TaskId::GenerateReport]),
            ("NATS", vec![TaskId::PublishEvent, TaskId::SubscribeTopic, TaskId::RequestReply]),
            ("AGENT", vec![TaskId::DispatchTask, TaskId::ReceiveResult, TaskId::HandoffTask]),
        ];

        for (name, tasks) in domains {
            dot.push_str(&format!("  subgraph cluster_{} {{\n", name.to_lowercase()));
            dot.push_str(&format!("    label=\"{}\";\n", name));
            for task in tasks {
                dot.push_str(&format!("    {:?};\n", task));
            }
            dot.push_str("  }\n\n");
        }

        // Edges
        for task in TASKS {
            for &req in task.requires {
                dot.push_str(&format!("  {:?} -> {:?};\n", req, task.id));
            }
        }

        dot.push_str("}\n");
        dot
    }
}

impl Default for TaskGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_roots() {
        let graph = TaskGraph::new();
        let roots = graph.roots();
        assert!(roots.contains(&TaskId::StartSession));
        assert!(roots.contains(&TaskId::ReadFile));
        assert!(roots.contains(&TaskId::FetchIssue));
    }

    #[test]
    fn test_graph_enables() {
        let graph = TaskGraph::new();
        let enables = graph.enables(TaskId::StageChanges);
        assert!(enables.contains(&TaskId::CreateCommit));
    }

    #[test]
    fn test_topo_sort() {
        let graph = TaskGraph::new();
        let sorted = graph.topo_sort();

        // StageChanges must come before CreateCommit
        let stage_pos = sorted.iter().position(|&t| t == TaskId::StageChanges).unwrap();
        let commit_pos = sorted.iter().position(|&t| t == TaskId::CreateCommit).unwrap();
        assert!(stage_pos < commit_pos);
    }

    #[test]
    fn test_layers() {
        let graph = TaskGraph::new();
        let layers = graph.layers();

        // First layer should contain roots
        assert!(layers[0].contains(&TaskId::StartSession));
    }

    #[test]
    fn test_path() {
        let graph = TaskGraph::new();
        let path = graph.path(TaskId::GatherContext, TaskId::GenerateCode);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path[0], TaskId::GatherContext);
        assert_eq!(*path.last().unwrap(), TaskId::GenerateCode);
    }
}
