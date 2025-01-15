use swarm_agent::Agent;

/// Implements a prime swarm where agents are arranged according to prime number indices
pub async fn prime_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    // First few prime numbers
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

    // Process tasks using prime number indices
    let mut task_iter = tasks.iter();
    for &prime in &primes {
        if prime < agents.len() {
            if let Some(task) = task_iter.next() {
                match agents[prime].run(task).await {
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
    async fn test_prime_swarm() {
        let config = AgentConfig::default();
        let mut agents: Vec<_> = (0..10)
            .map(|_| MockAgent::new(config.clone()))
            .collect();

        let tasks = vec![
            "Task 1".to_string(),
            "Task 2".to_string(),
            "Task 3".to_string(),
        ];

        let result = prime_swarm(&mut agents, &tasks).await.unwrap();
        assert!(!result.is_empty());
        // Should process tasks at prime indices (2, 3, 5, 7)
        assert!(result.len() <= tasks.len());
    }
}

