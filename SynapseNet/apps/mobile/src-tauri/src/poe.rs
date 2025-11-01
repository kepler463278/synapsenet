//! Proof of Emergence (PoE) reward system for mobile

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoEScore {
    pub novelty: f64,
    pub coherence: f64,
    pub reuse_count: u32,
    pub total_score: f64,
    pub ngt_reward: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardHistory {
    pub grain_id: String,
    pub timestamp: u64,
    pub score: PoEScore,
    pub synced: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletInfo {
    pub balance: f64,
    pub pending_rewards: f64,
    pub total_earned: f64,
    pub grain_count: u32,
    pub avg_poe_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoEBreakdown {
    pub novelty_contribution: f64,
    pub coherence_contribution: f64,
    pub reuse_bonus: f64,
    pub total_ngt: f64,
}

/// Calculate PoE score for a grain
#[tauri::command]
pub async fn calculate_poe_score(
    grain_id: String,
    embedding: Vec<f32>,
    existing_embeddings: Vec<Vec<f32>>,
) -> Result<PoEScore, String> {
    tracing::info!("Calculating PoE score for grain: {}", grain_id);
    
    // Calculate novelty (how different from existing grains)
    let novelty = calculate_novelty(&embedding, &existing_embeddings);
    
    // Calculate coherence (internal consistency)
    let coherence = calculate_coherence(&embedding);
    
    // Reuse count starts at 0
    let reuse_count = 0;
    
    // Calculate total score (weighted average)
    let total_score = (novelty * 0.6) + (coherence * 0.4);
    
    // Calculate NGT reward (base reward * score multiplier)
    let base_reward = 1.0;
    let ngt_reward = base_reward * total_score;
    
    Ok(PoEScore {
        novelty,
        coherence,
        reuse_count,
        total_score,
        ngt_reward,
    })
}

/// Update PoE score when grain is reused
#[tauri::command]
pub async fn update_poe_reuse(
    grain_id: String,
    current_score: PoEScore,
) -> Result<PoEScore, String> {
    tracing::info!("Updating PoE reuse for grain: {}", grain_id);
    
    let mut updated_score = current_score;
    updated_score.reuse_count += 1;
    
    // Reuse bonus: 0.1 NGT per reuse, diminishing returns
    let reuse_bonus = 0.1 * (1.0 / (1.0 + updated_score.reuse_count as f64 * 0.1));
    updated_score.ngt_reward += reuse_bonus;
    
    Ok(updated_score)
}

/// Get wallet information
#[tauri::command]
pub async fn get_wallet_info() -> Result<WalletInfo, String> {
    // TODO: Load from persistent storage
    // For now, return mock data
    Ok(WalletInfo {
        balance: 42.5,
        pending_rewards: 3.2,
        total_earned: 156.8,
        grain_count: 87,
        avg_poe_score: 0.73,
    })
}

/// Get reward history
#[tauri::command]
pub async fn get_reward_history(limit: usize) -> Result<Vec<RewardHistory>, String> {
    // TODO: Load from persistent storage
    // For now, return mock data
    let mock_history = vec![
        RewardHistory {
            grain_id: "grain_001".to_string(),
            timestamp: 1698768000,
            score: PoEScore {
                novelty: 0.85,
                coherence: 0.78,
                reuse_count: 3,
                total_score: 0.82,
                ngt_reward: 1.12,
            },
            synced: true,
        },
        RewardHistory {
            grain_id: "grain_002".to_string(),
            timestamp: 1698754400,
            score: PoEScore {
                novelty: 0.72,
                coherence: 0.81,
                reuse_count: 1,
                total_score: 0.76,
                ngt_reward: 0.86,
            },
            synced: true,
        },
    ];
    
    Ok(mock_history.into_iter().take(limit).collect())
}

/// Get PoE breakdown for a specific grain
#[tauri::command]
pub async fn get_poe_breakdown(grain_id: String) -> Result<PoEBreakdown, String> {
    tracing::info!("Getting PoE breakdown for grain: {}", grain_id);
    
    // TODO: Load actual grain data
    // For now, return mock breakdown
    Ok(PoEBreakdown {
        novelty_contribution: 0.51,
        coherence_contribution: 0.31,
        reuse_bonus: 0.30,
        total_ngt: 1.12,
    })
}

/// Sync rewards with network
#[tauri::command]
pub async fn sync_rewards() -> Result<SyncResult, String> {
    tracing::info!("Syncing rewards with network");
    
    // TODO: Implement actual network sync
    // 1. Get unsynced rewards
    // 2. Submit to network
    // 3. Verify with peers
    // 4. Update local state
    
    Ok(SyncResult {
        synced_count: 5,
        pending_count: 2,
        total_ngt_synced: 4.3,
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResult {
    pub synced_count: u32,
    pub pending_count: u32,
    pub total_ngt_synced: f64,
}

/// Export rewards data
#[tauri::command]
pub async fn export_rewards() -> Result<String, String> {
    tracing::info!("Exporting rewards data");
    
    // TODO: Export to JSON or CSV
    let export_data = serde_json::json!({
        "wallet": get_wallet_info().await?,
        "history": get_reward_history(100).await?,
        "exported_at": chrono::Utc::now().timestamp(),
    });
    
    Ok(export_data.to_string())
}

// Helper functions

fn calculate_novelty(embedding: &[f32], existing: &[Vec<f32>]) -> f64 {
    if existing.is_empty() {
        return 1.0; // First grain is maximally novel
    }
    
    // Calculate average similarity to existing grains
    let mut total_similarity = 0.0;
    
    for existing_emb in existing {
        let similarity = cosine_similarity(embedding, existing_emb);
        total_similarity += similarity;
    }
    
    let avg_similarity = total_similarity / existing.len() as f64;
    
    // Novelty is inverse of similarity
    // High similarity = low novelty
    1.0 - avg_similarity
}

fn calculate_coherence(embedding: &[f32]) -> f64 {
    // Coherence measures internal consistency
    // For embeddings, we can use:
    // 1. Magnitude (well-formed embeddings have consistent magnitude)
    // 2. Distribution (no extreme outliers)
    
    let magnitude = embedding.iter()
        .map(|&x| x * x)
        .sum::<f32>()
        .sqrt();
    
    // Normalize magnitude to 0-1 range
    // Typical embeddings have magnitude around 1.0
    let magnitude_score = (magnitude / 2.0).min(1.0);
    
    // Check for outliers (values far from mean)
    let mean = embedding.iter().sum::<f32>() / embedding.len() as f32;
    let variance = embedding.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f32>() / embedding.len() as f32;
    let std_dev = variance.sqrt();
    
    // Lower variance = higher coherence
    let variance_score = 1.0 / (1.0 + std_dev);
    
    // Combine scores
    ((magnitude_score + variance_score) / 2.0) as f64
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() {
        return 0.0;
    }
    
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    
    let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }
    
    (dot_product / (magnitude_a * magnitude_b)) as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);
        
        let c = vec![1.0, 0.0, 0.0];
        let d = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&c, &d) - 0.0).abs() < 0.001);
    }
    
    #[test]
    fn test_novelty_first_grain() {
        let embedding = vec![1.0, 0.0, 0.0];
        let existing = vec![];
        assert_eq!(calculate_novelty(&embedding, &existing), 1.0);
    }
    
    #[test]
    fn test_coherence() {
        let embedding = vec![0.5, 0.5, 0.5, 0.5];
        let coherence = calculate_coherence(&embedding);
        assert!(coherence > 0.0 && coherence <= 1.0);
    }
    
    #[tokio::test]
    async fn test_poe_score_calculation() {
        let score = calculate_poe_score(
            "test_grain".to_string(),
            vec![1.0, 0.0, 0.0],
            vec![],
        ).await.unwrap();
        
        assert!(score.novelty > 0.0);
        assert!(score.coherence > 0.0);
        assert_eq!(score.reuse_count, 0);
        assert!(score.ngt_reward > 0.0);
    }
}
