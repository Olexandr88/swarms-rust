use swarm_agent::{Agent, AgentConfig, mock::MockAgent};
use crate::{Swarm, SwarmConfig};
use uuid::Uuid;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Basic implementation of a swarm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockSwarm<A: Agent> {
    config: SwarmConfig,
    agents: Vec<A>,
    conversation_history: Vec<(String, String)>, // (agent_name, message)
}

#[async_trait]
impl<A: Agent> Swarm for MockSwarm<A> {
    type AgentType = A;

    fn config(&self) -> &SwarmConfig {
        &self.config
    }

    fn agents(&self) -> &[A] {
        &self.agents
    }

    fn agents_mut(&mut self) -> &mut [A] {
        &mut self.agents
    }

    fn add_agent(&mut self, agent: A) {
        self.agents.push(agent);
    }

    fn remove_agent(&mut self, agent_id: Uuid) -> Option<A> {
        if let Some(pos) = self.agents.iter().position(|a| a.id() == agent_id) {
            Some(self.agents.remove(pos))
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.conversation_history.clear();
    }

    async fn run(&mut self, task: &str) -> Result<Vec<String>, String> {
        let mut results = Vec::new();
        let agent_count = self.agents.len();

        for i in 0..agent_count {
            let response = self.agents[i].run(task).await?;
            let agent_name = self.agents[i].name().to_string();
            self.log_conversation(agent_name, response.clone());
            results.push(response);
        }

        Ok(results)
    }
}


impl<A: Agent> MockSwarm<A> {
    pub fn new(config: SwarmConfig, agents: Vec<A>) -> Self {
        Self {
            config,
            agents,
            conversation_history: Vec::new(),
        }
    }

    pub fn with_agents(agents: Vec<A>) -> Self {
        Self::new(SwarmConfig::default(), agents)
    }

    pub fn log_conversation(&mut self, agent_name: String, message: String) {
        if self.config.logging {
            self.conversation_history.push((agent_name, message));
        }
    }

    pub fn conversation_history(&self) -> &[(String, String)] {
        &self.conversation_history
    }

    pub fn get_agent_by_name(&self, name: &str) -> Option<&A> {
        self.agents.iter().find(|agent| agent.name() == name)
    }

    pub fn get_agent_by_id(&self, id: Uuid) -> Option<&A> {
        self.agents.iter().find(|agent| agent.id() == id)
    }
}


#[tokio::test]
async fn test_base_swarm() {
    let agent_config = AgentConfig::default();
    let agents = vec![
        MockAgent::new(agent_config.clone()),
        MockAgent::new(agent_config),
    ];

    let mut swarm = MockSwarm::with_agents(agents);
    let results = swarm.run("test task").await.unwrap();
    //println!("results: {:?} {}", results, results.len());

    assert_eq!(results.len(), 2);
    assert_eq!(swarm.conversation_history().len(), 2);
}

#[test]
fn test_swarm_agent_management() {
    let agent_config = AgentConfig::default();
    let agent = MockAgent::new(agent_config);
    let agent_id = agent.id();

    let mut swarm = MockSwarm::with_agents(vec![agent]);

    // Test agent retrieval
    assert!(swarm.get_agent_by_id(agent_id).is_some());

    // Test agent removal
    let removed = swarm.remove_agent(agent_id);
    assert!(removed.is_some());
    assert!(swarm.agents().is_empty());
}

