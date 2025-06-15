use std::collections::HashMap;

use crate::graph::action::{registrable_action::RegistrableAction};

pub struct ActionRegistry {
    actions: HashMap<String, RegistrableAction>,
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
        action_constructor: RegistrableAction,
    ) -> &mut Self {
        self.actions
            .insert(action_type.to_string(), action_constructor);
        self
    }

    pub fn get_actions(&self) -> &HashMap<String, RegistrableAction> {
        &self.actions
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::{action::{tests::action_implementation::TestAction}};

    use super::*;

    #[test]
    fn test_register_action() {
        let mut action_registry = ActionRegistry::new();

        action_registry.register_action(
            "test_action",
            TestAction::create_registrable_action(),
        );

        let actions = action_registry.get_actions();

        assert_eq!(actions.len(), 1);
    }
}
