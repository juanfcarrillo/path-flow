use std::fmt;

use super::node::NodeContext;

// Trait for condition evaluation
pub trait Condition<Context> {
    fn evaluate(&self, context: &Context) -> bool;
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
        }
    }

    pub fn add_condition(&mut self, condition: impl Condition<NodeContext>) {
        self.conditions.push(condition.clone_box());
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

    mod given_some_conditions {
        use super::*;

        struct PositiveCondition;

        impl Condition<NodeContext> for PositiveCondition {
            fn evaluate(&self, _context: &NodeContext) -> bool {
                true
            }
            fn clone_box(&self) -> Box<dyn Condition<NodeContext>> {
                Box::new(PositiveCondition)
            }
        }

        struct NegativeCondition;

        impl Condition<NodeContext> for NegativeCondition {
            fn evaluate(&self, _context: &NodeContext) -> bool {
                false
            }
            fn clone_box(&self) -> Box<dyn Condition<NodeContext>> {
                Box::new(NegativeCondition)
            }
        }

        #[test]
        fn test_positive_condition() {
            let condition = PositiveCondition;
            let mut edge = Edge::new(
                "welcome_to_help".to_string(),
                "welcome".to_string(),
                "help".to_string(),
            );

            edge.add_condition(condition);

            assert_eq!(edge.conditions.len(), 1);
            assert_eq!(edge.conditions[0].evaluate(&NodeContext::new()), true); 
        }

        #[test]
        fn test_negative_condition() {
            let condition = NegativeCondition;
            let mut edge = Edge::new(
                "welcome_to_help".to_string(),
                "welcome".to_string(),
                "help".to_string(),
            );

            edge.add_condition(condition);

            assert_eq!(edge.conditions.len(), 1);
            assert_eq!(edge.conditions[0].evaluate(&NodeContext::new()), false); 
        }
    }
}