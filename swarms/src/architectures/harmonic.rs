use swarm_agent::Agent;

/// Implements a harmonic swarm where agents are arranged according to harmonic series
pub async fn harmonic_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    let mut task_iter = tasks.iter();

    // Process tasks using harmonic series indices (1/1, 1/2, 1/3, ...)
    for i in 1..=agents.len() {
        let index = (agents.len() as f64 / i as f64) as usize;
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
    async fn test_harmonic_swarm() {
        let config = AgentConfig::default();
        let mut agents: Vec<_> = (0..6)
            .map(|_| MockAgent::new(config.clone()))
            .collect();

        let tasks = vec![
            "Task 1".to_string(),
            "Task 2".to_string(),
            "Task 3".to_string(),
        ];

        let result = harmonic_swarm(&mut agents, &tasks).await.unwrap();
        assert!(!result.is_empty());
        // Should process tasks at indices following harmonic series
        assert!(result.len() <= tasks.len());
    }
}

