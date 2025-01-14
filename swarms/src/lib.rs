use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use swarm_agent::Agent;

use uuid::Uuid;

/// Configuration for a swarm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmConfig {
    pub name: String,
    pub description: Option<String>,
    pub max_loops: usize,
    pub autosave: bool,
    pub logging: bool,
}

impl Default for SwarmConfig {
    fn default() -> Self {
        Self {
            name: "default-swarm".to_string(),
            description: None,
            max_loops: 200,
            autosave: false,
            logging: true,
        }
    }
}

/// Core trait defining swarm capabilities
#[async_trait]
pub trait Swarm {
    type AgentType: Agent;

    /// Get the swarm's configuration
    fn config(&self) -> &SwarmConfig;

    /// Get a reference to the swarm's agents
    fn agents(&self) -> &[Self::AgentType];

    /// Get a mutable reference to the swarm's agents
    fn agents_mut(&mut self) -> &mut [Self::AgentType];

    /// Add an agent to the swarm
    fn add_agent(&mut self, agent: Self::AgentType);

    /// Remove an agent from the swarm by ID
    fn remove_agent(&mut self, agent_id: Uuid) -> Option<Self::AgentType>;

    /// Reset all agents in the swarm
    fn reset(&mut self);

    /// Run the swarm on a task
    async fn run(&mut self, task: &str) -> Result<Vec<String>, String>;
}

pub mod mock;
pub mod architectures;