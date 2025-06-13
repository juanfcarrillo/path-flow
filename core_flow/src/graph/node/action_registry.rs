use serde::de::Error as SerdeError;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

use crate::graph::node::action::Action;

struct ActionRegistry {
    actions: HashMap<String, fn(&JsonValue) -> Box<dyn Action>>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        ActionRegistry {
            actions: HashMap::new(),
        }
    }

    pub fn register_action(
        &mut self,
        action_type: &str,
        action_constructor: fn(&JsonValue) -> Box<dyn Action>,
    ) -> &mut Self {
        self.actions
            .insert(action_type.to_string(), action_constructor);
        self
    }

    pub fn get_actions(&self) -> &HashMap<String, fn(&JsonValue) -> Box<dyn Action>> {
        &self.actions
    }
}

pub fn deserialize_actions_with_config(
    json_data: &str,
    action_registry: &HashMap<&str, fn(&JsonValue) -> Box<dyn Action>>,
) -> Result<Vec<Box<dyn Action>>, serde_json::Error> {
    let actions_data: Vec<HashMap<String, JsonValue>> = serde_json::from_str(json_data)?;
    let mut actions: Vec<Box<dyn Action>> = Vec::new();

    for action_data in actions_data {
        if let Some(action_type) = action_data.get("action_type").and_then(|v| v.as_str()) {
            if let Some(action_constructor) = action_registry.get(action_type) {
                let config = action_data.get("config").unwrap_or(&JsonValue::Null);
                actions.push(action_constructor(config));
            } else {
                return Err(serde_json::Error::custom(format!(
                    "Unknown action type: {}",
                    action_type
                )));
            }
        }
    }

    Ok(actions)
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use crate::graph::node::node_context::{NodeContext, Value};

    use super::*;

    #[test]
    fn test_register_action() {
        let mut action_registry = ActionRegistry::new();

        action_registry.register_action(
            "test_action",
            create_test_action as fn(&JsonValue) -> Box<dyn Action>,
        );

        let actions = action_registry.get_actions();

        assert_eq!(actions.len(), 1);
    }

    #[tokio::test]
    async fn test_deserialize_actions() {
        let json = r#"[
            {
                "action_type": "test_action",
                "config": {
                    "test_config": "test_value"
                }
            }
        ]"#;

        let action_registry = HashMap::from([(
            "test_action",
            create_test_action as fn(&JsonValue) -> Box<dyn Action>,
        )]);

        let actions = deserialize_actions_with_config(json, &action_registry).unwrap();

        let action = actions.get(0).unwrap();

        let mut temp_context = NodeContext::new();

        let final_context = action.execute(&mut temp_context).await.unwrap();

        assert_eq!(
            final_context.variables.get("test_var").unwrap(),
            &Value::String("test_value".to_string())
        );
        assert_eq!(actions.len(), 1);
    }

    fn create_test_action(config: &JsonValue) -> Box<dyn Action> {
        Box::new(TestAction::new(config))
    }

    struct TestAction {
        config: JsonValue,
    }

    impl TestAction {
        fn new(config: &JsonValue) -> Self {
            TestAction {
                config: config.clone(),
            }
        }
    }

    #[async_trait]
    impl Action for TestAction {
        async fn execute(
            &self,
            context: &mut NodeContext,
        ) -> Result<NodeContext, Box<dyn std::error::Error>> {
            context.variables.insert(
                "test_var".to_string(),
                Value::String(self.config["test_config"].as_str().unwrap().to_string()),
            );
            Ok(context.clone())
        }
        fn clone_box(&self) -> Box<dyn Action> {
            Box::new(TestAction {
                config: self.config.clone(),
            })
        }
    }
}
