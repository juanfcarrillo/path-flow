use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::flow::conversation::Message;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Null,
    Messages(Vec<Message>)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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