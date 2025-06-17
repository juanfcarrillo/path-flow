use async_trait::async_trait;
use core_flow::{
    flow::conversation::Message,
    graph::{
        action::{action::Action, utils::vars_parser::OutputVarsBuilder},
        node::node_context::{NodeContext, Value},
    },
};
use rig::{
    client::{CompletionClient, ProviderClient},
    completion::Chat,
    providers::openai,
};
use serde_json::Value as JsonValue;

use crate::ai_action::message_adapter::rig_message_adapter;

#[derive(Clone)]
pub struct AIAction {
    // Add any configuration fields needed for the AI action
    model: String,
    system_prompt: String,
    output_vars: JsonValue,
    config: JsonValue,
}

impl AIAction {
    pub fn new(
        model: String,
        system_prompt: String,
        output_vars: JsonValue,
        config: JsonValue,
    ) -> Self {
        AIAction {
            model,
            system_prompt,
            output_vars,
            config,
        }
    }

    async fn process_messages(
        &self,
        messages: Vec<Message>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let openai_client = openai::Client::from_env();

        let gpt4 = openai_client.agent(&self.model).build();

        let messages = messages
            .iter()
            .map(|m| rig_message_adapter(m.clone()))
            .collect::<Vec<rig::completion::Message>>();

        let response = gpt4.chat(&self.system_prompt, messages).await?;

        Ok(response)
    }

    pub fn create_ai_action(
        config: &JsonValue,
        _: &JsonValue,
        output_vars: &JsonValue,
    ) -> Box<dyn Action> {
        Box::new(AIAction::new(
            config["model"].as_str().unwrap().to_string(),
            config["system_prompt"].as_str().unwrap().to_string(),
            output_vars.clone(),
            config.clone(),
        ))
    }
}

#[async_trait]
impl Action for AIAction {
    async fn execute(
        &self,
        context: &mut NodeContext,
    ) -> Result<NodeContext, Box<dyn std::error::Error>> {
        let mut messages_vec = match context.variables.remove("messages") {
            Some(Value::Messages(msgs)) => msgs,
            _ => Vec::new(),
        };

        let ai_response = self.process_messages(messages_vec.clone()).await?;

        let new_ai_message = Message::new("ai".to_string(), ai_response, "user".to_string());

        messages_vec.push(new_ai_message);

        let mut output_builder = OutputVarsBuilder::new(&self.config, &self.output_vars, context.clone());

        output_builder.add_var("messages".to_string(), Value::Messages(messages_vec.clone()));

        let mut output_context = output_builder.build()?;
        
        output_context.variables.insert("messages".to_string(), Value::Messages(messages_vec));

        Ok(output_context)
    }

    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}
#[cfg(test)]
mod tests {
    use core_flow::graph::node::node_context::Value;
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn test_ai_action_execution() {
        let mut context = NodeContext::new();
        let messages = vec![Message::new(
            "user".to_string(),
            "Hello, how are you?".to_string(),
            "ai".to_string(),
        )];

        context
            .variables
            .insert("messages".to_string(), Value::Messages(messages));

        let ai_action = AIAction::new(
            "gpt-3.5-turbo".to_string(),
            "You are a helpful assistant".to_string(),
            json!(["messages"]),
            json!({
                "id": "ai_action",
                "name": "AI Action",
            }),
        );

        let result = ai_action.execute(&mut context).await;
        assert!(result.is_ok());
    }
}
