use serde_json::Value as JsonValue;
use std::collections::HashMap;

use crate::graph::action::action::Action;

pub struct ActionRegistry {
    actions: HashMap<String, fn(&JsonValue, &JsonValue, &JsonValue) -> Box<dyn Action>>,
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
        action_constructor: fn(&JsonValue, &JsonValue, &JsonValue) -> Box<dyn Action>,
    ) -> &mut Self {
        self.actions
            .insert(action_type.to_string(), action_constructor);
        self
    }

    pub fn get_actions(&self) -> &HashMap<String, fn(&JsonValue, &JsonValue, &JsonValue) -> Box<dyn Action>> {
        &self.actions
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::{action::tests::action_implementation::create_test_action};

    use super::*;

    #[test]
    fn test_register_action() {
        let mut action_registry = ActionRegistry::new();

        action_registry.register_action(
            "test_action",
            create_test_action,
        );

        let actions = action_registry.get_actions();

        assert_eq!(actions.len(), 1);
    }
}
