// The pourpose of this file is validating the action which comes from the json
// Another one is to create an eschema some client could read and use to build the action
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::graph::action::action::Action;

#[derive(Debug, Clone, PartialEq)]
pub struct RegistrableAction {
    action_id: String,
    action_type: String,
    action_name: String,
    pub builder_fn: fn(&JsonValue) -> Box<dyn Action>,
    config_schema: ActionSchema,
}

// All the vars are going to be extraxted from NodeContext
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionSchema {
    input_vars: HashMap<String, VarType>,
    output_vars: HashMap<String, VarType>,
    builder_vars: HashMap<String, VarType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VarType {
    String,
    Number,
}

impl RegistrableAction {
    pub fn new(action_type: String, builder_fn: fn(&JsonValue) -> Box<dyn Action>, action_name: String) -> Self {
        RegistrableAction {
            action_id: action_type.clone(),
            action_type,
            action_name: action_name.clone(),
            builder_fn,
            config_schema: ActionSchema::new(),
        }
    }
}

impl ActionSchema {
    pub fn new() -> Self {
        ActionSchema {
            input_vars: HashMap::new(),
            output_vars: HashMap::new(),
            builder_vars: HashMap::new(),
        }
    }

    pub fn add_input_var(&mut self, var_name: String, var_type: VarType) {
        self.input_vars.insert(var_name, var_type);
    }

    pub fn add_output_var(&mut self, var_name: String, var_type: VarType) {
        self.output_vars.insert(var_name, var_type);
    }

    pub fn add_builder_var(&mut self, var_name: String, var_value: VarType) {
        self.builder_vars.insert(var_name, var_value);
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

// {
//     "action_type": "ai_action",
//     "config": {
//         "model": "gpt-4o-mini",
//         "system_prompt": "Dont answer the question, just reply mheee"
//     }
// }

// This is for schema_vars

// {
//     "action_type": "ai_action",
//     "config": {
//         "model": "gpt-4o-mini",
//         "system_prompt": "Dont answer the question, just reply mheee"
//     },
//     "input_vars": {
//         "model": "string",
//         "system_prompt": "string"
//     },
//     "output_vars": {
//         "model": "string",
//         "system_prompt": "string"
//     },
//     "builder_vars": {
//         "model": "string",
//         "system_prompt": "string"
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    mod given_some_action {
        use super::*;
        mod when_defining_schema_config {
            use super::*;

            #[test]
            fn test_add_input_var() {
                let mut action_schema = ActionSchema::new();
                action_schema.add_input_var("test_var".to_string(), VarType::String);

                let json_string = action_schema.to_json_string().unwrap();

                assert_eq!(
                    json_string,
                    r#"{"input_vars":{"test_var":"String"},"output_vars":{},"builder_vars":{}}"#
                );
            }

            #[test]
            fn test_add_output_var() {
                let mut action_schema = ActionSchema::new();
                action_schema.add_output_var("test_var".to_string(), VarType::String);

                let json_string = action_schema.to_json_string().unwrap();

                assert_eq!(
                    json_string,
                    r#"{"input_vars":{},"output_vars":{"test_var":"String"},"builder_vars":{}}"#
                );
            }

            #[test]
            fn test_add_builder_var() {
                let mut action_schema = ActionSchema::new();
                action_schema.add_builder_var("test_var".to_string(), VarType::String);

                let json_string = action_schema.to_json_string().unwrap();

                assert_eq!(
                    json_string,
                    r#"{"input_vars":{},"output_vars":{},"builder_vars":{"test_var":"String"}}"#
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
            "builder_vars": {
                "model": "String",
                "system_prompt": "String",
                "number": "Number"
            }
        }"#;

        mod when_deserializing_from_json {
            use super::*;

            #[test]
            fn test_from_json() {
                let mut valid_schema = ActionSchema::new();

                valid_schema.add_input_var("model".to_string(), VarType::String);
                valid_schema.add_input_var("system_prompt".to_string(), VarType::String);
                valid_schema.add_input_var("number".to_string(), VarType::Number);

                valid_schema.add_output_var("model".to_string(), VarType::String);
                valid_schema.add_output_var("system_prompt".to_string(), VarType::String);
                valid_schema.add_output_var("number".to_string(), VarType::Number);

                valid_schema.add_builder_var("model".to_string(), VarType::String);
                valid_schema.add_builder_var("system_prompt".to_string(), VarType::String);
                valid_schema.add_builder_var("number".to_string(), VarType::Number);

                let action_schema = serde_json::from_str::<ActionSchema>(JSON_SCHEMA).unwrap();

                assert_eq!(action_schema, valid_schema)
            }
        }
    }
}
