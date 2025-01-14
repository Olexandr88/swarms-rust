use async_trait::async_trait;
use serde_json::Value;
use thiserror::Error;

/// Error type for memory operations
#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Failed to store vectors: {0}")]
    StoreError(String),

    #[error("Failed to query vectors: {0}")]
    QueryError(String),

    #[error("Failed to delete vectors: {0}")]
    DeleteError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

/// Trait for vector storage implementations
#[async_trait]
pub trait VectorStore: Send + Sync {
    /// Store vectors with metadata
    async fn store(
        &mut self,
        vectors: Vec<f32>,
        metadata: Option<Value>,
    ) -> Result<(), MemoryError>;

    /// Query vectors by similarity
    async fn query(
        &self,
        query_vector: Vec<f32>,
        top_k: usize,
    ) -> Result<Vec<(f32, Option<Value>)>, MemoryError>;

    /// Delete vectors by id
    async fn delete(&mut self, ids: Vec<String>) -> Result<(), MemoryError>;
}


mod chromadb;
mod pinecone;