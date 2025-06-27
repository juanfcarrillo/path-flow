use axum::{
    extract::{Json, Path, State},
};
use core_flow::flow::conversation::{Conversation, ConversationRepository, Message};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::api::{
    models::{CreateConversationRequest, CreateConversationResponse, SendMessageRequest, ConversationResponse},
    state::AppState,
};

pub async fn create_conversation(
    State(state): State<Arc<Mutex<AppState>>>,
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

pub async fn send_message(
    State(state): State<Arc<Mutex<AppState>>>,
    Path(conversation_id): Path<String>,
    Json(payload): Json<SendMessageRequest>,
) -> Json<ConversationResponse> {
    let mut state = state.lock().await;

    let message = Message::new(payload.sender, payload.content, payload.recipient);

    let result = state.flow_manager.trigger_conversation(conversation_id, message.clone()).await;
    
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
