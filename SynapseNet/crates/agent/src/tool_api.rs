//! Tool API - Unified interface for all tools

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Tool input parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInput {
    pub params: serde_json::Value,
    pub context: ExecutionContext,
}

/// Execution context for tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub goal_id: Uuid,
    pub episode_id: Option<Uuid>,
    pub user_id: String,
    pub timestamp: i64,
    pub metadata: HashMap<String, String>,
}

/// Tool output result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    pub result: serde_json::Value,
    pub metadata: ToolMetadata,
}

/// Tool execution metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub execution_time_ms: u64,
    pub resources_used: ResourceUsage,
    pub success: bool,
    pub error: Option<String>,
}

/// Resource usage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_ms: u64,
    pub memory_mb: u64,
    pub network_bytes: u64,
}

/// Tool schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSchema {
    pub name: String,
    pub description: String,
    pub version: String,
    pub parameters: Vec<ToolParameter>,
    pub returns: ToolReturn,
}

/// Tool parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameter {
    pub name: String,
    pub description: String,
    pub param_type: String,
    pub required: bool,
    pub default: Option<serde_json::Value>,
}

/// Tool return type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolReturn {
    pub return_type: String,
    pub description: String,
}

/// Tool information for registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub enabled: bool,
    pub requires_approval: bool,
}

/// Main Tool trait - all tools must implement this
#[async_trait]
pub trait Tool: Send + Sync {
    /// Tool name (unique identifier)
    fn name(&self) -> &'static str;
    
    /// Human-readable description
    fn description(&self) -> &'static str;
    
    /// Tool version
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    /// Tool schema (parameters and return type)
    fn schema(&self) -> ToolSchema;
    
    /// Execute the tool
    async fn execute(&self, input: ToolInput) -> Result<ToolOutput, ToolError>;
    
    /// Validate input before execution
    fn validate_input(&self, input: &ToolInput) -> Result<(), ToolError> {
        // Default implementation - can be overridden
        if input.params.is_null() {
            return Err(ToolError::InvalidInput("Input parameters are null".to_string()));
        }
        Ok(())
    }
}

/// Tool execution errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolError {
    InvalidInput(String),
    ExecutionFailed(String),
    Timeout,
    ResourceLimitExceeded,
    PermissionDenied,
    NotFound,
    NetworkError(String),
    SandboxViolation(String),
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::ExecutionFailed(msg) => write!(f, "Execution failed: {}", msg),
            Self::Timeout => write!(f, "Execution timeout"),
            Self::ResourceLimitExceeded => write!(f, "Resource limit exceeded"),
            Self::PermissionDenied => write!(f, "Permission denied"),
            Self::NotFound => write!(f, "Tool not found"),
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Self::SandboxViolation(msg) => write!(f, "Sandbox violation: {}", msg),
        }
    }
}

impl std::error::Error for ToolError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_input_creation() {
        let context = ExecutionContext {
            goal_id: Uuid::new_v4(),
            episode_id: None,
            user_id: "test_user".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            metadata: HashMap::new(),
        };
        
        let input = ToolInput {
            params: serde_json::json!({"key": "value"}),
            context,
        };
        
        assert!(!input.params.is_null());
    }

    #[test]
    fn test_tool_error_display() {
        let error = ToolError::InvalidInput("test error".to_string());
        assert_eq!(error.to_string(), "Invalid input: test error");
    }

    #[test]
    fn test_resource_usage() {
        let usage = ResourceUsage {
            cpu_ms: 100,
            memory_mb: 50,
            network_bytes: 1024,
        };
        
        assert_eq!(usage.cpu_ms, 100);
        assert_eq!(usage.memory_mb, 50);
    }
}
