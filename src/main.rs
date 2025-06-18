
use axum::{
    extract::{Json, Path}, routing::post, Router
};
use core_flow::{
    flow::{
        conversation::{Conversation, ConversationRepository, Message},
        flow_manager::{FlowManager},
    },
    graph::{
        action::action_registry::ActionRegistry, edge::condition_registry::ConditionRegistry,
        flow_graph::flow_graph::FlowGraph, node::node_context::Value,
    },
};
use implementations::{ai_action::ai_action::AIAction, send_message::send_message::SendMessage};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

// Request/Response structs
#[derive(Deserialize)]
struct CreateConversationRequest {
    conversation_id: String,
    initial_node: String,
}

#[derive(Deserialize)]
struct SendMessageRequest {
    content: String,
    sender: String,
    recipient: String,
}

#[derive(Serialize)]
struct ConversationResponse {
    context: HashMap<String, Value>,
}

#[derive(Serialize)]
struct CreateConversationResponse {
    conversation_id: String,
}

struct AppState {
    flow_manager: FlowManager,
    memory_conversation_repository: MemoryConversationRepository,
}

struct MemoryConversationRepository {
    conversations: HashMap<String, Conversation>,
}

impl MemoryConversationRepository {
    fn new() -> Self {
        MemoryConversationRepository {
            conversations: HashMap::new(),
        }
    }
}

impl ConversationRepository for MemoryConversationRepository {
    fn get_conversation(
        &self,
        conversation_id: String,
    ) -> Result<Conversation, Box<dyn std::error::Error>> {
        match self.conversations.get(&conversation_id) {
            Some(conversation) => Ok(conversation.clone()),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Conversation not found",
            ))),
        }
    }

    fn update_conversation(
        &mut self,
        conversation_id: String,
        conversation: Conversation,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.conversations.insert(conversation_id, conversation);
        Ok(())
    }

    fn save_conversation(
        &mut self,
        conversation: Conversation,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.conversations
            .insert(conversation.id.to_string(), conversation);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conversation_repository = MemoryConversationRepository::new();
    let mut action_registry = ActionRegistry::new();
    let condition_registry = ConditionRegistry::new();
    action_registry.register_action("ai_action", AIAction::create_ai_action);
    action_registry.register_action("send_message", SendMessage::create_send_message);

    conversation_repository.save_conversation(Conversation::new(
        "12123".to_string(),
        "first_node".to_string(),
    ))?;

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
                                "name": "AI Action",
                                "model": "gpt-4o-mini",
                                "system_prompt": "Dont answer the question, just reply mheee"
                            },
                            "input_vars": {},
                            "output_vars": ["messages"]
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
                                "name": "AI Action",
                                "model": "gpt-4o-mini",
                                "system_prompt": "Dont answer the question, just reply mheee"
                            },
                            "input_vars": {},
                            "output_vars": ["messages"]
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
                                "name": "AI Action",
                                "model": "gpt-4o-mini",
                                "system_prompt": "Dont answer the question, just reply mheee"
                            },
                            "input_vars": {},
                            "output_vars": ["messages"]
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
                            "condition_type": "positive_condition"
                        }
                    ]
                },
                {
                    "id": "second_node_to_third_node",
                    "source_node_id": "second_node",
                    "target_node_id": "third_node",
                    "conditions": [
                        {
                            "condition_type": "negative_condition"
                        }
                    ]
                }
            ]
        }"#;

    let flow_graph = FlowGraph::from_json(json_graph, &action_registry, &condition_registry)?;
    let flow_manager = FlowManager::new(Box::new(conversation_repository), flow_graph);
    let shared_state = Arc::new(Mutex::new(AppState { flow_manager, memory_conversation_repository: MemoryConversationRepository::new() }));

    let app = Router::new()
        .route("/conversations", post(create_conversation))
        .route("/conversations/{id}/messages", post(send_message))
        .with_state(shared_state);

    println!("Server starting on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn create_conversation(
    state: axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CreateConversationRequest>,
) -> Json<CreateConversationResponse> {
    let mut state = state.lock().await;
    let conversation = Conversation::new(payload.conversation_id.clone(), payload.initial_node);
    
    match state.memory_conversation_repository.save_conversation(conversation.clone()) {
        Ok(_) => Json(CreateConversationResponse {
            conversation_id: conversation.id,
        }),
        Err(_) => Json(CreateConversationResponse {
            conversation_id: "".to_string(),
        }),
    } 
}

async fn send_message(
    state: axum::extract::State<Arc<Mutex<AppState>>>,
    Path(conversation_id): Path<String>,
    Json(payload): Json<SendMessageRequest>,
) -> Json<ConversationResponse> {
    let mut state = state.lock().await;

    let message = Message::new(payload.sender, payload.content, payload.recipient);

    let result = state.flow_manager.trigger_conversation(conversation_id, message.clone()).await;
    println!("state: {:?}", result);

    // Trigger conversation through flow manager
    match result {
        Ok(context) => Json(ConversationResponse {
            context: context.variables,
        }),
        Err(_) => Json(ConversationResponse {
            context: HashMap::new(),
        }),
    }
}
