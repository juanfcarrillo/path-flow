use core_flow::{
    flow::{
        conversation::{Conversation, ConversationRepository, Message},
        flow_manager::FlowManager,
    },
    graph::{edge::{condition_registry::ConditionRegistry}, flow_graph::flow_graph::FlowGraph, node::{action::Action, action_registry::ActionRegistry}},
};
use implementations::ai_action::ai_action::AIAction;
use std::collections::HashMap;

struct MemoryConversationRepository {
    conversations: HashMap<String, Conversation>,
}

impl MemoryConversationRepository {
    fn new() -> Self {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conversation_repository = MemoryConversationRepository::new();

    conversation_repository.save_conversation(Conversation::new(
        "conversation_1".to_string(),
        "first_node".to_string(),
    ))?;

    let mut action_registry = ActionRegistry::new();
    action_registry.register_action(
        "ai_action",
        AIAction::create_ai_action as fn(&serde_json::Value) -> Box<dyn Action>,
    );
    let condition_registry= ConditionRegistry::new();

    let json_graph = r#"
        {
            "nodes": [
                {
                    "id": "first_node",
                    "node_type": "conversational",
                    "name": "First Node",
                    "description": "First Node Description",
                    "node_context": {
                        "variables": {}
                    },
                    "actions": [
                        {
                            "action_type": "ai_action",
                            "config": {
                                "model": "gpt-4o-mini",
                                "system_prompt": "You are a helpful assistant"
                            }
                        }
                    ]
                },
                {
                    "id": "second_node",
                    "node_type": "conversational",
                    "name": "Second Node",
                    "description": "Second Node Description",
                    "node_context": {
                        "variables": {}
                    },
                    "actions": [
                        {
                            "action_type": "ai_action",
                            "config": {
                                "model": "gpt-4o-mini",
                                "system_prompt": "You are a helpful assistant"
                            }
                        }
                    ]
                },
                {
                    "id": "third_node",
                    "node_type": "conversational",
                    "name": "Third Node",
                    "description": "Third Node Description",
                    "node_context": {
                        "variables": {}
                    },
                    "actions": [
                        {
                            "action_type": "ai_action",
                            "config": {
                                "model": "gpt-4o-mini",
                                "system_prompt": "You are a helpful assistant"
                            }
                        }
                    ]
                }
            ],
            "edges": [
                {
                    "id": "first_node_to_second_node",
                    "source_node_id": "first_node",
                    "target_node_id": "second_node",
                    "conditions": [
                        {
                            "condition_type": "positive_condition"
                        }
                    ]
                },
                {
                    "id": "second_node_to_third_node",
                    "source_node_id": "second_node",
                    "target_node_id": "third_node",
                    "conditions": [
                        {
                            "condition_type": "negative_condition"
                        }
                    ]
                }
            ]
        }"#;

    let flow_graph = FlowGraph::from_json(json_graph, &action_registry, &condition_registry).unwrap();

    let mut flow_manager = FlowManager::new(Box::new(conversation_repository), flow_graph);

    flow_manager
        .trigger_conversation(
            "conversation_1".to_string(),
            Message::new(
                "juan".to_string(),
                "Whats the capital of france ?".to_string(),
                "user".to_string(),
            ),
        )
        .await?;

    let context2 = flow_manager
        .trigger_conversation(
            "conversation_1".to_string(),
            Message::new(
                "juan".to_string(),
                "Ok so what was the first question ?".to_string(),
                "user".to_string(),
            ),
        )
        .await?;

    println!("Context: {:?}", context2);

    Ok(())
}
