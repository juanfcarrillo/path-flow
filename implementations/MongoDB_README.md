# MongoDB ConversationRepository Implementation

This implementation provides a MongoDB-based persistence layer for the `ConversationRepository` trait in the path-flow project.

## Features

- **Async/Await Support**: All operations are asynchronous using `async-trait`
- **Full CRUD Operations**: Create, read, update conversations
- **Message Querying**: Find conversations by sender, recipient, or get the latest conversation
- **Type Safety**: Strongly typed with proper serialization/deserialization
- **Error Handling**: Comprehensive error handling with custom error messages

## Dependencies

The implementation uses the following dependencies:
- `mongodb`: Official MongoDB driver for Rust
- `bson`: BSON serialization/deserialization
- `async-trait`: For async trait support
- `serde`: For JSON/BSON serialization

## Usage

### Basic Setup

```rust
use implementations::conversation_repository::MongoConversationRepository;
use core_flow::flow::conversation::{ConversationRepository, Conversation};

// Initialize with URI
let mut repo = MongoConversationRepository::new_with_uri(
    "mongodb://localhost:27017",
    "my_database"
).await?;

// Or initialize with existing client
let client = mongodb::Client::with_uri_str("mongodb://localhost:27017").await?;
let mut repo = MongoConversationRepository::new(client, "my_database").await?;
```

### Operations

```rust
// Save a conversation
let conversation = Conversation::new("conv_id".to_string(), "node_1".to_string());
repo.save_conversation(conversation).await?;

// Get a conversation by ID
let conversation = repo.get_conversation("conv_id".to_string()).await?;

// Update a conversation
repo.update_conversation("conv_id".to_string(), updated_conversation).await?;

// Find conversations by participant
let conversation = repo.get_conversation_by_sender("user_id".to_string()).await?;
let conversation = repo.get_conversation_by_recipient("user_id".to_string()).await?;

// Get the most recent conversation by recipient
let conversation = repo.get_last_conversation_by_recipient("user_id".to_string()).await?;
```

## Database Schema

### Collection: `conversations`

```json
{
  "_id": "conversation_id",
  "history": [
    {
      "id": "message_id",
      "content": {
        "type": "Text",
        "data": "Hello, world!"
      },
      "sender": "user_id",
      "recipient": "assistant_id", 
      "timestamp": "2025-06-27T10:30:00Z"
    }
  ],
  "current_node_id": "node_1",
  "timeout": 0
}
```

## Error Handling

All methods return `Result<T, Box<dyn std::error::Error + Send + Sync>>`. Common errors include:

- **Connection errors**: MongoDB connection issues
- **Not found errors**: When conversations don't exist
- **Serialization errors**: BSON conversion issues
- **Update errors**: When updates fail (e.g., conversation not found)

## Development

### Running Tests

```bash
# Start MongoDB (if using Docker)
docker run -d -p 27017:27017 --name mongodb mongo:latest

# Run tests
cargo test --package implementations
```

### Example

See `examples/mongo_conversation_example.rs` for a complete usage example.

## MongoDB Setup

### Local Development

```bash
# Using Docker
docker run -d -p 27017:27017 --name mongodb mongo:latest

# Using MongoDB Community Edition
# Follow installation instructions for your platform
```

### Production

For production usage, consider:
- Connection pooling (handled automatically by the MongoDB driver)
- Replica sets for high availability
- Sharding for horizontal scaling
- Proper indexing on frequently queried fields
- Authentication and SSL/TLS

### Recommended Indexes

```javascript
// Create indexes for better query performance
db.conversations.createIndex({ "history.sender": 1 })
db.conversations.createIndex({ "history.recipient": 1 })
db.conversations.createIndex({ "history.timestamp": -1 })
```
