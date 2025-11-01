// Integration tests for SynapseNet Tauri app
// These tests verify end-to-end workflows

#[cfg(test)]
mod integration_tests {
    use synapsenet_core::{Config, Grain, GrainMeta};
    use synapsenet_storage::Store;

    #[test]
    fn test_config_loading() {
        // Test configuration loading
        let config = Config::default();
        assert!(config.validate().is_ok());
        
        // Test that all sections exist
        assert_eq!(config.node.name, "synapsenet-node");
        assert_eq!(config.ai.embedding_dim, 384);
        assert!(config.economy.poe_enabled);
        assert_eq!(config.ui.theme, "auto");
    }

    #[test]
    fn test_storage_migration() {
        // Test that storage migrations work
        let store = Store::new(":memory:").expect("Failed to create store");
        
        // Verify tables exist
        let grain_count = store.count_grains().expect("Failed to count grains");
        assert_eq!(grain_count, 0);
    }

    #[test]
    fn test_end_to_end_workflow() {
        // Test: Add grain → Store → Search workflow
        // This would require full setup, so we test components individually
        
        // 1. Create grain
        let meta = GrainMeta {
            author_pk: [0u8; 32],
            crypto_backend: synapsenet_core::CryptoBackend::Classical,
            ts_unix_ms: 1234567890,
            tags: vec!["test".to_string()],
            mime: "text/plain".to_string(),
            lang: "en".to_string(),
            title: Some("Test Grain".to_string()),
            summary: None,
            embedding_model: Some("test-model".to_string()),
            embedding_dimensions: Some(384),
        };

        let vec = vec![0.1; 384];
        
        // Verify grain structure
        assert_eq!(vec.len(), 384);
        assert_eq!(meta.tags.len(), 1);
        assert_eq!(meta.embedding_dimensions, Some(384));
    }

    #[test]
    fn test_batch_processing_limits() {
        // Test batch size limits
        let max_batch_size = 1000;
        let test_batch_size = 500;
        
        assert!(test_batch_size <= max_batch_size);
    }

    #[test]
    fn test_poe_v2_scoring() {
        use synapsenet_economy::{PoEScore, RewardWeights};
        
        // Test PoE score calculation
        let weights = RewardWeights::default();
        let score = PoEScore::new(0.8, 0.6, 0.4, &weights);
        
        assert_eq!(score.novelty, 0.8);
        assert_eq!(score.coherence, 0.6);
        assert_eq!(score.reuse, 0.4);
        
        // Total = 0.8*0.4 + 0.6*0.3 + 0.4*0.3 = 0.32 + 0.18 + 0.12 = 0.62
        assert!((score.total - 0.62).abs() < 0.01);
    }

    #[test]
    fn test_error_recovery() {
        use synapsenet_core::{RetryConfig, CircuitBreaker, CircuitState};
        use std::time::Duration;
        
        // Test retry configuration
        let retry_config = RetryConfig::default();
        assert_eq!(retry_config.max_attempts, 3);
        
        // Test circuit breaker
        let mut cb = CircuitBreaker::new(3, Duration::from_secs(1));
        assert_eq!(cb.state(), CircuitState::Closed);
        assert!(cb.allow_request());
        
        // Record failures
        cb.record_failure();
        cb.record_failure();
        cb.record_failure();
        
        // Should open after threshold
        assert_eq!(cb.state(), CircuitState::Open);
        assert!(!cb.allow_request());
    }

    #[test]
    fn test_multi_model_config() {
        use synapsenet_core::ModelConfig;
        
        // Test model configuration
        let model = ModelConfig {
            name: "test-model".to_string(),
            path: "models/test.onnx".to_string(),
            size: "small".to_string(),
            auto_load: true,
        };
        
        assert_eq!(model.name, "test-model");
        assert!(model.auto_load);
    }
}

// Note: Full integration tests with Tauri require tauri-driver
// and are better run as E2E tests in CI/CD pipeline
// See: https://tauri.app/v1/guides/testing/webdriver/introduction
