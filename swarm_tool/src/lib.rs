use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;


/// Represents a tool that can be used by an agent
pub trait Tool: Send + Sync + Debug{
    fn run(&self, input: &str) -> Result<String, String>;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    /// Get the tool's schema for input validation
    fn schema(&self) -> Option<&ToolSchema> {
        None
    }
}

/// Schema for tool input validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSchema {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParameter>,
}

/// Parameter definition for tool schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    pub description: String,
    pub parameter_type: ParameterType,
    pub required: bool,
}

/// Supported parameter types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
    Array,
    Object,
}