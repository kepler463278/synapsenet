use crate::error::{NetworkError, SynapseNetError};
use std::time::Duration;
use tracing::{debug, warn};

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: usize,
    /// Initial backoff duration
    pub initial_backoff: Duration,
    /// Maximum backoff duration
    pub max_backoff: Duration,
    /// Backoff multiplier
    pub multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(30),
            multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    /// Create aggressive retry config (more attempts, faster)
    pub fn aggressive() -> Self {
        Self {
            max_attempts: 5,
            initial_backoff: Duration::from_millis(50),
            max_backoff: Duration::from_secs(10),
            multiplier: 1.5,
        }
    }

    /// Create conservative retry config (fewer attempts, slower)
    pub fn conservative() -> Self {
        Self {
            max_attempts: 2,
            initial_backoff: Duration::from_millis(500),
            max_backoff: Duration::from_secs(60),
            multiplier: 3.0,
        }
    }
}

/// Retry a fallible operation with exponential backoff
pub async fn retry_with_backoff<F, Fut, T>(
    operation: F,
    config: &RetryConfig,
    operation_name: &str,
) -> Result<T, SynapseNetError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, SynapseNetError>>,
{
    let mut attempt = 0;
    let mut backoff = config.initial_backoff;

    loop {
        attempt += 1;

        match operation().await {
            Ok(result) => {
                if attempt > 1 {
                    debug!(
                        "Operation '{}' succeeded after {} attempts",
                        operation_name, attempt
                    );
                }
                return Ok(result);
            }
            Err(err) => {
                if attempt >= config.max_attempts {
                    warn!(
                        "Operation '{}' failed after {} attempts: {}",
                        operation_name, attempt, err
                    );
                    return Err(SynapseNetError::Network(NetworkError::MaxRetriesExceeded(
                        attempt,
                    )));
                }

                warn!(
                    "Operation '{}' failed (attempt {}/{}): {}. Retrying in {:?}...",
                    operation_name, attempt, config.max_attempts, err, backoff
                );

                tokio::time::sleep(backoff).await;

                // Exponential backoff
                backoff = Duration::from_secs_f64(
                    (backoff.as_secs_f64() * config.multiplier).min(config.max_backoff.as_secs_f64()),
                );
            }
        }
    }
}

/// GPU fallback strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuFallbackStrategy {
    /// Fail immediately on GPU error
    Fail,
    /// Fallback to CPU on GPU error
    FallbackToCpu,
    /// Try CPU first, then GPU
    PreferCpu,
}

/// Model fallback configuration
#[derive(Debug, Clone)]
pub struct ModelFallbackConfig {
    /// Primary model name
    pub primary_model: String,
    /// Fallback models (in order of preference)
    pub fallback_models: Vec<String>,
    /// Enable automatic fallback on OOM
    pub auto_fallback_on_oom: bool,
}

impl ModelFallbackConfig {
    /// Create new fallback config
    pub fn new(primary: impl Into<String>) -> Self {
        Self {
            primary_model: primary.into(),
            fallback_models: Vec::new(),
            auto_fallback_on_oom: true,
        }
    }

    /// Add fallback model
    pub fn with_fallback(mut self, model: impl Into<String>) -> Self {
        self.fallback_models.push(model.into());
        self
    }

    /// Get next fallback model
    pub fn next_fallback(&self, current: &str) -> Option<&str> {
        if current == self.primary_model {
            self.fallback_models.first().map(|s| s.as_str())
        } else {
            let idx = self.fallback_models.iter().position(|m| m == current)?;
            self.fallback_models.get(idx + 1).map(|s| s.as_str())
        }
    }
}

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Circuit is closed, requests pass through
    Closed,
    /// Circuit is open, requests fail fast
    Open,
    /// Circuit is half-open, testing if service recovered
    HalfOpen,
}

/// Circuit breaker for preventing cascading failures
#[derive(Debug)]
pub struct CircuitBreaker {
    /// Current state
    state: CircuitState,
    /// Failure count
    failure_count: usize,
    /// Failure threshold before opening
    failure_threshold: usize,
    /// Time to wait before half-open
    timeout: Duration,
    /// Last failure time
    last_failure: Option<std::time::Instant>,
}

impl CircuitBreaker {
    /// Create new circuit breaker
    pub fn new(failure_threshold: usize, timeout: Duration) -> Self {
        Self {
            state: CircuitState::Closed,
            failure_count: 0,
            failure_threshold,
            timeout,
            last_failure: None,
        }
    }

    /// Check if request should be allowed
    pub fn allow_request(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if timeout elapsed
                if let Some(last_failure) = self.last_failure {
                    if last_failure.elapsed() >= self.timeout {
                        debug!("Circuit breaker transitioning to half-open");
                        self.state = CircuitState::HalfOpen;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }

    /// Record successful request
    pub fn record_success(&mut self) {
        match self.state {
            CircuitState::HalfOpen => {
                debug!("Circuit breaker closing after successful request");
                self.state = CircuitState::Closed;
                self.failure_count = 0;
                self.last_failure = None;
            }
            CircuitState::Closed => {
                self.failure_count = 0;
            }
            CircuitState::Open => {}
        }
    }

    /// Record failed request
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure = Some(std::time::Instant::now());

        match self.state {
            CircuitState::Closed => {
                if self.failure_count >= self.failure_threshold {
                    warn!(
                        "Circuit breaker opening after {} failures",
                        self.failure_count
                    );
                    self.state = CircuitState::Open;
                }
            }
            CircuitState::HalfOpen => {
                warn!("Circuit breaker reopening after failure in half-open state");
                self.state = CircuitState::Open;
            }
            CircuitState::Open => {}
        }
    }

    /// Get current state
    pub fn state(&self) -> CircuitState {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retry_success() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicU32, Ordering};
        
        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();
        
        let operation = move || {
            let attempts = attempts_clone.clone();
            async move {
                let count = attempts.fetch_add(1, Ordering::SeqCst) + 1;
                if count < 3 {
                    Err(SynapseNetError::Network(NetworkError::ConnectionFailed(
                        "test".to_string(),
                    )))
                } else {
                    Ok(42)
                }
            }
        };

        let config = RetryConfig::default();
        let result = retry_with_backoff(operation, &config, "test").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_retry_max_attempts() {
        let operation = || async {
            Err::<(), _>(SynapseNetError::Network(NetworkError::ConnectionFailed(
                "test".to_string(),
            )))
        };

        let config = RetryConfig {
            max_attempts: 2,
            initial_backoff: Duration::from_millis(1),
            max_backoff: Duration::from_millis(10),
            multiplier: 2.0,
        };

        let result = retry_with_backoff(operation, &config, "test").await;
        assert!(result.is_err());
    }

    #[test]
    fn test_model_fallback() {
        let config = ModelFallbackConfig::new("large-model")
            .with_fallback("medium-model")
            .with_fallback("small-model");

        assert_eq!(config.next_fallback("large-model"), Some("medium-model"));
        assert_eq!(config.next_fallback("medium-model"), Some("small-model"));
        assert_eq!(config.next_fallback("small-model"), None);
    }

    #[test]
    fn test_circuit_breaker() {
        let mut cb = CircuitBreaker::new(3, Duration::from_secs(1));

        // Initially closed
        assert_eq!(cb.state(), CircuitState::Closed);
        assert!(cb.allow_request());

        // Record failures
        cb.record_failure();
        cb.record_failure();
        assert_eq!(cb.state(), CircuitState::Closed);

        cb.record_failure();
        assert_eq!(cb.state(), CircuitState::Open);
        assert!(!cb.allow_request());

        // Success in half-open closes circuit
        std::thread::sleep(Duration::from_millis(1100));
        assert!(cb.allow_request());
        assert_eq!(cb.state(), CircuitState::HalfOpen);

        cb.record_success();
        assert_eq!(cb.state(), CircuitState::Closed);
    }
}
