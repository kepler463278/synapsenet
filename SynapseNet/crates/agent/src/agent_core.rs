//! Agent Core - Main action execution loop

use crate::sandbox::{Sandbox, SandboxConfig};
use crate::tool_api::{ExecutionContext, Tool, ToolError, ToolInput, ToolOutput};
use crate::tool_registry::ToolRegistry;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Action trace for a goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionTrace {
    pub goal_id: Uuid,
    pub actions: Vec<ActionLog>,
    pub total_execution_time_ms: u64,
    pub success: bool,
    pub final_result: Option<serde_json::Value>,
}

/// Single action log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionLog {
    pub id: Uuid,
    pub goal_id: Uuid,
    pub episode_id: Option<Uuid>,
    pub tool_name: String,
    pub input: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub timestamp: i64,
    pub execution_time_ms: u64,
    pub success: bool,
}

/// Agent core for executing actions
pub struct AgentCore {
    tool_registry: Arc<ToolRegistry>,
    sandbox: Sandbox,
}

impl AgentCore {
    /// Create new agent core
    pub fn new(tool_registry: Arc<ToolRegistry>, sandbox_config: SandboxConfig) -> Self {
        Self {
            tool_registry,
            sandbox: Sandbox::new(sandbox_config),
        }
    }

    /// Create with default configuration
    pub fn with_defaults(tool_registry: Arc<ToolRegistry>) -> Self {
        Self::new(tool_registry, SandboxConfig::default())
    }

    /// Execute a single action
    pub async fn execute_action(
        &self,
        tool_name: &str,
        input: ToolInput,
    ) -> Result<ActionLog, ToolError> {
        let start = std::time::Instant::now();
        let action_id = Uuid::new_v4();

        // Check policy
        self.tool_registry.check_policy(tool_name, &input.context.user_id)?;

        // Get tool
        let tool = self.tool_registry.get(tool_name)
            .ok_or(ToolError::NotFound)?;

        // Validate input
        tool.validate_input(&input)?;

        // Execute in sandbox
        let result = self.sandbox.execute_async(|| async {
            tool.execute(input.clone()).await
        }).await;

        let execution_time_ms = start.elapsed().as_millis() as u64;

        // Record usage
        let success = result.is_ok();
        self.tool_registry.record_usage(tool_name, success);

        // Create action log
        let log = match result {
            Ok(sandbox_result) => {
                let output = sandbox_result.value?;
                ActionLog {
                    id: action_id,
                    goal_id: input.context.goal_id,
                    episode_id: input.context.episode_id,
                    tool_name: tool_name.to_string(),
                    input: input.params,
                    output: Some(output.result),
                    error: None,
                    timestamp: chrono::Utc::now().timestamp(),
                    execution_time_ms,
                    success: true,
                }
            }
            Err(e) => ActionLog {
                id: action_id,
                goal_id: input.context.goal_id,
                episode_id: input.context.episode_id,
                tool_name: tool_name.to_string(),
                input: input.params,
                output: None,
                error: Some(e.to_string()),
                timestamp: chrono::Utc::now().timestamp(),
                execution_time_ms,
                success: false,
            },
        };

        if log.success {
            Ok(log)
        } else {
            Err(ToolError::ExecutionFailed(log.error.unwrap()))
        }
    }

    /// Execute multiple actions for a goal
    pub async fn execute_goal(&self, goal_id: Uuid, actions: Vec<(String, ToolInput)>) -> ActionTrace {
        let start = std::time::Instant::now();
        let mut action_logs = Vec::new();
        let mut final_result = None;
        let mut overall_success = true;

        for (tool_name, input) in actions {
            match self.execute_action(&tool_name, input).await {
                Ok(log) => {
                    final_result = log.output.clone();
                    action_logs.push(log);
                }
                Err(e) => {
                    overall_success = false;
                    let error_log = ActionLog {
                        id: Uuid::new_v4(),
                        goal_id,
                        episode_id: None,
                        tool_name,
                        input: serde_json::Value::Null,
                        output: None,
                        error: Some(e.to_string()),
                        timestamp: chrono::Utc::now().timestamp(),
                        execution_time_ms: 0,
                        success: false,
                    };
                    action_logs.push(error_log);
                    break;
                }
            }
        }

        let total_execution_time_ms = start.elapsed().as_millis() as u64;

        ActionTrace {
            goal_id,
            actions: action_logs,
            total_execution_time_ms,
            success: overall_success,
            final_result,
        }
    }

    /// Get tool registry
    pub fn registry(&self) -> &Arc<ToolRegistry> {
        &self.tool_registry
    }

    /// Get sandbox
    pub fn sandbox(&self) -> &Sandbox {
        &self.sandbox
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool_api::*;
    use async_trait::async_trait;
    use std::collections::HashMap;

    struct TestTool;

    #[async_trait]
    impl Tool for TestTool {
        fn name(&self) -> &'static str {
            "test_tool"
        }

        fn description(&self) -> &'static str {
            "Test tool"
        }

        fn schema(&self) -> ToolSchema {
            ToolSchema {
                name: "test_tool".to_string(),
                description: "Test".to_string(),
                version: "1.0.0".to_string(),
                parameters: vec![],
                returns: ToolReturn {
                    return_type: "string".to_string(),
                    description: "Result".to_string(),
                },
            }
        }

        async fn execute(&self, _input: ToolInput) -> Result<ToolOutput, ToolError> {
            Ok(ToolOutput {
                result: serde_json::json!("test_result"),
                metadata: ToolMetadata {
                    execution_time_ms: 10,
                    resources_used: ResourceUsage {
                        cpu_ms: 5,
                        memory_mb: 1,
                        network_bytes: 0,
                    },
                    success: true,
                    error: None,
                },
            })
        }
    }

    #[tokio::test]
    async fn test_agent_core_creation() {
        let registry = Arc::new(ToolRegistry::new());
        let agent = AgentCore::with_defaults(registry);
        
        assert!(agent.registry().list().is_empty());
    }

    #[tokio::test]
    async fn test_execute_action() {
        let registry = Arc::new(ToolRegistry::new());
        registry.register(Box::new(TestTool)).unwrap();
        
        let agent = AgentCore::with_defaults(registry);
        
        let context = ExecutionContext {
            goal_id: Uuid::new_v4(),
            episode_id: None,
            user_id: "test_user".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            metadata: HashMap::new(),
        };
        
        let input = ToolInput {
            params: serde_json::json!({}),
            context,
        };
        
        let result = agent.execute_action("test_tool", input).await;
        assert!(result.is_ok());
        
        let log = result.unwrap();
        assert!(log.success);
        assert_eq!(log.tool_name, "test_tool");
    }

    #[tokio::test]
    async fn test_execute_goal() {
        let registry = Arc::new(ToolRegistry::new());
        registry.register(Box::new(TestTool)).unwrap();
        
        let agent = AgentCore::with_defaults(registry);
        let goal_id = Uuid::new_v4();
        
        let context = ExecutionContext {
            goal_id,
            episode_id: None,
            user_id: "test_user".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            metadata: HashMap::new(),
        };
        
        let actions = vec![
            ("test_tool".to_string(), ToolInput {
                params: serde_json::json!({}),
                context: context.clone(),
            }),
        ];
        
        let trace = agent.execute_goal(goal_id, actions).await;
        assert!(trace.success);
        assert_eq!(trace.actions.len(), 1);
    }
}
