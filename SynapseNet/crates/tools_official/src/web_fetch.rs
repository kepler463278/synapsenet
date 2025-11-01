//! Web Fetch Tool - Safe HTTP requests with restrictions

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use synapsenet_agent::*;
use std::time::Duration;

/// Web fetch tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebFetchConfig {
    pub allowed_domains: Vec<String>,
    pub blocked_domains: Vec<String>,
    pub max_size_mb: u64,
    pub timeout_secs: u64,
    pub user_agent: String,
}

impl Default for WebFetchConfig {
    fn default() -> Self {
        Self {
            allowed_domains: vec![],
            blocked_domains: vec!["localhost".to_string(), "127.0.0.1".to_string()],
            max_size_mb: 10,
            timeout_secs: 30,
            user_agent: "SynapseNet/0.7".to_string(),
        }
    }
}

/// Web fetch tool for HTTP requests
pub struct WebFetchTool {
    config: WebFetchConfig,
    client: reqwest::Client,
}

impl WebFetchTool {
    /// Create new web fetch tool
    pub fn new(config: WebFetchConfig) -> Result<Self, String> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .user_agent(&config.user_agent)
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        Ok(Self { config, client })
    }

    /// Create with default configuration
    pub fn with_defaults() -> Result<Self, String> {
        Self::new(WebFetchConfig::default())
    }

    /// Check if domain is allowed
    fn is_domain_allowed(&self, url: &str) -> bool {
        let parsed = match url::Url::parse(url) {
            Ok(u) => u,
            Err(_) => return false,
        };

        let host = match parsed.host_str() {
            Some(h) => h,
            None => return false,
        };

        // Check blocked domains
        if self.config.blocked_domains.iter().any(|d| host.contains(d)) {
            return false;
        }

        // If whitelist is empty, allow all (except blocked)
        if self.config.allowed_domains.is_empty() {
            return true;
        }

        // Check whitelist
        self.config.allowed_domains.iter().any(|d| host.contains(d))
    }
}

#[async_trait]
impl Tool for WebFetchTool {
    fn name(&self) -> &'static str {
        "web_fetch"
    }

    fn description(&self) -> &'static str {
        "Fetch content from web URLs with safety restrictions"
    }

    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "web_fetch".to_string(),
            description: "Fetch web content via HTTP/HTTPS".to_string(),
            version: "1.0.0".to_string(),
            parameters: vec![
                ToolParameter {
                    name: "url".to_string(),
                    description: "URL to fetch".to_string(),
                    param_type: "string".to_string(),
                    required: true,
                    default: None,
                },
                ToolParameter {
                    name: "method".to_string(),
                    description: "HTTP method (GET, POST)".to_string(),
                    param_type: "string".to_string(),
                    required: false,
                    default: Some(serde_json::json!("GET")),
                },
            ],
            returns: ToolReturn {
                return_type: "object".to_string(),
                description: "Response with status, headers, and body".to_string(),
            },
        }
    }

    async fn execute(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let start = std::time::Instant::now();

        // Parse parameters
        let url = input.params.get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidInput("Missing 'url' parameter".to_string()))?;

        let method = input.params.get("method")
            .and_then(|v| v.as_str())
            .unwrap_or("GET");

        // Check domain
        if !self.is_domain_allowed(url) {
            return Err(ToolError::PermissionDenied);
        }

        // Make request
        let response = match method.to_uppercase().as_str() {
            "GET" => self.client.get(url).send().await,
            "POST" => {
                let body = input.params.get("body")
                    .cloned()
                    .unwrap_or(serde_json::Value::Null);
                self.client.post(url).json(&body).send().await
            }
            _ => return Err(ToolError::InvalidInput(format!("Unsupported method: {}", method))),
        };

        let response = response
            .map_err(|e| ToolError::NetworkError(e.to_string()))?;

        let status = response.status().as_u16();
        let headers: std::collections::HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        // Check content length
        let content_length = response.content_length().unwrap_or(0);
        if content_length > self.config.max_size_mb * 1024 * 1024 {
            return Err(ToolError::ResourceLimitExceeded);
        }

        let body = response.text().await
            .map_err(|e| ToolError::NetworkError(e.to_string()))?;

        // Check actual body size
        if body.len() > (self.config.max_size_mb * 1024 * 1024) as usize {
            return Err(ToolError::ResourceLimitExceeded);
        }

        let execution_time_ms = start.elapsed().as_millis() as u64;

        let result = serde_json::json!({
            "status": status,
            "headers": headers,
            "body": body,
            "url": url,
        });

        Ok(ToolOutput {
            result,
            metadata: ToolMetadata {
                execution_time_ms,
                resources_used: ResourceUsage {
                    cpu_ms: execution_time_ms,
                    memory_mb: (body.len() / 1024 / 1024) as u64,
                    network_bytes: body.len() as u64,
                },
                success: true,
                error: None,
            },
        })
    }

    fn validate_input(&self, input: &ToolInput) -> Result<(), ToolError> {
        if !input.params.is_object() {
            return Err(ToolError::InvalidInput("Parameters must be an object".to_string()));
        }

        let url = input.params.get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidInput("Missing 'url' parameter".to_string()))?;

        if url.is_empty() {
            return Err(ToolError::InvalidInput("URL cannot be empty".to_string()));
        }

        // Validate URL format
        url::Url::parse(url)
            .map_err(|e| ToolError::InvalidInput(format!("Invalid URL: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn test_web_fetch_creation() {
        let tool = WebFetchTool::with_defaults();
        assert!(tool.is_ok());
    }

    #[test]
    fn test_domain_blocking() {
        let tool = WebFetchTool::with_defaults().unwrap();
        
        assert!(!tool.is_domain_allowed("http://localhost:8080"));
        assert!(!tool.is_domain_allowed("http://127.0.0.1"));
        assert!(tool.is_domain_allowed("https://example.com"));
    }

    #[test]
    fn test_domain_whitelist() {
        let config = WebFetchConfig {
            allowed_domains: vec!["example.com".to_string()],
            ..Default::default()
        };
        
        let tool = WebFetchTool::new(config).unwrap();
        
        assert!(tool.is_domain_allowed("https://example.com"));
        assert!(tool.is_domain_allowed("https://api.example.com"));
        assert!(!tool.is_domain_allowed("https://other.com"));
    }

    #[tokio::test]
    async fn test_input_validation() {
        let tool = WebFetchTool::with_defaults().unwrap();
        
        let context = ExecutionContext {
            goal_id: Uuid::new_v4(),
            episode_id: None,
            user_id: "test".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            metadata: HashMap::new(),
        };
        
        // Missing URL
        let input = ToolInput {
            params: serde_json::json!({}),
            context: context.clone(),
        };
        assert!(tool.validate_input(&input).is_err());
        
        // Valid URL
        let input = ToolInput {
            params: serde_json::json!({"url": "https://example.com"}),
            context,
        };
        assert!(tool.validate_input(&input).is_ok());
    }
}
