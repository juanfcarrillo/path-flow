use super::{edge::Edge};
use crate::graph::{condition::condition::Condition, node::node_context::NodeContext};

pub struct EdgeBuilder {
    id: String,
    source_node_id: String,
    target_node_id: String,
    conditions: Vec<Box<dyn Condition<NodeContext>>>,
}

impl EdgeBuilder {
    pub fn new(id: String, source_node_id: String, target_node_id: String) -> Self {
        EdgeBuilder {
            id,
            source_node_id,
            target_node_id,
            conditions: Vec::new(),
        }
    }

    pub fn with_condition(mut self, condition: impl Condition<NodeContext>) -> Self {
        self.conditions.push(condition.clone_box());
        self
    }

    pub fn build(self) -> Edge {
        let mut edge = Edge::new(
            self.id,
            self.source_node_id,
            self.target_node_id,
        );

        for condition in self.conditions {
            edge.add_condition(condition);
        }

        edge
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use super::*;

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

    #[test]
    fn test_builder_creates_edge_with_basic_properties() {
        let edge = EdgeBuilder::new(
            "test_id".to_string(),
            "source_id".to_string(),
            "target_id".to_string()
        ).build();

        assert_eq!(edge.id, "test_id");
        assert_eq!(edge.source_node_id, "source_id");
        assert_eq!(edge.target_node_id, "target_id");
    }

    #[tokio::test]
    async fn test_builder_adds_condition() {
        let edge = EdgeBuilder::new(
            "test_id".to_string(),
            "source_id".to_string(),
            "target_id".to_string()
        )
        .with_condition(TestCondition::new(true))
        .build();

        assert!(edge.evaluate(&NodeContext::new()).await);
    }

    #[tokio::test]
    async fn test_builder_adds_multiple_conditions() {
        let edge = EdgeBuilder::new(
            "test_id".to_string(),
            "source_id".to_string(),
            "target_id".to_string()
        )
        .with_condition(TestCondition::new(true))
        .with_condition(TestCondition::new(true))
        .build();

        assert!(edge.evaluate(&NodeContext::new()).await);

        let edge_with_false = EdgeBuilder::new(
            "test_id".to_string(),
            "source_id".to_string(),
            "target_id".to_string()
        )
        .with_condition(TestCondition::new(true))
        .with_condition(TestCondition::new(false))
        .build();

        assert!(!edge_with_false.evaluate(&NodeContext::new()).await);
    }
}