//! Task planner for autonomous reasoning
//! 
//! Decomposes high-level goals into executable task graphs using
//! Hierarchical Task Network (HTN) style planning.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;

/// A node in the task graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskNode {
    /// Unique node identifier
    pub id: Uuid,
    /// Task description/query
    pub task: String,
    /// Node type
    pub node_type: NodeType,
    /// Dependencies (must complete before this)
    pub dependencies: Vec<Uuid>,
    /// Status
    pub status: TaskStatus,
    /// Estimated complexity (0.0 - 1.0)
    pub complexity: f64,
    /// Metadata
    pub metadata: serde_json::Value,
}

/// Type of task node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    /// Root goal
    Root,
    /// Decomposed sub-task
    SubTask,
    /// Leaf task (executable)
    Leaf,
}

/// Task execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    Ready,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

/// Task graph (DAG) representing the plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskGraph {
    /// All nodes in the graph
    pub nodes: HashMap<Uuid, TaskNode>,
    /// Root node ID
    pub root: Uuid,
    /// Adjacency list (node -> children)
    pub edges: HashMap<Uuid, Vec<Uuid>>,
}

impl TaskNode {
    /// Create a new task node
    pub fn new(task: impl Into<String>, node_type: NodeType) -> Self {
        Self {
            id: Uuid::new_v4(),
            task: task.into(),
            node_type,
            dependencies: Vec::new(),
            status: TaskStatus::Pending,
            complexity: 0.5,
            metadata: serde_json::json!({}),
        }
    }

    /// Add dependency
    pub fn add_dependency(&mut self, dep_id: Uuid) {
        if !self.dependencies.contains(&dep_id) {
            self.dependencies.push(dep_id);
        }
    }

    /// Check if all dependencies are completed
    pub fn dependencies_met(&self, graph: &TaskGraph) -> bool {
        self.dependencies.iter().all(|dep_id| {
            graph
                .nodes
                .get(dep_id)
                .map(|n| n.status == TaskStatus::Completed)
                .unwrap_or(false)
        })
    }

    /// Check if node is ready to execute
    pub fn is_ready(&self, graph: &TaskGraph) -> bool {
        self.status == TaskStatus::Pending && self.dependencies_met(graph)
    }
}

impl TaskGraph {
    /// Create a new task graph with root node
    pub fn new(root_task: impl Into<String>) -> Self {
        let root_node = TaskNode::new(root_task, NodeType::Root);
        let root_id = root_node.id;
        
        let mut nodes = HashMap::new();
        nodes.insert(root_id, root_node);
        
        Self {
            nodes,
            root: root_id,
            edges: HashMap::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: TaskNode) -> Uuid {
        let id = node.id;
        self.nodes.insert(id, node);
        id
    }

    /// Add an edge (parent -> child)
    pub fn add_edge(&mut self, parent: Uuid, child: Uuid) {
        self.edges.entry(parent).or_insert_with(Vec::new).push(child);
        
        // Add dependency to child
        if let Some(child_node) = self.nodes.get_mut(&child) {
            child_node.add_dependency(parent);
        }
    }

    /// Get node by ID
    pub fn get_node(&self, id: &Uuid) -> Option<&TaskNode> {
        self.nodes.get(id)
    }

    /// Get mutable node by ID
    pub fn get_node_mut(&mut self, id: &Uuid) -> Option<&mut TaskNode> {
        self.nodes.get_mut(id)
    }

    /// Get children of a node
    pub fn children(&self, id: &Uuid) -> Vec<&TaskNode> {
        self.edges
            .get(id)
            .map(|children| {
                children
                    .iter()
                    .filter_map(|child_id| self.nodes.get(child_id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all leaf nodes
    pub fn leaves(&self) -> Vec<&TaskNode> {
        self.nodes
            .values()
            .filter(|n| n.node_type == NodeType::Leaf)
            .collect()
    }

    /// Get all ready nodes (dependencies met, not started)
    pub fn ready_nodes(&self) -> Vec<&TaskNode> {
        self.nodes
            .values()
            .filter(|n| n.is_ready(self))
            .collect()
    }

    /// Topological sort (execution order)
    pub fn topological_sort(&self) -> Result<Vec<Uuid>, String> {
        let mut in_degree: HashMap<Uuid, usize> = HashMap::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        // Calculate in-degrees
        for node_id in self.nodes.keys() {
            in_degree.insert(*node_id, 0);
        }
        
        for (_, children) in &self.edges {
            for child_id in children {
                *in_degree.get_mut(child_id).unwrap() += 1;
            }
        }

        // Find nodes with no dependencies
        for (node_id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(*node_id);
            }
        }

        // Process queue
        while let Some(node_id) = queue.pop_front() {
            result.push(node_id);

            if let Some(children) = self.edges.get(&node_id) {
                for child_id in children {
                    let degree = in_degree.get_mut(child_id).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(*child_id);
                    }
                }
            }
        }

        // Check for cycles
        if result.len() != self.nodes.len() {
            return Err("Cycle detected in task graph".to_string());
        }

        Ok(result)
    }

    /// Validate graph structure
    pub fn validate(&self) -> Result<(), String> {
        // Check root exists
        if !self.nodes.contains_key(&self.root) {
            return Err("Root node not found".to_string());
        }

        // Check all edges reference valid nodes
        for (parent, children) in &self.edges {
            if !self.nodes.contains_key(parent) {
                return Err(format!("Invalid parent node: {}", parent));
            }
            for child in children {
                if !self.nodes.contains_key(child) {
                    return Err(format!("Invalid child node: {}", child));
                }
            }
        }

        // Check for cycles
        self.topological_sort()?;

        Ok(())
    }

    /// Calculate total complexity
    pub fn total_complexity(&self) -> f64 {
        self.nodes.values().map(|n| n.complexity).sum()
    }

    /// Get completion percentage
    pub fn completion_percentage(&self) -> f64 {
        if self.nodes.is_empty() {
            return 0.0;
        }

        let completed = self
            .nodes
            .values()
            .filter(|n| n.status == TaskStatus::Completed)
            .count();

        (completed as f64 / self.nodes.len() as f64) * 100.0
    }
}

/// Planner for decomposing goals into task graphs
pub struct Planner {
    /// Maximum decomposition depth
    max_depth: usize,
    /// Maximum nodes per graph
    max_nodes: usize,
}

impl Planner {
    /// Create a new planner
    pub fn new() -> Self {
        Self {
            max_depth: 4,
            max_nodes: 32,
        }
    }

    /// Set maximum depth
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Set maximum nodes
    pub fn with_max_nodes(mut self, nodes: usize) -> Self {
        self.max_nodes = nodes;
        self
    }

    /// Decompose a goal into a task graph
    pub fn plan(&self, goal: &str) -> Result<TaskGraph, String> {
        let mut graph = TaskGraph::new(goal);
        
        // Simple decomposition strategy
        // TODO: Implement more sophisticated HTN planning
        let sub_tasks = self.decompose_goal(goal)?;
        
        for (i, sub_task) in sub_tasks.iter().enumerate() {
            let mut node = TaskNode::new(sub_task, NodeType::SubTask);
            node.complexity = 0.3;
            
            // Sequential dependencies
            if i > 0 {
                let prev_id = graph.nodes.keys().nth(i).copied().unwrap();
                node.add_dependency(prev_id);
            } else {
                node.add_dependency(graph.root);
            }
            
            let node_id = graph.add_node(node);
            graph.add_edge(graph.root, node_id);
        }

        graph.validate()?;
        Ok(graph)
    }

    /// Simple goal decomposition (placeholder)
    fn decompose_goal(&self, goal: &str) -> Result<Vec<String>, String> {
        // TODO: Use LLM or rule-based decomposition
        // For now, simple heuristic decomposition
        
        let sub_tasks = vec![
            format!("Understand: {}", goal),
            format!("Gather information about: {}", goal),
            format!("Analyze and synthesize: {}", goal),
            format!("Formulate answer to: {}", goal),
        ];

        if sub_tasks.len() > self.max_nodes {
            return Err("Too many sub-tasks generated".to_string());
        }

        Ok(sub_tasks)
    }
}

impl Default for Planner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_node_creation() {
        let node = TaskNode::new("Test task", NodeType::Leaf);
        assert_eq!(node.task, "Test task");
        assert_eq!(node.node_type, NodeType::Leaf);
        assert_eq!(node.status, TaskStatus::Pending);
    }

    #[test]
    fn test_task_graph_creation() {
        let graph = TaskGraph::new("Root task");
        assert_eq!(graph.nodes.len(), 1);
        assert!(graph.nodes.contains_key(&graph.root));
    }

    #[test]
    fn test_add_nodes_and_edges() {
        let mut graph = TaskGraph::new("Root");
        
        let child1 = TaskNode::new("Child 1", NodeType::SubTask);
        let child1_id = child1.id;
        graph.add_node(child1);
        graph.add_edge(graph.root, child1_id);
        
        let child2 = TaskNode::new("Child 2", NodeType::SubTask);
        let child2_id = child2.id;
        graph.add_node(child2);
        graph.add_edge(graph.root, child2_id);
        
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.children(&graph.root).len(), 2);
    }

    #[test]
    fn test_dependencies() {
        let mut graph = TaskGraph::new("Root");
        
        let mut child = TaskNode::new("Child", NodeType::Leaf);
        child.add_dependency(graph.root);
        let child_id = child.id;
        graph.add_node(child);
        
        let child_node = graph.get_node(&child_id).unwrap();
        assert!(child_node.dependencies_met(&graph));
    }

    #[test]
    fn test_topological_sort() {
        let mut graph = TaskGraph::new("Root");
        
        let child1 = TaskNode::new("Child 1", NodeType::SubTask);
        let child1_id = child1.id;
        graph.add_node(child1);
        graph.add_edge(graph.root, child1_id);
        
        let child2 = TaskNode::new("Child 2", NodeType::Leaf);
        let child2_id = child2.id;
        graph.add_node(child2);
        graph.add_edge(child1_id, child2_id);
        
        let sorted = graph.topological_sort().unwrap();
        assert_eq!(sorted.len(), 3);
        
        // Root should come before children
        let root_pos = sorted.iter().position(|&id| id == graph.root).unwrap();
        let child1_pos = sorted.iter().position(|&id| id == child1_id).unwrap();
        let child2_pos = sorted.iter().position(|&id| id == child2_id).unwrap();
        
        assert!(root_pos < child1_pos);
        assert!(child1_pos < child2_pos);
    }

    #[test]
    fn test_planner() {
        let planner = Planner::new();
        let graph = planner.plan("How does PoE work?").unwrap();
        
        assert!(graph.nodes.len() > 1);
        assert!(graph.validate().is_ok());
    }

    #[test]
    fn test_completion_percentage() {
        let mut graph = TaskGraph::new("Root");
        
        let mut child = TaskNode::new("Child", NodeType::Leaf);
        child.status = TaskStatus::Completed;
        graph.add_node(child);
        
        // 1 of 2 nodes completed = 50%
        assert_eq!(graph.completion_percentage(), 50.0);
    }
}
