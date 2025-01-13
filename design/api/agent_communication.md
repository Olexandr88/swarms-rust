# Swarms-Rust Agent Communication API

## Overview
Defines the communication patterns and APIs for agent interaction within the swarm ecosystem.

## Communication Patterns

### 1. Direct Agent-to-Agent Communication
```rust
/// Agent communication message
pub struct AgentMessage {
    /// Message identifier
    id: MessageId,
    /// Sender agent ID
    sender: AgentId,
    /// Recipient agent ID
    recipient: AgentId,
    /// Message content
    content: MessageContent,
    /// Message metadata
    metadata: MessageMetadata,
    /// Optional signature
    signature: Option<String>,
}

/// Message content types
pub enum MessageContent {
    /// Task input/output
    TaskData(TaskData),
    /// Control messages
    Control(ControlMessage),
    /// State updates
    StateUpdate(StateData),
    /// Health reports
    HealthReport(HealthData),
    /// Consensus messages
    Consensus(ConsensusMessage),
    /// Vote messages
    Vote(VoteMessage),
    /// Result verification
    Verification(VerificationMessage),
}

/// Consensus message types
pub struct ConsensusMessage {
    /// Proposal identifier
    proposal_id: ProposalId,
    /// Task being voted on
    task: Task,
    /// Proposed result
    proposed_result: TaskResult,
    /// Supporting evidence
    evidence: Option<Evidence>,
    /// Required participants
    required_participants: Vec<AgentId>,
    /// Consensus deadline
    deadline: DateTime<Utc>,
}

/// Vote message types
pub struct VoteMessage {
    /// Proposal being voted on
    proposal_id: ProposalId,
    /// Voting agent
    voter: AgentId,
    /// Vote decision
    decision: VoteDecision,
    /// Optional confidence score (0.0-1.0)
    confidence: Option<f32>,
    /// Supporting reasoning
    reasoning: Option<String>,
    /// Vote signature
    signature: Option<String>,
}

/// Vote decisions
pub enum VoteDecision {
    /// Binary vote (true = approve, false = reject)
    Binary(bool),
    /// Weighted vote with confidence
    Weighted {
        approve: bool,
        confidence: f32,
    },
    /// Abstain from voting
    Abstain(String),
    /// Request clarification
    NeedsClarification(String),
}

/// Communication channel interface
#[async_trait]
pub trait CommunicationChannel {
    /// Send message to recipient
    async fn send_message(&self, message: AgentMessage) -> Result<(), CommError>;
    
    /// Receive messages for agent
    async fn receive_messages(&self, agent_id: AgentId) -> Result<Vec<AgentMessage>, CommError>;
    
    /// Subscribe to message type
    async fn subscribe(
        &self,
        agent_id: AgentId,
        message_type: MessageType,
    ) -> Result<MessageStream, CommError>;
}
```

### 2. Swarm-Mediated Communication
```rust
/// Swarm communication interface
#[async_trait]
pub trait SwarmCommunication {
    /// Broadcast message to all agents
    async fn broadcast(&self, message: BroadcastMessage) -> Result<(), SwarmError>;
    
    /// Send targeted message through swarm
    async fn route_message(&self, message: AgentMessage) -> Result<(), SwarmError>;
    
    /// Register agent communication handlers
    fn register_handler(
        &mut self,
        message_type: MessageType,
        handler: Box<dyn MessageHandler>,
    );
}

/// Message handler interface
#[async_trait]
pub trait MessageHandler: Send + Sync {
    /// Handle incoming message
    async fn handle_message(&self, message: AgentMessage) -> Result<(), HandlerError>;
    
    /// Get handler capabilities
    fn capabilities(&self) -> &[MessageType];
}
```

## API Endpoints

### 1. Agent Service API
```rust
/// Agent service endpoints
pub trait AgentService {
    /// Process task
    async fn process_task(&self, request: TaskRequest) -> Result<TaskResponse, ServiceError>;
    
    /// Get agent status
    async fn get_status(&self) -> Result<AgentStatus, ServiceError>;
    
    /// Update configuration
    async fn update_config(&self, config: AgentConfig) -> Result<(), ServiceError>;
}

/// Task processing request
pub struct TaskRequest {
    /// Task identifier
    task_id: TaskId,
    /// Task input data
    input: TaskInput,
    /// Processing parameters
    parameters: TaskParameters,
    /// Required capabilities
    required_capabilities: Vec<Capability>,
}

/// Task processing response
pub struct TaskResponse {
    /// Task identifier
    task_id: TaskId,
    /// Processing result
    result: TaskResult,
    /// Processing metadata
    metadata: TaskMetadata,
    /// Optional signature
    signature: Option<String>,
}
```

### 2. Swarm Management API
```rust
/// Swarm management interface
pub trait SwarmManagement {
    /// Register agent with swarm
    async fn register_agent(&self, agent: AgentRegistration) -> Result<(), SwarmError>;
    
    /// Deregister agent from swarm
    async fn deregister_agent(&self, agent_id: AgentId) -> Result<(), SwarmError>;
    
    /// Update agent status
    async fn update_agent_status(&self, status: AgentStatus) -> Result<(), SwarmError>;
    
    /// Get registered agents
    async fn get_agents(&self) -> Result<Vec<AgentInfo>, SwarmError>;
}

/// Agent registration
pub struct AgentRegistration {
    /// Agent identifier
    id: AgentId,
    /// Agent capabilities
    capabilities: Vec<Capability>,
    /// Service endpoint
    endpoint: Option<String>,
    /// Authentication credentials
    credentials: Option<Credentials>,
}
```

## Interaction Flows

### 1. Consensus Building Flow
```rust
// 1. Distribute task to multiple agents
let task_distribution = TaskDistribution {
    task: task.clone(),
    required_agents: find_capable_agents(task.required_capabilities()),
    consensus_config: ConsensusConfig {
        min_responses: 3,
        timeout: Duration::from_secs(30),
        consensus_type: ConsensusType::MajorityVote,
    },
};

// 2. Broadcast task and collect responses
let responses = swarm
    .broadcast_task_with_consensus(task_distribution)
    .await?;

// 3. Build consensus proposal
let proposal = ConsensusProposal::new(
    task.id(),
    responses,
    consensus_config,
);

// 4. Initiate voting round
let voting_result = consensus_manager
    .initiate_voting(proposal)
    .await?;

// 5. Process voting results
match voting_result {
    VotingResult::Consensus(result) => {
        // Execute consensus decision
        execute_consensus_decision(result).await?
    }
    VotingResult::Disagreement(reason) => {
        // Handle disagreement (e.g., LLM arbitration)
        handle_consensus_disagreement(reason).await?
    }
    VotingResult::Timeout => {
        // Handle timeout condition
        handle_consensus_timeout().await?
    }
}

// 6. Notify participants of outcome
broadcast_consensus_outcome(voting_result).await?;
```

### 2. Task Distribution Flow
```rust
// 1. Create task distribution
let distribution = TaskDistribution {
    task: task.clone(),
    strategy: DistributionStrategy::CapabilityBased(required_capabilities),
    min_responses: consensus_config.min_responses,
    timeout: consensus_config.timeout,
};

// 2. Find capable agents
let capable_agents = swarm
    .find_agents_with_capabilities(&required_capabilities)
    .await?;

// 3. Distribute task
let distribution_result = swarm
    .distribute_task(distribution, capable_agents)
    .await?;

// 4. Monitor distribution progress
let progress = swarm
    .monitor_distribution_progress(distribution_result.id())
    .await?;

// 5. Handle distribution completion
match progress {
    DistributionProgress::Complete(results) => {
        process_distributed_results(results).await?
    }
    DistributionProgress::Partial(results) => {
        handle_partial_distribution(results).await?
    }
    DistributionProgress::Failed(error) => {
        handle_distribution_failure(error).await?
    }
}

### 3. Result Collection Flow
```rust
// 1. Initialize result collection
let collection = ResultCollection::new(
    task_id,
    required_agents.clone(),
    collection_config,
);

// 2. Collect agent responses
while !collection.is_complete() {
    // Receive new results
    let new_results = collection.receive_results().await?;
    
    // Validate results
    for result in new_results {
        if result.requires_verification() {
            verify_result_signature(&result).await?;
        }
        collection.add_validated_result(result);
    }
    
    // Check collection status
    match collection.status() {
        CollectionStatus::Complete(results) => {
            return Ok(results);
        }
        CollectionStatus::InProgress { received, total } => {
            log_collection_progress(received, total);
        }
        CollectionStatus::Timeout => {
            handle_collection_timeout().await?;
        }
    }
}

// 3. Process collected results
let processed_results = ResultProcessor::new(collection.results())
    .filter_invalid()
    .aggregate_responses()
    .calculate_confidence()
    .build()?;

// 4. Prepare for consensus
initiate_consensus_round(processed_results).await?;
```

## Error Handling

### 1. Communication Errors
```rust
/// Communication error types
pub enum CommError {
    /// Connection failed
    ConnectionFailed(String),
    /// Message delivery failed
    DeliveryFailed(String),
    /// Invalid message format
    InvalidMessage(String),
    /// Authentication failed
    AuthenticationFailed(String),
}
```

### 2. Service Errors
```rust
/// Service error types
pub enum ServiceError {
    /// Service unavailable
    ServiceUnavailable(String),
    /// Invalid request
    InvalidRequest(String),
    /// Processing failed
    ProcessingFailed(String),
    /// Verification failed
    VerificationFailed(String),
}
```

## Security Considerations

1. Message Authentication
   - All messages should be authenticated
   - Optional message signing
   - Credential verification

2. Access Control
   - Agent authorization
   - Capability verification
   - Role-based access

3. Communication Security
   - Encrypted channels
   - Secure endpoints
   - Token validation

4. Error Handling
   - Secure error messages
   - Audit logging
   - Failure recovery
