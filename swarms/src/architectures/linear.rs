use swarm_agent::Agent;

/// Implements a linear swarm where agents process tasks sequentially
pub async fn linear_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    // Each agent processes one task in sequence
    let mut task_iter = tasks.iter();
    for agent in agents.iter_mut() {
        if let Some(task) = task_iter.next() {
            match agent.run(task).await {
                Ok(response) => results.push(response),
                Err(e) => return Err(format!("Agent error: {}", e)),
            }
        }
    }

    Ok(results)
}

// Tests moved to tests/architectures/linear_test.rs
