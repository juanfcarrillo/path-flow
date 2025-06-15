use std::collections::HashMap;

use crate::graph::action::{
    action::Action, action_registry::ActionRegistry, registrable_action::RegistrableActionInstace
};
use serde::ser::Error;
use serde_json::Value as JsonValue;

fn deserialize_action(
    action_data: HashMap<String, JsonValue>,
    action_registry: &ActionRegistry,
) -> Result<RegistrableActionInstace, serde_json::Error> {
    let action_type = action_data.get("action_type");
    if let None = action_type {
        return Err(serde_json::Error::custom("Action type not found"));
    }
    let action_type = action_type.unwrap().as_str().unwrap().to_string();

    let config = action_data.get("config");
    if let None = config {
        return Err(serde_json::Error::custom("Config not found"));
    }
    let config = config.unwrap();

    let input_vars = action_data.get("input_vars");
    if let None = input_vars {
        return Err(serde_json::Error::custom("Input vars not found"));
    }
    let input_vars = input_vars.unwrap();

    let output_vars = action_data.get("output_vars");
    if let None = output_vars {
        return Err(serde_json::Error::custom("Output vars not found"));
    }
    let output_vars = output_vars.unwrap();

    let name = action_data.get("name");
    if let None = name {
        return Err(serde_json::Error::custom("Name not found"));
    }
    let name = name.unwrap().as_str().unwrap().to_string();

    let builder_fn = action_registry.get_actions().get(&action_type);
    if let None = builder_fn {
        return Err(serde_json::Error::custom(format!(
            "Action type {} not found",
            action_type
        )));
    }

    let builder_fn = builder_fn.unwrap().builder_fn;

    Ok(RegistrableActionInstace::new(
        name.clone(),
        action_type.clone(),
        config.clone(),
        input_vars.clone(),
        output_vars.clone(),
        builder_fn
    ))
}

pub fn deserialize_actions(
    json_data: &str,
    action_registry: &ActionRegistry,
) -> Result<Vec<RegistrableActionInstace>, serde_json::Error> {
    let actions_data: Vec<HashMap<String, JsonValue>> = serde_json::from_str(json_data)?;

    let actions = actions_data.iter().map(|action_data| {
        deserialize_action(action_data.clone(), action_registry)
    }).collect::<Result<Vec<RegistrableActionInstace>, serde_json::Error>>()?;

    Ok(actions)
}

pub fn build_instances(
    actions: Vec<RegistrableActionInstace>,
) -> Vec<Box<dyn Action>> {
    actions.iter().map(|registrable_instance| {
        (registrable_instance.builder_fn)(
            &registrable_instance.config,
            &registrable_instance.input_vars,
            &registrable_instance.output_vars,
        ).clone_box()
    }).collect::<Vec<Box<dyn Action>>>()
}
