use std::{collections::HashMap, fmt::Debug};

use async_trait::async_trait;
use serde::de::Error as SerdeError;

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
}