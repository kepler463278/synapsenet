//! Math Evaluation Tool - Safe mathematical computations

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use synapsenet_agent::*;

/// Math evaluation tool
pub struct MathEvalTool;

impl MathEvalTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MathEvalTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for MathEvalTool {
    fn name(&self) -> &'static str {
        "math_eval"
    }

    fn description(&self) -> &'static str {
        "Evaluate mathematical expressions safely"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "math_eval".to_string(),
            description: "Safe mathematical evaluation".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                ToolParameter {
                    name: "expression".to_string(),
                    description: "Mathematical expression to evaluate".to_string(),
                    param_type: "string".to_string(),
                    required: true,
                    default: None,
                },
            ],
            returns: ToolReturn {
                return_type: "number".to_string(),
                description: "Evaluation result".to_string(),
            },
        }
    }

    async fn execute(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let start = std::time::Instant::now();

        let expression = input.params.get("expression")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidInput("Missing 'expression' parameter".to_string()))?;

        // Simple evaluation (placeholder - would use proper math parser)
        let result = self.evaluate_expression(expression)?;

        let execution_time_ms = start.elapsed().as_millis() as u64;

        Ok(ToolOutput {
            result: serde_json::json!({
                "expression": expression,
                "result": result,
            }),
            metadata: ToolMetadata {
                execution_time_ms,
                resources_used: ResourceUsage {
                    cpu_ms: execution_time_ms,
                    memory_mb: 1,
                    network_bytes: 0,
                },
                success: true,
                error: None,
            },
        })
    }

    fn validate_input(&self, input: &ToolInput) -> Result<(), ToolError> {
        let expression = input.params.get("expression")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidInput("Missing 'expression' parameter".to_string()))?;

        if expression.is_empty() {
            return Err(ToolError::InvalidInput("Expression cannot be empty".to_string()));
        }

        // Check for dangerous patterns
        if expression.contains("eval") || expression.contains("exec") {
            return Err(ToolError::SandboxViolation("Dangerous expression".to_string()));
        }

        Ok(())
    }
}

impl MathEvalTool {
    fn evaluate_expression(&self, expr: &str) -> Result<f64, ToolError> {
        // Simple parser for basic operations
        let expr = expr.trim();
        
        // Handle basic operations
        if let Some(pos) = expr.rfind('+') {
            let left = self.evaluate_expression(&expr[..pos])?;
            let right = self.evaluate_expression(&expr[pos+1..])?;
            return Ok(left + right);
        }
        
        if let Some(pos) = expr.rfind('-') {
            if pos > 0 {
                let left = self.evaluate_expression(&expr[..pos])?;
                let right = self.evaluate_expression(&expr[pos+1..])?;
                return Ok(left - right);
            }
        }
        
        if let Some(pos) = expr.rfind('*') {
            let left = self.evaluate_expression(&expr[..pos])?;
            let right = self.evaluate_expression(&expr[pos+1..])?;
            return Ok(left * right);
        }
        
        if let Some(pos) = expr.rfind('/') {
            let left = self.evaluate_expression(&expr[..pos])?;
            let right = self.evaluate_expression(&expr[pos+1..])?;
            if right == 0.0 {
                return Err(ToolError::ExecutionFailed("Division by zero".to_string()));
            }
            return Ok(left / right);
        }
        
        // Parse number
        expr.parse::<f64>()
            .map_err(|e| ToolError::InvalidInput(format!("Invalid number: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn test_math_eval_creation() {
        let tool = MathEvalTool::new();
        assert_eq!(tool.name(), "math_eval");
    }

    #[test]
    fn test_simple_evaluation() {
        let tool = MathEvalTool::new();
        
        assert_eq!(tool.evaluate_expression("42").unwrap(), 42.0);
        assert_eq!(tool.evaluate_expression("2+3").unwrap(), 5.0);
        assert_eq!(tool.evaluate_expression("10-4").unwrap(), 6.0);
        assert_eq!(tool.evaluate_expression("3*4").unwrap(), 12.0);
        assert_eq!(tool.evaluate_expression("15/3").unwrap(), 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        let tool = MathEvalTool::new();
        assert!(tool.evaluate_expression("5/0").is_err());
    }

    #[tokio::test]
    async fn test_input_validation() {
        let tool = MathEvalTool::new();
        
        let context = ExecutionContext {
            goal_id: Uuid::new_v4(),
            episode_id: None,
            user_id: "test".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            metadata: HashMap::new(),
        };
        
        // Valid expression
        let input = ToolInput {
            params: serde_json::json!({"expression": "2+2"}),
            context: context.clone(),
        };
        assert!(tool.validate_input(&input).is_ok());
        
        // Dangerous expression
        let input = ToolInput {
            params: serde_json::json!({"expression": "eval(malicious)"}),
            context,
        };
        assert!(tool.validate_input(&input).is_err());
    }

    #[tokio::test]
    async fn test_execute() {
        let tool = MathEvalTool::new();
        
        let context = ExecutionContext {
            goal_id: Uuid::new_v4(),
            episode_id: None,
            user_id: "test".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            metadata: HashMap::new(),
        };
        
        let input = ToolInput {
            params: serde_json::json!({"expression": "10+5"}),
            context,
        };
        
        let output = tool.execute(input).await.unwrap();
        assert!(output.metadata.success);
        assert_eq!(output.result["result"], 15.0);
    }
}
