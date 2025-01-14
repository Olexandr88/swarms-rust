use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Error type for LLM operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LLMError {
    ApiError(String),
    ConfigError(String),
    NetworkError(String),
    AuthError(String),
    EnvError(String),
}

impl fmt::Display for LLMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LLMError::ApiError(msg) => write!(f, "API error: {}", msg),
            LLMError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            LLMError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            LLMError::AuthError(msg) => write!(f, "Authentication error: {}", msg),
            LLMError::EnvError(msg) => write!(f, "Environment error: {}", msg),
        }
    }
}

impl From<LLMError> for String {
    fn from(err: LLMError) -> Self {
        err.to_string()
    }
}

/// Common interface for LLM configurations
pub trait LLMConfig: std::fmt::Debug + Send + Sync {
    fn model_name(&self) -> &str;
    fn temperature(&self) -> f32;
    fn max_tokens(&self) -> usize;
    fn mock_responses(&self) -> bool;
    fn clone_box(&self) -> Box<dyn LLMConfig>;
}

impl Clone for Box<dyn LLMConfig> {
    fn clone(&self) -> Box<dyn LLMConfig> {
        self.clone_box()
    }
}

/// Core trait for LLM providers
#[async_trait]
pub trait LLM: Send + Sync {
    /// Generate text from a prompt
    async fn generate(&self, prompt: &str) -> Result<String, LLMError>;

    /// Get the model name
    fn model_name(&self) -> &str;

    /// Get the provider name
    fn provider_name(&self) -> &str;
}




pub mod mock;
pub mod openai;