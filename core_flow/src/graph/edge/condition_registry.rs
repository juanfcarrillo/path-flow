// core_flow/src/graph/edge/condition_registry.rs

use std::collections::HashMap;
use crate::graph::edge::condition::Condition;
use crate::graph::node::node_context::NodeContext;
use serde_json::Value as JsonValue;

pub struct ConditionRegistry {
    conditions: HashMap<String, fn(&JsonValue) -> Box<dyn Condition<NodeContext>>>,
}

impl ConditionRegistry {
    pub fn new() -> Self {
        ConditionRegistry {
            conditions: HashMap::new(),
        }
    }

    pub fn register_condition(
        &mut self,
        condition_type: &str,
        condition_constructor: fn(&JsonValue) -> Box<dyn Condition<NodeContext>>,
    ) -> &mut Self {
        self.conditions
            .insert(condition_type.to_string(), condition_constructor);
        self
    }

    pub fn get_conditions(&self) -> &HashMap<String, fn(&JsonValue) -> Box<dyn Condition<NodeContext>>> {
        &self.conditions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edge::tests::condition_implementation::{
        PositiveCondition, NegativeCondition,
    };

    fn create_positive_condition(_config: &JsonValue) -> Box<dyn Condition<NodeContext>> {
        Box::new(PositiveCondition {})
    }

    fn create_negative_condition(_config: &JsonValue) -> Box<dyn Condition<NodeContext>> {
        Box::new(NegativeCondition {})
    }

    #[test]
    fn test_register_condition() {
        let mut condition_registry = ConditionRegistry::new();

        condition_registry.register_condition(
            "positive_condition",
            create_positive_condition as fn(&JsonValue) -> Box<dyn Condition<NodeContext>>,
        );

        condition_registry.register_condition(
            "negative_condition",
            create_negative_condition as fn(&JsonValue) -> Box<dyn Condition<NodeContext>>,
        );

        let conditions = condition_registry.get_conditions();

        assert_eq!(conditions.len(), 2);
    }
}