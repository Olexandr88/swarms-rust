use super::{Wallet, ChainType, WalletError};
use solana_sdk::{
    signature::{Keypair, Signer as SolanaSigner, Signature},
    pubkey::Pubkey,
    message::Message,
};
use async_trait::async_trait;
use std::str::FromStr;

/// Solana wallet implementation
pub struct SolanaWallet {
    signer: Keypair,
}

impl SolanaWallet {
    /// Create a new random wallet
    #[allow(dead_code)]
    pub fn new() -> Result<Self, WalletError> {
        let signer = Keypair::new();
        Ok(Self { signer })
    }

    /// Create from private key
    #[allow(dead_code)]
    pub fn from_private_key(key: &str) -> Result<Self, WalletError> {
        let key_bytes = bs58::decode(key)
            .into_vec()
            .map_err(|e| WalletError::KeyError(format!("Invalid private key: {}", e)))?;
        let signer = Keypair::from_bytes(&key_bytes)
            .map_err(|e| WalletError::KeyError(format!("Failed to create keypair: {}", e)))?;
        Ok(Self { signer })
    }
}

#[async_trait]
impl Wallet for SolanaWallet {
    fn address(&self) -> String {
        self.signer.pubkey().to_string()
    }

    async fn sign_message(&self, message: &str) -> Result<String, WalletError> {
        let message_bytes = message.as_bytes();
        let signature = self
            .signer
            .try_sign_message(message_bytes)
            .map_err(|e| WalletError::SigningError(format!("Failed to sign message: {}", e)))?;
        Ok(signature.to_string())
    }

    async fn verify_signature(
        &self,
        message: &str,
        signature: &str,
        address: &str,
    ) -> Result<bool, WalletError> {
        let message_bytes = message.as_bytes();
        let signature = Signature::from_str(signature)
            .map_err(|e| WalletError::SigningVerifyError(format!("Invalid signature: {}", e)))?;
        let pubkey = Pubkey::from_str(address)
            .map_err(|e| WalletError::SigningVerifyError(format!("Invalid address: {}", e)))?;
        Ok(signature.verify(pubkey.as_ref(), message_bytes))
    }

    fn chain_type(&self) -> ChainType {
        ChainType::Solana
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_solana_wallet_creation() {
        let wallet = SolanaWallet::new().unwrap();
        assert!(!wallet.address().is_empty());
    }

    #[tokio::test]
    async fn test_solana_wallet_signing() {
        let wallet = SolanaWallet::new().unwrap();
        let message = "Test message signing";
        let signature = wallet.sign_message(message).await.unwrap();
        assert!(!signature.is_empty());
    }

    #[tokio::test]
    async fn test_solana_wallet_signing_verify() {
        let wallet = SolanaWallet::new().unwrap();
        let message = "Test message signing and verification";
        let signature = wallet.sign_message(message).await.unwrap();
        let verified = wallet
            .verify_signature(&message, &signature, &wallet.address())
            .await
            .unwrap();
        assert!(verified);
    }
}
