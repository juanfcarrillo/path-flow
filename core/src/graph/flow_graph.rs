use std::collections::{HashMap};

use super::{edge::Edge, node::{Node, NodeContext}};

#[derive(Debug)]
pub struct FlowGraph {
    nodes: HashMap<String, Node>,
    edges: HashMap<String, Edge>,
    // Adjacency list for quick traversal
    adjacency_list: HashMap<String, Vec<String>>, // node_id -> vec of edge_ids
}

impl FlowGraph {
    pub fn new() -> Self {
        FlowGraph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> Result<(), FlowError> {
        let node_id: String = node.id.clone();

        if self.nodes.contains_key(&node_id) {
            return Err(FlowError::DuplicateNode(node_id));
        }

        self.nodes.insert(node_id.clone(), node);
        self.adjacency_list.insert(node_id, Vec::new());

        Ok(())
    }

    pub fn add_edge(&mut self, edge: Edge) -> Result<(), FlowError> {
        // Validate that source and target nodes exist
        if !self.nodes.contains_key(&edge.source_node_id) {
            return Err(FlowError::NodeNotFound(edge.source_node_id.clone()));
        }

        if !self.nodes.contains_key(&edge.target_node_id) {
            return Err(FlowError::NodeNotFound(edge.target_node_id.clone()));
        }

        let edge_id: String = edge.id.clone();
        if self.edges.contains_key(&edge_id) {
            return Err(FlowError::DuplicateEdge(edge_id));
        }

        let source_node_id = edge.source_node_id.as_str();

        // Add edge to adjacency list
        self.adjacency_list
            .get_mut(source_node_id)
            .unwrap()
            .push(edge_id.clone());

        self.edges.insert(edge_id, edge);

        Ok(())
    }

    /// Find next valid node based on current context
    pub fn find_next_node(&self, current_node_id: &str, context: &NodeContext) -> Option<String> {
        let edge_ids = self.adjacency_list.get(current_node_id)?;
        
        // Get all valid edges sorted by priority
        let mut valid_edges: Vec<_> = edge_ids
            .iter()
            .filter_map(|edge_id| self.edges.get(edge_id))
            .filter(|edge| edge.evaluate(context))
            .collect();

        // Sort by priority (highest first)
        valid_edges.sort_by_key(|edge| -edge.priority);

        // Return the target node of the highest priority valid edge
        valid_edges.first().map(|edge| edge.target_node_id.clone())
    }
}

#[derive(Debug)]
pub enum FlowError {
    NodeNotFound(String),
    DuplicateNode(String),
    DuplicateEdge(String),
    CycleDetected(Vec<String>),
    NoStartNodes,
    UnreachableNodes(Vec<String>),
}
