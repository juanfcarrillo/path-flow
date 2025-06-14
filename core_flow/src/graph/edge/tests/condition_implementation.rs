use async_trait::async_trait;

use crate::graph::{edge::condition::Condition, node::node_context::NodeContext};

pub struct PositiveCondition;

#[async_trait]
impl Condition<NodeContext> for PositiveCondition {
    async fn evaluate(&self, _context: &NodeContext) -> bool {
        true
    }
    fn clone_box(&self) -> Box<dyn Condition<NodeContext>> {
        Box::new(PositiveCondition)
    }
}

pub struct NegativeCondition;

#[async_trait]
impl Condition<NodeContext> for NegativeCondition {
    async fn evaluate(&self, _context: &NodeContext) -> bool {
        false
    }
    fn clone_box(&self) -> Box<dyn Condition<NodeContext>> {
        Box::new(NegativeCondition)
    }
}

pub struct ConfigurableCondition {
    config: serde_json::Value,
}

impl ConfigurableCondition {
    pub fn new(config: &serde_json::Value) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

#[async_trait]
impl Condition<NodeContext> for ConfigurableCondition {
    async fn evaluate(&self, _context: &NodeContext) -> bool {
        self.config.get("key").is_some()
    }
    fn clone_box(&self) -> Box<dyn Condition<NodeContext>> {
        Box::new(ConfigurableCondition {
            config: self.config.clone(),
        })
    }
}
