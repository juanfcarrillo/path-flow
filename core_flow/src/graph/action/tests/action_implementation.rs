use crate::graph::node::node_context::{NodeContext, Value};
use serde_json::Value as JsonValue;

#[derive(Clone)]
pub struct TestAction {
    #[allow(dead_code)]
    config: JsonValue,
}

impl TestAction {
    pub fn new(config: &JsonValue) -> Self {
        TestAction {
            config: config.clone(),
        }
    }
}

#[async_trait::async_trait]
impl crate::graph::action::action::Action for TestAction {
    async fn execute(
        &self,
        context: &mut NodeContext,
    ) -> Result<NodeContext, Box<dyn std::error::Error>> {
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
    fn clone_box(&self) -> Box<dyn crate::graph::action::action::Action> {
        Box::new(self.clone())
    }
}

pub fn create_test_action(config: &JsonValue, _: &JsonValue, _: &JsonValue) -> Box<dyn crate::graph::action::action::Action> {
    Box::new(TestAction::new(config))
}
