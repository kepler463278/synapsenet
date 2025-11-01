//! Sandbox - Isolated execution environment with resource limits

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::time::timeout;

/// Sandbox configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub cpu_limit_ms: u64,
    pub memory_limit_mb: u64,
    pub network_allowed: bool,
    pub file_access_path: PathBuf,
    pub max_file_size_mb: u64,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            cpu_limit_ms: 200,
            memory_limit_mb: 64,
            network_allowed: false,
            file_access_path: PathBuf::from("./capsule/work"),
            max_file_size_mb: 10,
        }
    }
}

/// Resource limits for execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxLimits {
    pub cpu_ms: u64,
    pub mem_mb: u64,
    pub max_steps: u32,
}

impl Default for SandboxLimits {
    fn default() -> Self {
        Self {
            cpu_ms: 200,
            mem_mb: 64,
            max_steps: 100,
        }
    }
}

/// Sandbox execution result
#[derive(Debug)]
pub struct SandboxResult<T> {
    pub value: T,
    pub execution_time_ms: u64,
    pub memory_used_mb: u64,
}

/// Sandbox errors
#[derive(Debug, Clone)]
pub enum SandboxError {
    Timeout,
    MemoryLimitExceeded,
    CpuLimitExceeded,
    FileAccessDenied(String),
    NetworkAccessDenied,
    ExecutionFailed(String),
}

impl std::fmt::Display for SandboxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout => write!(f, "Execution timeout"),
            Self::MemoryLimitExceeded => write!(f, "Memory limit exceeded"),
            Self::CpuLimitExceeded => write!(f, "CPU limit exceeded"),
            Self::FileAccessDenied(path) => write!(f, "File access denied: {}", path),
            Self::NetworkAccessDenied => write!(f, "Network access denied"),
            Self::ExecutionFailed(msg) => write!(f, "Execution failed: {}", msg),
        }
    }
}

impl std::error::Error for SandboxError {}

/// Sandbox for isolated execution
pub struct Sandbox {
    config: SandboxConfig,
}

impl Sandbox {
    /// Create new sandbox with config
    pub fn new(config: SandboxConfig) -> Self {
        Self { config }
    }

    /// Create sandbox with default config
    pub fn default_config() -> Self {
        Self::new(SandboxConfig::default())
    }

    /// Execute function in sandbox with timeout
    pub async fn execute<F, T>(&self, f: F) -> Result<SandboxResult<T>, SandboxError>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let start = Instant::now();
        let timeout_duration = Duration::from_millis(self.config.cpu_limit_ms);

        // Execute with timeout
        let result = timeout(timeout_duration, tokio::task::spawn_blocking(f))
            .await
            .map_err(|_| SandboxError::Timeout)?
            .map_err(|e| SandboxError::ExecutionFailed(e.to_string()))?;

        let execution_time_ms = start.elapsed().as_millis() as u64;

        // TODO: Actual memory tracking
        let memory_used_mb = 0;

        Ok(SandboxResult {
            value: result,
            execution_time_ms,
            memory_used_mb,
        })
    }

    /// Execute async function in sandbox
    pub async fn execute_async<F, Fut, T>(&self, f: F) -> Result<SandboxResult<T>, SandboxError>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let start = Instant::now();
        let timeout_duration = Duration::from_millis(self.config.cpu_limit_ms);

        let result = timeout(timeout_duration, f())
            .await
            .map_err(|_| SandboxError::Timeout)?;

        let execution_time_ms = start.elapsed().as_millis() as u64;
        let memory_used_mb = 0;

        Ok(SandboxResult {
            value: result,
            execution_time_ms,
            memory_used_mb,
        })
    }

    /// Check if file access is allowed
    pub fn check_file_access(&self, path: &PathBuf) -> Result<(), SandboxError> {
        let canonical_base = self.config.file_access_path.canonicalize()
            .map_err(|e| SandboxError::FileAccessDenied(e.to_string()))?;
        
        let canonical_path = path.canonicalize()
            .map_err(|e| SandboxError::FileAccessDenied(e.to_string()))?;

        if !canonical_path.starts_with(&canonical_base) {
            return Err(SandboxError::FileAccessDenied(
                format!("Path outside sandbox: {:?}", path)
            ));
        }

        Ok(())
    }

    /// Check if network access is allowed
    pub fn check_network_access(&self) -> Result<(), SandboxError> {
        if !self.config.network_allowed {
            return Err(SandboxError::NetworkAccessDenied);
        }
        Ok(())
    }

    /// Get sandbox configuration
    pub fn config(&self) -> &SandboxConfig {
        &self.config
    }
}

/// Sandbox builder for custom configuration
pub struct SandboxBuilder {
    config: SandboxConfig,
}

impl SandboxBuilder {
    pub fn new() -> Self {
        Self {
            config: SandboxConfig::default(),
        }
    }

    pub fn cpu_limit_ms(mut self, limit: u64) -> Self {
        self.config.cpu_limit_ms = limit;
        self
    }

    pub fn memory_limit_mb(mut self, limit: u64) -> Self {
        self.config.memory_limit_mb = limit;
        self
    }

    pub fn allow_network(mut self, allowed: bool) -> Self {
        self.config.network_allowed = allowed;
        self
    }

    pub fn file_access_path(mut self, path: PathBuf) -> Self {
        self.config.file_access_path = path;
        self
    }

    pub fn build(self) -> Sandbox {
        Sandbox::new(self.config)
    }
}

impl Default for SandboxBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sandbox_execution() {
        let sandbox = Sandbox::default_config();
        
        let result = sandbox.execute(|| {
            42
        }).await;
        
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.value, 42);
        assert!(result.execution_time_ms < 200);
    }

    #[tokio::test]
    async fn test_sandbox_timeout() {
        let sandbox = SandboxBuilder::new()
            .cpu_limit_ms(100)
            .build();
        
        let result = sandbox.execute(|| {
            std::thread::sleep(Duration::from_millis(200));
            42
        }).await;
        
        assert!(result.is_err());
        match result {
            Err(SandboxError::Timeout) => {},
            _ => panic!("Expected timeout error"),
        }
    }

    #[tokio::test]
    async fn test_sandbox_async_execution() {
        let sandbox = Sandbox::default_config();
        
        let result = sandbox.execute_async(|| async {
            tokio::time::sleep(Duration::from_millis(10)).await;
            "success"
        }).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value, "success");
    }

    #[test]
    fn test_network_access_check() {
        let sandbox = SandboxBuilder::new()
            .allow_network(false)
            .build();
        
        assert!(sandbox.check_network_access().is_err());
        
        let sandbox = SandboxBuilder::new()
            .allow_network(true)
            .build();
        
        assert!(sandbox.check_network_access().is_ok());
    }

    #[test]
    fn test_sandbox_builder() {
        let sandbox = SandboxBuilder::new()
            .cpu_limit_ms(500)
            .memory_limit_mb(128)
            .allow_network(true)
            .build();
        
        assert_eq!(sandbox.config().cpu_limit_ms, 500);
        assert_eq!(sandbox.config().memory_limit_mb, 128);
        assert!(sandbox.config().network_allowed);
    }
}
