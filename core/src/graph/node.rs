use std::fmt;
use std::{collections::HashMap, fmt::Error};

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

    pub fn add_action(&mut self, action: impl Action + 'static) {
        self.actions.push(Box::new(action));
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

    pub fn execute_actions(&mut self) -> Result<(), Error> {
        for action in self.actions.iter() {
            action.execute(&mut self.node_context)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Null,
}

#[derive(Debug, Clone)]
pub struct NodeContext {
    pub variables: HashMap<String, Value>,
}

impl NodeContext {
    pub fn new() -> Self {
        NodeContext {
            variables: HashMap::new(),
        }
    }
}

pub trait Action {
    fn execute(&self, context: &mut NodeContext) -> Result<(), Error>;
    fn clone_box(&self) -> Box<dyn Action>;
}

impl fmt::Debug for Box<dyn Action> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Debug")
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

        impl Action for TestAction {
            fn execute(&self, context: &mut NodeContext) -> Result<(), Error> {
                context.variables.insert("test_var".to_string(), Value::String("test_value".to_string()));
                Ok(())
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

        impl Action for FailTestAction {
            fn execute(&self, _context: &mut NodeContext) -> Result<(), Error> {
                Err(Error)
            }
            fn clone_box(&self) -> Box<dyn Action> {
                Box::new(FailTestAction)
            }
        }


        #[test]
        fn test_correctly_executes_the_actions() {
            let mut node = Node::new(
                "welcome".to_string(),
                "message".to_string(),
                "Welcome".to_string(),
                "Welcome message".to_string()
            );

            node.add_action(TestAction::new());

            match node.execute_actions() {
                Ok(_) => {},
                Err(error) => {
                    panic!("Error: {:?}", error);
                }
            }
        }

        #[test]
        fn test_fails_executing_the_actions() {
            let mut node = Node::new(
                "welcome".to_string(),
                "message".to_string(),
                "Welcome".to_string(),
                "Welcome message".to_string()
            );

            node.add_action(TestAction::new());
            node.add_action(FailTestAction::new());

            match node.execute_actions() {
                Ok(_) => {
                    panic!("Expected an error");
                },
                Err(error) => {
                    assert_eq!(error, Error);
                }
            }
        }

        #[test]
        fn test_correctly_modifies_the_node_context() {
            let mut node = Node::new(
                "welcome".to_string(),
                "message".to_string(),
                "Welcome".to_string(),
                "Welcome message".to_string()
            );
            node.add_action(TestAction::new());

            node.execute_actions().unwrap();

            let test_var = node.get_var_context("test_var".to_string());
            assert_eq!(test_var.unwrap(), Value::String("test_value".to_string()));
        }
    }
}