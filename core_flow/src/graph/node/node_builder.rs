use super::{node::{Action, Node}, node_context::{NodeContext, Value}};

pub struct NodeBuilder {
    id: String,
    node_type: String,
    name: String,
    description: String,
    actions: Vec<Box<dyn Action>>,
    node_context: NodeContext,
}

impl NodeBuilder {
    pub fn new(id: String, node_type: String, name: String, description: String) -> Self {
        NodeBuilder {
            id,
            node_type,
            name,
            description,
            actions: Vec::new(),
            node_context: NodeContext::new(),
        }
    }

    pub fn with_action(mut self, action: impl Action) -> Self {
        self.actions.push(action.clone_box());
        self
    }

    pub fn with_context_var(mut self, key: String, value: Value) -> Self {
        self.node_context.variables.insert(key, value);
        self
    }

    pub fn build(self) -> Node {
        let mut new_node = Node::new(self.id, self.node_type, self.name, self.description);

        new_node.set_node_context(self.node_context);

        for action in self.actions {
            new_node.add_action(action);
        }

        new_node
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use super::*;

    // Test action implementation
    struct TestAction;

    impl TestAction {
        fn new() -> Self {
            TestAction
        }
    }

    #[async_trait]
    impl Action for TestAction {
        async fn execute(&self, context: &mut NodeContext) -> Result<NodeContext, Box<dyn std::error::Error>> {
            context.variables.insert("test_var".to_string(), Value::String("test_value".to_string()));
            Ok(context.clone())
        }
        
        fn clone_box(&self) -> Box<dyn Action> {
            Box::new(TestAction)
        }
    }

    #[test]
    fn test_builder_creates_node_with_basic_properties() {
        let node = NodeBuilder::new(
            "test_id".to_string(),
            "test_type".to_string(),
            "Test Node".to_string(),
            "Test Description".to_string()
        ).build();

        assert_eq!(node.id, "test_id");
        assert_eq!(node.node_type, "test_type");
        assert_eq!(node.name, "Test Node");
        assert_eq!(node.description, "Test Description");
    }

    #[tokio::test]
    async fn test_builder_adds_action() {
        let mut node = NodeBuilder::new(
            "test_id".to_string(),
            "test_type".to_string(),
            "Test Node".to_string(),
            "Test Description".to_string()
        )
        .with_action(TestAction::new())
        .build();

        // Execute actions to verify the action was added
        node.execute_actions().await.unwrap();
        assert_eq!(
            node.get_var_context("test_var".to_string()),
            Some(Value::String("test_value".to_string()))
        );
    }

    #[test]
    fn test_builder_adds_context_variable() {
        let test_value = Value::String("test_value".to_string());
        let node = NodeBuilder::new(
            "test_id".to_string(),
            "test_type".to_string(),
            "Test Node".to_string(),
            "Test Description".to_string()
        )
        .with_context_var("test_key".to_string(), test_value.clone())
        .build();

        assert_eq!(
            node.get_var_context("test_key".to_string()),
            Some(test_value)
        );
    }

    #[tokio::test]
   async fn test_builder_chain_methods() {
        let test_value = Value::String("context_value".to_string());
        let mut node= NodeBuilder::new(
            "test_id".to_string(),
            "test_type".to_string(),
            "Test Node".to_string(),
            "Test Description".to_string()
        )
        .with_action(TestAction::new())
        .with_context_var("test_key".to_string(), test_value.clone())
        .build();

        // Verify context variable was set
        assert_eq!(
            node.get_var_context("test_key".to_string()),
            Some(test_value)
        );

        // Verify action was added
        node.execute_actions().await.unwrap();
        assert_eq!(
            node.get_var_context("test_var".to_string()),
            Some(Value::String("test_value".to_string()))
        );
    }
}