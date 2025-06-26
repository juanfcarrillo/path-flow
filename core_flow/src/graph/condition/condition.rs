use std::{collections::HashMap, fmt};

use serde_json::Value as JsonValue;

use async_trait::async_trait;

use crate::graph::{action::utils::action_deserializer::deserialize_input_vars, condition::condition_registry::ConditionRegistry, node::node_context::NodeContext};
use serde::de::Error as SerdeError;

// Trait for condition evaluation
#[async_trait]
pub trait Condition<Context>: Send + Sync {
    async fn evaluate(&self, context: &Context) -> bool;
    fn clone_box(&self) -> Box<dyn Condition<Context>>;
}

impl fmt::Debug for Box<dyn Condition<NodeContext>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Condition")
    }
}

impl Clone for Box<dyn Condition<NodeContext>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub fn deserialize_conditions_with_config(
    json_data: &str,
    condition_registry: &ConditionRegistry,
) -> Result<Vec<Box<dyn Condition<NodeContext>>>, serde_json::Error> {
    let conditions_data: Vec<HashMap<String, JsonValue>> = serde_json::from_str(json_data)?;
    let mut conditions: Vec<Box<dyn Condition<NodeContext>>> = Vec::new();

    for condition_data in conditions_data {
        if let Some(condition_type) = condition_data.get("condition_type").and_then(|v| v.as_str()) {
            if let Some(condition_constructor) = condition_registry.get_conditions().get(condition_type) {
                let config = condition_data.get("config");
                let input_vars = deserialize_input_vars(condition_data.get("input_vars").cloned())?; 
                if config.is_some() {
                    conditions.push(condition_constructor(config.unwrap(), &input_vars));
                } else {
                    conditions.push(condition_constructor(&JsonValue::Null, &input_vars));
                }
            } else {
                return Err(SerdeError::custom(format!(
                    "Unknown condition type: {}",
                    condition_type
                )));
            }
        }
    }
    Ok(conditions)
}

#[cfg(test)]
mod tests {
    use crate::graph::condition::tests::condition_implementation::{ConfigurableCondition, NegativeCondition, PositiveCondition};

    use super::*;

    fn create_positive_condition(_: &JsonValue, _: &JsonValue) -> Box<dyn Condition<NodeContext>> {
        Box::new(PositiveCondition {})
    }

    fn create_negative_condition(_: &JsonValue, _: &JsonValue) -> Box<dyn Condition<NodeContext>> {
        Box::new(NegativeCondition {})
    }

    fn create_configurable_condition(config: &JsonValue, _: &JsonValue) -> Box<dyn Condition<NodeContext>> {
        Box::new(ConfigurableCondition::new(config))
    }

    
    #[test]
    fn test_deserialize_conditions() {
        let json = r#"[
            {
                "condition_type": "positive_condition"
            },
            {
                "condition_type": "negative_condition"
            }
        ]"#;

        let mut condition_registry = ConditionRegistry::new();
        condition_registry.register_condition(
            "positive_condition",
            create_positive_condition as fn(&JsonValue, &JsonValue) -> Box<dyn Condition<NodeContext>>,
        );
        condition_registry.register_condition(
            "negative_condition",
            create_negative_condition as fn(&JsonValue, &JsonValue) -> Box<dyn Condition<NodeContext>>,
        );

        let conditions = deserialize_conditions_with_config(json, &condition_registry).unwrap();

        assert_eq!(conditions.len(), 2);
    }

    #[tokio::test]
    async fn test_deserialize_conditions_with_config() {
        let json = r#"[
            {
                "condition_type": "configurable_condition",
                "config": {
                    "key": "value"
                }
            }
        ]"#;

        let mut condition_registry = ConditionRegistry::new();
        condition_registry.register_condition(
            "configurable_condition",
            create_configurable_condition as fn(&JsonValue, &JsonValue) -> Box<dyn Condition<NodeContext>>,
        );

        let conditions = deserialize_conditions_with_config(json, &condition_registry).unwrap();

        assert_eq!(conditions.len(), 1);

        let configurable_condition = conditions.get(0).unwrap();

        assert!(configurable_condition.evaluate(&NodeContext::new()).await);
    }
}