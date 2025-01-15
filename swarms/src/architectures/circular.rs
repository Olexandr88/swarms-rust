use swarm_agent::Agent;

/// Implements a circular swarm where agents pass tasks in a circular manner.
pub async fn circular_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    // Ensure we have agents and tasks
    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    // Process each task through all agents in a circular pattern
    for task in tasks {
        for agent in agents.iter_mut() {
            match agent.run(task).await {
                Ok(response) => results.push(response),
                Err(e) => return Err(format!("Agent error: {}", e)),
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
    async fn test_circular_swarm() {
        let config = AgentConfig {
            name: "test-agent".to_string(),
            ..AgentConfig::default()
        };

        let mut agents = vec![
            MockAgent::new(config.clone()),
            MockAgent::new(config.clone()),
        ];

        let tasks = vec!["Task 1".to_string(), "Task 2".to_string()];

        let result = circular_swarm(&mut agents, &tasks).await.unwrap();
        assert_eq!(result.len(), 4); // 2 agents * 2 tasks
    }
}
