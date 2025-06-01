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
    inputs: HashMap<String, String>,  // Key: input name, Value: input type
    outputs: HashMap<String, String>, // Key: output name, Value: output type
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
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            actions: Vec::new(),
        }
    }

    pub fn add_input(&mut self, name: String, input_type: String) {
        self.inputs.insert(name, input_type);
    }

    pub fn add_output(&mut self, name: String, output_type: String) {
        self.outputs.insert(name, output_type);
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
}

#[derive(Debug, Clone)]
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
    fn execute(&self, context: &NodeContext) -> Result<(), Error>;
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