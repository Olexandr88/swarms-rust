use swarm_agent::Agent;

/// Implements a pyramid swarm where agents are arranged in a hierarchical structure
pub async fn pyramid_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    // Calculate pyramid levels
    let mut level = 0;
    let mut level_size = 1;
    let mut total_agents = 0;
    while total_agents + level_size <= agents.len() {
        total_agents += level_size;
        level += 1;
        level_size += 1;
    }

    if level == 0 {
        return Err("Not enough agents for pyramid structure".to_string());
    }

    // Process tasks through pyramid levels
    let mut task_iter = tasks.iter().cycle();
    let mut agent_idx = 0;

    for current_level in 0..level {
        let agents_in_level = current_level + 1;

        for _ in 0..agents_in_level {
            if agent_idx < agents.len() {
                if let Some(task) = task_iter.next() {
                    match agents[agent_idx].run(task).await {
                        Ok(response) => results.push(response),
                        Err(e) => return Err(format!("Agent error: {}", e)),
                    }
                }
                agent_idx += 1;
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
    async fn test_pyramid_swarm() {
        let config = AgentConfig::default();
        let mut agents = vec![
            MockAgent::new(config.clone()), // Top level
            MockAgent::new(config.clone()), // Second level
            MockAgent::new(config.clone()),         // Second level
        ];

        let tasks = vec![
            "Task 1".to_string(),
            "Task 2".to_string(),
            "Task 3".to_string(),
        ];

        let result = pyramid_swarm(&mut agents, &tasks).await.unwrap();
        assert_eq!(result.len(), 3); // One task per agent in pyramid
    }
}

