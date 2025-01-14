//! Mock implementations for testing and examples

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{LLM, LLMError, LLMConfig};
use async_trait::async_trait;


/// Configuration for mock LLM responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockConfig {
    //pub model_name: String,
    pub temperature: f32,
    pub max_tokens: usize,
    pub mock_responses: bool,

    /// Default response when no pattern matches
    pub default_response: String,
    /// Map of prompt patterns to responses
    pub response_patterns: HashMap<String, String>,
    /// Whether to include agent name in response
    pub include_agent_name: bool,
    /// Mock model name
    pub model_name: String,
    /// Mock provider name
    pub provider_name: String,
}

impl Default for MockConfig {
    fn default() -> Self {
        Self {
            temperature: 0.1,
            max_tokens: 1000,
            mock_responses: true,

            default_response: "MockLLM response".to_string(),
            response_patterns: HashMap::new(),
            include_agent_name: true,
            model_name: "mockllm-model".to_string(),
            provider_name: "mockllm-provider".to_string(),
        }
    }
}

impl LLMConfig for MockConfig {
    fn model_name(&self) -> &str {
        &self.model_name
    }

    fn temperature(&self) -> f32 {
        self.temperature
    }

    fn max_tokens(&self) -> usize {
        self.max_tokens
    }

    fn mock_responses(&self) -> bool {
        self.mock_responses
    }
    
    fn clone_box(&self) -> Box<dyn LLMConfig> {
        Box::new(self.clone()) // Clone the current instance and wrap it in a Box
    }
}

impl MockConfig {
    /// Add a response pattern
    pub fn add_pattern<S: Into<String>>(&mut self, pattern: S, response: S) {
        self.response_patterns
            .insert(pattern.into(), response.into());
    }

    /// Get response for prompt
    pub fn get_response(&self, prompt: &str) -> String {
        for (pattern, response) in &self.response_patterns {
            if prompt.contains(pattern) {
                return response.clone();
            }
        }
        self.default_response.clone()
    }
}



/// Mock LLM client for testing and examples
#[derive(Debug, Clone)]
pub struct MockLLM {
    config: MockConfig,
}

impl MockLLM {
    /// Create a new MockLLM with default configuration
    pub fn new() -> Self {
        Self {
            config: MockConfig::default(),
        }
    }

    /// Create a new MockLLM with custom configuration
    pub fn with_config(config: MockConfig) -> Self {
        Self { config }
    }

    /// Add a response pattern
    pub fn add_pattern<S: Into<String>>(&mut self, pattern: S, response: S) {
        self.config.add_pattern(pattern, response);
    }
}

impl Default for MockLLM {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LLM for MockLLM {
    async fn generate(&self, prompt: &str) -> Result<String, LLMError> {
        let response = self.config.get_response(prompt);

        if self.config.include_agent_name {
            Ok(format!("test-agent: {}", response))
        } else {
            Ok(response)
        }
    }

    fn model_name(&self) -> &str {
        &self.config.model_name
    }

    fn provider_name(&self) -> &str {
        &self.config.provider_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_response_generation() {
        let llm = MockLLM::new();
        let response = llm.generate("test prompt").await.unwrap();
        println!("mockllm response: {}", response);
        assert!(response.contains("test-agent"));
        assert!(response.contains("MockLLM response"));
    }

    #[tokio::test]
    async fn test_mock_llm_patterns() {
        let mut config = MockConfig::default();
        config.add_pattern("hello", "Hello, world!");
        config.add_pattern("bye", "Goodbye!");

        let llm = MockLLM::with_config(config);

        let response = llm.generate("say hello").await.unwrap();
        assert!(response.contains("Hello, world!"));

        let response = llm.generate("say bye").await.unwrap();
        assert!(response.contains("Goodbye!"));
    }
}
