//! File Operations Tool - Safe file read/write in designated directory

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use synapsenet_agent::*;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// File operations configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOpsConfig {
    pub base_path: PathBuf,
    pub max_file_size_mb: u64,
    pub allowed_extensions: Vec<String>,
}

impl Default for FileOpsConfig {
    fn default() -> Self {
        Self {
            base_path: PathBuf::from("./capsule/work"),
            max_file_size_mb: 10,
            allowed_extensions: vec![
                "txt".to_string(),
                "json".to_string(),
                "csv".to_string(),
                "md".to_string(),
            ],
        }
    }
}

/// File operations tool
pub struct FileOpsTool {
    config: FileOpsConfig,
}

impl FileOpsTool {
    /// Create new file ops tool
    pub fn new(config: FileOpsConfig) -> Self {
        Self { config }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(FileOpsConfig::default())
    }

    /// Resolve path relative to base
    fn resolve_path(&self, path: &str) -> Result<PathBuf, ToolError> {
        let requested = Path::new(path);
        
        // Prevent absolute paths and parent directory traversal
        if requested.is_absolute() || path.contains("..") {
            return Err(ToolError::PermissionDenied);
        }

        let full_path = self.config.base_path.join(requested);
        Ok(full_path)
    }

    /// Check if file extension is allowed
    fn is_extension_allowed(&self, path: &Path) -> bool {
        if self.config.allowed_extensions.is_empty() {
            return true;
        }

        path.extension()
            .and_then(|e| e.to_str())
            .map(|ext| self.config.allowed_extensions.iter().any(|a| a == ext))
            .unwrap_or(false)
    }

    /// Read file
    async fn read_file(&self, path: &str) -> Result<String, ToolError> {
        let full_path = self.resolve_path(path)?;

        if !self.is_extension_allowed(&full_path) {
            return Err(ToolError::PermissionDenied);
        }

        // Check file size
        let metadata = fs::metadata(&full_path).await
            .map_err(|e| ToolError::ExecutionFailed(format!("File not found: {}", e)))?;

        if metadata.len() > self.config.max_file_size_mb * 1024 * 1024 {
            return Err(ToolError::ResourceLimitExceeded);
        }

        let mut file = fs::File::open(&full_path).await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        Ok(contents)
    }

    /// Write file
    async fn write_file(&self, path: &str, content: &str) -> Result<(), ToolError> {
        let full_path = self.resolve_path(path)?;

        if !self.is_extension_allowed(&full_path) {
            return Err(ToolError::PermissionDenied);
        }

        // Check content size
        if content.len() > (self.config.max_file_size_mb * 1024 * 1024) as usize {
            return Err(ToolError::ResourceLimitExceeded);
        }

        // Create parent directories
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        }

        let mut file = fs::File::create(&full_path).await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        file.write_all(content.as_bytes()).await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        Ok(())
    }

    /// List files in directory
    async fn list_files(&self, path: &str) -> Result<Vec<String>, ToolError> {
        let full_path = self.resolve_path(path)?;

        let mut entries = fs::read_dir(&full_path).await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        let mut files = Vec::new();
        while let Some(entry) = entries.next_entry().await
            .map_err(|e| ToolError::ExecutionFailed(e.to_string()))? {
            
            if let Some(name) = entry.file_name().to_str() {
                files.push(name.to_string());
            }
        }

        Ok(files)
    }
}

#[async_trait]
impl Tool for FileOpsTool {
    fn name(&self) -> &'static str {
        "file_ops"
    }

    fn description(&self) -> &'static str {
        "Read, write, and list files in designated directory"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "file_ops".to_string(),
            description: "File operations (read/write/list)".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                ToolParameter {
                    name: "operation".to_string(),
                    description: "Operation: read, write, list".to_string(),
                    param_type: "string".to_string(),
                    required: true,
                    default: None,
                },
                ToolParameter {
                    name: "path".to_string(),
                    description: "File or directory path".to_string(),
                    param_type: "string".to_string(),
                    required: true,
                    default: None,
                },
                ToolParameter {
                    name: "content".to_string(),
                    description: "Content to write (for write operation)".to_string(),
                    param_type: "string".to_string(),
                    required: false,
                    default: None,
                },
            ],
            returns: ToolReturn {
                return_type: "object".to_string(),
                description: "Operation result".to_string(),
            },
        }
    }

    async fn execute(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let start = std::time::Instant::now();

        let operation = input.params.get("operation")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidInput("Missing 'operation' parameter".to_string()))?;

        let path = input.params.get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidInput("Missing 'path' parameter".to_string()))?;

        let result = match operation {
            "read" => {
                let content = self.read_file(path).await?;
                serde_json::json!({
                    "operation": "read",
                    "path": path,
                    "content": content,
                    "size": content.len(),
                })
            }
            "write" => {
                let content = input.params.get("content")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ToolError::InvalidInput("Missing 'content' parameter".to_string()))?;
                
                self.write_file(path, content).await?;
                
                serde_json::json!({
                    "operation": "write",
                    "path": path,
                    "size": content.len(),
                })
            }
            "list" => {
                let files = self.list_files(path).await?;
                serde_json::json!({
                    "operation": "list",
                    "path": path,
                    "files": files,
                    "count": files.len(),
                })
            }
            _ => return Err(ToolError::InvalidInput(format!("Unknown operation: {}", operation))),
        };

        let execution_time_ms = start.elapsed().as_millis() as u64;

        Ok(ToolOutput {
            result,
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
        let operation = input.params.get("operation")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidInput("Missing 'operation' parameter".to_string()))?;

        let path = input.params.get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidInput("Missing 'path' parameter".to_string()))?;

        if path.is_empty() {
            return Err(ToolError::InvalidInput("Path cannot be empty".to_string()));
        }

        if operation == "write" {
            if input.params.get("content").is_none() {
                return Err(ToolError::InvalidInput("Missing 'content' for write operation".to_string()));
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
    fn test_file_ops_creation() {
        let tool = FileOpsTool::with_defaults();
        assert_eq!(tool.name(), "file_ops");
    }

    #[test]
    fn test_path_resolution() {
        let tool = FileOpsTool::with_defaults();
        
        // Valid path
        assert!(tool.resolve_path("test.txt").is_ok());
        
        // Absolute path - denied
        assert!(tool.resolve_path("/etc/passwd").is_err());
        
        // Parent traversal - denied
        assert!(tool.resolve_path("../secret.txt").is_err());
    }

    #[test]
    fn test_extension_check() {
        let tool = FileOpsTool::with_defaults();
        
        assert!(tool.is_extension_allowed(Path::new("test.txt")));
        assert!(tool.is_extension_allowed(Path::new("data.json")));
        assert!(!tool.is_extension_allowed(Path::new("script.sh")));
    }

    #[tokio::test]
    async fn test_input_validation() {
        let tool = FileOpsTool::with_defaults();
        
        let context = ExecutionContext {
            goal_id: Uuid::new_v4(),
            episode_id: None,
            user_id: "test".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            metadata: HashMap::new(),
        };
        
        // Missing operation
        let input = ToolInput {
            params: serde_json::json!({"path": "test.txt"}),
            context: context.clone(),
        };
        assert!(tool.validate_input(&input).is_err());
        
        // Valid read
        let input = ToolInput {
            params: serde_json::json!({"operation": "read", "path": "test.txt"}),
            context: context.clone(),
        };
        assert!(tool.validate_input(&input).is_ok());
        
        // Write without content
        let input = ToolInput {
            params: serde_json::json!({"operation": "write", "path": "test.txt"}),
            context,
        };
        assert!(tool.validate_input(&input).is_err());
    }
}
