//! Tool Registry - Catalog and management of available tools

use crate::tool_api::{Tool, ToolError, ToolInfo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Tool policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPolicy {
    pub enabled: bool,
    pub requires_approval: bool,
    pub rate_limit: Option<RateLimit>,
    pub allowed_contexts: Vec<String>,
}

impl Default for ToolPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            requires_approval: false,
            rate_limit: None,
            allowed_contexts: vec![],
        }
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub calls_per_minute: u32,
    pub calls_per_hour: u32,
}

impl Default for RateLimit {
    fn default() -> Self {
        Self {
            calls_per_minute: 60,
            calls_per_hour: 1000,
        }
    }
}

/// Tool registry for managing available tools
pub struct ToolRegistry {
    tools: Arc<RwLock<HashMap<String, Box<dyn Tool>>>>,
    policies: Arc<RwLock<HashMap<String, ToolPolicy>>>,
    usage_stats: Arc<RwLock<HashMap<String, UsageStats>>>,
}

/// Usage statistics for a tool
#[derive(Debug, Clone, Default)]
struct UsageStats {
    total_calls: u64,
    successful_calls: u64,
    failed_calls: u64,
    last_call_timestamp: Option<i64>,
    recent_calls: Vec<i64>, // Timestamps for rate limiting
}

impl ToolRegistry {
    /// Create new tool registry
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
            policies: Arc::new(RwLock::new(HashMap::new())),
            usage_stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new tool
    pub fn register(&self, tool: Box<dyn Tool>) -> Result<(), String> {
        let name = tool.name().to_string();
        
        let mut tools = self.tools.write().unwrap();
        if tools.contains_key(&name) {
            return Err(format!("Tool '{}' already registered", name));
        }
        
        tools.insert(name.clone(), tool);
        
        // Initialize default policy
        let mut policies = self.policies.write().unwrap();
        policies.insert(name.clone(), ToolPolicy::default());
        
        // Initialize usage stats
        let mut stats = self.usage_stats.write().unwrap();
        stats.insert(name, UsageStats::default());
        
        Ok(())
    }

    /// Get a tool by name
    pub fn get(&self, name: &str) -> Option<Arc<Box<dyn Tool>>> {
        let tools = self.tools.read().unwrap();
        tools.get(name).map(|t| Arc::new(t.clone()))
    }

    /// List all registered tools
    pub fn list(&self) -> Vec<ToolInfo> {
        let tools = self.tools.read().unwrap();
        let policies = self.policies.read().unwrap();
        
        tools.iter().map(|(name, tool)| {
            let policy = policies.get(name).cloned().unwrap_or_default();
            
            ToolInfo {
                name: tool.name().to_string(),
                description: tool.description().to_string(),
                version: tool.version().to_string(),
                enabled: policy.enabled,
                requires_approval: policy.requires_approval,
            }
        }).collect()
    }

    /// Check if tool is available and allowed
    pub fn check_policy(&self, name: &str, context: &str) -> Result<(), ToolError> {
        let policies = self.policies.read().unwrap();
        
        let policy = policies.get(name)
            .ok_or(ToolError::NotFound)?;
        
        if !policy.enabled {
            return Err(ToolError::PermissionDenied);
        }
        
        if !policy.allowed_contexts.is_empty() && !policy.allowed_contexts.contains(&context.to_string()) {
            return Err(ToolError::PermissionDenied);
        }
        
        // Check rate limits
        if let Some(rate_limit) = &policy.rate_limit {
            if !self.check_rate_limit(name, rate_limit) {
                return Err(ToolError::ResourceLimitExceeded);
            }
        }
        
        Ok(())
    }

    /// Update tool policy
    pub fn update_policy(&self, name: &str, policy: ToolPolicy) -> Result<(), String> {
        let mut policies = self.policies.write().unwrap();
        
        if !self.tools.read().unwrap().contains_key(name) {
            return Err(format!("Tool '{}' not found", name));
        }
        
        policies.insert(name.to_string(), policy);
        Ok(())
    }

    /// Enable a tool
    pub fn enable_tool(&self, name: &str) -> Result<(), String> {
        let mut policies = self.policies.write().unwrap();
        
        let policy = policies.get_mut(name)
            .ok_or_else(|| format!("Tool '{}' not found", name))?;
        
        policy.enabled = true;
        Ok(())
    }

    /// Disable a tool
    pub fn disable_tool(&self, name: &str) -> Result<(), String> {
        let mut policies = self.policies.write().unwrap();
        
        let policy = policies.get_mut(name)
            .ok_or_else(|| format!("Tool '{}' not found", name))?;
        
        policy.enabled = false;
        Ok(())
    }

    /// Record tool usage
    pub fn record_usage(&self, name: &str, success: bool) {
        let mut stats = self.usage_stats.write().unwrap();
        
        let stat = stats.entry(name.to_string()).or_insert_with(UsageStats::default);
        
        stat.total_calls += 1;
        if success {
            stat.successful_calls += 1;
        } else {
            stat.failed_calls += 1;
        }
        
        let now = chrono::Utc::now().timestamp();
        stat.last_call_timestamp = Some(now);
        stat.recent_calls.push(now);
        
        // Keep only last hour of calls
        let hour_ago = now - 3600;
        stat.recent_calls.retain(|&t| t > hour_ago);
    }

    /// Check rate limit
    fn check_rate_limit(&self, name: &str, limit: &RateLimit) -> bool {
        let stats = self.usage_stats.read().unwrap();
        
        let stat = match stats.get(name) {
            Some(s) => s,
            None => return true,
        };
        
        let now = chrono::Utc::now().timestamp();
        let minute_ago = now - 60;
        let hour_ago = now - 3600;
        
        let calls_last_minute = stat.recent_calls.iter().filter(|&&t| t > minute_ago).count();
        let calls_last_hour = stat.recent_calls.iter().filter(|&&t| t > hour_ago).count();
        
        calls_last_minute < limit.calls_per_minute as usize &&
        calls_last_hour < limit.calls_per_hour as usize
    }

    /// Get usage statistics
    pub fn get_stats(&self, name: &str) -> Option<(u64, u64, u64)> {
        let stats = self.usage_stats.read().unwrap();
        stats.get(name).map(|s| (s.total_calls, s.successful_calls, s.failed_calls))
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool_api::*;
    use async_trait::async_trait;

    struct MockTool;

    #[async_trait]
    impl Tool for MockTool {
        fn name(&self) -> &'static str {
            "mock_tool"
        }

        fn description(&self) -> &'static str {
            "A mock tool for testing"
        }

        fn schema(&self) -> ToolSchema {
            ToolSchema {
                name: "mock_tool".to_string(),
                description: "Mock".to_string(),
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
                result: serde_json::json!("success"),
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

    #[test]
    fn test_registry_creation() {
        let registry = ToolRegistry::new();
        assert_eq!(registry.list().len(), 0);
    }

    #[test]
    fn test_tool_registration() {
        let registry = ToolRegistry::new();
        let tool = Box::new(MockTool);
        
        assert!(registry.register(tool).is_ok());
        assert_eq!(registry.list().len(), 1);
    }

    #[test]
    fn test_duplicate_registration() {
        let registry = ToolRegistry::new();
        
        registry.register(Box::new(MockTool)).unwrap();
        let result = registry.register(Box::new(MockTool));
        
        assert!(result.is_err());
    }

    #[test]
    fn test_tool_enable_disable() {
        let registry = ToolRegistry::new();
        registry.register(Box::new(MockTool)).unwrap();
        
        assert!(registry.disable_tool("mock_tool").is_ok());
        assert!(registry.enable_tool("mock_tool").is_ok());
    }

    #[test]
    fn test_usage_recording() {
        let registry = ToolRegistry::new();
        registry.register(Box::new(MockTool)).unwrap();
        
        registry.record_usage("mock_tool", true);
        registry.record_usage("mock_tool", false);
        
        let (total, success, failed) = registry.get_stats("mock_tool").unwrap();
        assert_eq!(total, 2);
        assert_eq!(success, 1);
        assert_eq!(failed, 1);
    }
}
