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
