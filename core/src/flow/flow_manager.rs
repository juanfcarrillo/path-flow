use std::fmt::{Error};

use crate::{graph::{flow_graph::flow_graph::FlowGraph, node::node_context::{NodeContext, Value}}};

use super::conversation::{ConversationRepository};

pub struct FlowManager {
    flow_graph: FlowGraph,
    conversation_repository: Box<dyn ConversationRepository>,
}

impl FlowManager {
    pub fn new(conversation_repository: Box<dyn ConversationRepository>, flow_graph: FlowGraph) -> Self {
        FlowManager {
            flow_graph: flow_graph,
            conversation_repository: conversation_repository,
        }
    }

    pub fn trigger_conversation(&mut self, conversation_id: String) -> Result<NodeContext, Box<dyn std::error::Error>> {
        let mut conversation = self.conversation_repository.get_conversation(conversation_id.clone())?;
        let current_node_id = conversation.get_current_node_id();

        let current_node = self.flow_graph.get_node_mut(&current_node_id)?;

        let messages = conversation.get_messages();

        current_node.set_var_context("messages".to_string(), Value::Messages(messages)); 

        current_node.execute_actions()?;

        let final_node_context = current_node.get_node_context().clone();

        let new_current_node_id = self.flow_graph.find_next_node(&current_node_id, &final_node_context);

        match new_current_node_id {
            Some(node_id) => {
                conversation.set_current_node_id(node_id);
                self.conversation_repository.update_conversation(conversation_id, conversation)?;
            },
            None => {
                return Err(Box::new(Error));
            },
        }

        Ok(final_node_context.clone())
    }
}
