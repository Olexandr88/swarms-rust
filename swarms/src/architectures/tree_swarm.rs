//! Tree Swarm architecture implementation
//!
//! This module implements the Tree Swarm pattern where agents are organized in a
//! hierarchical tree structure, similar to the Python implementation in
//! docs.swarms.world.

use swarm_agent::Agent;
use std::collections::{HashMap, HashSet};

/// Represents a node in the agent tree
#[derive(Debug)]
struct TreeNode {
    children: Vec<usize>,
    parent: Option<usize>,
}

/// Implements a Tree Swarm where agents are organized in a hierarchical structure
pub async fn tree_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
    parent_child_pairs: Option<Vec<(usize, usize)>>, // (parent_idx, child_idx)
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    // Build tree structure
    let mut nodes: HashMap<usize, TreeNode> = HashMap::new();

    // If no parent-child pairs provided, create a balanced binary tree
    let relationships = match parent_child_pairs {
        Some(pairs) => {
            if pairs
                .iter()
                .any(|(p, c)| *p >= agents.len() || *c >= agents.len())
            {
                return Err("Invalid parent-child indices.".to_string());
            }
            pairs
        }
        None => {
            // Create balanced binary tree relationships
            (0..agents.len())
                .flat_map(|i| {
                    let left = 2 * i + 1;
                    let right = 2 * i + 2;
                    let mut pairs = Vec::new();
                    if left < agents.len() {
                        pairs.push((i, left));
                    }
                    if right < agents.len() {
                        pairs.push((i, right));
                    }
                    pairs
                })
                .collect()
        }
    };

    // Initialize tree nodes
    for i in 0..agents.len() {
        nodes.insert(
            i,
            TreeNode {
                children: Vec::new(),
                parent: None,
            },
        );
    }

    // Build relationships
    for (parent_idx, child_idx) in relationships {
        if let Some(parent_node) = nodes.get_mut(&parent_idx) {
            parent_node.children.push(child_idx);
        }
        if let Some(child_node) = nodes.get_mut(&child_idx) {
            child_node.parent = Some(parent_idx);
        }
    }

    // Process tasks through the tree hierarchy
    for task in tasks {
        let mut processed = HashSet::new();
        let mut current_level: HashSet<usize> = vec![0].into_iter().collect(); // Start with root node

        while !current_level.is_empty() {
            let mut next_level = HashSet::new();

            // Process current level nodes
            for &node_idx in &current_level {
                if processed.contains(&node_idx) {
                    continue;
                }

                // Process task with current agent
                match agents[node_idx].run(task).await {
                    Ok(response) => results.push(response),
                    Err(e) => return Err(format!("Agent error: {}", e)),
                }

                processed.insert(node_idx);

                // Add child nodes to next level
                if let Some(node) = nodes.get(&node_idx) {
                    next_level.extend(&node.children);
                }
            }

            current_level = next_level;
        }
    }

    Ok(results)
}




#[cfg(test)]
mod tests {
    use swarm_agent::{AgentConfig, mock::MockAgent};
    use super::*;

    #[tokio::test]
    async fn test_tree_swarm() {
        let config = AgentConfig::default();
        let mut agents = vec![
            MockAgent::new(
                AgentConfig {
                    name: "Root".to_string(),
                    ..config.clone()
                }
            ),
            MockAgent::new(
                AgentConfig {
                    name: "LeftChild".to_string(),
                    ..config.clone()
                }
            ),
            MockAgent::new(
                AgentConfig {
                    name: "RightChild".to_string(),
                    ..config
                }
            ),
        ];

        let tasks = vec!["Test task".to_string()];
        
        // Test with custom parent-child relationships
        let relationships = vec![
            (0, 1), // Root -> LeftChild
            (0, 2), // Root -> RightChild
        ];

        let results = tree_swarm(&mut agents, &tasks, Some(relationships))
            .await
            .unwrap();
        assert_eq!(results.len(), 3);

        // Test with default balanced binary tree
        let results = tree_swarm(&mut agents, &tasks, None)
            .await
            .unwrap();
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_tree_swarm_empty_input() {
        let mut empty_agents: Vec<MockAgent> = vec![];
        let tasks = vec!["Task".to_string()];
        assert!(tree_swarm(&mut empty_agents, &tasks, None).await.is_err());

        let config = AgentConfig::default();
        let mut agents = vec![MockAgent::new(config)];
        let empty_tasks: Vec<String> = vec![];
        assert!(tree_swarm(&mut agents, &empty_tasks, None).await.is_err());
    }

    #[tokio::test]
    async fn test_tree_swarm_invalid_relationships() {
        let config = AgentConfig::default();
        let mut agents = vec![MockAgent::new(config)];
        let tasks = vec!["Task".to_string()];
        
        // Test with invalid parent index
        let invalid_relationships = vec![(1, 0)]; // Parent index 1 doesn't exist
        assert!(tree_swarm(&mut agents, &tasks, Some(invalid_relationships))
            .await
            .is_err());
    }

}