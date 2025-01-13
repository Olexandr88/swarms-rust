# Consensus Requirements

## Overview
Defines the consensus building mechanisms for multi-agent collaboration using blockchain-based vote collection and verification.

## Core Components

```rust
/// Vote collection mechanisms
pub enum VoteCollector {
    /// On-chain collector (smart contract)
    OnChain {
        chain: ChainType,
        contract: Address,
        config: OnChainConfig,
    },
    /// Off-chain collector (centralized)
    Centralized {
        endpoint: String,
        auth: AuthConfig,
        config: CentralizedConfig,
    },
    /// Hybrid collector (on-chain + off-chain)
    Hybrid {
        primary: Box<VoteCollector>,
        backup: Box<VoteCollector>,
        sync_config: SyncConfig,
    },
}

/// Vote structure
pub struct Vote {
    /// Proposal being voted on
    proposal_id: ProposalId,
    /// Voting agent
    voter: AgentId,
    /// Vote decision
    decision: VoteDecision,
    /// Supporting evidence
    evidence: Option<Evidence>,
    /// Vote signature
    signature: Option<String>,
    /// Timestamp
    timestamp: DateTime<Utc>,
}

/// Vote decisions
pub enum VoteDecision {
    /// Binary vote (true/false)
    Binary(bool),
    /// Weighted vote with confidence
    Weighted {
        approve: bool,
        confidence: f32,
    },
    /// Abstain from voting
    Abstain(String),
}

/// Consensus proposal
pub struct ConsensusProposal {
    /// Proposal identifier
    id: ProposalId,
    /// Proposal description
    description: String,
    /// Required participants
    required_participants: Vec<AgentId>,
    /// Voting deadline
    deadline: DateTime<Utc>,
    /// Consensus configuration
    config: ConsensusConfig,
}

/// Consensus configuration
pub struct ConsensusConfig {
    /// Minimum required votes
    min_votes: usize,
    /// Required majority percentage
    majority_threshold: f32,
    /// Allow weighted voting
    weighted_voting: bool,
    /// Allow binary-only votes
    binary_only: bool,
    /// Maximum voting rounds
    max_rounds: usize,
}
```

## Vote Collection Process

### 1. On-Chain Collection
- Votes submitted as blockchain transactions
- Smart contract validates signatures
- State updates trigger notifications
- Immutable vote record maintained
- Example:
```rust
impl OnChainCollector {
    async fn submit_vote(&self, vote: Vote) -> Result<(), CollectorError> {
        // Create transaction
        let tx = self.contract
            .submit_vote(
                vote.proposal_id,
                vote.decision,
                vote.signature,
            )
            .await?;
        
        // Wait for confirmation
        tx.wait_confirmation()
            .await
            .map_err(CollectorError::ChainError)
    }
    
    async fn get_votes(&self, proposal_id: ProposalId) -> Result<Vec<Vote>, CollectorError> {
        // Query contract state
        self.contract
            .get_votes(proposal_id)
            .await
            .map_err(CollectorError::ChainError)
    }
}
```

### 2. Off-Chain Collection
- Centralized server collects votes
- API endpoint for submission
- Database storage
- Signature verification required
- Example:
```rust
impl CentralizedCollector {
    async fn submit_vote(&self, vote: Vote) -> Result<(), CollectorError> {
        // Verify signature
        self.verify_signature(&vote)?;
        
        // Submit to API
        self.client
            .post(&self.endpoint)
            .json(&vote)
            .send()
            .await
            .map_err(CollectorError::ApiError)?;
        
        Ok(())
    }
}
```

### 3. Hybrid Collection
- Primary collector (usually on-chain)
- Backup collector for redundancy
- Automatic failover
- State synchronization
- Example:
```rust
impl HybridCollector {
    async fn submit_vote(&self, vote: Vote) -> Result<(), CollectorError> {
        // Try primary first
        match self.primary.submit_vote(vote.clone()).await {
            Ok(_) => Ok(()),
            Err(e) => {
                // Fallback to backup
                self.backup
                    .submit_vote(vote)
                    .await
                    .map_err(|backup_err| {
                        CollectorError::AllCollectorsFailed(vec![e, backup_err])
                    })
            }
        }
    }
}
```

## Consensus Building

### 1. Vote Processing
```rust
impl ConsensusManager {
    async fn process_votes(
        &self,
        votes: Vec<Vote>,
        config: &ConsensusConfig,
    ) -> Result<ConsensusDecision, ConsensusError> {
        // Validate votes
        let valid_votes = self.validate_votes(&votes)?;
        
        // Check minimum threshold
        if valid_votes.len() < config.min_votes {
            return Err(ConsensusError::InsufficientVotes);
        }
        
        // Calculate results
        let result = if config.weighted_voting {
            self.calculate_weighted_result(&valid_votes)
        } else {
            self.calculate_majority_result(&valid_votes)
        }?;
        
        // Check majority threshold
        if result.approval_percentage >= config.majority_threshold {
            Ok(ConsensusDecision::Approved(result))
        } else {
            Ok(ConsensusDecision::Rejected(result))
        }
    }
}
```

### 2. State Machine Model
- Blockchain treated as state machine
- Agents observe state changes
- Cannot interrupt on-chain execution
- Example:
```rust
impl ConsensusStateMachine {
    async fn observe_state(&self) -> Result<ConsensusState, StateError> {
        // Watch for state updates
        let mut watcher = self.chain
            .watch_state(StateFilter::Consensus)
            .await?;
        
        while let Some(update) = watcher.next().await {
            match update {
                StateUpdate::VoteSubmitted(vote) => {
                    self.process_new_vote(vote).await?;
                }
                StateUpdate::ConsensusReached(decision) => {
                    return Ok(ConsensusState::Decided(decision));
                }
                StateUpdate::Timeout => {
                    return Ok(ConsensusState::TimedOut);
                }
            }
        }
        
        Ok(ConsensusState::Observing)
    }
}
```

## Implementation Requirements

### 1. Security
- Signature verification
- Vote validation
- Duplicate detection
- Timestamp checking
- Access control

### 2. Reliability
- Failover handling
- State recovery
- Transaction retry
- Error propagation
- Timeout management

### 3. Performance
- Parallel processing
- Batch operations
- State caching
- Resource limits
- Load distribution

### 4. Monitoring
- Vote tracking
- State observation
- Error logging
- Metrics collection
- Health checking

## Usage Examples

### 1. Creating Proposal
```rust
let proposal = ConsensusProposal {
    id: generate_id(),
    description: "Approve task result".into(),
    required_participants: agents,
    deadline: Utc::now() + Duration::hours(1),
    config: ConsensusConfig {
        min_votes: 3,
        majority_threshold: 0.67,
        weighted_voting: true,
        binary_only: false,
        max_rounds: 1,
    },
};
```

### 2. Submitting Vote
```rust
let vote = Vote {
    proposal_id: proposal.id,
    voter: agent.id(),
    decision: VoteDecision::Weighted {
        approve: true,
        confidence: 0.8,
    },
    evidence: Some(evidence),
    signature: Some(agent.sign_vote(&proposal).await?),
    timestamp: Utc::now(),
};

collector.submit_vote(vote).await?;
```

### 3. Building Consensus
```rust
let consensus = ConsensusManager::new(config);

// Collect votes
let votes = collector
    .get_votes(proposal.id)
    .await?;

// Process votes
let decision = consensus
    .process_votes(votes, &proposal.config)
    .await?;

match decision {
    ConsensusDecision::Approved(result) => {
        execute_approved_action(result).await?;
    }
    ConsensusDecision::Rejected(result) => {
        handle_rejection(result).await?;
    }
}
```

## Error Handling

### 1. Vote Collection Errors
```rust
pub enum CollectorError {
    /// Chain interaction failed
    ChainError(ChainError),
    /// API request failed
    ApiError(ApiError),
    /// Invalid signature
    InvalidSignature(String),
    /// Duplicate vote
    DuplicateVote(VoteId),
    /// All collectors failed
    AllCollectorsFailed(Vec<CollectorError>),
}
```

### 2. Consensus Errors
```rust
pub enum ConsensusError {
    /// Insufficient votes
    InsufficientVotes,
    /// Invalid vote format
    InvalidVote(String),
    /// Deadline exceeded
    DeadlineExceeded,
    /// Processing error
    ProcessingError(String),
}
```

## Security Considerations

### 1. Vote Integrity
- Cryptographic signatures
- Chain of custody
- Immutable records
- Audit trails

### 2. Access Control
- Participant verification
- Role-based access
- Permission checks
- Identity management

### 3. Data Protection
- Secure storage
- Encrypted transmission
- Privacy preservation
- Data retention
