[![License](https://img.shields.io/github/license/georgesheth/swarms-rust)](https://github.com/georgesheth/swarms-rust/blob/main/LICENSE)
[![Discord](https://img.shields.io/discord/999382051935506503?label=swarms%20Discord&logo=discord)](https://discord.com/invite/jM3Z6M9uMq)
[![Twitter](https://img.shields.io/twitter/follow/swarms_corp)](https://x.com/swarms_corp)

# swarms-rust
Rust SDK and CLI for Swarm Framework with Multi-Agent Orchestration

----
swarms-rust aims to provide a Rust SDK and CLI to develop and deploy multi-agent workflows and orchestration systems. This project builds on the Python SDK and CLI of the Swarms Framework and Swarm Cloud ([https://docs.swarms.world/en/latest/](https://docs.swarms.world/en/latest/)). Suggestions, advice, and collaboration are highly encouraged.

----
## Introduction and Motivation

#### Why Rust?

1. Concurrency and Performance:
   - Swarms enables multi-agent communication and concurrency. Rust's powerful asynchronous features make it an excellent choice for building highly concurrent systems. Its growing popularity for high-performance software, such as databases and web servers, aligns with the needs of Swarms.
   - While Python dominates AI model development, Swarms focuses on agent-level orchestration and communication.
   - Rust’s async libraries, like Actix ([https://actix.rs/](https://actix.rs/)), are particularly well-suited for implementing the actor model, making it comparable to Java's Akka library ([https://akka.io/](https://akka.io/)).
   - Rust is a strong typing programming language. We know how useful it will be for network application. 

2. Platform Compatibility:
   - Real-world Swarms often operate in IoT and other low-level environments ([https://docs.swarms.world/en/latest/swarms/concept/swarm_ecosystem/#6-iotagents](https://docs.swarms.world/en/latest/swarms/concept/swarm_ecosystem/#6-iotagents)). Rust’s portability and performance make it ideal for these use cases.

3. Blockchain Integration:
   - Swarms aims to integrate with the Solana blockchain, which requires a Rust library. Rust also works well with other EVM blockchains. 
   - Additionally, my work at Mind Network ([https://www.mindnetwork.xyz/](https://www.mindnetwork.xyz/)), whose core systems are developed in Rust, includes developing agent model consensus functions using FHE technology for Swarms. A Rust library is essential for this integration.

#### Why Swarms?

1. Multi-Agent Orchestration:
    - Multi-agent systems are the future, requiring robust orchestration and consensus capabilities. Swarms focuses on orchestration, while Mind Network provides consensus.

    - Existing AI-agent frameworks (e.g., LangChain, LangGraph, AutoGen) primarily focus on workflow or single agents. Swarms complements these by orchestrating external agents ([https://docs.swarms.world/en/latest/swarms/agents/external_party_agents/](https://docs.swarms.world/en/latest/swarms/agents/external_party_agents/)).

2. Decentralized Orchestration:
   - Unlike frameworks like OpenAI Swarms ([https://github.com/openai/swarm](https://github.com/openai/swarm)) that emphasize routing and handoffs, or AWSLabs Multi-Agent Orchestrator ([https://awslabs.github.io/multi-agent-orchestrator/](https://awslabs.github.io/multi-agent-orchestrator/)) that focuses on centralized orchestration, Swarms enables decentralized orchestration with service mesh capabilities for agent registration and discovery.

3. Future Potential:
    - We envision Swarms becoming the standard for orchestration, with Mind Network’s consensus capabilities collaboratively enabling full-chain AI autonomy.

----
## Concept and Design

#### Agents

An AI agent is an intelligent actor capable of executing one or more tasks. Whether it has a UI or API, an agent processes inputs (e.g., prompts) and produces outputs (e.g., chat completions). While some agents are open source, others are treated as black boxes, requiring standard interfaces for interaction. This highlights the need for a framework to manage complex agent collaboration.


#### Swarms

Swarms provide the mechanism for enabling agents to:

- Communicate with each other.

- Resolve complex tasks by involving multiple agents.

- Operate as a single cohesive unit that can itself act as an agent.

![Concept: Agent and Swarms](/assets/concept-agent-and-swarms.png)

#### Collaboration Models

Agent collaboration in Swarms is modeled using three fundamental interaction patterns:

- Conversation: One agent asks, another responds.

- Speak: One agent speaks, others listen (or ignore).

- Forwarding: One agent forwards a request to another without processing it.

These patterns form the atomic units of Swarm orchestration, with more complex workflows built as combinations of these units. For a detailed explanation, see Swarm Architectures ([https://docs.swarms.world/en/latest/swarms/concept/swarm_architectures/](https://docs.swarms.world/en/latest/swarms/concept/swarm_architectures/)).

![Concept: Agent Collaboration](/assets/concept-agent-collabration-model.png)

#### Intelligent Orchestration

Orchestration extends beyond workflow management to include:

- Agent Discovery: Dynamically identifying and utilizing agents.

- Decision-Making: Integrating intelligence to manage tasks like registration, reflection, and conditioning.

Large Language Models (LLMs) bring new opportunities, such as:

- Recruiting LLMs as judges, supervisors, or digital twins to oversee workflows.
  
![Design: Orchestration](/assets/design-flows.png)

#### API Design

As a twin to the Python and JavaScript libraries, the Rust library will adopt a consistent and modular API design, ensuring seamless integration within the unified Swarms Framework.

![Design: Swarms APIs](/assets/design-swarms-apis.png)


----
## Detailed Design
The detailed design aims to give a standard and spec to implements a Rust-based Swarms framework. The framework enables both centralized and decentralized swarm architectures with blockchain-based identity verification and state tracking. The design details can be accessed in `/design` folder.

### `design/interfaces`
Core trait definitions and abstractions:
- `design/interfaces/agent.md`: Agent capabilities and lifecycle
- `design/interfaces/swarm.md`: Swarm orchestration patterns
- `design/interfaces/task.md`: Task processing and state management
- `design/interfaces/models.md`: LLM integration interfaces
- `design/interfaces/memory.md`: State and context management

### `design/api`
Integration patterns and protocols:
- `design/interfaces/agent_communication.md`: Message passing protocols
- `design/interfaces/swarm_orchestration.md`: Coordination patterns
- `design/interfaces/model_integration.md`: LLM interaction patterns
- `design/interfaces/storage_integration.md`: Data persistence patterns
- `design/interfaces/memory_integration.md`: Context management

### `design/requirements`
Implementation specifications:
- `design/interfaces/core_functions.md`: Essential capabilities
- `design/interfaces/consensus.md`: Agreement protocols
- `design/interfaces/error_handling.md`: Fault tolerance
- `design/interfaces/message_passing.md`: Communication patterns
- `design/interfaces/task_mutability.md`: State management
- `design/interfaces/blockchain_security.md`: Chain integration
- `design/interfaces/self_improvement.md`: Autonomous enhancement

### `design/diagrams`
Architecture visualization:
- `design/interfaces/component_diagram.md`: System structure
- `design/interfaces/sequence_diagrams.md`: Interaction flows
- `design/interfaces/state_diagrams.md`: State transitions
- `design/interfaces/deployment_diagram.md`: Runtime architecture


----
## Phase 1 Implementation
The Phase 1 implementation focuses on aligning with the functionality available in swarms-py and integrating blockchain functionality in preparation for the next phase of consensus implementation.

### mapping between `swarms-rust` and `swarms-py`:
| module | `swarms-rust` | `swarms-py` |
| ---- | ---- | ---- | 
| `swarm_memory` | Implemented and available in [crate](/swarm_memory/) | Implemented and avaialble in [pypi](https://pypi.org/project/swarms-memory/) |
| `swarm_tool` | Implemented and available in [crate](/swarm_tool/) | Implemented and avaialble by giving [examples](https://docs.swarms.world/en/latest/swarms/tools/main/) |
| `swarm_llm` | Implemented and available in [crate](/swarm_llm/) | Implemented and avaialble by [pypi](https://pypi.org/project/swarm-models/) |
| `swarm_wallet` | Implemented and available in [crate](/swarm_wallet/) | not available |
| `swarm_agent` | Implemented and available in [crate](/swarm_agent/) | part of `swarms` pypi |
| `swarms` | Implemented and available in [crate](/swarms/) | Implemented and avaialble  in [pypi](https://pypi.org/project/swarms/) |

#### Notes: 
- **Web3 Wallet Support**: Both **EVM** and **Solana** wallets are now supported in swarms-rust.
- **Testing Coverage**: Test code is provided in each crate, and you can run `cargo test --workspace` to execute all test cases.
- **LLM vs. Non-LLM models**: A dedicated crate for LLM is created. While swarms-py refers to models collectively, it would be better to separate LLM models from non-LLM models. LLMs could serve as control and decison making (e.g., LLM-as-judge), while other models, such as ML models, could focus on specific tasks.

#### Testcase coverages
```shell
$ cargo test --workspace
...
test mock::tests::test_mock_response_generation ... ok
test mock::tests::test_mock_llm_patterns ... ok
test openai::test_mock_response_generation ... ok
test openai::test_openai_client_creation_without_env ... ok
test chromadb::test_chromadb_errors ... ok
test chromadb::test_chromadb_store ... ok
test pinecone::test_pinecone_errors ... ok
test pinecone::test_pinecone_store ... ok
test solana::tests::test_solana_wallet_creation ... ok
test solana::tests::test_solana_wallet_signing ... ok
test evm::tests::test_evm_wallet_creation ... ok
test solana::tests::test_solana_wallet_signing_verify ... ok
test evm::tests::test_evm_wallet_signing ... ok
test evm::tests::test_evm_wallet_signing_verify ... ok
test mock::test_swarm_agent_management ... ok
test mock::test_base_swarm ... ok
test architectures::graph_swarm::tests::test_graph_swarm_empty_input ... ok
test architectures::graph_swarm::tests::test_graph_swarm_invalid_adjacency ... ok
test architectures::grid::tests::test_grid_swarm_invalid_size ... ok
test architectures::circular::tests::test_circular_swarm ... ok
test architectures::fibonacci::tests::test_fibonacci_swarm ... ok
test architectures::geometric::tests::test_geometric_swarm ... ok
test architectures::linear::tests::test_linear_swarm ... ok
test architectures::mesh::tests::test_mesh_swarm ... ok
test architectures::grid::tests::test_grid_swarm ... ok
test architectures::queue_swarm::tests::test_queue_swarm_empty_input ... ok
test architectures::harmonic::tests::test_harmonic_swarm ... ok
test architectures::graph_swarm::tests::test_graph_swarm ... ok
test architectures::round_robin::tests::test_round_robin_empty_input ... ok
test architectures::queue_swarm::tests::test_queue_swarm ... ok
test architectures::pyramid::tests::test_pyramid_swarm ... ok
test architectures::tree_swarm::tests::test_tree_swarm_empty_input ... ok
test architectures::tree_swarm::tests::test_tree_swarm_invalid_relationships ... ok
test architectures::round_robin::tests::test_round_robin_swarm ... ok
test architectures::star::tests::test_star_swarm ... ok
test architectures::prime::tests::test_prime_swarm ... ok
test architectures::tree_swarm::tests::test_tree_swarm ... ok
test architectures::patterns::tests::test_pattern_swarms ... ok

...
```

## Phase 2 Implementation
Phase 2 will utilize `swarms-rust` to develop a Bitcoin Price Prediction and Consensus Swarm in `/demo_btc_prediction_consensus_swarm`.
- Tools: 
  - coingeko
  - yahoo_finance
- Agents:
  - stastical model agent as embeded code
  - machine learning agent as a service
  - openai llm based agent as a service
- swarms:
- dockers: 

#### Testcase coverages
```shell
$ cargo test -p demo_btc_prediction_consensus_swarm
...
test agents::btc_price_statistical_predictor::tests::test_statistical_predictor_basic ... ok
test agents::btc_price_statistical_predictor::tests::test_statistical_predictor_validation ... ok
test tools::yahoo_finance::tests::test_yahoo_finance_tool_validation ... ok
test tools::yahoo_finance::tests::test_yahoo_finance_tool_schema ... ok
test tools::coingecko::test::test_coingecko_api_tool ... ok
test agents::btc_price_ml_predictor::test::test_ml_predictor_validation ... ok
test agents::btc_price_ml_predictor::test::test_ml_predictor_basic ... ok
test agents::btc_price_ml_predictor::test::test_ml_predictor_health_check ... ok
test tools::coingecko::test::test_coingecko_api_tool_run_mock ... ok
test tools::coingecko::test::test_coingecko_api_tool_run ... ok

...
```

----
## Development Roadmap

- [x] Highlevel Abstraction
- [x] Detailed Design: Finalize the detailed design for BaseAgent, BaseModel, and BaseSwarm.
- [x] Phase 1 Rust Implementation: replicated `swarm_memory`, `swarm_tool`, `swarm_llm`, `swarm_agent`, `swarms`. new added `swarms_wallet`.
- [x] Phase 1 test coverage completed.
- [ ] Phase 2 Rust Implementation: Btc Price Prediction and consensus Swarm
  - [x] tools: coingeko, yahoo_finance
  - [x] agents: stastical model agent as embeded code, machine learning agent as a service
  - [ ] agents: openai llm based agent as a service
  - [ ] swarms: 
  - [ ] docker:
- [ ] Rust Implementation on Utility modules
- [ ] Rust Testing: Develop comprehensive test cases based on the Python library.
- [ ] Rust Examples: Recreate examples from the Python library to demonstrate usage.
- [ ] Blockchain Integration: Integrate with blockchain wallet SDKs in Rust.
- [ ] Mind Network Integration: Connect with Mind Network’s SDK to implement agent model consensus.
- [ ] Swarms Cloud Integration

More todo will be added as project goes. 

---- 
## Getting Involved

We welcome contributions and collaboration! Here’s how you can get involved:

- **Suggestions**: Open an issue to share ideas or report bugs.

- **Pull Requests**: Submit pull requests to contribute code.

- **Community**: Join discussions on Swarms's Slack or Discord channels.

Let’s build the future of multi-agent orchestration together!

---- 
For more information, visit [Swarms Documentation](https://docs.swarms.world/en/latest/).
