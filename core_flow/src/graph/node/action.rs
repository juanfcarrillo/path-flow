use std::{collections::HashMap, fmt::Debug};
use serde::de::Error as SerdeError;
use serde_json::Value as JsonValue;

use async_trait::async_trait;

use crate::graph::node::node_context::NodeContext;


#[async_trait]
pub trait Action {
    async fn execute(&self, context: &mut NodeContext) -> Result<NodeContext, Box<dyn std::error::Error>>;
    fn clone_box(&self) -> Box<dyn Action>;
}

impl Debug for dyn Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Action")
    }
}

impl Clone for Box<dyn Action> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub fn deserialize_actions(json_data: &str, action_registry: &HashMap<&str, fn() -> Box<dyn Action>>) -> Result<Vec<Box<dyn Action>>, serde_json::Error> {
    let actions_data: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(json_data)?;
    let mut actions: Vec<Box<dyn Action>> = Vec::new();

    for action_data in actions_data {
        if let Some(action_type) = action_data.get("action_type").and_then(|v| v.as_str()) {
            if let Some(action_constructor) = action_registry.get(action_type) {
                actions.push(action_constructor());
            } else {
                return Err(SerdeError::custom(format!("Unknown action type: {}", action_type)));
            }
        }
    }

    Ok(actions)
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
    use crate::graph::node::node_context::Value;

    use super::*;

    #[test]
    fn test_deserialize_actions() {
        let json = r#"[
            {
                "action_type": "test_action"
            }
        ]"#;

        let action_registry = HashMap::from([("test_action", create_test_action as fn() -> Box<dyn Action>)]);

        let actions = deserialize_actions(json, &action_registry).unwrap();

        assert_eq!(actions.len(), 1);
    }

    #[tokio::test]
    async fn test_deserialize_actions_with_config() {
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
            create_test_action_config as fn(&JsonValue) -> Box<dyn Action>,
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

    fn create_test_action() -> Box<dyn Action> {
        Box::new(TestAction::new())
    }

    struct TestAction;

    impl TestAction {
        fn new() -> Self {
            TestAction
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
                Value::String("test_value".to_string()),
            );
            Ok(context.clone())
        }
        fn clone_box(&self) -> Box<dyn Action> {
            Box::new(TestAction)
        }
    }


    fn create_test_action_config(config: &JsonValue) -> Box<dyn Action> {
        Box::new(TestActionConfig::new(config))
    }

    struct TestActionConfig {
        config: JsonValue,
    }

    impl TestActionConfig {
        fn new(config: &JsonValue) -> Self {
            TestActionConfig {
                config: config.clone(),
            }
        }
    }

    #[async_trait]
    impl Action for TestActionConfig {
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
            Box::new(TestActionConfig {
                config: self.config.clone(),
            })
        }
    }
}