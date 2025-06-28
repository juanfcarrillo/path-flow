use async_trait::async_trait;
use core_flow::{graph::{
    action::{action::Action, utils::vars_parser::parse_input_vars},
    node::node_context::{NodeContext, Value},
}};
use reqwest::{header, Client};
use serde_json::{Value as JsonValue, json};
use std::time::Duration;

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
        let endpoint = config["post_endpoint"].as_str().unwrap().to_string();
        
        // Validate endpoint URL
        if endpoint.is_empty() {
            panic!("post_endpoint cannot be empty");
        }
        
        // Create HTTP client with timeout
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");
            
        Box::new(SendMessage {
            post_endpoint: endpoint,
            input_vars: input_vars.clone(),
            client,
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
                println!("Sending message to endpoint: {:?}", message);
                // Post message to endpoint using http client
                let response = self
                    .client
                    .post(&endpoint)
                    .json(&json!({
                        "message": message,
                    }))
                    .header("Content-Type", "application/json")
                    .header("Accept", "*/*")
                    .header(header::USER_AGENT, "core-flow/1.0")
                    .send()
                    .await;

                match response {
                    Ok(response) => {
                        let status = response.status();
                        
                        if status.is_success() {
                            // match response.text().await {
                            //     Ok(text) => println!("Response text: {}", text),
                            //     Err(e) => println!("Warning: Failed to read response body: {}", e),
                            // }
                        } else {
                            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());

                            return Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("HTTP request failed with status {}: {}", status, error_text),
                            )));
                        }
                    }
                    Err(e) => {
                        println!("Error in endpoint response: {:?}", e);
                        
                        // Provide more specific error messages based on error type
                        let error_message = if e.is_connect() {
                            format!("Connection failed to {}: {}", endpoint, e)
                        } else if e.is_timeout() {
                            format!("Request timeout to {}: {}", endpoint, e)
                        } else if e.is_request() {
                            format!("Request error to {}: {}", endpoint, e)
                        } else {
                            format!("Network error to {}: {}", endpoint, e)
                        };
                        
                        println!("Error in endpoint request: {}", error_message);
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            error_message,
                        )));
                    }
                }
            }
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Messages not found in input variables",
            )));
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

        assert!(result.is_err());
    }
}
