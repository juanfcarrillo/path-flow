use std::fmt;
use std::collections::HashMap;

use super::node::NodeContext;

// Trait for condition evaluation
pub trait Condition<Context> {
    fn evaluate(&self, context: &Context) -> bool;
    fn get_metadata(&self) -> &HashMap<String, String>;
    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String>;
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

#[derive(Debug, Clone)]
pub struct Edge {
    pub id: String,
    pub source_node_id: String,
    pub target_node_id: String,
    pub priority: i32,
    conditions: Vec<Box<dyn Condition<NodeContext>>>,
    metadata: HashMap<String, String>,
}

impl Edge {
    pub fn new(
        id: String,
        source_node_id: String,
        target_node_id: String,
    ) -> Self {
        Edge {
            id,
            source_node_id,
            target_node_id,
            conditions: Vec::new(),
            priority: 0,
            metadata: HashMap::new(),
        }
    }

    pub fn add_condition(&mut self, condition: impl Condition<NodeContext> + 'static) {
        self.conditions.push(Box::new(condition));
    }

    pub fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }

    pub fn evaluate(&self, context: &NodeContext) -> bool {
        self.conditions.iter().all(|condition| condition.evaluate(context))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge() {
        let edge = Edge::new(
            "welcome_to_help".to_string(),
            "welcome".to_string(),
            "help".to_string(),
        );

        assert_eq!(edge.id, "welcome_to_help");
        assert_eq!(edge.source_node_id, "welcome");
        assert_eq!(edge.target_node_id, "help");
        assert_eq!(edge.priority, 0);
        assert_eq!(edge.conditions.len(), 0);
    }
}