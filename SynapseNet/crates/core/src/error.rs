use thiserror::Error;

/// Main error type for SynapseNet
#[derive(Debug, Error)]
pub enum SynapseNetError {
    /// Network-related errors
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),

    /// Embedding/AI errors
    #[error("Embedding error: {0}")]
    Embedding(#[from] EmbeddingError),

    /// Storage errors
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    /// Batch processing errors
    #[error("Batch processing error: {0}")]
    Batch(#[from] BatchError),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Cryptography errors
    #[error("Cryptography error: {0}")]
    Crypto(String),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Network-related errors
#[derive(Debug, Error)]
pub enum NetworkError {
    /// Connection failed
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// Connection timeout
    #[error("Connection timeout after {0}s")]
    Timeout(u64),

    /// DHT operation failed
    #[error("DHT error: {0}")]
    DhtError(String),

    /// NAT traversal failed
    #[error("NAT traversal failed: {0}")]
    NatTraversal(String),

    /// Peer not found
    #[error("Peer not found: {0}")]
    PeerNotFound(String),

    /// Protocol error
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// Relay error
    #[error("Relay error: {0}")]
    Relay(String),

    /// Transport error
    #[error("Transport error: {0}")]
    Transport(String),

    /// Maximum retries exceeded
    #[error("Maximum retries exceeded ({0} attempts)")]
    MaxRetriesExceeded(usize),
}

/// Embedding/AI errors
#[derive(Debug, Error)]
pub enum EmbeddingError {
    /// Model not found
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    /// Model loading failed
    #[error("Failed to load model: {0}")]
    ModelLoadFailed(String),

    /// Inference failed
    #[error("Inference failed: {0}")]
    InferenceFailed(String),

    /// Out of memory
    #[error("Out of memory: {0}")]
    OutOfMemory(String),

    /// GPU error
    #[error("GPU error: {0}")]
    GpuError(String),

    /// Invalid model format
    #[error("Invalid model format: {0}")]
    InvalidFormat(String),

    /// Dimension mismatch
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    /// Tokenization error
    #[error("Tokenization error: {0}")]
    Tokenization(String),

    /// Provider not available
    #[error("Provider not available: {0}")]
    ProviderNotAvailable(String),
}

/// Storage errors
#[derive(Debug, Error)]
pub enum StorageError {
    /// Database error
    #[error("Database error: {0}")]
    Database(String),

    /// Index error
    #[error("Index error: {0}")]
    Index(String),

    /// Grain not found
    #[error("Grain not found: {0}")]
    GrainNotFound(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    Deserialization(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(String),

    /// Corruption detected
    #[error("Data corruption detected: {0}")]
    Corruption(String),

    /// Migration error
    #[error("Migration error: {0}")]
    Migration(String),

    /// Disk full
    #[error("Disk full: {0}")]
    DiskFull(String),
}

/// Batch processing errors
#[derive(Debug, Error)]
pub enum BatchError {
    /// Batch too large
    #[error("Batch too large: {size} items (max: {max})")]
    TooLarge { size: usize, max: usize },

    /// Partial failure
    #[error("Batch partially failed: {succeeded}/{total} succeeded")]
    PartialFailure { succeeded: usize, total: usize },

    /// Batch timeout
    #[error("Batch processing timeout after {0}s")]
    Timeout(u64),

    /// Invalid batch format
    #[error("Invalid batch format: {0}")]
    InvalidFormat(String),

    /// Resource exhausted
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),
}

impl From<std::io::Error> for SynapseNetError {
    fn from(err: std::io::Error) -> Self {
        SynapseNetError::Storage(StorageError::Io(err.to_string()))
    }
}

impl From<anyhow::Error> for SynapseNetError {
    fn from(err: anyhow::Error) -> Self {
        SynapseNetError::Internal(err.to_string())
    }
}

/// Result type alias for SynapseNet operations
pub type Result<T> = std::result::Result<T, SynapseNetError>;

/// Error context for better debugging
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// Operation that failed
    pub operation: String,
    /// Additional context
    pub context: Vec<(String, String)>,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}

impl ErrorContext {
    /// Create new error context
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            context: Vec::new(),
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// Add context information
    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.push((key.into(), value.into()));
        self
    }

    /// Format context for display
    pub fn format(&self) -> String {
        let mut result = format!("Operation: {}\n", self.operation);
        for (key, value) in &self.context {
            result.push_str(&format!("  {}: {}\n", key, value));
        }
        result
    }
}

/// Trait for adding context to errors
pub trait WithContext<T> {
    /// Add context to error
    fn with_context(self, context: ErrorContext) -> Result<T>;
}

impl<T, E: Into<SynapseNetError>> WithContext<T> for std::result::Result<T, E> {
    fn with_context(self, context: ErrorContext) -> Result<T> {
        self.map_err(|e| {
            let err: SynapseNetError = e.into();
            // In a real implementation, we'd attach the context to the error
            // For now, we just log it
            tracing::error!("Error context: {}", context.format());
            err
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = NetworkError::ConnectionFailed("timeout".to_string());
        assert_eq!(err.to_string(), "Connection failed: timeout");

        let err = EmbeddingError::DimensionMismatch {
            expected: 384,
            actual: 512,
        };
        assert_eq!(
            err.to_string(),
            "Dimension mismatch: expected 384, got 512"
        );
    }

    #[test]
    fn test_error_context() {
        let ctx = ErrorContext::new("add_grain")
            .with_context("grain_id", "abc123")
            .with_context("model", "all-MiniLM-L6-v2");

        let formatted = ctx.format();
        assert!(formatted.contains("Operation: add_grain"));
        assert!(formatted.contains("grain_id: abc123"));
        assert!(formatted.contains("model: all-MiniLM-L6-v2"));
    }

    #[test]
    fn test_error_conversion() {
        let network_err = NetworkError::Timeout(30);
        let synapse_err: SynapseNetError = network_err.into();
        assert!(matches!(synapse_err, SynapseNetError::Network(_)));
    }
}
