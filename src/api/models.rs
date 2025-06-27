use core_flow::graph::node::node_context::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Request structs
#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub conversation_id: String,
    pub initial_node: String,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
    pub sender: String,
    pub recipient: String,
}

// Response structs
#[derive(Serialize)]
pub struct ConversationResponse {
    pub context: HashMap<String, Value>,
}

#[derive(Serialize)]
pub struct CreateConversationResponse {
    pub conversation_id: String,
}
