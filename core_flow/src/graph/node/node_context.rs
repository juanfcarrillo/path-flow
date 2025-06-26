use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, Map as JsonMap};

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

impl Value {
    pub fn as_messages(&self) -> Option<&Vec<Message>> {
        if let Value::Messages(messages) = self {
            Some(messages)
        } else {
            None
        }
    }
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

impl From<JsonValue> for Value {
    fn from(json_value: JsonValue) -> Self {
        match json_value {
            JsonValue::String(s) => Value::String(s),
            JsonValue::Number(n) => Value::Number(n.as_f64().unwrap_or(0.0)),
            JsonValue::Bool(b) => Value::Boolean(b),
            JsonValue::Array(arr) => Value::List(arr.into_iter().map(Value::from).collect()),
            JsonValue::Object(obj) => Value::Map(
                obj.into_iter()
                    .map(|(k, v)| (k, Value::from(v)))
                    .collect(),
            ),
            JsonValue::Null => Value::Null,
        }
    }
}

impl Into<JsonValue> for Value {
    fn into(self) -> JsonValue {
        match self {
            Value::String(s) => JsonValue::String(s),
            Value::Number(n) => JsonValue::Number(serde_json::Number::from_f64(n).unwrap()),
            Value::Boolean(b) => JsonValue::Bool(b),
            Value::List(lst) => JsonValue::Array(lst.into_iter().map(Into::into).collect()),
            Value::Map(map) => JsonValue::Object(
                map.into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect::<JsonMap<String, JsonValue>>(),
            ),
            Value::Null => JsonValue::Null,
            Value::Messages(_) => unimplemented!("Conversion for Messages is not implemented"),
        }
    }
}