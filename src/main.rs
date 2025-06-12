use core_flow::{
    flow::{
        conversation::{Conversation, ConversationRepository, Message},
        flow_manager::FlowManager,
    },
    graph::{
        edge::{condition::Condition, edge::Edge},
        flow_graph::flow_graph::FlowGraph,
        node::{
            node::{Action, Node},
            node_context::{NodeContext, Value},
        },
    },
};
use implementations::ai_action::ai_action::AIAction;
use std::collections::HashMap;

use async_trait::async_trait;

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

struct TestAction;

impl TestAction {
    pub fn new() -> Self {
        TestAction
    }
}

#[async_trait]
impl Action for TestAction {
    async fn execute(&self, context: &mut NodeContext) -> Result<NodeContext, Box<dyn std::error::Error>> {
        context.variables.insert(
            "test_var".to_string(),
            Value::String("test_value".to_string()),
        );
        Ok(context.clone())
    }
    fn clone_box(&self) -> Box<dyn Action> {
        Box::new(TestAction)
    }
}

struct TestCondition;

impl TestCondition {
    pub fn new() -> Self {
        TestCondition
    }
}

#[async_trait]
impl Condition<NodeContext> for TestCondition {
    async fn evaluate(&self, context: &NodeContext) -> bool {
        context.variables.get("test_var").is_some()
    }

    fn clone_box(&self) -> Box<dyn Condition<NodeContext>> {
        Box::new(TestCondition)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conversation_repository = MemoryConversationRepository::new();

    conversation_repository.save_conversation(Conversation::new(
        "conversation_1".to_string(),
        "first_node".to_string(),
    ))?;

    let flow_graph = FlowGraph::builder()
        .with_node(
            Node::builder(
                "first_node".to_string(),
                "conversational".to_string(),
                "First Node".to_string(),
                "First Node Description".to_string(),
            )
            .with_action(AIAction::new(
                "gpt-4o-mini".to_string(),
                "You are a helpful assistant".to_string(),
            ))
            .with_action(
                TestAction::new()
            )
            .build(),
        )
        .with_node(
            Node::builder(
                "second_node".to_string(),
                "conversational".to_string(),
                "Second Node".to_string(),
                "Second Node Description".to_string(),
            )
            .with_action(AIAction::new(
                "gpt-4o-mini".to_string(),
                "You are a helpful assistant".to_string(),
            ))
            .build(),
        )
        .with_node(
            Node::builder(
                "third_node".to_string(),
                "conversational".to_string(),
                "Third Node".to_string(),
                "Third Node Description".to_string(),
            )
            .build(),
        )
        .with_edge(
            Edge::builder(
                "first_node_to_second_node".to_string(),
                "first_node".to_string(),
                "second_node".to_string(),
            )
            .with_condition(TestCondition::new())
            .build(),
        )
        .with_edge(
            Edge::builder(
                "second_node_to_third_node".to_string(),
                "second_node".to_string(),
                "third_node".to_string(),
            )
            .build(),
        )
        .build()?;

    let mut flow_manager = FlowManager::new(Box::new(conversation_repository), flow_graph);

    let context1 = flow_manager
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
