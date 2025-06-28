use axum::extract::{Json, Path, State};
use core_flow::flow::conversation::{Conversation, ConversationRepository, Message};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::api::{
    models::{
        ConversationResponse, CreateConversationRequest, CreateConversationResponse,
        SendMessageRequest, TriggerConversationRequest,
    },
    state::AppState,
};

// Helper function to handle conversation triggering and response creation
async fn execute_conversation_flow(
    state: &mut AppState,
    conversation_id: String,
    message: Message,
) -> Json<ConversationResponse> {
    let result = state
        .flow_manager
        .trigger_conversation(conversation_id, message)
        .await;

    match result {
        Ok(context) => Json(ConversationResponse {
            success: true,
            context: context.variables,
            error_message: None,
        }),
        Err(e) => Json(ConversationResponse {
            success: false,
            context: HashMap::new(),
            error_message: Some(format!("Failed to process conversation: {}", e)),
        }),
    }
}

pub async fn create_conversation(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CreateConversationRequest>,
) -> Json<CreateConversationResponse> {
    let mut state = state.lock().await;
    let conversation = Conversation::new(payload.conversation_id.clone(), payload.initial_node);

    match state
        .memory_conversation_repository
        .save_conversation(conversation.clone()).await
    {
        Ok(_) => Json(CreateConversationResponse {
            conversation_id: conversation.id,
        }),
        Err(_) => Json(CreateConversationResponse {
            conversation_id: "".to_string(),
        }),
    }
}

pub async fn send_message(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(conversation_id): Path<String>,
    Json(payload): Json<SendMessageRequest>,
) -> Json<ConversationResponse> {
    let mut state = state.lock().await;

    let message = Message::new(payload.sender, payload.content, payload.recipient);

    // Trigger conversation through flow manager
    execute_conversation_flow(&mut state, conversation_id, message.clone()).await
}

pub async fn trigger_conversation(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<TriggerConversationRequest>,
) -> Json<ConversationResponse> {
    let mut state = state.lock().await;

    let message = Message::new(
        payload.sender.clone(),
        payload.content,
        payload.recipient.clone(),
    );

    // Try to get existing conversation or create a new one
    let conversation_id = match state
        .memory_conversation_repository
        .get_conversation_by_recipient(payload.sender.clone()).await
    {
        Ok(conversation) => conversation.id,
        Err(_) => {
            // Create new conversation
            let new_conversation =
                Conversation::new(payload.recipient.clone(), "first_node".to_string());

            match state
                .memory_conversation_repository
                .save_conversation(new_conversation.clone()).await
            {
                Ok(_) => new_conversation.id,
                Err(e) => {
                    return Json(ConversationResponse {
                        success: false,
                        context: HashMap::new(),
                        error_message: Some(format!("Failed to create new conversation: {}", e)),
                    });
                }
            }
        }
    };

    execute_conversation_flow(&mut state, conversation_id, message).await
}
