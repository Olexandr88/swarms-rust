use swarm_agent::Agent;

/// Implements a geometric swarm where agents are arranged according to geometric progression
pub async fn geometric_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    let ratio: f32 = 2.0; // Common ratio for geometric progression
    let mut task_iter = tasks.iter();

    // Process tasks using geometric progression indices
    for i in 0..agents.len() {
        let index = (ratio.powi(i as i32)) as usize;
        if index < agents.len() {
            if let Some(task) = task_iter.next() {
                match agents[index].run(task).await {
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
    async fn test_geometric_swarm() {
        let config = AgentConfig::default();
        let mut agents: Vec<_> = (0..8)
            .map(|_| MockAgent::new(config.clone()))
            .collect();

        let tasks = vec![
            "Task 1".to_string(),
            "Task 2".to_string(),
            "Task 3".to_string(),
        ];

        let result = geometric_swarm(&mut agents, &tasks).await.unwrap();
        assert!(!result.is_empty());
        // Should process tasks at indices following geometric progression (1, 2, 4)
        assert!(result.len() <= tasks.len());
    }
}
