//! OpenAI API client implementation

use reqwest::Client;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::env;

use crate::{LLM, LLMError, LLMConfig};

/// Configuration for OpenAI API calls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    pub model_name: String,
    pub temperature: f32,
    pub max_tokens: usize,
    pub mock_responses: bool,
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        Self {
            model_name: "gpt-4o-mini".to_string(),
            temperature: 0.1,
            max_tokens: 1000,
            mock_responses: cfg!(test),
        }
    }
}

impl LLMConfig for OpenAIConfig {
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

/// Client for interacting with OpenAI's API
pub struct OpenAIClient {
    api_key: String,
    client: Client,
    config: OpenAIConfig,
}

impl std::fmt::Debug for OpenAIClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAIClient")
            .field("api_key", &"[redacted]")
            .field("config", &self.config)
            .finish()
    }
}

impl OpenAIClient {
    /// Create a new OpenAI client with the given configuration
    pub fn new(config: OpenAIConfig) -> Result<Self, LLMError> {
        // In mock mode, use test key
        if config.mock_responses {
            return Ok(Self {
                api_key: "test-key".to_string(),
                client: Client::new(),
                config,
            });
        }

        // In non-mock mode, require valid API key
        match env::var("OPENAI_API_KEY") {
            Ok(key) if key.is_empty() => Err(LLMError::ApiError("API key is empty".to_string())),
            Ok(key) => Ok(Self {
                api_key: key,
                client: Client::new(),
                config,
            }),
            Err(e) => Err(LLMError::EnvError(e.to_string())),
        }
    }

}
#[async_trait]
impl LLM for OpenAIClient{
    /// Generate a response for the given prompt
    async fn generate(&self, prompt: &str) -> Result<String, LLMError> {
        // Return mock response in test mode
        if self.config.mock_responses {
            return Ok(format!(
                "Mock OpenAI response for prompt: {}\n\nAnalysis:\n- ROTH IRA information\n- Tax implications\n- Investment strategies\n- Regulatory requirements",
                prompt
            ));
        }

        // Validate API key
        if self.api_key.is_empty() {
            return Err(LLMError::ApiError("API key not set".to_string()));
        }

        // Make real API call
        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "model": self.config.model_name,
                "messages": [{"role": "user", "content": prompt}],
                "temperature": self.config.temperature,
                "max_tokens": self.config.max_tokens
            }))
            .send()
            .await
            .map_err(|e| LLMError::NetworkError(e.to_string()))?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| LLMError::NetworkError(e.to_string()))?;

        // Extract response text
        response["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| LLMError::ApiError("Invalid response format".to_string()))
    }
    
    #[doc = " Get the model name"]
    fn model_name(&self) ->  &str {
        todo!()
    }
    
    #[doc = " Get the provider name"]
    fn provider_name(&self) ->  &str {
        todo!()
    }
}




#[cfg(test)]
#[tokio::test]
async fn test_openai_client_creation_without_env() {
    // Save original environment
    let original_key = env::var("OPENAI_API_KEY").ok();
    env::remove_var("OPENAI_API_KEY");

    // Test with mock responses enabled (should work without API key)
    let config = OpenAIConfig {
        mock_responses: true,
        ..OpenAIConfig::default()
    };
    let result = OpenAIClient::new(config);
    assert!(result.is_ok(), "Should succeed with mock_responses=true");
    let client = result.unwrap();
    assert_eq!(client.api_key, "test-key");

    // Test without mock responses (should fail without API key)
    let config = OpenAIConfig {
        mock_responses: false,
        ..OpenAIConfig::default()
    };
    let result = OpenAIClient::new(config);
    assert!(
        result.is_err(),
        "Should fail with mock_responses=false and no API key"
    );
    assert!(
        matches!(result, Err(LLMError::EnvError(_))),
        "Expected EnvError when API key is missing"
    );

    // Test with empty API key
    env::set_var("OPENAI_API_KEY", "");
    let config = OpenAIConfig {
        mock_responses: false,
        ..OpenAIConfig::default()
    };
    let result = OpenAIClient::new(config);
    assert!(result.is_err(), "Should fail with empty API key");
    assert!(
        matches!(result, Err(LLMError::ApiError(_))),
        "Expected ApiError with empty API key"
    );

    // Restore original environment
    if let Some(key) = original_key {
        env::set_var("OPENAI_API_KEY", key);
    } else {
        env::remove_var("OPENAI_API_KEY");
    }
}

#[tokio::test]
async fn test_mock_response_generation() {
    let config = OpenAIConfig {
        mock_responses: true,
        ..OpenAIConfig::default()
    };
    let client = OpenAIClient::new(config).unwrap();

    let response = client.generate("Test OpenAI prompt").await.unwrap();
    assert!(response.contains("Mock OpenAI response"));
    assert!(response.contains("ROTH IRA"));
}
