# Swarms-Rust Message Passing Requirements

## Overview
Defines the message passing system between agents and swarms following the actor model pattern.

## Core Components

```rust
/// Message types for agent communication
pub enum AgentMessage {
    /// Task assignment
    TaskAssignment {
        task: Task,
        deadline: Option<DateTime<Utc>>,
        priority: Priority,
    },
    /// Task result
    TaskResult {
        task_id: TaskId,
        result: TaskResults,
        signature: Option<String>,
    },
    /// Status update
    StatusUpdate {
        agent_id: AgentId,
        status: AgentStatus,
        metrics: Option<AgentMetrics>,
    },
    /// Resource request
    ResourceRequest {
        resource_type: ResourceType,
        quantity: usize,
        priority: Priority,
    },
    /// Resource response
    ResourceResponse {
        request_id: RequestId,
        resource: Option<Resource>,
        error: Option<String>,
    },
    /// Collaboration request
    CollaborationRequest {
        task: Task,
        required_capabilities: Vec<Capability>,
        deadline: Option<DateTime<Utc>>,
    },
    /// Collaboration response
    CollaborationResponse {
        request_id: RequestId,
        can_collaborate: bool,
        available_capabilities: Vec<Capability>,
    },
}

/// Message types for swarm communication
pub enum SwarmMessage {
    /// Agent registration
    AgentRegistration {
        agent_id: AgentId,
        capabilities: Vec<Capability>,
        endpoint: Option<String>,
    },
    /// Agent deregistration
    AgentDeregistration {
        agent_id: AgentId,
        reason: Option<String>,
    },
    /// Task distribution
    TaskDistribution {
        tasks: Vec<Task>,
        strategy: DistributionStrategy,
    },
    /// Result collection
    ResultCollection {
        task_id: TaskId,
        results: Vec<TaskResults>,
        consensus_required: bool,
    },
    /// State synchronization
    StateSynchronization {
        swarm_id: SwarmId,
        state: SwarmState,
        timestamp: DateTime<Utc>,
    },
    /// Resource coordination
    ResourceCoordination {
        resource_type: ResourceType,
        allocation: ResourceAllocation,
    },
}

/// Message delivery guarantees
pub enum DeliveryGuarantee {
    /// At most once delivery
    AtMostOnce,
    /// At least once delivery
    AtLeastOnce,
    /// Exactly once delivery
    ExactlyOnce,
}

/// Message routing strategy
pub enum RoutingStrategy {
    /// Direct point-to-point
    Direct {
        recipient: ActorId,
        timeout: Duration,
    },
    /// Broadcast to all
    Broadcast {
        filter: Option<BroadcastFilter>,
    },
    /// Multicast to group
    Multicast {
        group: GroupId,
        min_recipients: usize,
    },
    /// Content-based routing
    ContentBased {
        predicate: Box<dyn Fn(&Message) -> bool>,
    },
}

/// Message handler interface
#[async_trait]
pub trait MessageHandler: Send + Sync {
    /// Handle incoming message
    async fn handle_message(&mut self, message: Message) -> Result<(), MessageError>;
    
    /// Get supported message types
    fn supported_messages(&self) -> Vec<MessageType>;
    
    /// Get message handling metrics
    fn metrics(&self) -> MessageMetrics;
}
```

## Message Flow Patterns

### 1. Task Distribution
```rust
impl Swarm {
    async fn distribute_task(&mut self, task: Task) -> Result<(), SwarmError> {
        // Create task assignment message
        let message = AgentMessage::TaskAssignment {
            task: task.clone(),
            deadline: task.deadline(),
            priority: task.priority(),
        };
        
        // Select capable agents
        let capable_agents = self.select_capable_agents(&task);
        
        // Distribute to agents
        for agent in capable_agents {
            self.message_broker
                .send(
                    message.clone(),
                    RoutingStrategy::Direct {
                        recipient: agent.id(),
                        timeout: Duration::from_secs(30),
                    },
                    DeliveryGuarantee::AtLeastOnce,
                )
                .await?;
        }
        
        Ok(())
    }
}
```

### 2. Result Collection
```rust
impl Swarm {
    async fn collect_results(&mut self, task_id: TaskId) -> Result<Vec<TaskResults>, SwarmError> {
        // Create result collection message
        let message = SwarmMessage::ResultCollection {
            task_id,
            results: Vec::new(),
            consensus_required: true,
        };
        
        // Broadcast to all agents
        self.message_broker
            .send(
                message,
                RoutingStrategy::Broadcast {
                    filter: Some(BroadcastFilter::TaskParticipants(task_id)),
                },
                DeliveryGuarantee::ExactlyOnce,
            )
            .await?;
            
        // Wait for and collect results
        let results = self.result_collector
            .collect_results(task_id)
            .await?;
            
        Ok(results)
    }
}
```

### 3. State Synchronization
```rust
impl Swarm {
    async fn synchronize_state(&mut self) -> Result<(), SwarmError> {
        // Create state sync message
        let message = SwarmMessage::StateSynchronization {
            swarm_id: self.id(),
            state: self.current_state(),
            timestamp: Utc::now(),
        };
        
        // Multicast to swarm group
        self.message_broker
            .send(
                message,
                RoutingStrategy::Multicast {
                    group: self.group_id(),
                    min_recipients: self.quorum_size(),
                },
                DeliveryGuarantee::AtLeastOnce,
            )
            .await?;
            
        Ok(())
    }
}
```

## Implementation Requirements

### 1. Message Delivery
- Guaranteed delivery
- Order preservation
- Duplicate detection
- Timeout handling
- Error recovery

### 2. Message Routing
- Dynamic routing
- Content-based routing
- Priority handling
- Load balancing
- Flow control

### 3. State Management
- Message persistence
- State synchronization
- Consistency checking
- Recovery points
- History tracking

### 4. Performance
- Asynchronous processing
- Batch operations
- Message compression
- Resource management
- Monitoring

## Security Considerations

### 1. Message Security
- Authentication
- Authorization
- Encryption
- Integrity checking
- Replay protection

### 2. Access Control
- Sender verification
- Recipient validation
- Permission checking
- Role enforcement
- Audit logging

### 3. Resource Protection
- Rate limiting
- Resource quotas
- DoS prevention
- Circuit breaking
- Isolation

## Usage Examples

### 1. Agent Communication
```rust
impl Agent {
    async fn handle_task_assignment(
        &mut self,
        message: AgentMessage,
    ) -> Result<(), MessageError> {
        match message {
            AgentMessage::TaskAssignment { task, deadline, priority } => {
                // Verify capabilities
                if !self.can_handle_task(&task) {
                    return Ok(());
                }
                
                // Process task
                let result = self.process_task(&task).await?;
                
                // Sign result if required
                let signature = if task.requires_verification() {
                    Some(self.sign_result(&result).await?)
                } else {
                    None
                };
                
                // Send result message
                self.message_broker
                    .send(
                        AgentMessage::TaskResult {
                            task_id: task.id(),
                            result,
                            signature,
                        },
                        RoutingStrategy::Direct {
                            recipient: task.owner(),
                            timeout: Duration::from_secs(30),
                        },
                        DeliveryGuarantee::AtLeastOnce,
                    )
                    .await?;
            }
            _ => return Err(MessageError::UnsupportedMessage),
        }
        
        Ok(())
    }
}
```

### 2. Swarm Coordination
```rust
impl Swarm {
    async fn coordinate_resources(
        &mut self,
        allocation: ResourceAllocation,
    ) -> Result<(), SwarmError> {
        // Create coordination message
        let message = SwarmMessage::ResourceCoordination {
            resource_type: allocation.resource_type(),
            allocation: allocation.clone(),
        };
        
        // Send to all affected agents
        for agent in allocation.affected_agents() {
            self.message_broker
                .send(
                    message.clone(),
                    RoutingStrategy::Direct {
                        recipient: agent,
                        timeout: Duration::from_secs(30),
                    },
                    DeliveryGuarantee::ExactlyOnce,
                )
                .await?;
        }
        
        Ok(())
    }
}
```

## Error Handling

### 1. Message Errors
```rust
#[derive(Debug, thiserror::Error)]
pub enum MessageError {
    #[error("Message delivery failed: {0}")]
    DeliveryFailed(String),
    
    #[error("Message timeout: {0}")]
    Timeout(String),
    
    #[error("Invalid message format: {0}")]
    InvalidFormat(String),
    
    #[error("Unsupported message type")]
    UnsupportedMessage,
    
    #[error("Authorization failed: {0}")]
    AuthorizationFailed(String),
}
```

### 2. Recovery Patterns
```rust
impl MessageBroker {
    async fn handle_delivery_failure(
        &mut self,
        error: MessageError,
        message: Message,
        strategy: RetryStrategy,
    ) -> Result<(), MessageError> {
        match strategy {
            RetryStrategy::Immediate { max_attempts } => {
                self.retry_immediately(message, max_attempts).await
            }
            RetryStrategy::Delayed { max_attempts, delay } => {
                self.retry_with_delay(message, max_attempts, delay).await
            }
            RetryStrategy::ExponentialBackoff { max_attempts, base_delay } => {
                self.retry_with_backoff(message, max_attempts, base_delay).await
            }
        }
    }
}
```
