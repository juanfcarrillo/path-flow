use std::collections::HashMap;

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