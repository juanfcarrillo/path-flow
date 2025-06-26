use super::flow_graph::{FlowGraph, FlowError};
use crate::graph::{
    node::node::Node,
    edge::edge::Edge,
};

pub struct FlowGraphBuilder {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl FlowGraphBuilder {
    pub fn new() -> Self {
        FlowGraphBuilder {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn with_node(mut self, node: Node) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn with_edge(mut self, edge: Edge) -> Self {
        self.edges.push(edge);
        self
    }

    pub fn build(self) -> Result<FlowGraph, FlowError> {
        let mut flow_graph = FlowGraph::new();

        // First add all nodes
        for node in self.nodes {
            flow_graph.add_node(node)?;
        }

        // Then add all edges
        for edge in self.edges {
            flow_graph.add_edge(edge)?;
        }

        Ok(flow_graph)
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use super::*;
    use crate::graph::{
        condition::condition::Condition, node::node_context::NodeContext
    };

    // Test condition implementation
    struct TestCondition {
        result: bool,
    }

    impl TestCondition {
        fn new(result: bool) -> Self {
            TestCondition { result }
        }
    }

    #[async_trait]
    impl Condition<NodeContext> for TestCondition {
        async fn evaluate(&self, _context: &NodeContext) -> bool {
            self.result
        }

        fn clone_box(&self) -> Box<dyn Condition<NodeContext>> {
            Box::new(TestCondition { result: self.result })
        }
    }

    #[tokio::test]
    async fn test_build_empty_graph() {
        let graph = FlowGraphBuilder::new().build().unwrap();
        assert!(graph.find_next_node("non_existent", &NodeContext::new()).await.is_none());
    }

    #[tokio::test]
    async fn test_build_graph_with_nodes() {
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

        let graph = FlowGraphBuilder::new()
            .with_node(node1)
            .with_node(node2)
            .build()
            .unwrap();

        assert!(graph.find_next_node("node1", &NodeContext::new()).await.is_none());
    }

    #[tokio::test]
    async fn test_build_graph_with_nodes_and_edges() {
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

        let mut edge = Edge::new(
            "edge1".to_string(),
            "node1".to_string(),
            "node2".to_string(),
        );
        edge.add_condition(TestCondition::new(true).clone_box());

        let graph = FlowGraphBuilder::new()
            .with_node(node1)
            .with_node(node2)
            .with_edge(edge)
            .build()
            .unwrap();

        assert_eq!(
            graph.find_next_node("node1", &NodeContext::new()).await,
            Some("node2".to_string())
        );
    }

    #[test]
    fn test_build_fails_with_missing_node_for_edge() {
        let node1 = Node::new(
            "node1".to_string(),
            "message".to_string(),
            "Node 1".to_string(),
            "Node 1 description".to_string(),
        );

        let edge = Edge::new(
            "edge1".to_string(),
            "node1".to_string(),
            "non_existent".to_string(),
        );

        let result = FlowGraphBuilder::new()
            .with_node(node1)
            .with_edge(edge)
            .build();

        assert!(matches!(result, Err(FlowError::NodeNotFound(_))));
    }

    #[test]
    fn test_build_fails_with_duplicate_nodes() {
        let node1 = Node::new(
            "node1".to_string(),
            "message".to_string(),
            "Node 1".to_string(),
            "Node 1 description".to_string(),
        );

        let node1_duplicate = Node::new(
            "node1".to_string(),
            "message".to_string(),
            "Node 1 Duplicate".to_string(),
            "Node 1 duplicate description".to_string(),
        );

        let result = FlowGraphBuilder::new()
            .with_node(node1)
            .with_node(node1_duplicate)
            .build();

        assert!(matches!(result, Err(FlowError::DuplicateNode(_))));
    }

    #[test]
    fn test_build_fails_with_duplicate_edges() {
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

        let edge1 = Edge::new(
            "edge1".to_string(),
            "node1".to_string(),
            "node2".to_string(),
        );

        let edge1_duplicate = Edge::new(
            "edge1".to_string(),
            "node1".to_string(),
            "node2".to_string(),
        );

        let result = FlowGraphBuilder::new()
            .with_node(node1)
            .with_node(node2)
            .with_edge(edge1)
            .with_edge(edge1_duplicate)
            .build();

        assert!(matches!(result, Err(FlowError::DuplicateEdge(_))));
    }
}