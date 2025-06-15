// The pourpose of this file is validating the action which comes from the json
// Another one is to create an eschema some client could read and use to build the action
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::graph::action::action::Action;

#[derive(Debug, Clone, PartialEq)]
pub struct RegistrableActionMold {
    action_type: String,
    config_schema: ActionSchemaMold,
    pub builder_fn: fn(&JsonValue, &JsonValue, &JsonValue) -> Box<dyn Action>,
}

// All the vars are going to be extraxted from NodeContext
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionSchemaMold {
    input_vars: HashMap<String, VarType>,
    output_vars: HashMap<String, VarType>,
    config: HashMap<String, VarType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VarType {
    String,
    Number,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegistrableActionInstace {
    pub id: String,
    pub action_type: String,
    pub name: String,
    pub config: JsonValue,
    pub input_vars: JsonValue,
    pub output_vars: JsonValue,
    pub builder_fn: fn(&JsonValue, &JsonValue, &JsonValue) -> Box<dyn Action>,
}

impl RegistrableActionMold {
    pub fn new(action_type: String, builder_fn: fn(&JsonValue, &JsonValue, &JsonValue) -> Box<dyn Action>) -> Self {
        RegistrableActionMold {
            action_type,
            builder_fn,
            config_schema: ActionSchemaMold::new(),
        }
    }
}

impl ActionSchemaMold {
    pub fn new() -> Self {
        ActionSchemaMold {
            input_vars: HashMap::new(),
            output_vars: HashMap::new(),
            config: HashMap::new(),
        }
    }

    pub fn add_input_var(&mut self, var_name: String, var_type: VarType) {
        self.input_vars.insert(var_name, var_type);
    }

    pub fn add_output_var(&mut self, var_name: String, var_type: VarType) {
        self.output_vars.insert(var_name, var_type);
    }

    pub fn add_builder_var(&mut self, var_name: String, var_value: VarType) {
        self.config.insert(var_name, var_value);
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl RegistrableActionInstace {
    pub fn new(name: String, action_type: String, config: JsonValue, input_vars: JsonValue, output_vars: JsonValue, builder_fn: fn(&JsonValue, &JsonValue, &JsonValue) -> Box<dyn Action>) -> Self {
        RegistrableActionInstace {
            id: String::new(),
            name,
            action_type,
            builder_fn,
            config,
            input_vars,
            output_vars,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod given_some_action {
        use super::*;
        mod when_defining_schema_config {
            use super::*;

            #[test]
            fn test_add_input_var() {
                let mut action_schema = ActionSchemaMold::new();
                action_schema.add_input_var("test_var".to_string(), VarType::String);

                let json_string = action_schema.to_json_string().unwrap();

                assert_eq!(
                    json_string,
                    r#"{"input_vars":{"test_var":"String"},"output_vars":{},"config":{}}"#
                );
            }

            #[test]
            fn test_add_output_var() {
                let mut action_schema = ActionSchemaMold::new();
                action_schema.add_output_var("test_var".to_string(), VarType::String);

                let json_string = action_schema.to_json_string().unwrap();

                assert_eq!(
                    json_string,
                    r#"{"input_vars":{},"output_vars":{"test_var":"String"},"config":{}}"#
                );
            }

            #[test]
            fn test_add_builder_var() {
                let mut action_schema = ActionSchemaMold::new();
                action_schema.add_builder_var("test_var".to_string(), VarType::String);

                let json_string = action_schema.to_json_string().unwrap();

                assert_eq!(
                    json_string,
                    r#"{"input_vars":{},"output_vars":{},"config":{"test_var":"String"}}"#
                );
            }
        }
    }

    mod given_some_schema {
        use super::*;

        static JSON_SCHEMA: &str = r#"{
            "input_vars": {
                "model": "String",
                "system_prompt": "String",
                "number": "Number"
            },
            "output_vars": {
                "model": "String",
                "system_prompt": "String",
                "number": "Number"
            },
            "config": {
                "model": "String",
                "system_prompt": "String",
                "number": "Number"
            }
        }"#;

        mod when_deserializing_from_json {
            use super::*;

            #[test]
            fn test_from_json() {
                let mut valid_schema = ActionSchemaMold::new();

                valid_schema.add_input_var("model".to_string(), VarType::String);
                valid_schema.add_input_var("system_prompt".to_string(), VarType::String);
                valid_schema.add_input_var("number".to_string(), VarType::Number);

                valid_schema.add_output_var("model".to_string(), VarType::String);
                valid_schema.add_output_var("system_prompt".to_string(), VarType::String);
                valid_schema.add_output_var("number".to_string(), VarType::Number);

                valid_schema.add_builder_var("model".to_string(), VarType::String);
                valid_schema.add_builder_var("system_prompt".to_string(), VarType::String);
                valid_schema.add_builder_var("number".to_string(), VarType::Number);

                let action_schema = serde_json::from_str::<ActionSchemaMold>(JSON_SCHEMA).unwrap();

                assert_eq!(action_schema, valid_schema)
            }
        }
    }


}
