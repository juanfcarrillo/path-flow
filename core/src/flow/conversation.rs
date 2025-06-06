use std::fmt::{Error};
use std::result::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    Text,
    Image,
    Audio,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    id: String,
    content: MessageType,
    sender: String,
    recipient: String,
    timestamp: String,
}

pub struct Conversation {
    id: String,
    history: Vec<Message>,
    current_node_id: String,
    timeout: i16,
}

impl Conversation {
    fn new(id: String) -> Self {
        Conversation {
            id,
            history: Vec::new(),
            current_node_id: String::new(),
            timeout: 0,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.history.push(message);
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

pub trait ConversationRepository {
    fn get_conversation(&self, conversation_id: String) -> Result<Conversation, Error>;
    fn save_conversation(&mut self, conversation: Conversation) -> Result<(), Error>;
    fn update_conversation(&mut self, conversation_id: String, conversation: Conversation) -> Result<(), Error>;
}