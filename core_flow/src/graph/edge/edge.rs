use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::graph::{edge::{condition::{deserialize_conditions_with_config}, condition_registry::ConditionRegistry}, node::node_context::NodeContext};

use super::{condition::Condition, edge_builder::EdgeBuilder};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub source_node_id: String,
    pub target_node_id: String,
    #[serde(skip)]
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

    pub fn from_json(
        json: &str,
        condition_registry: &ConditionRegistry,
    ) -> Result<Self, serde_json::Error> {
        let mut edge: Edge = serde_json::from_str(json)?;

        let json_map: HashMap<String, serde_json::Value> = serde_json::from_str(json)?;

        if let Some(conditions_value) = json_map.get("conditions") {
            if let Ok(conditions) = deserialize_conditions_with_config(conditions_value.to_string().as_str(), condition_registry) {
                edge.conditions = conditions;
            }
        }
        Ok(edge)
    }

    pub fn builder(id: String, source_node_id: String, target_node_id: String) -> EdgeBuilder {
        EdgeBuilder::new(id, source_node_id, target_node_id)
    }

    pub fn add_condition(&mut self, condition: Box<dyn Condition<NodeContext>>) {
        self.conditions.push(condition.clone_box());
    }

    pub async fn evaluate(&self, context: &NodeContext) -> bool {
        for condition in &self.conditions {
            if !condition.evaluate(context).await {
                return false;
            }
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod given_some_conditions {
        use crate::graph::edge::tests::condition_implementation::{NegativeCondition, PositiveCondition};

        use super::*;

        #[tokio::test]
        async fn test_positive_condition() {
            let condition = PositiveCondition;
            let mut edge = Edge::new(
                "welcome_to_help".to_string(),
                "welcome".to_string(),
                "help".to_string(),
            );

            edge.add_condition(condition.clone_box());

            assert_eq!(edge.conditions.len(), 1);
            assert_eq!(edge.conditions[0].evaluate(&NodeContext::new()).await, true); 
        }

        #[tokio::test]
        async fn test_negative_condition() {
            let condition = NegativeCondition;
            let mut edge = Edge::new(
                "welcome_to_help".to_string(),
                "welcome".to_string(),
                "help".to_string(),
            );

            edge.add_condition(condition.clone_box());

            assert_eq!(edge.conditions.len(), 1);
            assert_eq!(edge.conditions[0].evaluate(&NodeContext::new()).await, false); 
        }
    }

    mod given_json {

        use crate::graph::edge::tests::condition_implementation::{NegativeCondition, PositiveCondition};

        use super::*;

        fn create_positive_condition(_: &serde_json::Value) -> Box<dyn Condition<NodeContext>> {
            Box::new(PositiveCondition {})
        }

        fn create_negative_condition(_: &serde_json::Value) -> Box<dyn Condition<NodeContext>> {
            Box::new(NegativeCondition {})
        }

        #[test]
        fn test_from_json() {
            let json = r#"{
                "id": "welcome_to_help",
                "source_node_id": "welcome",
                "target_node_id": "help",
                "conditions": [
                    {
                        "condition_type": "positive_condition"
                    },
                    {
                        "condition_type": "negative_condition"
                    }
                ]
            }"#;

            let mut condition_registry = ConditionRegistry::new();
            condition_registry.register_condition(
                "positive_condition",
                create_positive_condition as fn(&serde_json::Value) -> Box<dyn Condition<NodeContext>>,
            );
            condition_registry.register_condition(
                "negative_condition",
                create_negative_condition as fn(&serde_json::Value) -> Box<dyn Condition<NodeContext>>,
            );

            let edge = Edge::from_json(json, &condition_registry).unwrap();

            assert_eq!(edge.id, "welcome_to_help");
            assert_eq!(edge.source_node_id, "welcome");
            assert_eq!(edge.target_node_id, "help");
            assert_eq!(edge.conditions.len(), 2);
        }
    }
}