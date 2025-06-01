use std::collections::{HashMap, VecDeque};

use super::{edge::Edge, node::{Node, NodeContext}};

#[derive(Debug)]
pub struct FlowGraph {
    nodes: HashMap<String, Node>,
    edges: HashMap<String, Edge>,
    // Adjacency list for quick traversal
    adjacency_list: HashMap<String, Vec<String>>, // node_id -> vec of edge_ids
}

impl FlowGraph {
    pub fn new() -> Self {
        FlowGraph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> Result<(), FlowError> {
        let node_id: String = node.id.clone();

        if self.nodes.contains_key(&node_id) {
            return Err(FlowError::DuplicateNode(node_id));
        }

        self.nodes.insert(node_id.clone(), node);
        self.adjacency_list.insert(node_id, Vec::new());

        Ok(())
    }

    pub fn add_edge(&mut self, edge: Edge) -> Result<(), FlowError> {
        // Validate that source and target nodes exist
        if !self.nodes.contains_key(&edge.source_node_id) {
            return Err(FlowError::NodeNotFound(edge.source_node_id.clone()));
        }

        if !self.nodes.contains_key(&edge.target_node_id) {
            return Err(FlowError::NodeNotFound(edge.target_node_id.clone()));
        }

        let edge_id: String = edge.id.clone();
        if self.edges.contains_key(&edge_id) {
            return Err(FlowError::DuplicateEdge(edge_id));
        }

        let source_node_id = edge.source_node_id.as_str();

        // Add edge to adjacency list
        self.adjacency_list
            .get_mut(source_node_id)
            .unwrap()
            .push(edge_id.clone());

        self.edges.insert(edge_id, edge);

        Ok(())
    }

    /// Find next valid node based on current context
    pub fn find_next_node(&self, current_node_id: &str, context: &NodeContext) -> Option<String> {
        let edge_ids = self.adjacency_list.get(current_node_id)?;
        
        // Get all valid edges sorted by priority
        let mut valid_edges: Vec<_> = edge_ids
            .iter()
            .filter_map(|edge_id| self.edges.get(edge_id))
            .filter(|edge| edge.evaluate(context))
            .collect();

        // Sort by priority (highest first)
        valid_edges.sort_by_key(|edge| -edge.priority);

        // Return the target node of the highest priority valid edge
        valid_edges.first().map(|edge| edge.target_node_id.clone())
    }

    /// BFS traversal for path finding
    pub fn find_path(&self, start_node_id: &str, end_node_id: &str) -> Option<Vec<String>> {
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();
        queue.push_back(start_node_id.to_string());
        visited.insert(start_node_id.to_string(), None);

        while let Some(current_node_id) = queue.pop_front() {
            if current_node_id == end_node_id {
                return Some(self.reconstruct_path(&visited, start_node_id, end_node_id));
            }

            if let Some(edge_ids) = self.adjacency_list.get(&current_node_id) {
                for edge_id in edge_ids {
                    if let Some(edge) = self.edges.get(edge_id) {
                        let next_node = &edge.target_node_id;
                        if !visited.contains_key(next_node) {
                            visited.insert(next_node.clone(), Some(current_node_id.clone()));
                            queue.push_back(next_node.clone());
                        }
                    }
                }
            }
        }
        None
    }

    /// Reconstruct path from visited map
    fn reconstruct_path(
        &self,
        visited: &HashMap<String, Option<String>>,
        start_node_id: &str,
        end_node_id: &str,
    ) -> Vec<String> {
        let mut path = Vec::new();
        let mut current = end_node_id.to_string();

        while current != start_node_id {
            path.push(current.clone());
            current = visited[&current].clone().unwrap();
        }
        path.push(start_node_id.to_string());
        path.reverse();
        path
    }

    /// Get all possible paths from a node (useful for validation)
    pub fn get_all_paths(&self, start_node_id: &str) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        let mut visited = HashMap::new();
        let mut current_path = Vec::new();

        self.dfs_paths(
            start_node_id,
            &mut visited,
            &mut current_path,
            &mut paths
        );

        paths
    }

    fn dfs_paths(
        &self,
        current_node_id: &str,
        visited: &mut HashMap<String, bool>,
        current_path: &mut Vec<String>,
        all_paths: &mut Vec<Vec<String>>
    ) {
        visited.insert(current_node_id.to_string(), true);
        current_path.push(current_node_id.to_string());

        if let Some(edge_ids) = self.adjacency_list.get(current_node_id) {
            for edge_id in edge_ids {
                if let Some(edge) = self.edges.get(edge_id) {
                    let next_node = &edge.target_node_id;
                    if !visited.contains_key(next_node) {
                        self.dfs_paths(next_node, visited, current_path, all_paths);
                    }
                }
            }
        } else {
            // We've reached an end node, save the path
            all_paths.push(current_path.clone());
        }

        current_path.pop();
        visited.remove(current_node_id);
    }

    fn has_cycle(
        &self,
        node_id: &str,
        visited: &mut HashMap<String, bool>,
        path: &mut Vec<String>
    ) -> bool {
        visited.insert(node_id.to_string(), true);
        path.push(node_id.to_string());

        if let Some(edge_ids) = self.adjacency_list.get(node_id) {
            for edge_id in edge_ids {
                if let Some(edge) = self.edges.get(edge_id) {
                    let next_node = &edge.target_node_id;
                    if !visited.contains_key(next_node) {
                        if self.has_cycle(next_node, visited, path) {
                            return true;
                        }
                    } else if path.contains(&next_node) {
                        path.push(next_node.to_string());
                        return true;
                    }
                }
            }
        }

        path.pop();
        false
    }

    fn get_start_nodes(&self) -> Vec<String> {
        let mut incoming_edges: HashMap<String, usize> = self.nodes.keys()
            .map(|node_id| (node_id.clone(), 0))
            .collect();

        for edge in self.edges.values() {
            if let Some(count) = incoming_edges.get_mut(&edge.target_node_id) {
                *count += 1;
            }
        }

        incoming_edges.into_iter()
            .filter(|(_, count)| *count == 0)
            .map(|(node_id, _)| node_id)
            .collect()
    }

    fn get_reachable_nodes(&self, start_node: &str) -> Vec<String> {
        let mut reachable = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();

        queue.push_back(start_node.to_string());
        visited.insert(start_node.to_string(), true);

        while let Some(node_id) = queue.pop_front() {
            reachable.push(node_id.clone());

            if let Some(edge_ids) = self.adjacency_list.get(&node_id) {
                for edge_id in edge_ids {
                    if let Some(edge) = self.edges.get(edge_id) {
                        let next_node = &edge.target_node_id;
                        if !visited.contains_key(next_node) {
                            visited.insert(next_node.clone(), true);
                            queue.push_back(next_node.clone());
                        }
                    }
                }
            }
        }

        reachable
    }
    
    /// Validate the graph
    pub fn validate(&self) -> Result<(), FlowError> {
        // Check for cycles
        let mut visited = HashMap::new();
        let mut path = Vec::new();

        for node_id in self.nodes.keys() {
            if !visited.contains_key(node_id) {
                if self.has_cycle(node_id, &mut visited, &mut path) {
                    return Err(FlowError::CycleDetected(path));
                }
            }
        }

        // Check for unreachable nodes
        let start_nodes = self.get_start_nodes();
        if start_nodes.is_empty() {
            return Err(FlowError::NoStartNodes);
        }

        let reachable = self.get_reachable_nodes(&start_nodes[0]);
        let unreachable: Vec<_> = self.nodes.keys()
            .filter(|node_id| !reachable.contains(*node_id))
            .cloned()
            .collect();

        if !unreachable.is_empty() {
            return Err(FlowError::UnreachableNodes(unreachable));
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum FlowError {
    NodeNotFound(String),
    DuplicateNode(String),
    DuplicateEdge(String),
    CycleDetected(Vec<String>),
    NoStartNodes,
    UnreachableNodes(Vec<String>),
}

// Example usage:
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_graph_traversal() {
        let mut graph = FlowGraph::new();

        // Create nodes
        let welcome_node = Node::new(
            "welcome".to_string(),
            "message".to_string(),
            "Welcome".to_string(),
            "Welcome message".to_string(),
        );

        let help_node = Node::new(
            "help".to_string(),
            "message".to_string(),
            "Help".to_string(),
            "Help message".to_string(),
        );

        // Add nodes to graph
        graph.add_node(welcome_node).unwrap();
        graph.add_node(help_node).unwrap();

        // Create and add edge
        let edge = Edge::new(
            "welcome_to_help".to_string(),
            "welcome".to_string(),
            "help".to_string(),
        );

        graph.add_edge(edge).unwrap();

        // Test path finding
        let path = graph.find_path("welcome", "help").unwrap();
        assert_eq!(path, vec!["welcome", "help"]);

        // Test next node finding with context
        // let context: NodeContext = NodeContext::new();
        // let next_node: Option<String> = graph.find_next_node("welcome", &context);
        // assert_eq!(next_node, Some("help".to_string()));
    }
}