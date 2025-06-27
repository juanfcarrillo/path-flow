use core_flow::flow::conversation::{Conversation, ConversationRepository};
use std::collections::HashMap;

pub struct MemoryConversationRepository {
    pub conversations: HashMap<String, Conversation>,
}

impl MemoryConversationRepository {
    pub fn new() -> Self {
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
