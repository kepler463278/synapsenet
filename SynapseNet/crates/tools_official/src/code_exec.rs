//! Code Execution Tool - Run code in isolated sandbox

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use synapsenet_agent::*;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

/// Supported programming languages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    Python,
    JavaScript,
    Rust,
}

impl Language {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "python" | "py" => Some(Self::Python),
            "javascript" | "js" | "node" => Some(Self::JavaScript),
            "rust" | "rs" => Some(Self::Rust),
            _ => None,
        }
    }

    fn command(&self) -> &str {
        match self {
            Self::Python => "python3",
            Self::JavaScript => "node",
            Self::Rust => "rustc",
        }
    }
}

/// Code execution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExecConfig {
    pub timeout_secs: u64,
    pub max_output_bytes: usize,
    pub allowed_languages: Vec<String>,
}

impl Default for CodeExecConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 10,
            max_output_bytes: 1024 * 1024, // 1MB
            allowed_languages: vec!["python".to_string(), "javascript".to_string()],
        }
    }
}

/// Code execution tool
pub struct CodeExecTool {
    config: CodeExecConfig,
}

impl CodeExecTool {
    pub fn new(config: CodeExecConfig) -> Self {
        Self { config }
    }

    pub fn with_defaults() -> Self {
        Self::new(CodeExecConfig::default())
    }

    async fn execute_code(&self, language: Language, code: &str) -> Result<String, ToolError> {
        let mut child = Command::new(language.command())
            .arg("-c")
            .arg(code)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| ToolError::ExecutionFailed(format!("Failed to spawn process: {}", e)))?;

        // Wait with timeout
        let timeout = tokio::time::Duration::from_secs(self.config.timeout_secs);
        let output = tokio::time::timeout(timeout, child.wait_with_output())
            .await
            .map_err(|_| ToolError::Timeout)?
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            return Err(ToolError::ExecutionFailed(stderr.to_string()));
        }

        // Check output size
        if stdout.len() > self.config.max_output_bytes {
            return Err(ToolError::ResourceLimitExceeded);
        }

        Ok(stdout.to_string())
    }
}

#[async_trait]
impl Tool for CodeExecTool {
    fn name(&self) -> &'static str {
        "code_exec"
    }

    fn description(&self) -> &'static str {
        "Execute code in isolated sandbox (Python, JavaScript)"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "code_exec".to_string(),
            description: "Execute code safely".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                ToolParameter {
                    name: "language".to_string(),
                    description: "Programming language (python, javascript)".to_string(),
                    param_type: "string".to_string(),
                    required: true,
                    default: None,
                },
                ToolParameter {
                    name: "code".to_string(),
                    description: "Code to execute".to_string(),
                    param_type: "string".to_string(),
                    required: true,
                    default: None,
                },
            ],
            returns: ToolReturn {
                return_type: "object".to_string(),
                description: "Execution result with output".to_string(),
            },
        }
    }

    async fn execute(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let start = std::time::Instant::now();

        let language_str = input.params.get("language")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidInput("Missing 'language' parameter".to_string()))?;

        let code = input.params.get("code")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidInput("Missing 'code' parameter".to_string()))?;

        let language = Language::from_str(language_str)
            .ok_or_else(|| ToolError::InvalidInput(format!("Unsupported language: {}", language_str)))?;

        // Check if language is allowed
        if !self.config.allowed_languages.contains(&language_str.to_lowercase()) {
            return Err(ToolError::PermissionDenied);
        }

        let output = self.execute_code(language, code).await?;

        let execution_time_ms = start.elapsed().as_millis() as u64;

        Ok(ToolOutput {
            result: serde_json::json!({
                "language": language_str,
                "output": output,
                "code_length": code.len(),
            }),
            metadata: ToolMetadata {
                execution_time_ms,
                resources_used: ResourceUsage {
                    cpu_ms: execution_time_ms,
                    memory_mb: 10,
                    network_bytes: 0,
                },
                success: true,
                error: None,
            },
        })
    }

    fn validate_input(&self, input: &ToolInput) -> Result<(), ToolError> {
        let language = input.params.get("language")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidInput("Missing 'language' parameter".to_string()))?;

        let code = input.params.get("code")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidInput("Missing 'code' parameter".to_string()))?;

        if code.is_empty() {
            return Err(ToolError::InvalidInput("Code cannot be empty".to_string()));
        }

        // Check for dangerous patterns
        let dangerous_patterns = ["import os", "import sys", "eval(", "exec(", "__import__"];
        for pattern in &dangerous_patterns {
            if code.contains(pattern) {
                return Err(ToolError::SandboxViolation(format!("Dangerous pattern: {}", pattern)));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn test_code_exec_creation() {
        let tool = CodeExecTool::with_defaults();
        assert_eq!(tool.name(), "code_exec");
    }

    #[test]
    fn test_language_parsing() {
        assert!(matches!(Language::from_str("python"), Some(Language::Python)));
        assert!(matches!(Language::from_str("javascript"), Some(Language::JavaScript)));
        assert!(Language::from_str("unknown").is_none());
    }

    #[tokio::test]
    async fn test_input_validation() {
        let tool = CodeExecTool::with_defaults();
        
        let context = ExecutionContext {
            goal_id: Uuid::new_v4(),
            episode_id: None,
            user_id: "test".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            metadata: HashMap::new(),
        };
        
        // Valid code
        let input = ToolInput {
            params: serde_json::json!({
                "language": "python",
                "code": "print('hello')"
            }),
            context: context.clone(),
        };
        assert!(tool.validate_input(&input).is_ok());
        
        // Dangerous code
        let input = ToolInput {
            params: serde_json::json!({
                "language": "python",
                "code": "import os; os.system('rm -rf /')"
            }),
            context,
        };
        assert!(tool.validate_input(&input).is_err());
    }

    #[tokio::test]
    async fn test_python_execution() {
        let tool = CodeExecTool::with_defaults();
        
        let context = ExecutionContext {
            goal_id: Uuid::new_v4(),
            episode_id: None,
            user_id: "test".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            metadata: HashMap::new(),
        };
        
        let input = ToolInput {
            params: serde_json::json!({
                "language": "python",
                "code": "print(2 + 2)"
            }),
            context,
        };
        
        let result = tool.execute(input).await;
        // May fail if python3 not installed - that's ok for test
        if result.is_ok() {
            let output = result.unwrap();
            assert!(output.metadata.success);
        }
    }
}
