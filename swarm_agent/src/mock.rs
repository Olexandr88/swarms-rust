//! Mock agent implementation for testing and development

use super::{Agent, AgentConfig};
use swarm_tool::Tool;
use swarm_wallet::{Wallet, WalletError, evm::EvmWallet};
use async_trait::async_trait;
use swarm_llm::{LLM, mock::MockLLM};
use uuid::Uuid;

/// Mock agent implementation
pub struct MockAgent {
    config: AgentConfig,
    llm: MockLLM,
    wallet: Option<Box<dyn Wallet>>,
}

impl MockAgent {
    /// Create a new mock agent
    pub fn new(config: AgentConfig) -> Self {
        Self {
            config,
            llm: MockLLM::new(),
            wallet: Some(Box::new(EvmWallet::new().unwrap()))
        }
    }

    /// Add wallet-based identity verification
    pub fn with_wallet(mut self, wallet: Box<dyn Wallet>) -> Self {
        self.wallet = Some(wallet);
        self
    }

    /// Configure mock responses
    pub fn with_responses<I, S>(mut self, responses: I) -> Self
    where
        I: IntoIterator<Item = (S, S)>,
        S: Into<String>,
    {
        let mut llm = MockLLM::new();
        for (pattern, response) in responses {
            llm.add_pattern(pattern, response);
        }
        self.llm = llm;
        self
    }
}

#[async_trait]
impl Agent for MockAgent {
    async fn run(&mut self, task: &str) -> Result<String, String> {
        // Generate response using MockLLM
        let response = self.llm.generate(task).await.map_err(|e| e.to_string())?;

        // Sign response if identity verification is enabled
        if let Some(wallet) = &self.wallet {
            let signature = wallet
                .sign_message(&response)
                .await
                .map_err(|e| e.to_string())?;

            Ok(format!(
                "Response: {}\nSignature: {}\nAddress: {}",
                response,
                signature,
                wallet.address()
            ))
        } else {
            Ok(response)
        }
    }

    fn id(&self) -> Uuid {
        self.config.id
    }

    fn name(&self) -> &str {
        &self.config.name
    }

    fn system_prompt(&self) -> &str {
        &self.config.system_prompt
    }

    fn add_tool(&mut self, _tool: Box<dyn Tool>) {
        // Tools not implemented in mock agent
    }

    fn config(&self) -> &AgentConfig {
        &self.config
    }
}

impl MockAgent {
    fn address(&self) -> Option<String> {
        match &self.wallet {
            Some(wallet) => Some(wallet.address()),
            None => None,
        }
    }

    async fn sign_message(&self, message: &str) -> Result<String, WalletError> {
        if let Some(wallet) = &self.wallet {
            wallet
                .sign_message(message)
                .await
                .map_err(|e| WalletError::SigningError(e.to_string()))
        } else {
            Err(WalletError::KeyError(
                "No wallet configured".to_string(),
            ))
        }
    }

    async fn verify_signature(
        &self,
        message: &str,
        signature: &str,
        address: &str,
    ) -> Result<bool, WalletError> {
        if let Some(wallet) = &self.wallet {
            wallet
                .verify_signature(message, signature, address)
                .await
                .map_err(|e| WalletError::SigningVerifyError(e.to_string()))
        } else {
            Err(WalletError::KeyError(
                "No wallet configured".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use swarm_wallet::evm::EvmWallet;

    use crate::{mock::MockAgent, Agent, AgentConfig};

    
    #[tokio::test]
    async fn test_mock_agent_run() {
        let config = AgentConfig::default();
        let mut agent = MockAgent::new(config);

        let response = agent.run("mockagent task").await.unwrap();
        //println!("mockagent response: {}", response);
        assert!(response.contains("MockLLM response"));
    }

    #[tokio::test]
    async fn test_mock_agent_with_responses() {
        let config = AgentConfig::default();
        let mut agent = MockAgent::new(config)
            .with_responses(vec![("hello", "Hello, world!"), ("bye", "Goodbye!")]);

        let response = agent.run("say hello").await.unwrap();
        assert!(response.contains("Hello, world!"));
    }

    #[tokio::test]
    async fn test_mock_agent_with_wallet() {
        let config = AgentConfig::default();
        let wallet = EvmWallet::new().unwrap();
        let mut agent = MockAgent::new(config).with_wallet(Box::new(wallet));

        let response = agent.run("mockagent task").await.unwrap();
        assert!(response.contains("Response:"));
        assert!(response.contains("Signature:"));
        assert!(response.contains("Address:"));
    }
}
