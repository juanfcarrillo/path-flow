use std::result::Result;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    Text(String),
    Image,
    Audio,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    id: String,
    pub content: MessageType,
    pub sender: String,
    pub recipient: String,
    pub timestamp: String,
}

impl Message {
    pub fn new(sender: String, content: String, recipient: String) -> Self {
        Message {
            id: Uuid::new_v4().to_string(),
            content: MessageType::Text(content),
            sender,
            recipient,
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    pub fn new_with_id(id: String, sender: String, content: MessageType, recipient: String, timestamp: String) -> Self {
        Message {
            id,
            content,
            sender,
            recipient,
            timestamp,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conversation {
    pub id: String,
    history: Vec<Message>,
    current_node_id: String,
    timeout: i16,
}

impl Conversation {
    pub fn new(id: String, current_node_id: String) -> Self {
        Conversation {
            id,
            history: Vec::new(),
            current_node_id: current_node_id,
            timeout: 0,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.history.push(message);
    }

    pub fn add_messages(&mut self, messages: Vec<Message>) {
        self.history.extend(messages);
    }

    pub fn get_messages(&self) -> Vec<Message> {
        self.history.clone()
    }

    pub fn set_current_node_id(&mut self, node_id: String) {
        self.current_node_id = node_id;
    }

    pub fn get_current_node_id(&self) -> String {
        self.current_node_id.clone()
    }
}

#[async_trait]
pub trait ConversationRepository : Send + Sync {
    async fn get_conversation(&self, conversation_id: String) -> Result<Conversation, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_conversation_by_recipient(&self, recipient: String) -> Result<Conversation, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_conversation_by_sender(&self, sender: String) -> Result<Conversation, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_last_conversation_by_recipient(&self, recipient: String) -> Result<Conversation, Box<dyn std::error::Error + Send + Sync>>;
    async fn save_conversation(&mut self, conversation: Conversation) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn update_conversation(&mut self, conversation_id: String, conversation: Conversation) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}