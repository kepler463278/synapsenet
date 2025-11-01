//! Unit tests for PoE v2 economic model

use synapsenet_economy::{PoEScore, PoEv2Engine, RewardWeights, ReuseTracker};
use synapsenet_core::{Grain, GrainMeta, UnifiedSigningKey, CryptoBackend};
use synapsenet_storage::{Store, HnswIndex};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tempfile::TempDir;

fn create_test_grain(text: &str, tags: Vec<String>) -> Grain {
    let vec = vec![0.1f32; 384];
    let meta = GrainMeta {
        author_pk: [0u8; 32],
        crypto_backend: CryptoBackend::Classical,
        ts_unix_ms: chrono::Utc::now().timestamp_millis(),
        tags,
        mime: "text/plain".to_string(),
        lang: "en".to_string(),
        title: Some(text.to_string()),
        summary: None,
        embedding_model: Some("test-model".to_string()),
        embedding_dimensions: Some(384),
    };
    
    let signing_key = UnifiedSigningKey::generate_classical();
    Grain::new(vec, meta, &signing_key).unwrap()
}

#[test]
fn test_poe_score_creation() {
    let score = PoEScore {
        novelty: 0.8,
        coherence: 0.6,
        reuse: 0.4,
        total: 0.6,
    };
    
    assert_eq!(score.novelty, 0.8);
    assert_eq!(score.coherence, 0.6);
    assert_eq!(score.reuse, 0.4);
    assert_eq!(score.total, 0.6);
}

#[test]
fn test_reward_weights_default() {
    let weights = RewardWeights::default();
    
    assert_eq!(weights.novelty, 0.4);
    assert_eq!(weights.coherence, 0.3);
    assert_eq!(weights.reuse, 0.3);
    
    // Weights should sum to 1.0
    let sum = weights.novelty + weights.coherence + weights.reuse;
    assert!((sum - 1.0).abs() < 0.001);
}

#[test]
fn test_reward_weights_custom() {
    let weights = RewardWeights {
        novelty: 0.5,
        coherence: 0.3,
        reuse: 0.2,
    };
    
    assert_eq!(weights.novelty, 0.5);
    assert_eq!(weights.coherence, 0.3);
    assert_eq!(weights.reuse, 0.2);
}

#[test]
fn test_reuse_tracker_creation() {
    let tracker = ReuseTracker::new();
    assert_eq!(tracker.get_access_count(&[0u8; 32]), 0);
}

#[test]
fn test_reuse_tracker_record_access() {
    let tracker = ReuseTracker::new();
    let grain_id = [1u8; 32];
    
    // Record some accesses
    tracker.record_access(&grain_id, "peer1", "search");
    tracker.record_access(&grain_id, "peer2", "retrieve");
    tracker.record_access(&grain_id, "peer1", "search");
    
    // Should have 3 accesses
    assert_eq!(tracker.get_access_count(&grain_id), 3);
}

#[test]
fn test_reuse_tracker_unique_peers() {
    let tracker = ReuseTracker::new();
    let grain_id = [1u8; 32];
    
    tracker.record_access(&grain_id, "peer1", "search");
    tracker.record_access(&grain_id, "peer2", "search");
    tracker.record_access(&grain_id, "peer1", "retrieve");
    
    let unique_peers = tracker.get_unique_peers(&grain_id);
    assert_eq!(unique_peers, 2);
}

#[test]
fn test_reuse_score_calculation() {
    let tracker = ReuseTracker::new();
    let grain_id = [1u8; 32];
    
    // No accesses = 0 score
    assert_eq!(tracker.calculate_reuse_score(&grain_id), 0.0);
    
    // Add some accesses
    for i in 0..10 {
        tracker.record_access(&grain_id, &format!("peer{}", i), "search");
    }
    
    let score = tracker.calculate_reuse_score(&grain_id);
    assert!(score > 0.0 && score <= 1.0);
}

#[tokio::test]
async fn test_poe_engine_creation() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    let store = Arc::new(Mutex::new(Store::new(&db_path).unwrap()));
    let index = Arc::new(RwLock::new(HnswIndex::new(1000, 384)));
    let reuse_tracker = Arc::new(ReuseTracker::new());
    let weights = RewardWeights::default();
    
    let engine = PoEv2Engine::new(
        store,
        index,
        reuse_tracker,
        weights,
        0.1,
    );
    
    assert!(engine.is_ok());
}

#[tokio::test]
async fn test_novelty_calculation_first_grain() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    let store = Arc::new(Mutex::new(Store::new(&db_path).unwrap()));
    let index = Arc::new(RwLock::new(HnswIndex::new(1000, 384)));
    let reuse_tracker = Arc::new(ReuseTracker::new());
    let weights = RewardWeights::default();
    
    let engine = PoEv2Engine::new(
        store,
        index,
        reuse_tracker,
        weights,
        0.1,
    ).unwrap();
    
    let grain = create_test_grain("First grain", vec!["test".to_string()]);
    
    // First grain should have maximum novelty
    let score = engine.calculate_score(&grain).await.unwrap();
    assert_eq!(score.novelty, 1.0);
}

#[tokio::test]
async fn test_ngt_reward_calculation() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    let store = Arc::new(Mutex::new(Store::new(&db_path).unwrap()));
    let index = Arc::new(RwLock::new(HnswIndex::new(1000, 384)));
    let reuse_tracker = Arc::new(ReuseTracker::new());
    let weights = RewardWeights::default();
    
    let engine = PoEv2Engine::new(
        store,
        index,
        reuse_tracker,
        weights,
        0.1,
    ).unwrap();
    
    // Test various scores
    let score1 = PoEScore {
        novelty: 1.0,
        coherence: 1.0,
        reuse: 1.0,
        total: 1.0,
    };
    let reward1 = engine.calculate_ngt_reward(&score1);
    assert_eq!(reward1, 11.0); // 1 + 1.0 * 10
    
    let score2 = PoEScore {
        novelty: 0.5,
        coherence: 0.5,
        reuse: 0.5,
        total: 0.5,
    };
    let reward2 = engine.calculate_ngt_reward(&score2);
    assert_eq!(reward2, 6.0); // 1 + 0.5 * 10
    
    let score3 = PoEScore {
        novelty: 0.0,
        coherence: 0.0,
        reuse: 0.0,
        total: 0.0,
    };
    let reward3 = engine.calculate_ngt_reward(&score3);
    assert_eq!(reward3, 1.0); // 1 + 0.0 * 10
}

#[test]
fn test_poe_score_bounds() {
    // All scores should be between 0 and 1
    let score = PoEScore {
        novelty: 0.8,
        coherence: 0.6,
        reuse: 0.4,
        total: 0.6,
    };
    
    assert!(score.novelty >= 0.0 && score.novelty <= 1.0);
    assert!(score.coherence >= 0.0 && score.coherence <= 1.0);
    assert!(score.reuse >= 0.0 && score.reuse <= 1.0);
    assert!(score.total >= 0.0 && score.total <= 1.0);
}

#[test]
fn test_ngt_reward_bounds() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    let store = Arc::new(Mutex::new(Store::new(&db_path).unwrap()));
    let index = Arc::new(RwLock::new(HnswIndex::new(1000, 384)));
    let reuse_tracker = Arc::new(ReuseTracker::new());
    let weights = RewardWeights::default();
    
    let engine = PoEv2Engine::new(
        store,
        index,
        reuse_tracker,
        weights,
        0.1,
    ).unwrap();
    
    // Test boundary conditions
    let min_score = PoEScore {
        novelty: 0.0,
        coherence: 0.0,
        reuse: 0.0,
        total: 0.0,
    };
    let min_reward = engine.calculate_ngt_reward(&min_score);
    assert_eq!(min_reward, 1.0);
    
    let max_score = PoEScore {
        novelty: 1.0,
        coherence: 1.0,
        reuse: 1.0,
        total: 1.0,
    };
    let max_reward = engine.calculate_ngt_reward(&max_score);
    assert_eq!(max_reward, 11.0);
}
