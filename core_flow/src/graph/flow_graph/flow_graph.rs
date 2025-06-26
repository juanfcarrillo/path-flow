use std::collections::HashMap;
use std::fmt;

use serde_json::Value;

use crate::graph::action::action_registry::ActionRegistry;
use crate::graph::condition::condition_registry::ConditionRegistry;
use crate::graph::flow_graph::flow_graph_builder::FlowGraphBuilder;
use crate::graph::{
    edge::edge::Edge,
    node::{node::Node, node_context::NodeContext},
};

#[derive(Debug)]
pub struct  FlowGraph {
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
    // Json Structure
    // {
    //     "nodes": [
    //         {
    //             "id": "node_id",
    //             "node_type": "conversational",
    //             "name": "Node Name",
    //             "description": "Node Description",
    //             "node_context": {
    //                 "variables": {}
    //             },
    //             "actions": [
    //                 {
    //                     "action_type": "test_action"
    //                 }
    //             ]
    //         }
    //     ],
    //     "edges": [
    //         {
    //             "id": "edge_id",
    //             "source_node_id": "node_id",
    //             "target_node_id": "node_id",
    //             "conditions": [
    //                 {
    //                     "condition_type": "positive_condition"
    //                 }
    //             ]
    //         }
    //     ]
    // }

    pub fn from_json(
        json: &str,
        action_registry: &ActionRegistry,
        condition_registry: &ConditionRegistry,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let json_map: HashMap<String, serde_json::Value> = serde_json::from_str(json)?;

        let mut graph = FlowGraph::new();

        if let Some(Value::Array(nodes)) = json_map.get("nodes") {
            let nodes = nodes
                .iter()
                .map(|node| Node::from_json(node.to_string().as_str(), action_registry))
                .collect::<Result<Vec<Node>, serde_json::Error>>()?;

            for node in nodes {
                graph.add_node(node)?;
            }
        }

        if let Some(Value::Array(edges)) = json_map.get("edges") {
            let edges = edges
                .iter()
                .map(|edge| Edge::from_json(edge.to_string().as_str(), condition_registry))
                .collect::<Result<Vec<Edge>, serde_json::Error>>()?;

            for edge in edges {
                graph.add_edge(edge)?;
            }
        }

        Ok(graph)
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
    pub async fn find_next_node(
        &self,
        current_node_id: &str,
        context: &NodeContext,
    ) -> Option<String> {
        let edge_ids = self.adjacency_list.get(current_node_id)?;

        // Get all valid edges and evaluate them asynchronously
        let mut valid_edges = Vec::new();
        for edge_id in edge_ids {
            if let Some(edge) = self.edges.get(edge_id) {
                if edge.evaluate(context).await {
                    valid_edges.push(edge);
                }
            }
        }

        // Sort by priority (highest first)
        // Assuming Edge has a priority field, you might want to add:
        // valid_edges.sort_by(|a, b| b.priority.cmp(&a.priority));

        valid_edges.get(0).map(|edge| edge.target_node_id.clone())
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
            FlowError::UnreachableNodes(unreachable_nodes) => {
                write!(f, "Unreachable nodes: {:?}", unreachable_nodes)
            }
        }
    }
}

impl std::error::Error for FlowError {}

#[cfg(test)]

mod tests {
    use super::*;

    mod given_no_edges {
        use super::*;
        #[tokio::test]
        async fn should_return_none() {
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
            assert_eq!(graph.find_next_node("node1", &context).await, None);
        }
    }

    #[tokio::test]
    async fn should_determine_next_node_with_priority() {
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
        assert_eq!(
            graph.find_next_node("node1", &context).await,
            Some("node2".to_string())
        );
    }

    mod given_some_conditions {

        use crate::graph::condition::{condition::Condition, tests::condition_implementation::{NegativeCondition, PositiveCondition}};

        use super::*;

        #[tokio::test]
        async fn should_determine_next_node_with_positive_condition() {
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
            assert_eq!(
                graph.find_next_node("node1", &context).await,
                Some("node2".to_string())
            );
        }

        #[tokio::test]
        async fn should_determine_first_valid_node() {
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
            assert_eq!(
                graph.find_next_node("node1", &context).await,
                Some("node3".to_string())
            );
        }
    }

    mod given_json {
        use crate::graph::{action::tests::action_implementation::create_test_action, condition::{condition::Condition, tests::condition_implementation::PositiveCondition}};

        use super::*;

        #[test]
        fn test_from_json() {
            let json = r#"{
                "nodes": [
                    {
                        "id": "node1",
                        "node_type": "conversational",
                        "name": "Node 1",
                        "description": "Node 1 description",
                        "node_context": {
                            "variables": {}
                        },
                        "actions": [
                            {
                                "config": {
                                    "name": "test_action",
                                    "id": "test_action"
                                },
                                "input_vars": {},
                                "output_vars": {},
                                "action_type": "test_action"
                            }
                        ]
                    },
                    {
                        "id": "node2",
                        "node_type": "conversational",
                        "name": "Node 2",
                        "description": "Node 2 description",
                        "node_context": {
                            "variables": {}
                        },
                        "actions": [
                            {
                                "config": {
                                    "name": "test_action",
                                    "id": "test_action"
                                },
                                "input_vars": {},
                                "output_vars": [],
                                "action_type": "test_action"
                            }
                        ]
                    }
                ],
                "edges": [
                    {
                        "id": "edge1",
                        "source_node_id": "node1",
                        "target_node_id": "node2",
                        "conditions": [
                            {
                                "condition_type": "positive_condition"
                            }
                        ]
                    }
                ]
            }"#;

            let mut action_registry = ActionRegistry::new();
            action_registry.register_action(
                "test_action",
                create_test_action
            );

            let mut condition_registry = ConditionRegistry::new();
            condition_registry.register_condition(
                "positive_condition",
                PositiveCondition::create_positive_condition,
            );

            let graph = FlowGraph::from_json(json, &action_registry, &condition_registry).unwrap();

            assert_eq!(graph.nodes.len(), 2);
            assert_eq!(graph.edges.len(), 1);
        }
    }
}
