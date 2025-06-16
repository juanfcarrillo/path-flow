use std::{collections::HashMap, error::Error, fmt, thread::panicking};

use serde_json::Value as JsonValue;

use crate::graph::node::node_context::{NodeContext, Value};

#[derive(Debug)]
pub enum VarParseError {
    VariableNotFound,
    MissingVars(String),
}

impl fmt::Display for VarParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VarParseError::VariableNotFound => write!(f, "Variable not found"),
            VarParseError::MissingVars(var) => write!(f, "Missing vars: {}", var),
        }
    }
}

impl Error for VarParseError {}

pub fn parse_input_vars(
    input_vars: &JsonValue,
    node_context: &NodeContext,
) -> Result<HashMap<String, JsonValue>, serde_json::Error> {
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

        let out_put_vars: Vec<String> = output_vars
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();

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
        } else {
            panic!("Variable '{}' not found", var);
        }

        self
    }

    pub fn build(&self) -> Result<NodeContext, VarParseError> {
        if self.out_put_vars.len() > 0 {
            return Err(VarParseError::MissingVars(self.out_put_vars.join(", ")));
        }

        Ok(self.node_context.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    use crate::graph::action::utils::vars_parser::{OutputVarsBuilder, parse_input_vars};
    use crate::graph::node::node_context::{NodeContext, Value};

    #[test]
    fn test_parse_input_vars() {
        let mut node_context = NodeContext::new();
        let input_vars = json!({
            "var1": "test_id.var1",
            "var2": "test_id.var2",
            "var3": "test_id.var3",
        });

        node_context
            .variables
            .insert("test_id.var1".to_string(), Value::String("foo".to_string()));
        node_context
            .variables
            .insert("test_id.var2".to_string(), Value::String("bar".to_string()));
        node_context
            .variables
            .insert("test_id.var3".to_string(), Value::String("baz".to_string()));

        let vars = parse_input_vars(&input_vars, &node_context).unwrap();

        assert_eq!(vars.len(), 3);

        let var1 = vars.get("var1").unwrap();
        assert_eq!(var1.as_str().unwrap(), "foo");

        let var2 = vars.get("var2").unwrap();
        assert_eq!(var2.as_str().unwrap(), "bar");

        let var3 = vars.get("var3").unwrap();
        assert_eq!(var3.as_str().unwrap(), "baz");
    }

    #[test]
    fn test_output_vars_builder() {
        let node_context = NodeContext::new();
        let output_vars = json!(["var1", "var2", "var3"]);
        let mut output_vars_builder =
            OutputVarsBuilder::new(json!({"name": "node_name"}), output_vars, node_context);
        
        output_vars_builder.add_var("var1".to_string(), Value::String("foo".to_string()));
        output_vars_builder.add_var("var2".to_string(), Value::String("bar".to_string()));
        output_vars_builder.add_var("var3".to_string(), Value::String("baz".to_string()));

        let node_context = output_vars_builder.build().unwrap();

        assert_eq!(node_context.variables.len(), 3);

        let var1 = node_context.variables.get("node_name.var1").unwrap();
        assert_eq!(var1, &Value::String("foo".to_string()));

        let var2 = node_context.variables.get("node_name.var2").unwrap();
        assert_eq!(var2, &Value::String("bar".to_string()));

        let var3 = node_context.variables.get("node_name.var3").unwrap();
        assert_eq!(var3, &Value::String("baz".to_string()));
    }

    #[test]
    #[should_panic]
    fn test_output_vars_builder_with_not_found_var() {
        let node_context = NodeContext::new();
        let output_vars = json!(["var1", "var2", "var3"]);
        let mut output_vars_builder =
            OutputVarsBuilder::new(json!({"name": "node_name"}), output_vars, node_context);
        
        output_vars_builder.add_var("var123".to_string(), Value::String("foo".to_string()));
        output_vars_builder.add_var("var1".to_string(), Value::String("bar".to_string()));
        output_vars_builder.add_var("var2".to_string(), Value::String("bar".to_string()));
        output_vars_builder.add_var("var3".to_string(), Value::String("baz".to_string()));

        output_vars_builder.build().unwrap();
    }

    #[test]
    fn test_output_vars_builder_with_missing_vars() {
        let node_context = NodeContext::new();
        let output_vars = json!(["var1", "var2", "var3"]);
        let mut output_vars_builder =
            OutputVarsBuilder::new(json!({"name": "node_name"}), output_vars, node_context);
        
        output_vars_builder.add_var("var1".to_string(), Value::String("foo".to_string()));
        output_vars_builder.add_var("var1".to_string(), Value::String("bar".to_string()));
        output_vars_builder.add_var("var3".to_string(), Value::String("baz".to_string()));

        let node_context = output_vars_builder.build();
        assert!(node_context.is_err(), "Expected an error due to duplicated variables");
        if let Err(VarParseError::MissingVars(missing_vars)) = node_context {
            assert!(missing_vars.contains("var2"), "Expected 'var2' to be missing");
        } else {
            panic!("Expected VarParseError::MissingVars");
        }
    }
}
