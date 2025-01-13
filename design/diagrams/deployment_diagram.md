# Swarms-Rust Deployment Architecture

## Deployment Patterns

### 1. Embedded Deployment
```mermaid
graph TB
    subgraph Client Applications
        CLI[CLI Tools]
        SDK[SDK Integration]
        API[HTTP API]
    end

    subgraph Swarm
        subgraph Internal Components
            SO[Swarm Orchestrator]
            SB[Swarm Brain]
            SC[Swarm Consensus]
            DB1[Local Storage]
        end

        subgraph Embedded Agents
            EA1[Agent 1]
            EA2[Agent 2]
            EA3[Agent 3]
        end
    end

    subgraph External Services
        LLM[Language Models]
        BC[Blockchain]
        ES[External Services]
    end

    CLI --> API
    SDK --> API
    API --> SO

    SO --> EA1
    SO --> EA2
    SO --> EA3

    EA1 --> DB1
    EA2 --> DB1
    EA3 --> DB1

    EA1 --> ES
    EA2 --> ES
    EA3 --> ES
    
    EA1 --> LLM
    EA2 --> LLM
    EA3 --> LLM
    
    EA1 --> BC
    EA2 --> BC
    EA3 --> BC

    SB --> LLM
    SC --> BC
```

### 2. Service-Based Deployment
```mermaid
graph TB
    subgraph Client Applications
        CLI[CLI Tools]
        SDK[SDK Integration]
        API[HTTP API]
    end

    subgraph Swarm Service
        subgraph Swarm Components
            SO[Swarm Orchestrator]
            SB[Swarm Brain]
            SC[Swarm Consensus]
            DB1[Local Storage]
        end
    end

    subgraph Agent Services
        subgraph Agent Service 1
            AS1[Agent Service]
            DB2[Local Storage]
        end

        subgraph Agent Service 2
            AS2[Agent Service]
            DB3[Local Storage]
        end
    end

    subgraph External Services
        LLM[Language Models]
        BC[Blockchain]
        ES[External Services]
    end

    CLI --> API
    SDK --> API
    API --> SO

    SO --> AS1
    SO --> AS2

    AS1 --> DB2
    AS2 --> DB3
    SO --> DB1

    AS1 --> ES
    AS2 --> ES
    
    AS1 --> LLM
    AS2 --> LLM
    
    AS1 --> BC
    AS2 --> BC

    SB --> LLM
    SC --> BC

    subgraph Service Access Methods
        HTTP[HTTP REST API]
        RPC[RPC Service]
        
        SO --> HTTP
        SO --> RPC
        HTTP --> AS1
        HTTP --> AS2
        RPC --> AS1
        RPC --> AS2
    end

    SO --- note1["Orchestrator invokes agents via: <br> - HTTP REST endpoints <br> - RPC service endpoints <br> Each agent can support one or both"]
```

### 3. Hybrid Deployment
```mermaid
graph TB
    subgraph Client Applications
        CLI[CLI Tools]
        SDK[SDK Integration]
        API[HTTP API]
    end

    subgraph Swarm
        subgraph Swarm Components
            SO[Swarm Orchestrator]
            SB[Swarm Brain]
            SC[Swarm Consensus]
            DB1[Local Storage]
        end

        subgraph Embedded Agents
            EA1[Agent 1]
            EA2[Agent 2]
        end
    end

    subgraph External Agent Services
        subgraph Agent Service 1
            AS1[Agent Service]
            DB2[Local Storage]
        end
    end

    subgraph External Services
        LLM[Language Models]
        BC[Blockchain]
        ES[External Services]
    end

    CLI --> API
    SDK --> API
    API --> SO

    SO --> EA1
    SO --> EA2
    SO --> AS1

    EA1 --> DB1
    EA2 --> DB1
    AS1 --> DB2

    EA1 --> ES
    EA2 --> ES
    AS1 --> ES
    
    EA1 --> LLM
    EA2 --> LLM
    AS1 --> LLM
    
    EA1 --> BC
    EA2 --> BC
    AS1 --> BC

    SB --> LLM
    SC --> BC

    EA1 ---|"Note"| EmbeddedAgentsNote
    EmbeddedAgentsNote[Note: Embedded agents are called directly as callbacks by the orchestrator]
    
    AS1 ---|"Note"| ServiceAgentsNote
    ServiceAgentsNote[Note: Service agents support both HTTP REST endpoints and RPC service endpoints]


    subgraph Service Access Methods
        HTTP[HTTP REST API]
        RPC[RPC Service]
        
        SO --> HTTP
        SO --> RPC
        HTTP --> AS1
        RPC --> AS1
    end note
```

## Deployment Details

### Client Layer
- CLI Tools: Command-line interface
- SDK Integration: Library integration
- HTTP API: RESTful interface

### Core Services
1. Swarm Service
   - Orchestrator: Task management
   - Brain: Decision making
   - Consensus: Agreement protocols

2. Agent Service
   - Manager: Agent lifecycle
   - Pool: Agent instances
   - Tools: Processing capabilities

3. Storage Services
   - Key-Value: Fast access
   - Relational: Structured data
   - File: Large objects
   - Blockchain: Immutable ledger storage

4. Memory Service
   - Vector: Embeddings
   - Context: History
   - Metadata: System data

### External Services
- Language Models: AI processing
- Blockchain: Identity/verification
- External Services: Integration points

## Scaling and Availability

### Atomic Unit Scaling
```mermaid
graph TB
    subgraph Gateway
        GLB[Gateway LB]
    end

    subgraph Swarm Unit 1
        subgraph S1[Swarm Instance 1]
            SO1[Orchestrator]
            LB1[Load Balancer]
            DB1[Storage]
            
            subgraph Agents 1
                A11[Agent 1]
                A12[Agent 2]
            end
        end
    end

    subgraph Swarm Unit 2
        subgraph S2[Swarm Instance 2]
            SO2[Orchestrator]
            LB2[Load Balancer]
            DB2[Storage]
            
            subgraph Agents 2
                A21[Agent 1]
                A22[Agent 2]
            end
        end
    end

    GLB --> LB1
    GLB --> LB2

    LB1 --> SO1
    SO1 --> A11
    SO1 --> A12
    A11 --> DB1
    A12 --> DB1

    LB2 --> SO2
    SO2 --> A21
    SO2 --> A22
    A21 --> DB2
    A22 --> DB2
```

### High Availability with Atomic Units
```mermaid
graph TB
    subgraph Primary Region
        subgraph P_Swarm[Primary Swarm]
            P_LB[Load Balancer]
            P_O[Orchestrator]
            P_DB[Storage]
            
            subgraph P_Agents[Agents]
                PA1[Agent 1]
                PA2[Agent 2]
            end
        end
    end

    subgraph Backup Region
        subgraph B_Swarm[Backup Swarm]
            B_LB[Load Balancer]
            B_O[Orchestrator]
            B_DB[Storage]
            
            subgraph B_Agents[Agents]
                BA1[Agent 1]
                BA2[Agent 2]
            end
        end
    end

    P_LB --> P_O
    P_O --> PA1
    P_O --> PA2
    PA1 --> P_DB
    PA2 --> P_DB
    P_DB --> B_DB

    B_LB --> B_O
    B_O --> BA1
    B_O --> BA2
    BA1 --> B_DB
    BA2 --> B_DB
```
