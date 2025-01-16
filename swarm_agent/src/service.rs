use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};
use uuid::Uuid;

use crate::{Agent, AgentConfig};

/// Service agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAgentConfig {
    /// Service endpoint URL
    pub endpoint: String,
    /// Optional API key for service authentication
    pub api_key: Option<String>,
    /// Request timeout
    pub timeout: Duration,
    /// Maximum retries for failed requests
    pub max_retries: u32,
    /// Health check endpoint (optional)
    pub health_endpoint: Option<String>,
    /// Health check interval in seconds
    pub health_check_interval: u64
}

impl Default for ServiceAgentConfig {
    fn default() -> Self {
        Self {
            endpoint: String::new(),
            api_key: None,
            timeout: Duration::from_secs(30),
            max_retries: 3,
            health_endpoint: None,
            health_check_interval: 60,
        }
    }
}

/// Error types for service agent operations
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Service request failed: {0}")]
    RequestFailed(String),
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    #[error("Identity verification failed: {0}")]
    IdentityVerificationFailed(String),
    #[error("Task execution failed: {0}")]
    TaskExecutionError(String),
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    #[error("Initialization error: {0}")]
    InitializationError(String),
}


/// Extended agent trait for service-based implementations
#[async_trait]
pub trait ServiceAgent: Agent {
    /// Get service configuration
    fn service_config(&self) -> &ServiceAgentConfig;

    /// Check service health
    async fn health_check(&self) -> Result<bool, ServiceError>;

    /// Sign agent output for verification
    async fn sign_message(&self, output: &str) -> Result<String, ServiceError>;

    /// Verify signed output from another agent
    async fn verify_message(
        &self,
        output: &str,
        signature: &str,
        agent_address: &str,
    ) -> Result<bool, ServiceError>;
}