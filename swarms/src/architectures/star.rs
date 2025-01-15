use swarm_agent::Agent;

/// Implements a star swarm where a central agent processes tasks first, followed by others
pub async fn star_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.len() < 2 {
        return Err("Star swarm requires at least 2 agents (1 center + 1 peripheral)".to_string());
    }
    if tasks.is_empty() {
        return Err("Tasks list cannot be empty.".to_string());
    }

    // Central agent processes each task first
    for task in tasks {
        // Central agent (first in the list) processes the task
        match agents[0].run(task).await {
            Ok(response) => results.push(response),
            Err(e) => return Err(format!("Central agent error: {}", e)),
        }

        // Other agents process the same task
        for agent in agents[1..].iter_mut() {
            match agent.run(task).await {
                Ok(response) => results.push(response),
                Err(e) => return Err(format!("Peripheral agent error: {}", e)),
            }
        }
    }

    Ok(results)
}



#[cfg(test)]
mod tests {
    use swarm_agent::{AgentConfig, mock::MockAgent};
    use super::*;

    #[tokio::test]
    async fn test_star_swarm() {
        let config = AgentConfig::default();
        
        let mut agents = vec![
            MockAgent::new(config.clone()), // Central agent
            MockAgent::new(config.clone()), // Peripheral agent 1
            MockAgent::new(config),         // Peripheral agent 2
        ];

        let tasks = vec!["Task 1".to_string()];

        let result = star_swarm(&mut agents, &tasks).await.unwrap();
        assert_eq!(result.len(), 3); // 1 central + 2 peripheral agents * 1 task
    }
}
