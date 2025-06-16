use std::{collections::HashMap, fmt::Error};

use serde_json::{Value as JsonValue};

use crate::graph::node::node_context::{NodeContext, Value};

pub fn parse_input_vars(input_vars: &JsonValue, node_context: &NodeContext) -> Result<HashMap<String, JsonValue>, serde_json::Error> {
    let object_input_vars = input_vars.as_object().unwrap();
    let mut input_vars: HashMap<String, JsonValue> = HashMap::new();

    for (key, value) in object_input_vars {
        let value = value.as_str().unwrap();
        let variable = node_context.variables.get(value);
        if let Some(variable) = variable {
            input_vars.insert(key.to_string(), variable.clone().into());
        }
    }

    Ok(input_vars)
}

#[derive(Debug, Clone)]
pub struct OutputVarsBuilder {
    node_name: String,
    node_context: NodeContext,
    out_put_vars: Vec<String>,
}

impl OutputVarsBuilder {
    pub fn new(config: JsonValue, output_vars: JsonValue, node_context: NodeContext) -> Self {
        let node_name = config.get("name").unwrap().as_str().unwrap().to_string();

        let out_put_vars: Vec<String> = output_vars.as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();

        OutputVarsBuilder {
            node_name,
            node_context,
            out_put_vars,
        }
    }

    pub fn add_var(&mut self, var: String, value: Value) -> &mut Self {
        if self.out_put_vars.contains(&var) {
            let index = self.out_put_vars.iter().position(|v| v == &var).unwrap();
            self.out_put_vars.remove(index);
            let new_var = format!("{}.{}", self.node_name, var);
            self.node_context.variables.insert(new_var, value);
        }

        self
    }

    pub fn build(&self) -> Result<NodeContext, Error> {
        if self.out_put_vars.len() > 0 {
            return Err(Error)
        }

        Ok(self.node_context.clone())
    }
}