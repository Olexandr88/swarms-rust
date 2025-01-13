# Swarms-Rust Storage Interface

## Overview
Defines the storage capabilities for agents and swarms to manage metadata, configuration, and temporary data.

## Core Traits

```rust
/// Core storage capabilities
pub trait Storage: Send + Sync {
    /// Store data with key
    async fn set(&mut self, key: &str, value: &StorageValue) -> Result<(), StorageError>;
    
    /// Retrieve data by key
    async fn get(&self, key: &str) -> Result<Option<StorageValue>, StorageError>;
    
    /// Delete data by key
    async fn delete(&mut self, key: &str) -> Result<(), StorageError>;
    
    /// Check if key exists
    async fn exists(&self, key: &str) -> Result<bool, StorageError>;
    
    /// List all keys with optional prefix
    async fn list_keys(&self, prefix: Option<&str>) -> Result<Vec<String>, StorageError>;
    
    /// Clear all data
    async fn clear(&mut self) -> Result<(), StorageError>;
}

/// Key-value storage implementation
pub trait KeyValueStorage: Storage {
    /// Atomic increment
    async fn increment(&mut self, key: &str, amount: i64) -> Result<i64, StorageError>;
    
    /// Set with expiration
    async fn set_ex(
        &mut self,
        key: &str,
        value: &StorageValue,
        expiry: Duration,
    ) -> Result<(), StorageError>;
    
    /// Get time-to-live for key
    async fn ttl(&self, key: &str) -> Result<Option<Duration>, StorageError>;
}

/// Relational storage capabilities
pub trait RelationalStorage: Storage {
    /// Execute query
    async fn query(
        &self,
        query: &str,
        params: &[QueryParam],
    ) -> Result<QueryResult, StorageError>;
    
    /// Begin transaction
    async fn begin_transaction(&mut self) -> Result<Transaction, StorageError>;
    
    /// Get schema information
    fn schema(&self) -> &StorageSchema;
    
    /// Migrate schema
    async fn migrate(&mut self, migrations: &[Migration]) -> Result<(), StorageError>;
}

/// File storage capabilities
pub trait FileStorage: Storage {
    /// Write file
    async fn write_file(
        &mut self,
        path: &str,
        content: &[u8],
    ) -> Result<(), StorageError>;
    
    /// Read file
    async fn read_file(&self, path: &str) -> Result<Vec<u8>, StorageError>;
    
    /// Delete file
    async fn delete_file(&mut self, path: &str) -> Result<(), StorageError>;
    
    /// List files in directory
    async fn list_files(
        &self,
        directory: &str,
    ) -> Result<Vec<FileInfo>, StorageError>;
}

/// Storage value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageValue {
    /// String value
    String(String),
    /// Integer value
    Integer(i64),
    /// Float value
    Float(f64),
    /// Boolean value
    Boolean(bool),
    /// Binary data
    Binary(Vec<u8>),
    /// JSON value
    Json(serde_json::Value),
    /// Custom serializable value
    Custom(Box<dyn StorageSerialize>),
}

/// Storage configuration
pub struct StorageConfig {
    /// Storage type
    storage_type: StorageType,
    /// Connection information
    connection: ConnectionInfo,
    /// Storage options
    options: StorageOptions,
    /// Encryption configuration
    encryption: Option<EncryptionConfig>,
}

/// Storage types
pub enum StorageType {
    /// In-memory storage
    Memory,
    /// File system storage
    FileSystem,
    /// Redis storage
    Redis,
    /// SQLite storage
    SQLite,
    /// PostgreSQL storage
    PostgreSQL,
    /// Custom storage implementation
    Custom(String),
}
```

## Responsibilities

### 1. Data Management
- Key-value operations
- Query execution
- File operations
- Transaction handling
- Data validation

### 2. Configuration
- Connection management
- Options handling
- Schema management
- Migration support
- Encryption setup

### 3. Error Handling
- Connection errors
- Operation failures
- Validation errors
- Schema errors
- Security errors

### 4. Performance
- Connection pooling
- Query optimization
- Caching strategy
- Resource management
- Monitoring

## Implementation Requirements

### 1. Concurrency
- Thread-safe operations
- Transaction isolation
- Lock management
- Connection pooling
- Error propagation

### 2. Security
- Data encryption
- Access control
- Audit logging
- Secure connections
- Key management

### 3. Reliability
- Error recovery
- Data consistency
- Backup support
- Migration handling
- Monitoring

### 4. Performance
- Connection pooling
- Query optimization
- Index management
- Cache utilization
- Resource limits

## Usage Patterns

### 1. Key-Value Storage
```rust
// Store configuration
let config = StorageValue::Json(serde_json::json!({
    "api_key": "secret",
    "endpoint": "https://api.example.com"
}));
storage.set("agent_config", &config).await?;

// Retrieve configuration
let config = storage.get("agent_config").await?
    .and_then(|v| v.as_json())
    .ok_or_else(|| "Config not found")?;
```

### 2. Relational Storage
```rust
// Execute query
let result = storage
    .query(
        "SELECT * FROM tasks WHERE status = ?",
        &[QueryParam::String("pending")],
    )
    .await?;

// Process results
for row in result.rows() {
    process_task(row.get("task_id")?);
}
```

### 3. File Storage
```rust
// Store temporary data
let data = serialize_task_result(&result)?;
storage
    .write_file("tmp/task_123/result.bin", &data)
    .await?;

// Read temporary data
let data = storage
    .read_file("tmp/task_123/result.bin")
    .await?;
let result = deserialize_task_result(&data)?;
```

## Error Handling

### 1. Storage Errors
```rust
/// Storage error types
pub enum StorageError {
    /// Connection failed
    ConnectionFailed(String),
    /// Operation failed
    OperationFailed(String),
    /// Invalid data
    InvalidData(String),
    /// Schema error
    SchemaError(String),
    /// Security error
    SecurityError(String),
}
```

### 2. Recovery Strategies
1. Retry failed operations
2. Use fallback storage
3. Clear corrupted data
4. Rebuild indices
5. Restore from backup

## Security Considerations

### 1. Data Protection
- Encrypt sensitive data
- Secure connections
- Access control
- Audit logging
- Key rotation

### 2. Error Handling
- Secure error messages
- Validation checks
- Input sanitization
- Output encoding
- Error logging
