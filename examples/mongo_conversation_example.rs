use implementations::conversation_repository::MongoConversationRepository;
use core_flow::flow::conversation::{Conversation, ConversationRepository, Message};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize MongoDB connection
    let uri = "mongodb://localhost:27017"; // Replace with your MongoDB URI
    let database_name = "path_flow_db";
    
    let mut mongo_repo = MongoConversationRepository::new_with_uri(uri, database_name).await?;
    
    // Create a new conversation
    let conversation = Conversation::new("example_conv_id".to_string(), "initial_node".to_string());
    
    // Save the conversation
    mongo_repo.save_conversation(conversation.clone()).await?;
    println!("Conversation saved to MongoDB");
    
    // Retrieve the conversation
    let retrieved_conversation = mongo_repo.get_conversation("example_conv_id".to_string()).await?;
    println!("Retrieved conversation: {:?}", retrieved_conversation);
    
    // Add a message to the conversation
    let mut updated_conversation = retrieved_conversation;
    let message = Message::new(
        "user".to_string(),
        "Hello, world!".to_string(),
        "assistant".to_string(),
    );
    updated_conversation.add_message(message);
    
    // Update the conversation
    mongo_repo.update_conversation("example_conv_id".to_string(), updated_conversation).await?;
    println!("Conversation updated with new message");
    
    // Retrieve conversations by sender/recipient
    let conversation_by_recipient = mongo_repo.get_conversation_by_recipient("assistant".to_string()).await?;
    println!("Conversation by recipient: {:?}", conversation_by_recipient.id);
    
    let conversation_by_sender = mongo_repo.get_conversation_by_sender("user".to_string()).await?;
    println!("Conversation by sender: {:?}", conversation_by_sender.id);
    
    println!("MongoDB ConversationRepository example completed successfully!");
    
    Ok(())
}
