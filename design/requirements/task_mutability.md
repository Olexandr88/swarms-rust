# Swarms-Rust Task Mutability Requirements

## Overview
Defines the mutable task state management system that enables collaborative processing by multiple agents while maintaining state traceability.

## Core Components

```rust
/// Mutable task state
pub struct MutableTask {
    /// Task identifier
    id: TaskId,
    /// Current state
    state: TaskState,
    /// State history
    history: Vec<StateTransition>,
    /// Active processors
    processors: HashSet<AgentId>,
    /// State lock
    lock: Option<StateLock>,
    /// Verification info
    verification: Option<VerificationInfo>,
}

/// Task state transition
pub struct StateTransition {
    /// Previous state
    previous: TaskState,
    /// New state
    new: TaskState,
    /// Transition timestamp
    timestamp: DateTime<Utc>,
    /// Processor agent
    processor: AgentId,
    /// Change description
    description: String,
    /// State signature
    signature: Option<String>,
}

/// Task state lock
pub struct StateLock {
    /// Lock holder
    holder: AgentId,
    /// Lock timestamp
    acquired: DateTime<Utc>,
    /// Lock duration
    duration: Duration,
    /// Lock purpose
    purpose: String,
}

/// State verification info
pub struct VerificationInfo {
    /// Chain type
    chain: ChainType,
    /// Contract address
    contract: Option<Address>,
    /// State hash
    hash: String,
    /// Block number
    block: Option<u64>,
    /// Transaction hash
    transaction: Option<String>,
}

/// Task state storage strategy
pub enum StateStorageStrategy {
    /// In-memory only (for small swarms)
    InMemory {
        cache_size: usize,
    },
    /// Persistent storage
    Persistent {
        storage: Box<dyn StateStorage>,
    },
    /// Blockchain storage
    Blockchain {
        chain: ChainType,
        contract: Address,
    },
    /// Hybrid storage
    Hybrid {
        primary: Box<StateStorageStrategy>,
        backup: Box<StateStorageStrategy>,
    },
}
```

## State Management Patterns

### 1. Collaborative Processing
```rust
impl MutableTask {
    async fn process_collaboratively(
        &mut self,
        processors: Vec<Agent>,
    ) -> Result<TaskResults, TaskError> {
        // Initialize collaborative processing
        self.start_collaboration(processors.iter().map(|a| a.id()).collect())?;
        
        // Process sequentially through agents
        for agent in processors {
            // Acquire state lock
            self.acquire_lock(agent.id(), "Processing")?;
            
            // Process current state
            let result = agent.process_state(&self.state).await?;
            
            // Create state transition
            let transition = StateTransition {
                previous: self.state.clone(),
                new: result.new_state,
                timestamp: Utc::now(),
                processor: agent.id(),
                description: result.description,
                signature: Some(agent.sign_state(&result.new_state).await?),
            };
            
            // Apply transition
            self.apply_transition(transition)?;
            
            // Release lock
            self.release_lock(agent.id())?;
        }
        
        // Finalize results
        self.finalize_collaboration()
    }
}
```

### 2. Blockchain State Tracking
```rust
impl BlockchainStateTracker {
    async fn track_state_changes(
        &mut self,
        task: &MutableTask,
    ) -> Result<(), TrackerError> {
        // Create state checkpoint
        let checkpoint = StateCheckpoint {
            task_id: task.id,
            state: task.state.clone(),
            timestamp: Utc::now(),
        };
        
        // Hash state data
        let hash = self.hash_state(&checkpoint)?;
        
        // Submit to blockchain
        let tx = self.contract
            .submit_state(
                checkpoint.task_id,
                hash,
                checkpoint.timestamp,
            )
            .await?;
            
        // Wait for confirmation
        let receipt = tx.wait_confirmation().await?;
        
        // Update verification info
        task.update_verification(VerificationInfo {
            chain: self.chain_type(),
            contract: Some(self.contract_address()),
            hash,
            block: Some(receipt.block_number),
            transaction: Some(receipt.transaction_hash),
        })?;
        
        Ok(())
    }
}
```

### 3. State Synchronization
```rust
impl StateManager {
    async fn synchronize_state(
        &mut self,
        task: &mut MutableTask,
        strategy: StateStorageStrategy,
    ) -> Result<(), StateError> {
        match strategy {
            StateStorageStrategy::InMemory { cache_size } => {
                // Update in-memory cache
                self.memory_cache
                    .insert(task.id, task.state.clone())
                    .await?;
            }
            StateStorageStrategy::Persistent { storage } => {
                // Store in persistent storage
                storage
                    .store_state(task.id, &task.state)
                    .await?;
            }
            StateStorageStrategy::Blockchain { chain, contract } => {
                // Track on blockchain
                self.blockchain_tracker
                    .track_state_changes(task)
                    .await?;
            }
            StateStorageStrategy::Hybrid { primary, backup } => {
                // Try primary first
                if let Err(e) = self.synchronize_state(task, *primary).await {
                    // Fallback to backup
                    self.synchronize_state(task, *backup)
                        .await
                        .map_err(|backup_err| {
                            StateError::SyncFailed(vec![e, backup_err])
                        })?;
                }
            }
        }
        
        Ok(())
    }
}
```

## Implementation Requirements

### 1. State Consistency
- Atomic transitions
- Lock management
- Conflict resolution
- Version tracking
- History preservation

### 2. Collaboration Support
- Multi-agent access
- State locking
- Change tracking
- Result aggregation
- Progress monitoring

### 3. Verification Support
- State signatures
- Chain tracking
- Proof generation
- History validation
- Audit support

### 4. Storage Flexibility
- Strategy selection
- Data persistence
- State recovery
- Performance optimization
- Resource management

## Security Considerations

### 1. Access Control
- Lock management
- Permission checking
- Role validation
- Operation auditing
- History protection

### 2. State Protection
- Transition validation
- Signature verification
- Hash verification
- Tampering prevention
- Recovery support

### 3. Collaboration Security
- Agent verification
- Change validation
- Lock enforcement
- Resource isolation
- Communication security

## Usage Examples

### 1. Centralized Small Swarm
```rust
// Create mutable task with in-memory storage
let mut task = MutableTask::new(
    task_id,
    StateStorageStrategy::InMemory {
        cache_size: 1000,
    },
);

// Process collaboratively
let results = task
    .process_collaboratively(agents)
    .await?;
```

### 2. Blockchain-Tracked Processing
```rust
// Create mutable task with blockchain tracking
let mut task = MutableTask::new(
    task_id,
    StateStorageStrategy::Blockchain {
        chain: ChainType::Ethereum,
        contract: contract_address,
    },
);

// Process with state tracking
let results = task
    .process_collaboratively(agents)
    .await?;

// Verify state history
let verification = task
    .verify_state_history()
    .await?;
```

### 3. Hybrid State Management
```rust
// Create task with hybrid storage
let mut task = MutableTask::new(
    task_id,
    StateStorageStrategy::Hybrid {
        primary: Box::new(StateStorageStrategy::Persistent {
            storage: Box::new(DatabaseStorage::new(config)),
        }),
        backup: Box::new(StateStorageStrategy::Blockchain {
            chain: ChainType::Ethereum,
            contract: contract_address,
        }),
    },
);

// Process with redundant state tracking
let results = task
    .process_collaboratively(agents)
    .await?;
```

## Error Handling

### 1. State Errors
```rust
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("Lock acquisition failed: {0}")]
    LockFailed(String),
    
    #[error("Invalid transition: {0}")]
    InvalidTransition(String),
    
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Sync failed: {0:?}")]
    SyncFailed(Vec<StateError>),
}
```

### 2. Recovery Patterns
```rust
impl StateManager {
    async fn recover_from_error(
        &mut self,
        error: StateError,
        task: &mut MutableTask,
    ) -> Result<(), StateError> {
        match error {
            StateError::LockFailed(_) => {
                // Force release lock
                self.force_release_lock(task).await
            }
            StateError::InvalidTransition(_) => {
                // Rollback to last valid state
                self.rollback_to_last_valid(task).await
            }
            StateError::VerificationFailed(_) => {
                // Rebuild from history
                self.rebuild_from_history(task).await
            }
            StateError::StorageError(_) => {
                // Switch to backup storage
                self.switch_to_backup_storage(task).await
            }
            StateError::SyncFailed(errors) => {
                // Attempt emergency sync
                self.emergency_sync(task, errors).await
            }
        }
    }
}
```
