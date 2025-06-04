use crate::graph::{edge::condition::Condition, node::node_context::NodeContext};

pub struct PositiveCondition;

impl Condition<NodeContext> for PositiveCondition {
    fn evaluate(&self, _context: &NodeContext) -> bool {
        true
    }
    fn clone_box(&self) -> Box<dyn Condition<NodeContext>> {
        Box::new(PositiveCondition)
    }
}

pub struct NegativeCondition;

impl Condition<NodeContext> for NegativeCondition {
    fn evaluate(&self, _context: &NodeContext) -> bool {
        false
    }
    fn clone_box(&self) -> Box<dyn Condition<NodeContext>> {
        Box::new(NegativeCondition)
    }
}
