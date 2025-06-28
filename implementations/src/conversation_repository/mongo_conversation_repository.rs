use async_trait::async_trait;
use bson::{doc};
use core_flow::flow::conversation::{Conversation, ConversationRepository, Message, MessageType};
use mongodb::{Client, Collection, Database};
use serde::{Deserialize, Serialize};
use std::result::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConversationDocument {
    #[serde(rename = "_id")]
    pub id: String,
    pub history: Vec<MessageDocument>,
    pub current_node_id: String,
    pub timeout: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MessageDocument {
    pub id: String,
    pub content: MessageTypeDocument,
    pub sender: String,
    pub recipient: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum MessageTypeDocument {
    Text(String),
    Image,
    Audio,
}

impl From<Conversation> for ConversationDocument {
    fn from(conversation: Conversation) -> Self {
        let id = conversation.id.clone();
        let current_node_id = conversation.get_current_node_id();
        ConversationDocument {
            id,
            history: conversation.get_messages().into_iter().map(|msg| msg.into()).collect(),
            current_node_id,
            timeout: 0, // Default timeout since it's not accessible from Conversation
        }
    }
}

impl From<ConversationDocument> for Conversation {
    fn from(doc: ConversationDocument) -> Self {
        let mut conversation = Conversation::new(doc.id, doc.current_node_id);
        let messages: Vec<Message> = doc.history.into_iter().map(|msg| msg.into()).collect();
        conversation.add_messages(messages);
        conversation
    }
}

impl From<Message> for MessageDocument {
    fn from(message: Message) -> Self {
        MessageDocument {
            id: message.get_id(),
            content: message.content.into(),
            sender: message.sender,
            recipient: message.recipient,
            timestamp: message.timestamp,
        }
    }
}

impl From<MessageDocument> for Message {
    fn from(doc: MessageDocument) -> Self {
        Message::new_with_id(
            doc.id,
            doc.sender,
            doc.content.into(),
            doc.recipient,
            doc.timestamp,
        )
    }
}

impl From<MessageType> for MessageTypeDocument {
    fn from(msg_type: MessageType) -> Self {
        match msg_type {
            MessageType::Text(text) => MessageTypeDocument::Text(text),
            MessageType::Image => MessageTypeDocument::Image,
            MessageType::Audio => MessageTypeDocument::Audio,
        }
    }
}

impl From<MessageTypeDocument> for MessageType {
    fn from(doc: MessageTypeDocument) -> Self {
        match doc {
            MessageTypeDocument::Text(text) => MessageType::Text(text),
            MessageTypeDocument::Image => MessageType::Image,
            MessageTypeDocument::Audio => MessageType::Audio,
        }
    }
}

pub struct MongoConversationRepository {
    collection: Collection<ConversationDocument>,
}

impl MongoConversationRepository {
    pub async fn new(client: Client, database_name: &str) -> Result<Self, mongodb::error::Error> {
        let database: Database = client.database(database_name);
        let collection: Collection<ConversationDocument> = database.collection("conversations");
        
        Ok(MongoConversationRepository { collection })
    }

    pub async fn new_with_uri(
        uri: &str,
        database_name: &str,
    ) -> Result<Self, mongodb::error::Error> {
        let client = Client::with_uri_str(uri).await?;
        Self::new(client, database_name).await
    }
}

#[async_trait]
impl ConversationRepository for MongoConversationRepository {
    async fn get_conversation(
        &self,
        conversation_id: String,
    ) -> Result<Conversation, Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { "_id": &conversation_id };
        
        match self.collection.find_one(filter, None).await? {
            Some(doc) => Ok(doc.into()),
            None => Err(format!("Conversation with id {} not found", conversation_id).into()),
        }
    }

    async fn get_conversation_by_recipient(
        &self,
        recipient: String,
    ) -> Result<Conversation, Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { "history.recipient": &recipient };
        
        match self.collection.find_one(filter, None).await? {
            Some(doc) => Ok(doc.into()),
            None => Err(format!("Conversation with recipient {} not found", recipient).into()),
        }
    }

    async fn get_conversation_by_sender(
        &self,
        sender: String,
    ) -> Result<Conversation, Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { "history.sender": &sender };
        
        match self.collection.find_one(filter, None).await? {
            Some(doc) => Ok(doc.into()),
            None => Err(format!("Conversation with sender {} not found", sender).into()),
        }
    }

    async fn get_last_conversation_by_recipient(
        &self,
        recipient: String,
    ) -> Result<Conversation, Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { "history.recipient": &recipient };
        let options = mongodb::options::FindOneOptions::builder()
            .sort(doc! { "history.timestamp": -1 })
            .build();
        
        match self.collection.find_one(filter, options).await? {
            Some(doc) => Ok(doc.into()),
            None => Err(format!("No conversation found for recipient {}", recipient).into()),
        }
    }

    async fn save_conversation(
        &mut self,
        conversation: Conversation,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let doc: ConversationDocument = conversation.into();
        self.collection.insert_one(doc, None).await?;
        Ok(())
    }

    async fn update_conversation(
        &mut self,
        conversation_id: String,
        conversation: Conversation,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let filter = doc! { "_id": &conversation_id };
        let doc: ConversationDocument = conversation.into();
        
        // Convert the struct to a BSON document for the update
        let update_doc = bson::to_document(&doc)?;
        let update = doc! { "$set": update_doc };
        
        let result = self.collection.update_one(filter, update, None).await?;
        
        if result.matched_count == 0 {
            return Err(format!("Conversation with id {} not found", conversation_id).into());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::options::ClientOptions;
    use tokio;

    #[tokio::test]
    async fn test_mongo_conversation_repository() {
        // This is a basic test - in practice you'd use a test database
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        
        let mut repo = MongoConversationRepository::new(client, "test_db").await.unwrap();
        
        let conversation = Conversation::new("test_id".to_string(), "node_1".to_string());
        
        // Test save
        let result = repo.save_conversation(conversation.clone()).await;
        assert!(result.is_ok());
        
        // Test get
        let retrieved = repo.get_conversation("test_id".to_string()).await;
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap().id, "test_id");
    }
}
