use std::collections::HashMap;
use serde::de::Error;
use serde_json::Value as JsonValue;
use crate::graph::action::{action::Action, action_registry::ActionRegistry};

fn deserialize_config(config: Option<JsonValue>) -> Result<JsonValue, serde_json::Error> {
    match config {
        Some(config) => {
            if config.get("name").is_none() {
                return Err(serde_json::Error::custom("Action config name is required"));
            }

            if config.get("id").is_none() {
                return Err(serde_json::Error::custom("Action config id is required"));
            }

            Ok(config)
        }
        None => Err(serde_json::Error::custom("Action config is required")),
    }
}

fn deserialize_input_vars(input_vars: Option<JsonValue>) -> Result<JsonValue, serde_json::Error> {
    match input_vars {
        Some(input_vars) => {
            Ok(input_vars)
        }
        None => Err(serde_json::Error::custom("Input vars is required")),
    }
}

fn deserialize_output_vars(output_vars: Option<JsonValue>) -> Result<JsonValue, serde_json::Error> {
    match output_vars {
        Some(output_vars) => {
            Ok(output_vars)
        }
        None => Err(serde_json::Error::custom("Output vars is required")),
    }
}

pub fn deserialize_actions(
    json_data: &str,
    action_registry: &ActionRegistry,
) -> Result<Vec<Box<dyn Action>>, serde_json::Error> {
    let actions_data: Vec<HashMap<String, JsonValue>> = serde_json::from_str(json_data)?;
    let mut actions: Vec<Box<dyn Action>> = Vec::new();

    for action_data in actions_data {
        if let Some(action_type) = action_data.get("action_type").and_then(|v| v.as_str()) {
            if let Some(action_constructor) = action_registry.get_actions().get(action_type) {
                let config = deserialize_config(action_data.get("config").cloned())?;
                let input_vars = deserialize_input_vars(action_data.get("input_vars").cloned())?;
                let output_vars = deserialize_output_vars(action_data.get("output_vars").cloned())?;

                actions.push(action_constructor(&config, &input_vars, &output_vars));
            } else {
                return Err(serde_json::Error::custom(format!(
                    "Unknown action type: {}",
                    action_type
                )));
            }
        }
    }

    Ok(actions)
}