//! Integration tests for SynapseNet v0.4
//! 
//! These tests verify end-to-end functionality across all components.

use anyhow::Result;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::sleep;

// Test utilities
mod utils {
    use super::*;
    use synapsenet_core::{Config, Grain, GrainMeta, UnifiedSigningKey, CryptoBackend};
    use synapsenet_storage::Store;
    
    pub fn create_test_config(data_dir: PathBuf) -> Config {
        let mut config = Config::default();
        config.node.data_dir = data_dir.to_string_lossy().to_string();
        config.ai.model_name = "test-model".to_string();
        config.ai.embedding_dim = 384;
        config
    }
    
    pub fn create_test_grain(text: &str, tags: Vec<String>) -> Result<Grain> {
        let vec = vec![0.1f32; 384]; // Mock embedding
        let meta = GrainMeta {
            author_pk: [0u8; 32],
            crypto_backend: CryptoBackend::Classical,
            ts_unix_ms: chrono::Utc::now().timestamp_millis(),
            tags,
            mime: "text/plain".to_string(),
            lang: "en".to_string(),
            title: Some(text.chars().take(50).collect()),
            summary: None,
            embedding_model: Some("test-model".to_string()),
            embedding_dimensions: Some(384),
        };
        
        // Create mock signing key
        let signing_key = UnifiedSigningKey::generate_classical();
        
        Ok(Grain::new(vec, meta, &signing_key)?)
    }
}

/// Test basic grain storage and retrieval
#[tokio::test]
async fn test_grain_storage_retrieval() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    
    // Create store
    let mut store = synapsenet_storage::Store::new(&db_path)?;
    
    // Create test grain
    let grain = utils::create_test_grain("Test knowledge", vec!["test".to_string()])?;
    let grain_id = grain.id;
    
    // Store grain
    store.insert_grain(&grain)?;
    
    // Retrieve grain
    let retrieved = store.get_grain(&grain_id)?;
    assert!(retrieved.is_some());
    
    let retrieved_grain = retrieved.unwrap();
    assert_eq!(retrieved_grain.id, grain_id);
    assert_eq!(retrieved_grain.meta.tags, vec!["test".to_string()]);
    
    Ok(())
}

/// Test multi-model embedding system
#[tokio::test]
async fn test_multi_model_system() -> Result<()> {
    use synapsenet_ai::MultiModelManager;
    
    let temp_dir = TempDir::new()?;
    let models_dir = temp_dir.path().join("models");
    std::fs::create_dir_all(&models_dir)?;
    
    // Create multi-model manager
    let mut manager = MultiModelManager::new(models_dir)?;
    
    // Test model registration
    manager.register_model("test-model-1", 384, 22.0)?;
    manager.register_model("test-model-2", 768, 45.0)?;
    
    // Test model listing
    let models = manager.list_models();
    assert_eq!(models.len(), 2);
    assert!(models.contains_key("test-model-1"));
    assert!(models.contains_key("test-model-2"));
    
    // Test default model
    manager.set_default_model("test-model-1")?;
    assert_eq!(manager.get_default_model(), Some("test-model-1"));
    
    Ok(())
}

/// Test batch processing system
#[tokio::test]
async fn test_batch_processing() -> Result<()> {
    use synapsenet_ai::{BatchProcessor, BatchConfig};
    
    let config = BatchConfig {
        batch_size: 10,
        parallel_workers: 2,
        use_gpu: false,
        model_name: "test-model".to_string(),
        timeout: Duration::from_secs(30),
    };
    
    let processor = BatchProcessor::new(config);
    
    // Create test batch
    let texts = vec![
        "First test text".to_string(),
        "Second test text".to_string(),
        "Third test text".to_string(),
    ];
    
    // Process batch (mock)
    let results = processor.process_texts_mock(texts).await?;
    
    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|r| r.is_ok()));
    
    Ok(())
}

/// Test PoE v2 scoring system
#[tokio::test]
async fn test_poe_v2_scoring() -> Result<()> {
    use synapsenet_economy::{PoEv2Engine, RewardWeights, ReuseTracker};
    use synapsenet_storage::{Store, HnswIndex};
    use std::sync::Arc;
    use tokio::sync::{Mutex, RwLock};
    
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    
    // Create components
    let store = Arc::new(Mutex::new(Store::new(&db_path)?));
    let index = Arc::new(RwLock::new(HnswIndex::new(1000, 384)));
    let reuse_tracker = Arc::new(ReuseTracker::new());
    let weights = RewardWeights::default();
    
    // Create PoE engine
    let engine = PoEv2Engine::new(
        store.clone(),
        index.clone(),
        reuse_tracker.clone(),
        weights,
        0.1, // min_novelty_threshold
    )?;
    
    // Create test grain
    let grain = utils::create_test_grain("Novel knowledge", vec!["novel".to_string()])?;
    
    // Add to index first
    {
        let mut idx = index.write().await;
        idx.add(&grain)?;
    }
    
    // Calculate PoE score
    let score = engine.calculate_score(&grain).await?;
    
    assert!(score.novelty >= 0.0 && score.novelty <= 1.0);
    assert!(score.coherence >= 0.0 && score.coherence <= 1.0);
    assert!(score.reuse >= 0.0 && score.reuse <= 1.0);
    assert!(score.total >= 0.0 && score.total <= 1.0);
    
    // Test NGT reward calculation
    let ngt_reward = engine.calculate_ngt_reward(&score);
    assert!(ngt_reward >= 1.0 && ngt_reward <= 11.0);
    
    Ok(())
}

/// Test configuration system
#[tokio::test]
async fn test_configuration_system() -> Result<()> {
    use synapsenet_core::Config;
    
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");
    
    // Create and save config
    let mut config = utils::create_test_config(temp_dir.path().to_path_buf());
    config.economy.poe_enabled = true;
    config.ui.theme = "dark".to_string();
    
    config.save(&config_path)?;
    
    // Load and verify config
    let loaded_config = Config::load(&config_path)?;
    
    assert_eq!(loaded_config.economy.poe_enabled, true);
    assert_eq!(loaded_config.ui.theme, "dark");
    assert_eq!(loaded_config.ai.embedding_dim, 384);
    
    // Test validation
    assert!(loaded_config.validate().is_ok());
    
    // Test invalid config
    let mut invalid_config = loaded_config.clone();
    invalid_config.economy.novelty_weight = 2.0; // Invalid (> 1.0)
    assert!(invalid_config.validate().is_err());
    
    Ok(())
}

/// Test storage migrations
#[tokio::test]
async fn test_storage_migrations() -> Result<()> {
    use synapsenet_storage::{Store, run_migrations};
    
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");
    
    // Create store (should run migrations)
    let store = Store::new(&db_path)?;
    
    // Verify tables exist
    let conn = rusqlite::Connection::open(&db_path)?;
    
    // Check main tables
    let tables: Vec<String> = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")?
        .query_map([], |row| Ok(row.get::<_, String>(0)?))?
        .collect::<Result<Vec<_>, _>>()?;
    
    assert!(tables.contains(&"grains".to_string()));
    assert!(tables.contains(&"grain_access".to_string()));
    assert!(tables.contains(&"embedding_models".to_string()));
    assert!(tables.contains(&"peer_clusters".to_string()));
    assert!(tables.contains(&"schema_version".to_string()));
    
    // Check schema version
    let version: i32 = conn.query_row(
        "SELECT version FROM schema_version LIMIT 1",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(version, 4); // Current schema version
    
    Ok(())
}

/// Test error handling and recovery
#[tokio::test]
async fn test_error_handling() -> Result<()> {
    use synapsenet_core::{retry_with_backoff, RetryConfig, SynapseNetError, NetworkError};
    
    let config = RetryConfig {
        max_attempts: 3,
        initial_backoff: Duration::from_millis(10),
        max_backoff: Duration::from_millis(100),
        multiplier: 2.0,
    };
    
    // Test successful retry
    let mut attempt_count = 0;
    let result = retry_with_backoff(
        || async {
            attempt_count += 1;
            if attempt_count < 3 {
                Err(SynapseNetError::Network(NetworkError::ConnectionFailed(
                    "test".to_string(),
                )))
            } else {
                Ok(42)
            }
        },
        &config,
        "test_operation",
    )
    .await;
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
    assert_eq!(attempt_count, 3);
    
    // Test max retries exceeded
    let result = retry_with_backoff(
        || async {
            Err::<(), _>(SynapseNetError::Network(NetworkError::ConnectionFailed(
                "persistent failure".to_string(),
            )))
        },
        &config,
        "failing_operation",
    )
    .await;
    
    assert!(result.is_err());
    
    Ok(())
}

/// Test REST API v2 endpoints (mock)
#[tokio::test]
async fn test_api_v2_endpoints() -> Result<()> {
    // This would test actual API endpoints in a real scenario
    // For now, we'll test the data structures
    
    use synapsenet_api::v2::{BatchImportRequest, BatchItem, ModelInfo};
    
    // Test batch import request
    let request = BatchImportRequest {
        items: vec![
            BatchItem {
                text: "Test item 1".to_string(),
                tags: Some(vec!["test".to_string()]),
                title: Some("Test Title".to_string()),
            },
            BatchItem {
                text: "Test item 2".to_string(),
                tags: None,
                title: None,
            },
        ],
        model: Some("test-model".to_string()),
    };
    
    assert_eq!(request.items.len(), 2);
    assert_eq!(request.items[0].text, "Test item 1");
    
    // Test model info
    let model_info = ModelInfo {
        name: "test-model".to_string(),
        dimensions: 384,
        file_size_mb: 22.0,
        loaded_at: chrono::Utc::now().timestamp(),
        status: "loaded".to_string(),
    };
    
    assert_eq!(model_info.dimensions, 384);
    assert_eq!(model_info.status, "loaded");
    
    Ok(())
}

/// Test monitoring and metrics
#[tokio::test]
async fn test_monitoring_metrics() -> Result<()> {
    use synapsenet_api::metrics::{
        record_poe_v2_score, record_batch, update_models_loaded, update_clusters,
    };
    
    // Test PoE metrics recording
    record_poe_v2_score(0.8, 0.6, 0.4, 0.123);
    
    // Test batch metrics
    record_batch(10, 0.5, 0.9);
    
    // Test model metrics
    update_models_loaded(3);
    
    // Test cluster metrics
    update_clusters(5, 8.5);
    
    // In a real test, we'd verify the metrics were recorded
    // For now, just ensure no panics
    
    Ok(())
}

/// End-to-end workflow test
#[tokio::test]
async fn test_end_to_end_workflow() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("e2e_test.db");
    
    // 1. Initialize storage
    let mut store = synapsenet_storage::Store::new(&db_path)?;
    
    // 2. Create and store grains
    let grains = vec![
        utils::create_test_grain("Rust is a systems programming language", vec!["rust".to_string()])?,
        utils::create_test_grain("Machine learning with neural networks", vec!["ai".to_string(), "ml".to_string()])?,
        utils::create_test_grain("Distributed systems and consensus", vec!["distributed".to_string()])?,
    ];
    
    for grain in &grains {
        store.insert_grain(grain)?;
    }
    
    // 3. Verify storage
    let count = store.count_grains()?;
    assert_eq!(count, 3);
    
    // 4. Test grain access tracking
    store.record_grain_access(&grains[0].id, "peer1", "search")?;
    store.record_grain_access(&grains[0].id, "peer2", "retrieve")?;
    
    let access_count = store.get_grain_access_count(&grains[0].id)?;
    assert_eq!(access_count, 2);
    
    // 5. Test model registration
    store.register_embedding_model("test-model", 384, 22.0)?;
    
    let model_info = store.get_embedding_model("test-model")?;
    assert!(model_info.is_some());
    let (dims, size, _) = model_info.unwrap();
    assert_eq!(dims, 384);
    assert_eq!(size, 22.0);
    
    // 6. Test cluster operations
    store.upsert_peer_cluster("rust", "peer1", 0.9)?;
    store.upsert_peer_cluster("rust", "peer2", 0.8)?;
    store.upsert_peer_cluster("ai", "peer2", 0.7)?;
    
    let rust_peers = store.get_cluster_peers("rust", 10)?;
    assert_eq!(rust_peers.len(), 2);
    
    let peer2_topics = store.get_peer_topics("peer2")?;
    assert_eq!(peer2_topics.len(), 2);
    
    // 7. Test statistics
    let (topic_count, peer_count) = store.get_cluster_stats()?;
    assert_eq!(topic_count, 2); // rust, ai
    assert_eq!(peer_count, 2); // peer1, peer2
    
    Ok(())
}

/// Performance benchmark test
#[tokio::test]
async fn test_performance_benchmarks() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("perf_test.db");
    
    let mut store = synapsenet_storage::Store::new(&db_path)?;
    
    // Benchmark grain insertion
    let start = std::time::Instant::now();
    
    for i in 0..100 {
        let grain = utils::create_test_grain(
            &format!("Performance test grain {}", i),
            vec!["perf".to_string()],
        )?;
        store.insert_grain(&grain)?;
    }
    
    let insert_duration = start.elapsed();
    println!("Inserted 100 grains in {:?}", insert_duration);
    
    // Should be reasonably fast (< 1 second for 100 grains)
    assert!(insert_duration < Duration::from_secs(1));
    
    // Benchmark grain retrieval
    let start = std::time::Instant::now();
    
    let all_grains = store.get_all_grains()?;
    
    let retrieve_duration = start.elapsed();
    println!("Retrieved {} grains in {:?}", all_grains.len(), retrieve_duration);
    
    assert_eq!(all_grains.len(), 100);
    assert!(retrieve_duration < Duration::from_millis(100));
    
    Ok(())
}
