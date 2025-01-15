//! Graph Swarm architecture implementation
//!
//! This module implements the Graph Swarm pattern where agents are organized in a
//! graph structure with defined connections between nodes, similar to the Python
//! implementation in docs.swarms.world.

use swarm_agent::Agent;
use std::collections::{HashMap, HashSet};

/// Represents a node in the agent graph
#[derive(Debug)]
struct GraphNode {
    connections: HashSet<usize>,
}

/// Implements a Graph Swarm where agents are connected in a network structure
pub async fn graph_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
    adjacency_list: Option<Vec<Vec<usize>>>,
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    // Build graph structure
    let mut nodes: HashMap<usize, GraphNode> = HashMap::new();

    // If no adjacency list provided, create fully connected graph
    let connections = match adjacency_list {
        Some(adj_list) => {
            if adj_list.len() != agents.len() {
                return Err("Adjacency list must match number of agents.".to_string());
            }
            adj_list
        }
        None => {
            // Create fully connected graph by default
            (0..agents.len())
                .map(|i| {
                    (0..agents.len())
                        .filter(|&j| j != i)
                        .collect::<Vec<usize>>()
                })
                .collect()
        }
    };

    // Initialize graph nodes
    for (i, connected_nodes) in connections.iter().enumerate() {
        nodes.insert(
            i,
            GraphNode {
                connections: connected_nodes.iter().copied().collect(),
            },
        );
    }

    // Process tasks through the graph network
    for task in tasks {
        let mut processed = HashSet::new();
        let mut current_nodes: HashSet<usize> = vec![0].into_iter().collect(); // Start with first node

        while !current_nodes.is_empty() {
            let mut next_nodes = HashSet::new();

            // Process current level nodes
            for &node_idx in &current_nodes {
                if processed.contains(&node_idx) {
                    continue;
                }

                // Process task with current agent
                match agents[node_idx].run(task).await {
                    Ok(response) => results.push(response),
                    Err(e) => return Err(format!("Agent error: {}", e)),
                }

                processed.insert(node_idx);

                // Add connected nodes to next level
                if let Some(node) = nodes.get(&node_idx) {
                    next_nodes.extend(&node.connections);
                }
            }

            current_nodes = next_nodes;
        }
    }

    Ok(results)
}







#[cfg(test)]
mod tests {
    use swarm_agent::{AgentConfig, mock::MockAgent};
    use super::*;

    #[tokio::test]
    async fn test_graph_swarm() {
        let config = AgentConfig::default();
        let mut agents = vec![
            MockAgent::new(
                AgentConfig {
                    name: "Node1".to_string(),
                    ..config.clone()
                }
            ),
            MockAgent::new(
                AgentConfig {
                    name: "Node2".to_string(),
                    ..config.clone()
                }
            ),
            MockAgent::new(
                AgentConfig {
                    name: "Node3".to_string(),
                    ..config
                }
            ),
        ];

        let tasks = vec!["Test task".to_string()];
        
        // Test with custom adjacency list (1->2->3 chain)
        let adj_list = vec![
            vec![1],       // Node 0 connected to 1
            vec![2],       // Node 1 connected to 2
            vec![],        // Node 2 has no outgoing connections
        ];

        let results = graph_swarm(&mut agents, &tasks, Some(adj_list))
            .await
            .unwrap();
        assert_eq!(results.len(), 3);

        // Test with default fully connected graph
        let results = graph_swarm(&mut agents, &tasks, None)
            .await
            .unwrap();
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_graph_swarm_empty_input() {
        let mut empty_agents: Vec<MockAgent> = vec![];
        let tasks = vec!["Task".to_string()];
        assert!(graph_swarm(&mut empty_agents, &tasks, None).await.is_err());

        let config = AgentConfig::default();
        let mut agents = vec![MockAgent::new(config)];
        let empty_tasks: Vec<String> = vec![];
        assert!(graph_swarm(&mut agents, &empty_tasks, None).await.is_err());
    }

    #[tokio::test]
    async fn test_graph_swarm_invalid_adjacency() {
        let config = AgentConfig::default();
        let mut agents = vec![MockAgent::new(config)];
        let tasks = vec!["Task".to_string()];
        
        // Test with mismatched adjacency list length
        let invalid_adj_list = vec![vec![1], vec![0]]; // Two nodes for one agent
        assert!(graph_swarm(&mut agents, &tasks, Some(invalid_adj_list))
            .await
            .is_err());
    }

}