use uuid::Uuid;
use async_trait::async_trait;
use swarm_tool::Tool;
use swarm_llm::{LLMConfig, mock::MockConfig};

/// Core trait defining agent capabilities
#[async_trait]
pub trait Agent: Send + Sync {
    /// Run the agent on a given task
    async fn run(&mut self, task: &str) -> Result<String, String>;

    /// Get the agent's unique identifier
    fn id(&self) -> Uuid;

    /// Get the agent's name
    fn name(&self) -> &str;

    /// Get the agent's system prompt
    fn system_prompt(&self) -> &str;

    /// Add a tool to the agent
    fn add_tool(&mut self, tool: Box<dyn Tool>);

    /// Get the agent's configuration
    fn config(&self) -> &AgentConfig;
}

/// Configuration for an agent
#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub id: Uuid,
    pub name: String,
    pub system_prompt: String,
    pub max_loops: usize,
    pub llm_config: Box<dyn LLMConfig>, // Use Box for trait objects
    pub stopping_condition: Option<String>,
    pub test_mode: bool,
    pub autosave: bool, // Matches Python: autosave=True/False
    pub cloud_enabled: bool,
    pub cloud_endpoint: Option<String>,
    pub cloud_api_key: Option<String>,
    pub distributed: bool,          // Enable distributed processing
    pub wallet_enabled: bool,       // Enable wallet-based identity verification
    pub wallet_key: Option<String>, // Optional wallet private key
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "default-agent".to_string(),
            system_prompt: "You are a helpful AI assistant.".to_string(),
            max_loops: 1,
            llm_config: Box::new(MockConfig::default()),
            stopping_condition: None,
            test_mode: cfg!(test),
            autosave: false, // Default to false, enable explicitly when needed
            cloud_enabled: false,
            cloud_endpoint: None,
            cloud_api_key: None,
            distributed: false,    // Default to local processing
            wallet_enabled: false, // Default to disabled
            wallet_key: None,
        }
    }
}

pub mod mock;