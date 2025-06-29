mod api;
use mongodb::{options::ClientOptions, Client};

use axum::{routing::post, Router};
use core_flow::{
    flow::{
        flow_manager::FlowManager,
    },
    graph::{
        action::action_registry::ActionRegistry, 
        condition::condition_registry::ConditionRegistry, 
        flow_graph::flow_graph::FlowGraph,
    },
};
use implementations::{ai_action::ai_action::AIAction, conversation_repository::MongoConversationRepository, send_message::send_message::SendMessage};
use std::sync::Arc;
use tokio::sync::Mutex;

use api::{handlers, AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let conversation_repository = MongoConversationRepository::new(
        client.clone(),
        "path_flow_db"
    ).await?;

    let mut action_registry = ActionRegistry::new();
    let condition_registry = ConditionRegistry::new();
    action_registry.register_action("ai_action", AIAction::create_ai_action);
    action_registry.register_action("send_message", SendMessage::create_send_message);

    let json_graph = r#"
        {
            "nodes": [
                {
                    "id": "first_node",
                    "node_type": "conversational",
                    "name": "First Node",
                    "description": "First Node Description",
                    "node_context": {
                        "variables": {}
                    },
                    "actions": [
                        {
                            "name": "ai_action",
                            "action_type": "ai_action",
                            "config": {
                                "id": "ai_action",
                                "name": "ai_action",
                                "model": "gemini-2.0-flash",
                                "system_prompt": "Dont answer the question, just reply mheee"
                            },
                            "input_vars": {},
                            "output_vars": ["messages"]
                        },
                        {
                            "name": "send_message",
                            "action_type": "send_message",
                            "config": {
                                "id": "send_message",
                                "name": "Send Message",
                                "post_endpoint": "http://localhost:3000/webhook/send"
                            },
                            "input_vars": {
                                "messages": "ai_action.messages"
                            },
                            "output_vars": []
                        }
                    ]
                },
                {
                    "id": "second_node",
                    "node_type": "conversational",
                    "name": "Second Node",
                    "description": "Second Node Description",
                    "node_context": {
                        "variables": {}
                    },
                    "actions": [
                        {
                            "name": "ai_action",
                            "action_type": "ai_action",
                            "config": {
                                "id": "ai_action",
                                "name": "ai_action",
                                "model": "gemini-2.0-flash",
                                "system_prompt": "Dont answer the question, just reply mheee"
                            },
                            "input_vars": {},
                            "output_vars": ["messages"]
                        },
                        {
                            "name": "send_message",
                            "action_type": "send_message",
                            "config": {
                                "id": "send_message",
                                "name": "Send Message",
                                "post_endpoint": "http://localhost:3000/webhook/send"
                            },
                            "input_vars": {
                                "messages": "ai_action.messages"
                            },
                            "output_vars": []
                        }
                    ]
                },
                {
                    "id": "third_node",
                    "node_type": "conversational",
                    "name": "Third Node",
                    "description": "Third Node Description",
                    "node_context": {
                        "variables": {}
                    },
                    "actions": [
                        {
                            "name": "ai_action",
                            "action_type": "ai_action",
                            "config": {
                                "id": "ai_action",
                                "name": "ai_action",
                                "model": "gemini-2.0-flash",
                                "system_prompt": "Dont answer the question, just reply mheee"
                            },
                            "input_vars": {},
                            "output_vars": ["messages"]
                        },
                        {
                            "name": "send_message",
                            "action_type": "send_message",
                            "config": {
                                "id": "send_message",
                                "name": "Send Message",
                                "post_endpoint": "http://localhost:3000/webhook/send"
                            },
                            "input_vars": {
                                "messages": "ai_action.messages"
                            },
                            "output_vars": []
                        }
                    ]
                }
            ],
            "edges": [
                {
                    "id": "first_node_to_second_node",
                    "source_node_id": "first_node",
                    "target_node_id": "second_node",
                    "conditions": [
                        {
                            "condition_type": "positive_condition",
                            "input_vars": {}
                        }
                    ]
                },
                {
                    "id": "second_node_to_third_node",
                    "source_node_id": "second_node",
                    "target_node_id": "third_node",
                    "conditions": [
                        {
                            "condition_type": "negative_condition",
                            "input_vars": {}
                        }
                    ]
                },
                {
                    "id": "third_node_to_first_node",
                    "source_node_id": "third_node",
                    "target_node_id": "first_node",
                    "conditions": [
                        {
                            "condition_type": "positive_condition",
                            "input_vars": {}
                        }
                    ]
                }
            ]
        }"#;

    let flow_graph = FlowGraph::from_json(json_graph, &action_registry, &condition_registry)
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) })?;
    let flow_manager = FlowManager::new(Box::new(conversation_repository), flow_graph);
    let shared_state = Arc::new(Mutex::new(AppState { flow_manager, mongo_conversation_repository: MongoConversationRepository::new(client, "path_flow_db").await? }));

    let app = Router::new()
        .route("/conversations", post(handlers::create_conversation))
        .route("/conversations/{id}/messages", post(handlers::send_message))
        .route("/conversations/trigger", post(handlers::trigger_conversation))
        .with_state(shared_state);

    println!("Server starting on http://localhost:8000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}