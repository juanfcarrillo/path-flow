use core_flow::flow::conversation::{Message, MessageType};
use rig::{completion::Message as RigMessage, message::{AssistantContent, Text, UserContent}, OneOrMany};


pub fn rig_message_adapter(message: Message) -> RigMessage {
    if message.recipient != "ai" {
        match message.content {
            MessageType::Text(text) => RigMessage::User { content: OneOrMany::one(UserContent::Text(Text { text })) },
            MessageType::Image => RigMessage::User { content: OneOrMany::one(UserContent::Text(Text { text: "asdasd".to_string() })) },
            MessageType::Audio => RigMessage::User { content: OneOrMany::one(UserContent::Text(Text { text: "asdasd".to_string() })) },
        }
    } else {
        RigMessage::Assistant { content: OneOrMany::one(AssistantContent::Text(Text { text: "asdasd".to_string() })) }
    }
}

#[allow(dead_code)]
pub fn flow_message_adapter(message: RigMessage) -> Message {
    match message {
        RigMessage::User { content } => {
            match content.first() {
                UserContent::Text(Text { text }) => Message::new("user".to_string(), text, "ai".to_string()),
                _ => panic!("Unexpected message type"),
            }
        }
        RigMessage::Assistant { content } => {
            match content.first() {
                AssistantContent::Text(Text { text }) => Message::new("ai".to_string(), text, "user".to_string()),
                _ => panic!("Unexpected message type"),
            }
        }
    }
}