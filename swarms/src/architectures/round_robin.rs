//! Round Robin swarm architecture implementation
//!
//! This module implements the Round Robin swarm pattern where tasks are distributed
//! evenly among agents in a circular fashion, similar to the Python implementation
//! in docs.swarms.world.

use swarm_agent::Agent;

/// Implements a Round Robin swarm where tasks are distributed evenly among agents
pub async fn round_robin_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    // Distribute tasks evenly among agents in round-robin fashion
    for (task_idx, task) in tasks.iter().enumerate() {
        let agent_idx = task_idx % agents.len();
        match agents[agent_idx].run(task).await {
            Ok(response) => results.push(response),
            Err(e) => return Err(format!("Agent error: {}", e)),
        }
    }

    Ok(results)
}


#[cfg(test)]
mod tests {
    use swarm_agent::{AgentConfig, mock::MockAgent};
    use super::*;

    #[tokio::test]
    async fn test_round_robin_swarm() {
        let config = AgentConfig::default();
        let mut agents = vec![
            MockAgent::new(
                AgentConfig {
                    name: "Agent1".to_string(),
                    ..config.clone()
                }
            ),
            MockAgent::new(
                AgentConfig {
                    name: "Agent2".to_string(),
                    ..config
                }
            ),
        ];

        let tasks = vec![
            "Task1".to_string(),
            "Task2".to_string(),
            "Task3".to_string(),
        ];

        let results = round_robin_swarm(&mut agents, &tasks).await.unwrap();
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_round_robin_empty_input() {
        let mut empty_agents: Vec<MockAgent> = vec![];
        let tasks = vec!["Task".to_string()];
        assert!(round_robin_swarm(&mut empty_agents, &tasks).await.is_err());

        let config = AgentConfig::default();
        let mut agents = vec![MockAgent::new(config)];
        let empty_tasks: Vec<String> = vec![];
        assert!(round_robin_swarm(&mut agents, &empty_tasks).await.is_err());
    }

}

