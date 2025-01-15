use swarm_agent::Agent;

/// Implements a grid swarm where agents are arranged in a square grid pattern
pub async fn grid_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    // Calculate grid size (assuming square grid)
    let grid_size = (agents.len() as f64).sqrt() as usize;
    if grid_size * grid_size != agents.len() {
        return Err("Number of agents must form a perfect square.".to_string());
    }

    let mut task_iter = tasks.iter();
    for i in 0..grid_size {
        for j in 0..grid_size {
            if let Some(task) = task_iter.next() {
                match agents[i * grid_size + j].run(task).await {
                    Ok(response) => results.push(response),
                    Err(e) => return Err(format!("Agent error: {}", e)),
                }
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
    async fn test_grid_swarm() {
        let config = AgentConfig::default();
        let mut agents: Vec<_> = (0..4)
            .map(|_| MockAgent::new(config.clone()))
            .collect();

        let tasks = vec![
            "Task 1".to_string(),
            "Task 2".to_string(),
            "Task 3".to_string(),
            "Task 4".to_string(),
        ];

        let result = grid_swarm(&mut agents, &tasks).await.unwrap();
        assert_eq!(result.len(), 4); // 2x2 grid, 4 tasks
    }

    #[tokio::test]
    async fn test_grid_swarm_invalid_size() {
        let config = AgentConfig::default();
        let mut agents: Vec<_> = (0..3)
            .map(|_| MockAgent::new(config.clone()))
            .collect();

        let tasks = vec!["Task 1".to_string()];

        let result = grid_swarm(&mut agents, &tasks).await;
        assert!(result.is_err()); // 3 agents can't form a square grid
    }

}