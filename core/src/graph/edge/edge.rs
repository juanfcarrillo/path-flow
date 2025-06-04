use crate::graph::node::node_context::NodeContext;

use super::{condition::Condition, edge_builder::EdgeBuilder};

#[derive(Debug, Clone)]
pub struct Edge {
    pub id: String,
    pub source_node_id: String,
    pub target_node_id: String,
    conditions: Vec<Box<dyn Condition<NodeContext>>>,
}

impl Edge {
    pub fn new(
        id: String,
        source_node_id: String,
        target_node_id: String,
    ) -> Self {
        Edge {
            id,
            source_node_id,
            target_node_id,
            conditions: Vec::new(),
        }
    }

    pub fn builder(id: String, source_node_id: String, target_node_id: String) -> EdgeBuilder {
        EdgeBuilder::new(id, source_node_id, target_node_id)
    }

    pub fn add_condition(&mut self, condition: Box<dyn Condition<NodeContext>>) {
        self.conditions.push(condition.clone_box());
    }

    pub fn evaluate(&self, context: &NodeContext) -> bool {
        self.conditions.iter().all(|condition| condition.evaluate(context))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod given_some_conditions {
        use crate::graph::edge::tests::condition_implementation::{NegativeCondition, PositiveCondition};

        use super::*;

        #[test]
        fn test_positive_condition() {
            let condition = PositiveCondition;
            let mut edge = Edge::new(
                "welcome_to_help".to_string(),
                "welcome".to_string(),
                "help".to_string(),
            );

            edge.add_condition(condition.clone_box());

            assert_eq!(edge.conditions.len(), 1);
            assert_eq!(edge.conditions[0].evaluate(&NodeContext::new()), true); 
        }

        #[test]
        fn test_negative_condition() {
            let condition = NegativeCondition;
            let mut edge = Edge::new(
                "welcome_to_help".to_string(),
                "welcome".to_string(),
                "help".to_string(),
            );

            edge.add_condition(condition.clone_box());

            assert_eq!(edge.conditions.len(), 1);
            assert_eq!(edge.conditions[0].evaluate(&NodeContext::new()), false); 
        }
    }
}