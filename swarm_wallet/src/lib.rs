use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Core trait for wallet implementations
#[async_trait]
pub trait Wallet: Send + Sync {
    /// Get wallet address
    fn address(&self) -> String;

    /// Sign a message
    async fn sign_message(&self, message: &str) -> Result<String, WalletError>;

    /// Verify a signature
    async fn verify_signature(
        &self,
        message: &str,
        signature: &str,
        address: &str,
    ) -> Result<bool, WalletError>;

    /// Get chain type
    fn chain_type(&self) -> ChainType;
}

/// Supported chain types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainType {
    Evm,
    Solana,
}


/// Error types for wallet operations
#[derive(Debug, Error)]
pub enum WalletError {
    #[error("Wallet Key error: {0}")]
    KeyError(String),
    #[error("Wallet Signing error: {0}")]
    SigningError(String),
    #[error("Wallet Signing Verify error: {0}")]
    SigningVerifyError(String),
    #[error("Wallet Chain error: {0}")]
    ChainError(String),
}

pub mod evm;
pub mod solana;