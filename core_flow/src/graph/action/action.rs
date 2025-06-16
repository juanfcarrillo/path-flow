use std::{fmt::Debug};

use async_trait::async_trait;

use crate::graph::{node::node_context::NodeContext};


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

#[cfg(test)]
mod tests {
    use crate::graph::{action::{action_registry::ActionRegistry, tests::action_implementation::create_test_action, utils::action_deserializer::deserialize_actions}, node::node_context::Value};

    use super::*;

    #[tokio::test]
    async fn test_deserialize_actions_with_config() {
        let json = r#"[
            {
                "action_type": "test_action",
                "config": {
                    "name": "test_action",
                    "id": "test_action",
                    "test_config": "test_value"
                },
                "input_vars": {},
                "output_vars": {}
            }
        ]"#;

        let mut action_registry = ActionRegistry::new();
        action_registry.register_action(
            "test_action",
            create_test_action
        );

        let actions = deserialize_actions(json, &action_registry).unwrap();

        let action = actions.get(0).unwrap();

        let mut temp_context = NodeContext::new();

        let final_context = action.execute(&mut temp_context).await.unwrap();

        assert_eq!(
            final_context.variables.get("test_var").unwrap(),
            &Value::String("test_value".to_string())
        );
        assert_eq!(actions.len(), 1);
    }
}