use std::collections::{HashMap};
use std::fmt;

use crate::graph::flow_graph::flow_graph_builder::FlowGraphBuilder;
use crate::graph::{edge::edge::Edge, node::{node::Node, node_context::NodeContext}};

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

    pub fn builder() -> FlowGraphBuilder {
        FlowGraphBuilder::new()
    }

    pub fn get_node_mut(&mut self, node_id: &str) -> Result<&mut Node, FlowError> {
        if !self.nodes.contains_key(node_id) {
            return Err(FlowError::NodeNotFound(node_id.to_string()));
        }

        Ok(self.nodes.get_mut(node_id).unwrap())
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
        let valid_edges: Vec<_> = edge_ids
            .iter()
            .filter_map(|edge_id| self.edges.get(edge_id))
            .filter(|edge| edge.evaluate(context))
            .collect();

        // Sort by priority (highest first)

        let edge = valid_edges.get(0)?;

        Some(edge.target_node_id.clone())
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

impl fmt::Display for FlowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FlowError::NodeNotFound(node_id) => write!(f, "Node not found: {}", node_id),
            FlowError::DuplicateNode(node_id) => write!(f, "Duplicate node: {}", node_id),
            FlowError::DuplicateEdge(edge_id) => write!(f, "Duplicate edge: {}", edge_id),
            FlowError::CycleDetected(cycle) => write!(f, "Cycle detected: {:?}", cycle),
            FlowError::NoStartNodes => write!(f, "No start nodes"),
            FlowError::UnreachableNodes(unreachable_nodes) => write!(f, "Unreachable nodes: {:?}", unreachable_nodes),
        }
    }
}

impl std::error::Error for FlowError {}

#[cfg(test)]

mod tests {
    use super::*;

    mod given_no_edges {
        use super::*;
        #[test]
        fn should_return_none() {
            let mut graph = FlowGraph::new();

            let node1 = Node::new(
                "node1".to_string(),
                "message".to_string(),
                "Node 1".to_string(),
                "Node 1 description".to_string(),
            );

            let node2 = Node::new(
                "node2".to_string(),
                "message".to_string(),
                "Node 2".to_string(),
                "Node 2 description".to_string(),
            );

            graph.add_node(node1).unwrap();
            graph.add_node(node2).unwrap();

            let context = NodeContext::new();
            assert_eq!(graph.find_next_node("node1", &context), None);
        }
    }

    #[test]
        fn should_determine_next_node_with_priority() {
            let mut graph = FlowGraph::new();

            let node1 = Node::new(
                "node1".to_string(),
                "message".to_string(),
                "Node 1".to_string(),
                "Node 1 description".to_string(),
            );

            let node2 = Node::new(
                "node2".to_string(),
                "message".to_string(),
                "Node 2".to_string(),
                "Node 2 description".to_string(),
            );

            let node3 = Node::new(
                "node3".to_string(),
                "message".to_string(),
                "Node 3".to_string(),
                "Node 3 description".to_string(),
            );

            graph.add_node(node1).unwrap();
            graph.add_node(node2).unwrap();
            graph.add_node(node3).unwrap();

            let edge1 = Edge::new(
                "edge1".to_string(),
                "node1".to_string(),
                "node2".to_string(),
            );
            
            let edge2 = Edge::new(
                "edge3".to_string(),
                "node1".to_string(),
                "node3".to_string(),
            );

            graph.add_edge(edge1).unwrap();
            graph.add_edge(edge2).unwrap();

            // Test with valid context
            let context = NodeContext::new();
            assert_eq!(graph.find_next_node("node1", &context), Some("node2".to_string()));
        }


    mod given_some_conditions {
        use crate::graph::edge::{self, condition::Condition, tests::condition_implementation::{NegativeCondition, PositiveCondition}};

        use super::*;

        #[test]
        fn should_determine_next_node_with_positive_condition() {
            let mut graph = FlowGraph::new();

            let node1 = Node::new(
                "node1".to_string(),
                "message".to_string(),
                "Node 1".to_string(),
                "Node 1 description".to_string(),
            );

            let node2 = Node::new(
                "node2".to_string(),
                "message".to_string(),
                "Node 2".to_string(),
                "Node 2 description".to_string(),
            );

            graph.add_node(node1).unwrap();
            graph.add_node(node2).unwrap();

            let mut edge1 = Edge::new(
                "edge1".to_string(),
                "node1".to_string(),
                "node2".to_string(),
            );

            edge1.add_condition(PositiveCondition.clone_box());
            graph.add_edge(edge1).unwrap();

            // Test with valid context
            let context = NodeContext::new();
            assert_eq!(graph.find_next_node("node1", &context), Some("node2".to_string()));
        }
    
        #[test]
        fn should_determine_first_valid_node() {
            let mut graph = FlowGraph::new();

            let node1 = Node::new(
                "node1".to_string(),
                "message".to_string(),
                "Node 1".to_string(),
                "Node 1 description".to_string(),
            );

            let node2 = Node::new(
                "node2".to_string(),
                "message".to_string(),
                "Node 2".to_string(),
                "Node 2 description".to_string(),
            );

            let node3 = Node::new(
                "node3".to_string(),
                "message".to_string(),
                "Node 3".to_string(),
                "Node 3 description".to_string(),
            );

            graph.add_node(node1).unwrap();
            graph.add_node(node2).unwrap();
            graph.add_node(node3).unwrap();

            let mut edge1 = Edge::new(
                "edge1".to_string(),
                "node1".to_string(),
                "node2".to_string(),
            );
            edge1.add_condition(NegativeCondition.clone_box());

            let mut edge2 = Edge::new(
                "edge3".to_string(),
                "node1".to_string(),
                "node3".to_string(),
            );
            edge2.add_condition(PositiveCondition.clone_box());

            graph.add_edge(edge1).unwrap();
            graph.add_edge(edge2).unwrap();

            // Test with valid context
            let context = NodeContext::new();
            assert_eq!(graph.find_next_node("node1", &context), Some("node3".to_string()));
        }
    }

}