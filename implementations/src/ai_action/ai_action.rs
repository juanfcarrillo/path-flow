use async_trait::async_trait;
use core_flow::{
    flow::conversation::Message,
    graph::node::{
        action::Action, node_context::{NodeContext, Value}
    },
};
use rig::{
    client::{CompletionClient, ProviderClient},
    completion::Chat,
    providers::openai,
};

use crate::ai_action::message_adapter::{rig_message_adapter};

pub struct AIAction {
    // Add any configuration fields needed for the AI action
    model: String,
    system_prompt: String,
}

impl AIAction {
    pub fn new(model: String, system_prompt: String) -> Self {
        AIAction {
            model,
            system_prompt,
        }
    }

    async fn process_messages(
        &self,
        messages: Vec<Message>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let openai_client = openai::Client::from_env();

        let gpt4 = openai_client.agent(openai::GPT_4O).build();

        let messages = messages.iter().map(|m| rig_message_adapter(m.clone())).collect::<Vec<rig::completion::Message>>();

        let response = gpt4
            .chat("You are a helpful assistant", messages)
            .await?;

        Ok(response)
    }

    pub fn create_ai_action(config: &serde_json::Value) -> Box<dyn Action> {
        Box::new(AIAction::new(
            config["model"].as_str().unwrap().to_string(),
            config["system_prompt"].as_str().unwrap().to_string(),
        ))
    }
}

#[async_trait]
impl Action for AIAction {
    async fn execute(&self, context: &mut NodeContext) -> Result<NodeContext, Box<dyn std::error::Error>> {
        // Get messages from context
        let messages = match context.variables.get("messages") {
            Some(Value::Messages(msgs)) => msgs.clone(),
            _ => Vec::new(), // Handle case where no messages exist
        };

        // Process messages through AI
        let ai_response = self.process_messages(messages).await?;

        let messages = context.variables.get_mut("messages").unwrap();

        if let Value::Messages(messages) = messages {
            messages.push(Message::new("ai".to_string(), ai_response, "user".to_string()));
        }

        Ok(context.clone())
    }

    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(AIAction {
            model: self.model.clone(),
            system_prompt: self.system_prompt.clone(),
        })

    }
}

#[cfg(test)]
mod tests {
    use core_flow::graph::node::node_context::Value;

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
        );

        let result = ai_action.execute(&mut context).await;
        assert!(result.is_ok());
    }
}
