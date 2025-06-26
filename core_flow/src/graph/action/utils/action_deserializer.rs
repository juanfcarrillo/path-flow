use std::{collections::HashMap, fmt};
use serde::de::Error;
use serde_json::Value as JsonValue;
use crate::graph::action::{action::Action, action_registry::ActionRegistry};

#[derive(Debug)]
pub enum DeserializeActionError {
    MissingName,
    MissingId,
    #[allow(dead_code)]
    MissingActionType,
    MissingConfig,
    MissingInputVars,
    MissingOutputVars,
    IncorrectOutputVarsType(String),
    DeserializeError(serde_json::Error),
}

impl fmt::Display for DeserializeActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeserializeActionError::MissingName => write!(f, "Action config name is required"),
            DeserializeActionError::MissingId => write!(f, "Action config id is required"),
            DeserializeActionError::MissingActionType => write!(f, "Action type is required"),
            DeserializeActionError::MissingConfig => write!(f, "Action config is required"),
            DeserializeActionError::MissingInputVars => write!(f, "Input vars is required"),
            DeserializeActionError::MissingOutputVars => write!(f, "Output vars is required"),
            DeserializeActionError::IncorrectOutputVarsType(type_name) => write!(f, "Output vars must be an array, found {}", type_name),
            DeserializeActionError::DeserializeError(error) => write!(f, "Deserialize error: {}", error),
        }
    }
}

impl std::error::Error for DeserializeActionError {}

impl From<DeserializeActionError> for serde_json::Error {
    fn from(error: DeserializeActionError) -> Self {
        serde_json::Error::custom(error.to_string())
    }
}

impl From<serde_json::Error> for DeserializeActionError {
    fn from(error: serde_json::Error) -> Self {
        DeserializeActionError::DeserializeError(error)
    }
}

fn deserialize_config(config: Option<JsonValue>) -> Result<JsonValue, DeserializeActionError> {
    match config {
        Some(config) => {
            if config.get("name").is_none() {
                return Err(DeserializeActionError::MissingName);
            }

            if config.get("id").is_none() {
                return Err(DeserializeActionError::MissingId);
            }

            Ok(config)
        }
        None => Err(DeserializeActionError::MissingConfig),
    }
}

pub fn deserialize_input_vars(input_vars: Option<JsonValue>) -> Result<JsonValue, DeserializeActionError> {
    match input_vars {
        Some(input_vars) => {
            Ok(input_vars)
        }
        None => Err(DeserializeActionError::MissingInputVars),
    }
}

fn deserialize_output_vars(output_vars: Option<JsonValue>) -> Result<JsonValue, DeserializeActionError> {
    match output_vars {
        Some(output_vars) => {
            if !output_vars.is_array() {
                return Err(DeserializeActionError::IncorrectOutputVarsType(output_vars.to_string()));
            }
            Ok(output_vars)
        }
        None => Err(DeserializeActionError::MissingOutputVars),
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