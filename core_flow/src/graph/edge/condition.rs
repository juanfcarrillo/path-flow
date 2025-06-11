use std::fmt;

use async_trait::async_trait;

use crate::graph::node::node_context::NodeContext;

// Trait for condition evaluation
#[async_trait]
pub trait Condition<Context> {
    async fn evaluate(&self, context: &Context) -> bool;
    fn clone_box(&self) -> Box<dyn Condition<Context>>;
}

impl fmt::Debug for Box<dyn Condition<NodeContext>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Condition")
    }
}

impl Clone for Box<dyn Condition<NodeContext>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}