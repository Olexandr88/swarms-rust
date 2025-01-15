use swarm_agent::Agent;

/// Implements a mesh swarm where agents work on tasks from a shared queue
pub async fn mesh_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    // Process tasks using all agents in a mesh pattern
    let mut task_queue: Vec<_> = tasks.iter().collect();
    while !task_queue.is_empty() {
        for agent in agents.iter_mut() {
            if let Some(task) = task_queue.first() {
                match agent.run(task).await {
                    Ok(response) => results.push(response),
                    Err(e) => return Err(format!("Agent error: {}", e)),
                }
            }
        }
        if !task_queue.is_empty() {
            task_queue.remove(0);
        }
    }

    Ok(results)
}




#[cfg(test)]
mod tests {
    use swarm_agent::{AgentConfig, mock::MockAgent};
    use super::*;

    #[tokio::test]
    async fn test_mesh_swarm() {
        let config = AgentConfig::default();
        let mut agents = vec![
            MockAgent::new(config.clone()),
            MockAgent::new(config.clone()),
        ];

        let tasks = vec!["Task 1".to_string(), "Task 2".to_string()];

        let result = mesh_swarm(&mut agents, &tasks).await.unwrap();
        assert_eq!(result.len(), 4); // 2 agents * 2 tasks
    }
}
