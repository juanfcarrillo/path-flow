use async_trait::async_trait;
use serde_json::Value as JsonValue;

use crate::graph::{action::{action::Action, registrable_action::{RegistrableActionMold}}, node::node_context::{NodeContext, Value}};

#[derive(Clone)]
pub struct TestAction {
    #[allow(dead_code)]
    config: JsonValue,
}

impl TestAction {
    pub fn new(json_config: &JsonValue) -> Self {
        TestAction {
            config: json_config.clone(),
        }
    }

    pub fn create_registrable_action() -> RegistrableActionMold {
        RegistrableActionMold::new(
            "test_action".to_string(),
            create_test_action_config as fn(&JsonValue, &JsonValue, &JsonValue) -> Box<dyn Action>,
        )
    }
}

#[async_trait]
impl Action for TestAction {
    async fn execute(&self, context: &mut NodeContext) -> Result<NodeContext, Box<dyn std::error::Error>> {
        match context.variables.get("test_config") {
            Some(Value::String(test_var)) => {
                context.variables.insert("test_var".to_string(), Value::String(test_var.clone()));
            }
            _ => {
                context.variables.insert("test_var".to_string(), Value::String("test_value".to_string()));
            }
        }
        Ok(context.clone())
    }
    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}

// Lets register the action
fn create_test_action_config(json_config: &JsonValue, input_vars: &JsonValue, output_vars: &JsonValue) -> Box<dyn Action> {
    Box::new(TestAction::new(json_config))
}