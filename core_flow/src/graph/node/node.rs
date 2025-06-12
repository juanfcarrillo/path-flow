use std::fmt::Debug;

use async_trait::async_trait;

use super::node_builder::NodeBuilder;
use super::node_context::{NodeContext, Value};

/// Represents a node in the conversation flow
#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub node_type: String,
    pub name: String,
    pub description: String,
    node_context: NodeContext,
    actions: Vec<Box<dyn Action>>,           // Actions to perform
}

impl Node {
    pub fn new(
        id: String,
        node_type: String,
        name: String,
        description: String,
    ) -> Self {
        Node {
            id,
            node_type,
            name,
            description,
            node_context: NodeContext::new(),
            actions: Vec::new(),
        }
    }

    pub fn builder(
        id: String,
        node_type: String,
        name: String,
        description: String,
    ) -> NodeBuilder {
        NodeBuilder::new(
            id,
            node_type,
            name,
            description,
        )
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

#[async_trait]
pub trait Action {
    async fn execute(&self, context: &mut NodeContext) -> Result<NodeContext, Box<dyn std::error::Error>>;
    fn clone_box(&self) -> Box<dyn Action>;
}

impl Debug for dyn Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Action")
    }
}

impl Clone for Box<dyn Action> {
    fn clone(&self) -> Self {
        self.clone_box()
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

    // Tests the actions behaviour of the node
    mod given_certain_actions {

        use super::*;

        // It represents an action
        struct TestAction;

        impl TestAction {
            fn new() -> Self {
                TestAction
            }
        }

        #[async_trait]
        impl Action for TestAction {
            async fn execute(&self, context: &mut NodeContext) -> Result<NodeContext, Box<dyn std::error::Error>> {
                context.variables.insert("test_var".to_string(), Value::String("test_value".to_string()));
                Ok(context.clone())
            }
            fn clone_box(&self) -> Box<dyn Action> {
                Box::new(TestAction)
            }
        }

        struct FailTestAction;

        impl FailTestAction {
            fn new() -> Self {
                FailTestAction
            }
        }

        #[async_trait]
        impl Action for FailTestAction {
            async fn execute(&self, _context: &mut NodeContext) -> Result<NodeContext, Box<dyn std::error::Error>> {
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
                "Welcome message".to_string()
            );

            node.add_action(TestAction::new().clone_box());

            node.execute_actions().await.unwrap();


            // match node.execute_actions() {
            //     Ok(_) => {},
            //     Err(error) => {
            //         panic!("Error: {:?}", error);
            //     }
            // }
        }

        #[tokio::test]
        async fn test_fails_executing_the_actions() {
            let mut node = Node::new(
                "welcome".to_string(),
                "message".to_string(),
                "Welcome".to_string(),
                "Welcome message".to_string()
            );

            node.add_action(TestAction::new().clone_box());
            node.add_action(FailTestAction::new().clone_box());

            match node.execute_actions().await {
                Ok(_) => {
                    panic!("Expected an error");
                },
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
                "Welcome message".to_string()
            );
            node.add_action(TestAction::new().clone_box());

            node.execute_actions().await.unwrap();

            let test_var = node.get_var_context("test_var".to_string());
            assert_eq!(test_var.unwrap(), Value::String("test_value".to_string()));
        }
    }
}