use super::{Wallet, ChainType, WalletError};
use alloy::hex;
use alloy::primitives::{bytes, Address};
use alloy::signers::{local::PrivateKeySigner, SignerSync};
use alloy::signers::{Signer, Signature};
use async_trait::async_trait;
use std::str::FromStr;

/// EVM wallet implementation
pub struct EvmWallet {
    signer: PrivateKeySigner,
}

impl EvmWallet {
    /// Create a new random wallet
    #[allow(dead_code)]
    pub fn new() -> Result<Self, WalletError> {
        let signer = PrivateKeySigner::random();
        Ok(Self { signer })
    }

    /// Create from private key
    #[allow(dead_code)]
    pub fn from_private_key(key: &str) -> Result<Self, WalletError> {
        PrivateKeySigner::from_str(key)
            .map(|signer| Self { signer })
            .map_err(|e| WalletError::KeyError(e.to_string()))
    }
}

#[async_trait]
impl Wallet for EvmWallet {
    fn address(&self) -> String {
        format!("{:?}", self.signer.address())
    }

    async fn sign_message(&self, message: &str) -> Result<String, WalletError> {
        // Sign the message directly
        let signature = self
            .signer
            .sign_message(message.as_bytes())
            .await
            .map_err(|e| WalletError::SigningError(e.to_string()))?;

        // Serialize the signature to hexadecimal
        Ok(hex::encode(signature.as_bytes())) // Serialize to hexadecimal
    }

    async fn verify_signature(
        &self,
        message: &str,
        signature: &str,
        address: &str,
    ) -> Result<bool, WalletError> {
        // Parse the signature
        let sig = Signature::from_str(signature)
            .map_err(|e| WalletError::SigningVerifyError(format!("Invalid signature: {}", e)))?;

        // Recover the address from the message and signature
        let recovered_address = sig
            .recover_address_from_msg(message.as_bytes())
            .map_err(|e| WalletError::SigningVerifyError(format!("Recovery failed: {}", e)))?;

        // Parse the expected address
        let expected_address = Address::from_str(address)
            .map_err(|e| WalletError::SigningVerifyError(format!("Invalid address: {}", e)))?;

        // Compare the recovered address with the expected address
        Ok(recovered_address == expected_address)
    }

    fn chain_type(&self) -> ChainType {
        ChainType::Evm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_evm_wallet_creation() {
        let wallet = EvmWallet::new().unwrap();
        assert!(!wallet.address().is_empty());
    }

    #[tokio::test]
    async fn test_evm_wallet_signing() {
        let wallet = EvmWallet::new().unwrap();
        let message = "Test message signing";
        let signature = wallet.sign_message(message).await.unwrap();
        assert!(!signature.is_empty());
    }

    #[tokio::test]
    async fn test_evm_wallet_signing_verify() {
        let wallet = EvmWallet::new().unwrap();
        let message = "Test message signing and verification";

        // Sign the message
        let signature = wallet.sign_message(message).await.unwrap();
        println!("Generated Signature: {}", signature);

        // Verify the signature
        let verified = wallet
            .verify_signature(message, &signature, &wallet.address())
            .await
            .unwrap();

        assert!(
            verified,
            "Signature verification failed. Signature: {}, Address: {}",
            signature,
            wallet.address()
        );
    }
}