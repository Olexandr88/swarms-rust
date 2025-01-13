# Swarms-Rust Swarm Interface

## Overview
Defines the swarm orchestration and management capabilities.

## Core Traits

```rust
/// Core swarm capabilities for agent orchestration and coordination.
/// Provides the fundamental interface for managing agent collectives,
/// including task distribution, consensus building, and state management.
pub trait Swarm: Send + Sync {
    /// Associated agent type
    type AgentType: Agent;
    
    /// Get swarm configuration
    fn config(&self) -> &SwarmConfig;
    
    /// Get swarm agents
    fn agents(&self) -> &[Self::AgentType];
    
    /// Add agent to swarm
    fn add_agent(&mut self, agent: Self::AgentType);
    
    /// Remove agent from swarm
    fn remove_agent(&mut self, agent_id: AgentId) -> Option<Self::AgentType>;
    
    /// Process task through swarm
    async fn process_task(&mut self, task: &Task) -> Result<Vec<TaskResults>, SwarmError>;
    
    /// Get swarm state
    fn state(&self) -> &SwarmState;
    
    /// Update swarm configuration
    fn update_config(&mut self, config: SwarmConfig) -> Result<(), SwarmError>;

    /// Perform swarm-level reflection and improvement
    async fn reflect_and_improve(&mut self) -> Result<SwarmImprovementReport, SwarmError>;

    /// Analyze swarm performance and composition
    async fn analyze_swarm_performance(&self) -> Result<SwarmPerformanceAnalysis, SwarmError>;

    /// Optimize agent composition and roles
    async fn optimize_composition(&mut self) -> Result<CompositionChanges, SwarmError>;

    /// Improve coordination patterns
    async fn improve_coordination(&mut self) -> Result<CoordinationImprovements, SwarmError>;

    /// Learn from collective experience
    async fn learn_from_collective(&mut self, experiences: Vec<SwarmExperience>) -> Result<(), SwarmError>;

    /// Get swarm's blockchain wallet
    fn wallet(&self) -> &Wallet;

    /// Coordinate chain operations across agents
    async fn coordinate_chain_operations(
        &mut self,
        operations: Vec<ChainOperation>,
    ) -> Result<Vec<ChainResponse>, SwarmError>;

    /// Build consensus through on-chain voting
    async fn build_chain_consensus(
        &mut self,
        proposal: ConsensusProposal,
        collector: &VoteCollector,
    ) -> Result<ConsensusDecision, SwarmError>;

    /// Monitor chain state for all agents
    async fn monitor_chain_state(
        &self,
        filter: StateFilter,
    ) -> Result<StateWatcher, SwarmError>;
}

/// Swarm configuration with blockchain support
pub struct SwarmConfig {
    /// Swarm identifier
    id: SwarmId,
    /// Swarm name
    name: String,
    /// Maximum processing loops
    max_loops: usize,
    /// Execution strategy
    strategy: ExecutionStrategy,
    /// Brain configuration
    brain_config: Option<BrainConfig>,
    /// Memory configuration
    memory_config: Option<MemoryConfig>,
    /// Blockchain wallet configuration
    wallet_config: Option<WalletConfig>,
    /// Consensus configuration
    consensus_config: Option<ConsensusConfig>,
}

```

## Implementation Examples

### Blockchain-Enabled Swarm
```rust
/// Example implementation of blockchain-enabled swarm
impl Swarm for SmartSwarm {
    fn wallet(&self) -> &Wallet {
        &self.wallet
    }

    async fn coordinate_chain_operations(
        &mut self,
        operations: Vec<ChainOperation>,
    ) -> Result<Vec<ChainResponse>, SwarmError> {
        let mut responses = Vec::new();
        
        for operation in operations {
            // Select appropriate agent for operation
            let agent = self.select_agent_for_operation(&operation)?;
            
            // Execute operation through agent
            let response = agent
                .execute_chain_operation(operation)
                .await
                .map_err(SwarmError::AgentError)?;
            
            responses.push(response);
        }
        
        Ok(responses)
    }

    async fn build_chain_consensus(
        &mut self,
        proposal: ConsensusProposal,
        collector: &VoteCollector,
    ) -> Result<ConsensusDecision, SwarmError> {
        // Collect votes from agents
        let mut votes = Vec::new();
        
        for agent in self.agents() {
            // Get agent's vote
            let vote = agent
                .process_proposal(&proposal)
                .await
                .map_err(SwarmError::AgentError)?;
            
            // Sign vote with agent's wallet
            let signature = agent
                .sign_with_chain(proposal.chain(), &vote.to_bytes())
                .await
                .map_err(SwarmError::AgentError)?;
            
            // Submit signed vote
            collector
                .submit_vote(vote, signature)
                .await
                .map_err(SwarmError::ConsensusError)?;
            
            votes.push(vote);
        }
        
        // Process votes and reach consensus
        self.consensus_manager
            .build_consensus(votes, &self.config.consensus_config)
            .await
            .map_err(SwarmError::ConsensusError)
    }

    async fn monitor_chain_state(
        &self,
        filter: StateFilter,
    ) -> Result<StateWatcher, SwarmError> {
        // Create state watcher
        let watcher = self.wallet()
            .watch_state(filter)
            .await
            .map_err(SwarmError::ChainError)?;
        
        Ok(watcher)
    }
}
```

## State Management

### Chain State Monitoring
```rust
/// Implementation of blockchain state monitoring
impl StateWatcher {
    /// Watch for state updates
    pub async fn watch(&mut self) -> Option<StateUpdate> {
        match self.updates.recv().await {
            Some(update) => {
                // Process update based on type
                match update {
                    StateUpdate::NewBlock(block) => {
                        // Check block for relevant transactions
                        self.process_block(block).await
                    }
                    StateUpdate::Event(event) => {
                        // Handle contract event
                        self.process_event(event).await
                    }
                    StateUpdate::Error(error) => {
                        // Log error and continue
                        self.handle_error(error).await
                    }
                }
            }
            None => None,
        }
    }
}
```

## Decision Making Interface

### Brain Capabilities
```rust
/// Core decision making and coordination capabilities for swarm orchestration
pub trait SwarmBrain: Send + Sync {
    /// Select next agent for task
    async fn select_agent(&self, task: &Task, agents: &[Agent]) -> Result<AgentId, SwarmError>;
    
    /// Evaluate task results
    async fn evaluate_results(
        &self,
        task: &Task,
        results: &[TaskResults],
    ) -> Result<EvaluationResult, SwarmError>;
    
    /// Make routing decisions
    async fn make_decision(
        &self,
        state: &SwarmState,
        task: &Task,
    ) -> Result<SwarmDecision, SwarmError>;
}

/// Swarm configuration
pub struct SwarmConfig {
    /// Swarm identifier
    id: SwarmId,
    /// Swarm name
    name: String,
    /// Maximum processing loops
    max_loops: usize,
    /// Execution strategy
    strategy: ExecutionStrategy,
    /// Brain configuration
    brain_config: Option<BrainConfig>,
    /// Memory configuration
    memory_config: Option<MemoryConfig>,
}

/// Swarm state
pub struct SwarmState {
    /// Active agents
    active_agents: Vec<AgentId>,
    /// Task history
    task_history: Vec<TaskRecord>,
    /// Performance metrics
    metrics: SwarmMetrics,
    /// Current status
    status: SwarmStatus,
}

/// Execution strategy
pub enum ExecutionStrategy {
    Sequential,
    Parallel,
    Adaptive,
    Custom(Box<dyn ExecutionPolicy>),
}

/// Swarm decision
pub enum SwarmDecision {
    AssignTask(AgentId),
    RetryTask(RetryPolicy),
    CompleteTask(TaskResults),
    AbortTask(String),
}
```

## Responsibilities

1. Agent Management
   - Agent addition/removal
   - Health monitoring
   - Load balancing
   - State tracking

2. Task Orchestration
   - Task distribution
   - Result collection
   - Error handling
   - Retry management

3. Decision Making
   - Agent selection
   - Result evaluation
   - Task routing
   - Execution strategy

4. State Management
   - State persistence
   - Metrics collection
   - History tracking
   - Status updates

## Implementation Requirements

1. Concurrency
   - Thread-safe operations
   - Parallel task processing
   - State synchronization

2. Fault Tolerance
   - Agent failure handling
   - Task retry mechanisms
   - State recovery

3. Scalability
   - Dynamic agent scaling
   - Load distribution
   - Resource management

4. Monitoring
   - Performance metrics
   - Health checking
   - Error tracking
   - State logging

## Usage Patterns

1. Swarm Creation
```rust
let swarm = Swarm::new(
    config,
    brain,
    memory,
);
```

2. Agent Management
```rust
swarm.add_agent(agent);
let removed = swarm.remove_agent(agent_id);
```

3. Task Processing
```rust
let results = swarm.process_task(&task).await?;
brain.evaluate_results(&task, &results).await?;
```

4. Decision Making
```rust
let decision = brain.make_decision(&swarm.state(), &task).await?;
execute_decision(swarm, decision).await?;
```
