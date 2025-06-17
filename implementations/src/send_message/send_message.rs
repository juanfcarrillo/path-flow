use std::os::macos::raw::stat;

use async_trait::async_trait;
use core_flow::graph::{
    action::{action::Action, utils::vars_parser::parse_input_vars},
    node::node_context::{NodeContext, Value},
};
use reqwest::Client;
use serde_json::Value as JsonValue;

#[derive(Clone)]
struct SendMessage {
    post_endpoint: String,
    input_vars: JsonValue,
    client: Client,
}

impl SendMessage {
    pub fn create_send_message(
        config: &JsonValue,
        input_vars: &JsonValue,
        _: &JsonValue,
    ) -> Box<dyn Action> {
        Box::new(SendMessage {
            post_endpoint: config["post_endpoint"].as_str().unwrap().to_string(),
            input_vars: input_vars.clone(),
            client: Client::new(),
        })
    }
}

#[async_trait]
impl Action for SendMessage {
    async fn execute(
        &self,
        context: &mut NodeContext,
    ) -> Result<NodeContext, Box<dyn std::error::Error>> {
        let mut input_vars = parse_input_vars(&self.input_vars, &context)?;
        let messages = input_vars.get_mut("messages");

        if let Some(Value::Messages(messages)) = messages {
            let endpoint = self.post_endpoint.clone();

            for message in messages {
                // Post message to endpoint using http client
                let response = self.client.post(&endpoint).json(&message).send().await;

                if let Ok(response) = response {
                    let status = response.status();
                    let text = response.text().await.unwrap();

                    println!("Status: {:?}", status);
                    println!("Text: {:?}", text);
                } else {
                    println!("Error: {:?}", response);
                }
            }
        }

        Ok(context.clone())
    }

    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}
