# Swarms-Rust Prompt Management API

## Overview
Defines the prompt management system for agent-swarm interactions, including system prompts, user prompts, and assistant prompts.

## Prompt Types

### 1. System Prompts
```rust
/// System prompt configuration
pub struct SystemPrompt {
    /// Base behavior definition
    base_behavior: String,
    /// Role-specific instructions
    role_instructions: Vec<String>,
    /// Capability definitions
    capabilities: Vec<Capability>,
    /// Interaction guidelines
    interaction_rules: Vec<String>,
    /// Context window
    context_window: usize,
}

/// System prompt management
pub trait SystemPromptManager {
    /// Create system prompt
    fn create_prompt(&self, config: SystemPromptConfig) -> SystemPrompt;
    
    /// Update system prompt
    fn update_prompt(&mut self, prompt: &mut SystemPrompt, updates: PromptUpdates);
    
    /// Validate system prompt
    fn validate_prompt(&self, prompt: &SystemPrompt) -> Result<(), PromptError>;
    
    /// Format system prompt
    fn format_prompt(&self, prompt: &SystemPrompt) -> String;
}
```

### 2. User Prompts
```rust
/// User prompt structure
pub struct UserPrompt {
    /// Task description
    task: String,
    /// Input parameters
    parameters: HashMap<String, Value>,
    /// Context information
    context: Option<Context>,
    /// Constraints
    constraints: Vec<Constraint>,
}

/// User prompt management
pub trait UserPromptManager {
    /// Create user prompt
    fn create_prompt(&self, task: &str, params: Option<Parameters>) -> UserPrompt;
    
    /// Add context to prompt
    fn add_context(&mut self, prompt: &mut UserPrompt, context: Context);
    
    /// Add constraints
    fn add_constraints(&mut self, prompt: &mut UserPrompt, constraints: Vec<Constraint>);
    
    /// Format user prompt
    fn format_prompt(&self, prompt: &UserPrompt) -> String;
}
```

### 3. Assistant Prompts
```rust
/// Assistant prompt structure
pub struct AssistantPrompt {
    /// Response content
    content: String,
    /// Response type
    response_type: ResponseType,
    /// Metadata
    metadata: ResponseMetadata,
    /// Citations/references
    citations: Vec<Citation>,
}

/// Assistant prompt management
pub trait AssistantPromptManager {
    /// Create assistant prompt
    fn create_prompt(&self, content: &str, metadata: ResponseMetadata) -> AssistantPrompt;
    
    /// Add citations
    fn add_citations(&mut self, prompt: &mut AssistantPrompt, citations: Vec<Citation>);
    
    /// Format assistant prompt
    fn format_prompt(&self, prompt: &AssistantPrompt) -> String;
    
    /// Validate response
    fn validate_response(&self, prompt: &AssistantPrompt) -> Result<(), ResponseError>;
}
```

## Prompt Integration

### 1. Agent Integration
```rust
/// Agent prompt interface
pub trait AgentPromptHandler {
    /// Get system prompt
    fn system_prompt(&self) -> &SystemPrompt;
    
    /// Process user prompt
    async fn process_prompt(&mut self, prompt: &UserPrompt) -> Result<AssistantPrompt, PromptError>;
    
    /// Update system prompt
    fn update_system_prompt(&mut self, updates: SystemPromptUpdates);
    
    /// Get prompt history
    fn prompt_history(&self) -> Vec<PromptInteraction>;
}

/// Prompt interaction record
pub struct PromptInteraction {
    /// Interaction ID
    id: InteractionId,
    /// User prompt
    user_prompt: UserPrompt,
    /// Assistant prompt
    assistant_prompt: AssistantPrompt,
    /// Timestamp
    timestamp: DateTime<Utc>,
    /// Metadata
    metadata: InteractionMetadata,
}
```

### 2. Swarm Integration
```rust
/// Swarm prompt coordination
pub trait SwarmPromptCoordinator {
    /// Distribute prompt to agents
    async fn distribute_prompt(
        &mut self,
        prompt: &UserPrompt,
        agents: &[AgentId],
    ) -> Result<Vec<PromptAssignment>, PromptError>;
    
    /// Collect responses
    async fn collect_responses(
        &mut self,
        assignments: &[PromptAssignment],
    ) -> Result<Vec<AssistantPrompt>, PromptError>;
    
    /// Coordinate system prompts
    fn coordinate_system_prompts(
        &mut self,
        updates: SystemPromptUpdates,
    ) -> Result<(), PromptError>;
}

/// Prompt assignment
pub struct PromptAssignment {
    /// Assignment ID
    id: AssignmentId,
    /// Agent ID
    agent_id: AgentId,
    /// User prompt
    prompt: UserPrompt,
    /// Assignment status
    status: AssignmentStatus,
}
```

## Prompt Templates

### 1. Template Management
```rust
/// Template management
pub trait TemplateManager {
    /// Register template
    fn register_template(&mut self, template: Template) -> Result<TemplateId, TemplateError>;
    
    /// Get template
    fn get_template(&self, id: TemplateId) -> Option<Template>;
    
    /// Render template
    fn render_template(
        &self,
        template: &Template,
        params: &Parameters,
    ) -> Result<String, TemplateError>;
}

/// Template structure
pub struct Template {
    /// Template ID
    id: TemplateId,
    /// Template content
    content: String,
    /// Parameter definitions
    parameters: Vec<ParameterDefinition>,
    /// Validation rules
    validation: Vec<ValidationRule>,
}
```

### 2. Template Categories
```rust
/// Template categories
pub enum TemplateCategory {
    /// System prompt templates
    SystemPrompt(SystemPromptTemplate),
    /// Task templates
    Task(TaskTemplate),
    /// Response templates
    Response(ResponseTemplate),
    /// Error templates
    Error(ErrorTemplate),
}

/// Template repository
pub trait TemplateRepository {
    /// Get templates by category
    fn get_templates(&self, category: TemplateCategory) -> Vec<Template>;
    
    /// Add template to category
    fn add_template(&mut self, category: TemplateCategory, template: Template);
    
    /// Remove template
    fn remove_template(&mut self, id: TemplateId);
}
```

## Error Handling

### 1. Prompt Errors
```rust
/// Prompt error types
pub enum PromptError {
    /// Invalid format
    InvalidFormat(String),
    /// Missing parameters
    MissingParameters(Vec<String>),
    /// Context error
    ContextError(String),
    /// Validation failed
    ValidationFailed(String),
}
```

### 2. Template Errors
```rust
/// Template error types
pub enum TemplateError {
    /// Invalid template
    InvalidTemplate(String),
    /// Parameter error
    ParameterError(String),
    /// Rendering failed
    RenderingFailed(String),
    /// Validation error
    ValidationError(String),
}
```

## Usage Examples

### 1. System Prompt Configuration
```rust
// 1. Create system prompt
let config = SystemPromptConfig {
    base_behavior: "You are a helpful AI assistant",
    role_instructions: vec!["Focus on technical analysis"],
    capabilities: vec![Capability::TextProcessing],
    interaction_rules: vec!["Be concise", "Show reasoning"],
};

let system_prompt = prompt_manager.create_prompt(config);

// 2. Update prompt
let updates = SystemPromptUpdates::new()
    .add_capability(Capability::DataAnalysis)
    .add_rule("Include citations");

prompt_manager.update_prompt(&mut system_prompt, updates);
```

### 2. User Prompt Processing
```rust
// 1. Create user prompt
let user_prompt = UserPrompt::new(
    "Analyze market trends",
    Some(market_parameters),
);

// 2. Add context
prompt_manager.add_context(
    &mut user_prompt,
    market_context,
);

// 3. Process prompt
let response = agent.process_prompt(&user_prompt).await?;
```

### 3. Template Usage
```rust
// 1. Register template
let template = Template::new(
    "market_analysis",
    "Analyze {market} trends over {period}",
);

template_manager.register_template(template)?;

// 2. Render template
let params = Parameters::new()
    .add("market", "crypto")
    .add("period", "1 month");

let prompt = template_manager.render_template(
    &template,
    &params,
)?;
```
