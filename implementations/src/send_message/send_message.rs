use async_trait::async_trait;
use core_flow::graph::{
    action::{action::Action, utils::vars_parser::parse_input_vars},
    node::node_context::{NodeContext, Value},
};
use reqwest::Client;
use serde_json::{Value as JsonValue, json};

#[derive(Clone)]
pub struct SendMessage {
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
                let response = self
                    .client
                    .post(&endpoint)
                    .json(&json!({
                        "message": message
                    }))
                    .send()
                    .await;

                if let Ok(response) = response {
                    let status = response.status();
                    let text = response.text().await.unwrap();
                    println!("Status: {:?}", status);
                    println!("Text: {:?}", text);
                } else {
                    println!("Error: {:?}", response);
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to send message",
                    )));
                }
            }
        }

        Ok(context.clone())
    }

    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_flow::flow::conversation::Message;
    use mockito::Server;
    use serde_json::json;

    #[tokio::test]
    async fn test_send_message() {
        let mut server = Server::new_async().await;

        let mock = server
            .mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status": "ok"}"#)
            .create();

        // Create test configuration
        let config = json!({
            "name": "send_message",
            "id": "send_message",
            "post_endpoint": server.url()
        });

        // Create test input variables
        let input_vars = json!({
            "messages": "messages"
        });

        // Create SendMessage action
        let action = SendMessage::create_send_message(&config, &input_vars, &json!({}));

        // Create test context
        let mut context = NodeContext::new();
        context.variables.insert(
            "messages".to_string(),
            Value::Messages(vec![Message::new(
                "ai".to_string(),
                "Hello, world!".to_string(),
                "user".to_string(),
            )]),
        );

        // Execute action
        let result = action.execute(&mut context).await;

        // Verify
        assert!(result.is_ok());
        mock.assert();
    }

    #[tokio::test]
    async fn test_send_message_error() {
        // Create test configuration with invalid endpoint
        let config = json!({
            "post_endpoint": "http://invalid-endpoint"
        });

        // Create test input variables
        let input_vars = json!({
            "messages": "messages"
        });

        // Create SendMessage action
        let action = SendMessage::create_send_message(&config, &input_vars, &json!({}));

        // Create test context
        let mut context = NodeContext::new();
        context.variables.insert(
            "messages".to_string(),
            Value::Messages(vec![Message::new(
                "ai".to_string(),
                "Hello, world!".to_string(),
                "user".to_string(),
            )]),
        );

        // Execute action
        let result = action.execute(&mut context).await;

        println!("Result: {:?}", result);

        assert!(result.is_err());
    }
}
