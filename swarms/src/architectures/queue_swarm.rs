//! Queue Swarm architecture implementation
//!
//! This module implements the Queue Swarm pattern where tasks are processed in a
//! FIFO (First In, First Out) order, similar to the Python implementation in
//! docs.swarms.world.

use swarm_agent::Agent;

/// Implements a Queue Swarm where tasks are processed in FIFO order
pub async fn queue_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    // Process tasks in FIFO order using available agents
    let mut agent_idx = 0;
    for task in tasks {
        // Get next available agent (round-robin if needed)
        if agent_idx >= agents.len() {
            agent_idx = 0;
        }

        // Process task with current agent
        match agents[agent_idx].run(task).await {
            Ok(response) => results.push(response),
            Err(e) => return Err(format!("Agent error: {}", e)),
        }

        agent_idx += 1;
    }

    Ok(results)
}




#[cfg(test)]
mod tests {
    use swarm_agent::{AgentConfig, mock::MockAgent};
    use super::*;

    #[tokio::test]
    async fn test_queue_swarm() {
        let config = AgentConfig::default();
        let mut agents = vec![
            MockAgent::new(
                AgentConfig {
                    name: "QueueProcessor1".to_string(),
                    ..config.clone()
                }
            ),
            MockAgent::new(
                AgentConfig {
                    name: "QueueProcessor2".to_string(),
                    ..config
                }
            ),
        ];

        let tasks = vec![
            "Task1".to_string(),
            "Task2".to_string(),
            "Task3".to_string(),
        ];

        let results = queue_swarm(&mut agents, &tasks).await.unwrap();
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_queue_swarm_empty_input() {
        let mut empty_agents: Vec<MockAgent> = vec![];
        let tasks = vec!["Task".to_string()];
        assert!(queue_swarm(&mut empty_agents, &tasks).await.is_err());

        let config = AgentConfig::default();
        let mut agents = vec![MockAgent::new(config)];
        let empty_tasks: Vec<String> = vec![];
        assert!(queue_swarm(&mut agents, &empty_tasks).await.is_err());
    }

}
