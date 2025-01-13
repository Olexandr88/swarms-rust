# Swarms-Rust Interaction Sequences

## Task Processing Sequence
```mermaid
sequenceDiagram
    participant U as User
    participant S as Swarm
    participant SB as SwarmBrain
    participant A1 as Agent1
    participant A2 as Agent2
    participant ST as Storage
    participant M as Memory

    U->>S: Submit Task
    S->>SB: Request Agent Selection
    SB->>M: Load Context
    SB-->>S: Selected Agents
    
    S->>A1: Assign Task
    A1->>ST: Load Config
    A1->>M: Load Context
    A1->>A1: Process Task
    A1->>ST: Store Result
    A1-->>S: Task Result

    S->>A2: Verify Result
    A2->>ST: Load Result
    A2->>A2: Verify
    A2-->>S: Verification

    S->>ST: Store Final Result
    S-->>U: Return Result
```

## On-Chain Vote Collection and Consensus Sequence
```mermaid
sequenceDiagram
    participant S as Swarm
    participant A1 as VotingAgent1
    participant A2 as VotingAgent2
    participant SC as SmartContract
    participant W as Wallet
    participant BC as Blockchain

    S->>BC: Deploy VoteCollector
    S->>A1: Request Vote
    S->>A2: Request Vote

    A1->>W: Sign Vote
    A1->>SC: Submit Vote + Signature
    SC->>BC: Record Vote
    BC-->>A1: Transaction Receipt

    A2->>W: Sign Vote
    A2->>SC: Submit Vote + Signature
    SC->>BC: Record Vote
    BC-->>A2: Transaction Receipt

    S->>BC: Query Votes
    BC-->>S: Vote Results
    S->>S: Process Consensus
```

## Off-Chain Vote Collection and Consensus Sequence
```mermaid
sequenceDiagram
    participant S as Swarm
    participant A1 as VotingAgent1
    participant A2 as VotingAgent2
    participant VC as VoteCollector
    participant W as Wallet
    participant DB as Database

    S->>VC: Initialize Collection
    S->>A1: Request Vote
    S->>A2: Request Vote

    A1->>W: Sign Vote
    A1->>VC: Submit Vote + Signature
    VC->>W: Verify Signature
    VC->>DB: Store Vote
    VC-->>A1: Confirmation

    A2->>W: Sign Vote
    A2->>VC: Submit Vote + Signature
    VC->>W: Verify Signature
    VC->>DB: Store Vote
    VC-->>A2: Confirmation

    S->>VC: Query Votes
    VC->>DB: Fetch Votes
    DB-->>VC: Vote Data
    VC-->>S: Vote Results
    S->>S: Process Consensus
```

## Hybrid On-Chain/Off-Chain Consensus Sequence
```mermaid
sequenceDiagram
    participant S as Swarm
    participant A as Agent
    participant PC as PrimaryCollector
    participant BC as BackupCollector
    participant W as Wallet
    participant BL as Blockchain

    S->>PC: Initialize Primary
    S->>BC: Initialize Backup
    S->>A: Request Vote

    A->>W: Sign Vote
    A->>PC: Submit Vote + Signature
    
    alt Primary Success
        PC->>BL: Record Vote
        BL-->>A: Transaction Receipt
    else Primary Fails
        PC-->>A: Error
        A->>BC: Submit Vote + Signature
        BC->>BL: Record Vote
        BL-->>A: Transaction Receipt
    end

    S->>PC: Query Votes
    PC->>BL: Query Primary Chain
    alt Primary Available
        BL-->>PC: Vote Data
        PC-->>S: Vote Results
    else Primary Unavailable
        S->>BC: Query Votes
        BC->>BL: Query Backup Chain
        BL-->>BC: Vote Data
        BC-->>S: Vote Results
    end

    S->>S: Process Consensus
```

## On-Chain Consensus Building and Decision Sequence
```mermaid
sequenceDiagram
    participant S as Swarm
    participant CM as ConsensusManager
    participant A1 as Agent1
    participant A2 as Agent2
    participant W as Wallet
    participant BL as Blockchain

    S->>CM: Initialize Consensus
    CM->>BL: Create Proposal

    par Collect Votes
        CM->>A1: Request Vote
        A1->>W: Sign Vote
        A1->>BL: Submit Vote + Signature
        BL-->>A1: Transaction Receipt
    and
        CM->>A2: Request Vote
        A2->>W: Sign Vote
        A2->>BL: Submit Vote + Signature
        BL-->>A2: Transaction Receipt
    end

    CM->>BL: Query Votes
    BL-->>CM: Vote Results

    CM->>CM: Validate Votes
    CM->>CM: Calculate Results
    
    alt Consensus Reached
        CM->>BL: Record Decision
        CM-->>S: Consensus Decision
        S->>S: Execute Decision
    else No Consensus
        CM->>BL: Record Failure
        CM-->>S: Consensus Failed
        S->>S: Handle Failure
    end
```

## Consensus Building Sequence
```mermaid
sequenceDiagram
    participant S as Swarm
    participant A1 as Agent1
    participant A2 as Agent2
    participant A3 as Agent3
    participant C as Consensus
    participant W as Wallet
    participant BL as Blockchain

    S->>+C: Initialize Consensus
    C->>A1: Request Vote
    C->>A2: Request Vote
    C->>A3: Request Vote

    A1->>W: Sign Vote
    A1->>BL: Submit Vote + Signature
    BL-->>A1: Transaction Receipt
    
    A2->>W: Sign Vote
    A2->>BL: Submit Vote + Signature
    BL-->>A2: Transaction Receipt
    
    A3->>W: Sign Vote
    A3->>BL: Submit Vote + Signature
    BL-->>A3: Transaction Receipt

    C->>BL: Query Votes
    BL-->>C: Vote Results
    C->>C: Process Votes
    C->>BL: Record Decision
    C-->>-S: Consensus Result
```

## Storage Operation Sequence
```mermaid
sequenceDiagram
    participant A as Agent
    participant KV as KeyValue
    participant R as Relational
    participant F as File
    participant E as Encryption

    A->>KV: Store Config
    KV->>E: Encrypt Data
    E-->>KV: Encrypted
    KV-->>A: Stored

    A->>R: Store Metadata
    R->>E: Encrypt Data
    E-->>R: Encrypted
    R-->>A: Stored

    A->>F: Store Temp
    F-->>A: Stored
```

## Memory Integration Sequence
```mermaid
sequenceDiagram
    participant A as Agent
    participant V as VectorStore
    participant C as Context
    participant E as Embeddings
    participant M as Models

    A->>M: Generate Embedding
    M-->>A: Vector

    A->>V: Store Vector
    V-->>A: Stored

    A->>C: Update Context
    C->>V: Query Similar
    V-->>C: Results
    C-->>A: Context
```

## Wallet Integration Sequence
```mermaid
sequenceDiagram
    participant A as Agent
    participant W as Wallet
    participant BC as Blockchain
    participant S as Swarm
    participant V as Verifier

    A->>W: Initialize Wallet
    W->>BC: Get Address
    BC-->>W: Address
    W-->>A: Ready

    A->>W: Sign Message
    W->>W: Generate Signature
    W-->>A: Signature

    A->>S: Submit Signed Result
    S->>V: Verify Signature
    V->>BC: Check Address
    BC-->>V: Valid
    V-->>S: Verified
    S-->>A: Accepted

    A->>W: Sign Transaction
    W->>BC: Submit Transaction
    BC-->>W: Transaction Hash
    W-->>A: Confirmation
```

## Wallet Operation States
```mermaid
sequenceDiagram
    participant A as Agent
    participant W as Wallet
    participant K as KeyStore
    participant P as Permission
    participant C as Contract

    A->>W: Request Operation
    W->>K: Load Keys
    K-->>W: Keys Loaded
    
    W->>P: Check Permission
    P-->>W: Authorized
    
    W->>C: Verify Contract
    C-->>W: Safe
    
    W->>W: Sign Operation
    W-->>A: Operation Ready
```

## Multi-Wallet Consensus
```mermaid
sequenceDiagram
    participant S as Swarm
    participant A1 as Agent1
    participant W1 as Wallet1
    participant A2 as Agent2
    participant W2 as Wallet2
    participant BC as Blockchain

    S->>A1: Request Vote
    A1->>W1: Sign Vote
    W1-->>A1: Signature1
    A1-->>S: Signed Vote1

    S->>A2: Request Vote
    A2->>W2: Sign Vote
    W2-->>A2: Signature2
    A2-->>S: Signed Vote2

    S->>BC: Submit Votes
    BC->>BC: Verify Signatures
    BC-->>S: Consensus Result
```
