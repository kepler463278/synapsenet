// SynapseNet Core - Grain, Link, Graph primitives

pub mod config;
pub mod crypto;
pub mod error;
pub mod grain;
pub mod graph;
pub mod link;
pub mod logging;
pub mod metrics;
pub mod poe;
pub mod recovery;

#[cfg(any(target_os = "ios", target_os = "android"))]
pub mod mobile;

pub use config::{Config, ModelConfig};
pub use crypto::{
    CryptoBackend, SigningKeyTrait, UnifiedSigningKey, UnifiedVerifyingKey, VerifyingKeyTrait,
};
pub use error::{
    BatchError, EmbeddingError, ErrorContext, NetworkError, StorageError, SynapseNetError,
    WithContext,
};
pub use grain::{Grain, GrainMeta};
pub use graph::Graph;
pub use link::Link;
pub use logging::{
    init_logging, LogConfig, LogLevel, LogRotationConfig, LogRotationManager, PerformanceLogger,
};
pub use metrics::{MetricsTimer, NodeMetrics};
pub use poe::ProofOfEmergence;
pub use recovery::{
    retry_with_backoff, CircuitBreaker, CircuitState, GpuFallbackStrategy, ModelFallbackConfig,
    RetryConfig,
};
