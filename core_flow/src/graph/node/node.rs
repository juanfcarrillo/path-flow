use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

use crate::graph::action::action::Action;
use crate::graph::action::action_registry::ActionRegistry;
use crate::graph::action::utils::{build_instances, deserialize_actions};

use super::node_builder::NodeBuilder;
use super::node_context::{NodeContext, Value};

/// Represents a node in the conversation flow
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    pub id: String,
    pub node_type: String,
    pub name: String,
    pub description: String,
    node_context: NodeContext,
    #[serde(skip)]
    pub actions: Vec<Box<dyn Action>>, // Actions to perform
}

impl Node {
    pub fn new(id: String, node_type: String, name: String, description: String) -> Self {
        Node {
            id,
            node_type,
            name,
            description,
            node_context: NodeContext::new(),
            actions: Vec::new(),
        }
    }

    pub fn from_json(
        json: &str,
        action_registry: &ActionRegistry,
    ) -> Result<Self, serde_json::Error> {
        let mut node: Node = serde_json::from_str(json)?;

        let json_map: HashMap<String, serde_json::Value> = serde_json::from_str(json)?;

        if let Some(actions_value) = json_map.get("actions") {
            let actions = deserialize_actions(actions_value.to_string().as_str(), action_registry)?;
            node.actions = build_instances(actions);
        }
        Ok(node)
    }

    pub fn builder(
        id: String,
        node_type: String,
        name: String,
        description: String,
    ) -> NodeBuilder {
        NodeBuilder::new(id, node_type, name, description)
    }

    pub fn set_node_context(&mut self, node_context: NodeContext) {
        self.node_context = node_context;
    }

    pub fn add_action(&mut self, action: Box<dyn Action>) {
        self.actions.push(action);
    }

    pub fn set_var_context(&mut self, key: String, value: Value) {
        self.node_context.variables.insert(key, value);
    }

    pub fn get_var_context(&self, key: String) -> Option<Value> {
        self.node_context.variables.get(&key).cloned()
    }

    pub fn get_node_context(&self) -> &NodeContext {
        &self.node_context
    }

    pub async fn execute_actions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut new_context = self.node_context.clone();
        for action in self.actions.iter() {
            new_context = action.execute(&mut new_context).await?;
        }
        self.node_context = new_context;
        Ok(())
    }
}

// enum NodeContext {
//     MessageNode,
//     InputNode,
//     DecisionNode,
//     IntegrationNode,
//     HandoffNode,
// }

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use serde_json::Value as JsonValue;

    // Tests the actions behaviour of the node
    mod given_certain_actions {
        use crate::graph::action::tests::action_implementation::TestAction;

        use super::*;

        struct FailTestAction;

        impl FailTestAction {
            fn new() -> Self {
                FailTestAction
            }
        }

        #[async_trait]
        impl Action for FailTestAction {
            async fn execute(
                &self,
                _context: &mut NodeContext,
            ) -> Result<NodeContext, Box<dyn std::error::Error>> {
                Err("Action failed".into())
            }
            fn clone_box(&self) -> Box<dyn Action> {
                Box::new(FailTestAction)
            }
        }

        #[tokio::test]
        async fn test_correctly_executes_the_actions() {
            let mut node = Node::new(
                "welcome".to_string(),
                "message".to_string(),
                "Welcome".to_string(),
                "Welcome message".to_string(),
            );

            node.add_action(TestAction::new(&JsonValue::Null).clone_box());

            node.execute_actions().await.unwrap();
        }

        #[tokio::test]
        async fn test_fails_executing_the_actions() {
            let mut node = Node::new(
                "welcome".to_string(),
                "message".to_string(),
                "Welcome".to_string(),
                "Welcome message".to_string(),
            );

            node.add_action(TestAction::new(&JsonValue::Null).clone_box());
            node.add_action(FailTestAction::new().clone_box());

            match node.execute_actions().await {
                Ok(_) => {
                    panic!("Expected an error");
                }
                Err(error) => {
                    assert_eq!(error.to_string(), "Action failed");
                }
            }
        }

        #[tokio::test]
        async fn test_correctly_modifies_the_node_context() {
            let mut node = Node::new(
                "welcome".to_string(),
                "message".to_string(),
                "Welcome".to_string(),
                "Welcome message".to_string(),
            );
            node.add_action(TestAction::new(&JsonValue::Null).clone_box());

            node.execute_actions().await.unwrap();

            let test_var = node.get_var_context("test_var".to_string());
            assert_eq!(test_var.unwrap(), Value::String("test_value".to_string()));
        }
    }

    mod given_json {
        use crate::graph::action::tests::action_implementation::TestAction;

        use super::*;

        #[test]
        fn test_from_json() {
            let json = r#"{
                "id": "welcome",
                "node_type": "conversational",
                "name": "Welcome",
                "description": "Welcome message",
                "node_context": {
                    "variables": {}
                },
                "actions": [
                ]
            }"#;

            let mut action_registry = ActionRegistry::new();
            action_registry.register_action("test_action", TestAction::create_registrable_action());

            let node = Node::from_json(json, &action_registry).unwrap();

            assert_eq!(node.id, "welcome");
            assert_eq!(node.node_type, "conversational");
            assert_eq!(node.name, "Welcome");
            assert_eq!(node.description, "Welcome message");
            assert_eq!(node.actions.len(), 0);
        }
    }
}
