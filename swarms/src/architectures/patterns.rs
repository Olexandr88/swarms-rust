//! Mathematical and pattern-based swarm architectures

use swarm_agent::Agent;
use std::f64::consts::PI;

/// Power-law distribution swarm
pub async fn power_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    let agents_len = agents.len();
    for (i, agent) in agents.iter_mut().enumerate() {
        let power_idx = 2_u32.pow(i as u32) as usize;
        if power_idx < agents_len && !tasks.is_empty() {
            match agent.run(&tasks[0]).await {
                Ok(response) => results.push(response),
                Err(e) => return Err(format!("Agent error: {}", e)),
            }
        }
    }

    Ok(results)
}

/// Logarithmic distribution swarm
pub async fn log_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    let agents_len = agents.len();
    for (i, agent) in agents.iter_mut().enumerate() {
        let log_idx = ((i + 1) as f64).ln() as usize;
        if log_idx < agents_len && !tasks.is_empty() {
            match agent.run(&tasks[0]).await {
                Ok(response) => results.push(response),
                Err(e) => return Err(format!("Agent error: {}", e)),
            }
        }
    }

    Ok(results)
}

/// Exponential distribution swarm
pub async fn exponential_swarm<A: Agent>(
    agents: &mut [A],
    tasks: &[String],
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() || tasks.is_empty() {
        return Err("Agents and tasks lists cannot be empty.".to_string());
    }

    let agents_len = agents.len();
    for (i, agent) in agents.iter_mut().enumerate() {
        let exp_idx = ((i as f64) * 0.5).exp() as usize;
        if exp_idx < agents_len && !tasks.is_empty() {
            match agent.run(&tasks[0]).await {
                Ok(response) => results.push(response),
                Err(e) => return Err(format!("Agent error: {}", e)),
            }
        }
    }

    Ok(results)
}

/// Staircase pattern swarm
pub async fn staircase_swarm<A: Agent>(
    agents: &mut [A],
    task: &str,
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() {
        return Err("Agents list cannot be empty.".to_string());
    }

    let step_size = agents.len() / 5;
    if step_size == 0 {
        return Err("Not enough agents for staircase pattern".to_string());
    }

    for i in 0..agents.len() {
        let step_idx = (i / step_size) * step_size;
        if step_idx < agents.len() {
            match agents[step_idx].run(task).await {
                Ok(response) => results.push(response),
                Err(e) => return Err(format!("Agent error: {}", e)),
            }
        }
    }

    Ok(results)
}

/// Sigmoid pattern swarm
pub async fn sigmoid_swarm<A: Agent>(agents: &mut [A], task: &str) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() {
        return Err("Agents list cannot be empty.".to_string());
    }

    for i in 0..agents.len() {
        let sigmoid = 1.0 / (1.0 + (-((i as f64) - (agents.len() as f64 / 2.0))).exp());
        let idx = (sigmoid * (agents.len() as f64 - 1.0)) as usize;

        match agents[idx].run(task).await {
            Ok(response) => results.push(response),
            Err(e) => return Err(format!("Agent error: {}", e)),
        }
    }

    Ok(results)
}

/// Sinusoidal pattern swarm
pub async fn sinusoidal_swarm<A: Agent>(
    agents: &mut [A],
    task: &str,
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();

    if agents.is_empty() {
        return Err("Agents list cannot be empty.".to_string());
    }

    for i in 0..agents.len() {
        let sin_val = ((i as f64) * 2.0 * PI / (agents.len() as f64)).sin();
        let idx = ((sin_val + 1.0) / 2.0 * (agents.len() as f64 - 1.0)) as usize;

        match agents[idx].run(task).await {
            Ok(response) => results.push(response),
            Err(e) => return Err(format!("Agent error: {}", e)),
        }
    }

    Ok(results)
}






#[cfg(test)]
mod tests {
    use swarm_agent::{AgentConfig, mock::MockAgent};
    use super::*;

    #[tokio::test]
    async fn test_pattern_swarms() {
        let config = AgentConfig::default();
        let mut agents: Vec<_> = (0..5)
            .map(|_| MockAgent::new(config.clone()))
            .collect();

        let tasks = vec!["Test task".to_string()];

        // Test each pattern
        let patterns = [
            power_swarm(&mut agents, &tasks).await,
            log_swarm(&mut agents, &tasks).await,
            exponential_swarm(&mut agents, &tasks).await,
        ];

        for pattern in patterns {
            let result = pattern.unwrap();
            assert!(!result.is_empty());
        }

        // Test single task patterns
        let single_task = "Single test task";
        let single_patterns = [
            staircase_swarm(&mut agents, single_task).await,
            sigmoid_swarm(&mut agents, single_task).await,
            sinusoidal_swarm(&mut agents, single_task).await,
        ];

        for pattern in single_patterns {
            let result = pattern.unwrap();
            assert!(!result.is_empty());
        }
    }

}