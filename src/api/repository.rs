use core_flow::flow::conversation::{Conversation, ConversationRepository};
use std::collections::HashMap;
use chrono::{Utc, DateTime};
use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq)]
pub struct MemoryConversationRepository {
    pub conversations: HashMap<String, Conversation>,
}

impl MemoryConversationRepository {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MemoryConversationRepository {
            conversations: HashMap::new(),
        }
    }
}

#[async_trait]
impl ConversationRepository for MemoryConversationRepository {
    async fn get_conversation(
        &self,
        conversation_id: String,
    ) -> Result<Conversation, Box<dyn std::error::Error + Send + Sync>> {
        match self.conversations.get(&conversation_id) {
            Some(conversation) => Ok(conversation.clone()),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Conversation not found",
            ))),
        }
    }

    async fn update_conversation(
        &mut self,
        conversation_id: String,
        conversation: Conversation,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.conversations.insert(conversation_id, conversation);
        Ok(())
    }

    async fn save_conversation(
        &mut self,
        conversation: Conversation,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.conversations
            .insert(conversation.id.to_string(), conversation);
        Ok(())
    }

    async fn get_conversation_by_recipient(&self, recipient: String) -> Result<Conversation, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.conversations.values().find(|conversation| {
            conversation.get_messages().iter().any(|msg| msg.recipient == recipient)
        });
        
        match result {
            Some(conversation) => Ok(conversation.clone()),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Conversation not found",
            ))),
        }
    }

    async fn get_conversation_by_sender(&self, sender: String) -> Result<Conversation, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.conversations.values().find(|conversation| {
            conversation.get_messages().iter().any(|msg| msg.sender == sender)
        });
        
        match result {
            Some(conversation) => Ok(conversation.clone()),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Conversation not found",
            ))),
        }
    }

    async fn get_last_conversation_by_recipient(&self, recipient: String) -> Result<Conversation, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.conversations.values().max_by_key(|conversation| {
            let conv_messages = conversation.get_messages();
            let messages = conv_messages.iter().filter(|msg| msg.recipient == recipient);
            let last_message = messages.last();

            if let Some(last_message) = last_message {
                let timestamp: DateTime<Utc> = last_message.timestamp.parse().unwrap();

                return timestamp.timestamp()
            }

            0
        });
        
        match result {
            Some(conversation) => Ok(conversation.clone()),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Conversation not found",
            ))),
        }
    }


}
