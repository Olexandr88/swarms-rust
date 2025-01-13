# Swarms-Rust Agent Interface

## Overview
Defines the core agent capabilities and interactions within the swarm ecosystem.

## Core Traits

```rust
/// Core agent capabilities for task processing and identity management.
/// Provides the fundamental interface that all agents must implement,
/// including task processing, identity verification, and configuration management.
pub trait Agent: Send + Sync {
    /// Process a task and produce results
    async fn process_task(&mut self, task: &Task) -> Result<TaskResults, AgentError>;
    
    /// Get agent identity
    fn identity(&self) -> &AgentIdentity;
    
    /// Get agent capabilities
    fn capabilities(&self) -> &[Capability];
    
    /// Check agent health
    async fn health_check(&self) -> Result<HealthStatus, AgentError>;
    
    /// Get agent configuration
    fn config(&self) -> &AgentConfig;
    
    /// Update agent configuration
    fn update_config(&mut self, config: AgentConfig) -> Result<(), AgentError>;

    /// Perform self-reflection and improvement
    async fn reflect_and_improve(&mut self) -> Result<ImprovementReport, AgentError>;

    /// Learn from task experience
    async fn learn_from_experience(&mut self, experience: TaskExperience) -> Result<(), AgentError>;

    /// Analyze performance metrics
    async fn analyze_performance(&self) -> Result<PerformanceAnalysis, AgentError>;

    /// Apply improvements based on analysis
    async fn apply_improvements(&mut self, improvements: Vec<ImprovementArea>) -> Result<(), AgentError>;

    /// Get agent's blockchain wallet
    fn wallet(&self) -> &Wallet;

    /// Execute blockchain operation
    async fn execute_chain_operation(
        &self,
        operation: ChainOperation,
    ) -> Result<ChainResponse, AgentError>;

    /// Sign data with specific chain
    async fn sign_with_chain(
        &self,
        chain: ChainType,
        data: &[u8],
    ) -> Result<Vec<u8>, AgentError>;

    /// Execute smart contract call
    async fn execute_contract(
        &self,
        chain: ChainType,
        contract: Address,
        function: &str,
        params: &[Value],
    ) -> Result<TransactionHash, AgentError>;

    /// Query chain state
    async fn query_chain(
        &self,
        chain: ChainType,
        query: ChainQuery,
    ) -> Result<ChainData, AgentError>;
}

/// Chain operations that agents can perform
pub enum ChainOperation {
    /// Contract interaction
    Contract {
        chain: ChainType,
        contract: Address,
        function: String,
        params: Vec<Value>,
    },
    /// Token transfer
    Transfer {
        chain: ChainType,
        token: Address,
        recipient: Address,
        amount: U256,
    },
    /// State query
    Query {
        chain: ChainType,
        query: ChainQuery,
    },
}

/// Chain operation response
pub enum ChainResponse {
    /// Transaction hash
    Transaction(TransactionHash),
    /// Query result
    QueryResult(Value),
    /// State update
    StateUpdate(StateData),
}

/// Agent configuration with blockchain support
pub struct AgentConfig {
    /// Agent identifier
    id: AgentId,
    /// Agent name
    name: String,
    /// System prompt/instructions
    system_prompt: String,
    /// Maximum processing loops
    max_loops: usize,
    /// LLM configuration
    llm_config: LLMConfig,
    /// Blockchain wallet configuration
    wallet_config: Option<WalletConfig>,
    /// Service configuration
    service_config: Option<ServiceConfig>,
}
```

## Implementation Examples

### Blockchain Agent Implementation
```rust
/// Example implementation of blockchain-enabled agent
impl Agent for BlockchainAgent {
    fn wallet(&self) -> &Wallet {
        &self.wallet
    }

    async fn execute_chain_operation(
        &self,
        operation: ChainOperation,
    ) -> Result<ChainResponse, AgentError> {
        match operation {
            ChainOperation::Contract { chain, contract, function, params } => {
                // Execute contract call
                let tx_hash = self.wallet()
                    .execute_contract(contract, &function, &params)
                    .await
                    .map_err(AgentError::ChainError)?;
                
                Ok(ChainResponse::Transaction(tx_hash))
            }
            ChainOperation::Transfer { chain, token, recipient, amount } => {
                // Transfer tokens
                let tx_hash = self.wallet()
                    .transfer_tokens(token, recipient, amount)
                    .await
                    .map_err(AgentError::ChainError)?;
                
                Ok(ChainResponse::Transaction(tx_hash))
            }
            ChainOperation::Query { chain, query } => {
                // Query chain state
                let result = self.wallet()
                    .query_chain(query)
                    .await
                    .map_err(AgentError::ChainError)?;
                
                Ok(ChainResponse::QueryResult(result))
            }
        }
    }

    async fn sign_with_chain(
        &self,
        chain: ChainType,
        data: &[u8],
    ) -> Result<Vec<u8>, AgentError> {
        self.wallet()
            .sign_message(data)
            .await
            .map_err(AgentError::ChainError)
    }
}
```

## Service Integration

### Service Agent Extension
```rust
/// Extension trait for agents that interact with external services
pub trait ServiceAgent: Agent {
    /// Get service endpoint
    fn endpoint(&self) -> &str;
    
    /// Check service availability
    async fn check_availability(&self) -> Result<bool, ServiceError>;
    
    /// Get service metrics
    async fn metrics(&self) -> Result<ServiceMetrics, ServiceError>;
}
```

## Identity Management

### Identity Verification Interface
```rust
/// Agent identity verification for blockchain operations
pub trait IdentityVerification: Send + Sync {
    /// Get blockchain address
    fn address(&self) -> Option<String>;
    
    /// Sign a message
    async fn sign_message(&self, message: &str) -> Result<String, VerificationError>;
    
    /// Verify a signature
    async fn verify_signature(
        &self,
        message: &str,
        signature: &str,
        address: &str,
    ) -> Result<bool, VerificationError>;
}
```

## Core Capabilities

### Supported Capabilities
```rust
/// Agent capabilities and supported operations
pub enum Capability {
    TextProcessing,
    CodeGeneration,
    ImageAnalysis,
    DataAnalysis,
    FinancialAnalysis,
    Custom(String),
}

### Health Status Types
```rust
/// Agent health status indicators
pub enum HealthStatus {
    Healthy,
    Degraded(String),
    Unhealthy(String),
}
```

## Responsibilities

1. Task Processing
   - Task validation
   - Result generation
   - Error handling
   - Status updates

2. Identity Management
   - Unique identification
   - Blockchain wallet integration
   - Message signing
   - Signature verification

3. Health Monitoring
   - Status reporting
   - Availability checking
   - Metric collection
   - Error reporting

4. Configuration Management
   - Config validation
   - Updates handling
   - Persistence

## Implementation Requirements

1. Concurrency
   - Thread-safe operations
   - Async task processing
   - Concurrent health checks

2. Error Handling
   - Clear error types
   - Recovery mechanisms
   - Timeout handling

3. Security
   - Secure key management
   - Signature verification
   - Access control

4. Performance
   - Efficient task processing
   - Resource management
   - Caching strategies

## Usage Patterns

1. Agent Creation
```rust
let agent = Agent::new(
    config,
    capabilities,
    identity,
);
```

2. Task Processing
```rust
let results = agent.process_task(&task).await?;
verify_results(&results, agent.identity());
```

3. Health Monitoring
```rust
let status = agent.health_check().await?;
match status {
    HealthStatus::Healthy => continue_processing(),
    HealthStatus::Degraded(reason) => log_warning(reason),
    HealthStatus::Unhealthy(reason) => initiate_recovery(reason),
}
```

4. Identity Verification
```rust
let signature = agent.sign_message(message).await?;
let is_valid = agent.verify_signature(message, &signature, &address).await?;
```
