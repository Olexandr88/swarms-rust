use swarm_agent::Agent;

/// Implements a Fibonacci swarm where agents are arranged according to Fibonacci sequence
pub async fn fibonacci_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    // Generate Fibonacci sequence up to agents.len()
    let mut fib = vec![1, 1];
    while fib.last().unwrap() < &agents.len() {
        let next = fib[fib.len() - 1] + fib[fib.len() - 2];
        if next >= agents.len() {
            break;
        }
        fib.push(next);
    }

    // Process tasks according to Fibonacci pattern
    let mut task_iter = tasks.iter();
    for &fib_num in &fib {
        if let Some(task) = task_iter.next() {
            if fib_num < agents.len() {
                match agents[fib_num].run(task).await {
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
    async fn test_fibonacci_swarm() {
        let config = AgentConfig::default();
        let mut agents: Vec<_> = (0..5)
            .map(|_| MockAgent::new(config.clone()))
            .collect();

        let tasks = vec![
            "Task 1".to_string(),
            "Task 2".to_string(),
            "Task 3".to_string(),
        ];

        let result = fibonacci_swarm(&mut agents, &tasks).await.unwrap();
        assert!(!result.is_empty());
        // Should process tasks at indices 1, 1, 2, 3 (Fibonacci sequence)
        assert!(result.len() <= tasks.len());
    }

}
