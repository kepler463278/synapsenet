use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tracing::{debug, info};

use synapsenet_core::Grain;
use synapsenet_storage::{HnswIndex, Store};

/// PoE v2 score with three components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoEScore {
    /// Novelty score (0.0 - 1.0)
    pub novelty: f32,
    /// Coherence score (0.0 - 1.0)
    pub coherence: f32,
    /// Reuse score (0.0 - 1.0)
    pub reuse: f32,
    /// Total weighted score
    pub total: f32,
}

impl PoEScore {
    /// Create new PoE score
    pub fn new(novelty: f32, coherence: f32, reuse: f32, weights: &RewardWeights) -> Self {
        let total = novelty * weights.novelty + coherence * weights.coherence + reuse * weights.reuse;
        
        Self {
            novelty,
            coherence,
            reuse,
            total,
        }
    }

    /// Calculate NGT reward from score
    pub fn to_ngt_reward(&self) -> f32 {
        // Base reward: 1 NGT
        // Bonus: up to 10 NGT for exceptional contributions
        1.0 + (self.total * 10.0)
    }
}

/// Reward weights for PoE calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardWeights {
    /// Weight for novelty (default: 0.4)
    pub novelty: f32,
    /// Weight for coherence (default: 0.3)
    pub coherence: f32,
    /// Weight for reuse (default: 0.3)
    pub reuse: f32,
}

impl Default for RewardWeights {
    fn default() -> Self {
        Self {
            novelty: 0.4,
            coherence: 0.3,
            reuse: 0.3,
        }
    }
}

impl RewardWeights {
    /// Validate weights sum to 1.0
    pub fn validate(&self) -> Result<()> {
        let sum = self.novelty + self.coherence + self.reuse;
        if (sum - 1.0).abs() > 0.01 {
            return Err(anyhow::anyhow!(
                "Reward weights must sum to 1.0, got {}",
                sum
            ));
        }
        Ok(())
    }
}

/// PoE v2 engine
pub struct PoEv2Engine {
    store: Arc<tokio::sync::Mutex<Store>>,
    index: Arc<RwLock<HnswIndex<'static>>>,
    reuse_tracker: Arc<RwLock<ReuseTracker>>,
    weights: RewardWeights,
}

impl PoEv2Engine {
    /// Create new PoE v2 engine
    pub fn new(
        store: Arc<tokio::sync::Mutex<Store>>,
        index: Arc<RwLock<HnswIndex<'static>>>,
        reuse_tracker: Arc<RwLock<ReuseTracker>>,
        weights: RewardWeights,
    ) -> Result<Self> {
        weights.validate()?;
        
        Ok(Self {
            store,
            index,
            reuse_tracker,
            weights,
        })
    }

    /// Calculate PoE score for a grain
    pub async fn calculate_score(&self, grain: &Grain) -> Result<PoEScore> {
        // Calculate novelty
        let novelty = self.calculate_novelty(&grain.vec).await?;
        
        // Calculate coherence
        let coherence = self.calculate_coherence(grain).await?;
        
        // Calculate reuse
        let reuse = self.calculate_reuse(&grain.id).await?;
        
        let score = PoEScore::new(novelty, coherence, reuse, &self.weights);
        
        debug!(
            "PoE score for grain {:?}: novelty={:.2}, coherence={:.2}, reuse={:.2}, total={:.2}",
            hex_encode(&grain.id[..8]),
            score.novelty,
            score.coherence,
            score.reuse,
            score.total
        );
        
        Ok(score)
    }

    /// Calculate novelty score
    async fn calculate_novelty(&self, embedding: &[f32]) -> Result<f32> {
        let index = self.index.read().await;
        
        // Search for K nearest neighbors
        let neighbors = index.search(embedding, 10)?;
        
        if neighbors.is_empty() {
            // First grain = maximum novelty
            return Ok(1.0);
        }
        
        // Calculate average similarity
        let avg_similarity: f32 = neighbors.iter().map(|n| n.similarity).sum::<f32>() / neighbors.len() as f32;
        
        // Novelty = 1 - similarity
        let novelty = (1.0 - avg_similarity).max(0.0).min(1.0);
        
        Ok(novelty)
    }

    /// Calculate coherence score
    async fn calculate_coherence(&self, grain: &Grain) -> Result<f32> {
        let index = self.index.read().await;
        
        // Find related grains (similarity > 0.6)
        let related = index.search(&grain.vec, 20)?
            .into_iter()
            .filter(|r| r.similarity > 0.6)
            .collect::<Vec<_>>();
        
        if related.len() < 2 {
            // No connections = no coherence
            return Ok(0.0);
        }
        
        // Calculate topic diversity
        let store = self.store.lock().await;
        let mut all_tags = HashSet::new();
        
        for result in &related {
            if let Ok(Some(related_grain)) = store.get_grain(&result.grain_id) {
                for tag in &related_grain.meta.tags {
                    all_tags.insert(tag.clone());
                }
            }
        }
        
        // Topic diversity = unique tags / total connections
        let diversity = if related.len() > 0 {
            (all_tags.len() as f32 / related.len() as f32).min(1.0)
        } else {
            0.0
        };
        
        // Connection score = number of connections (normalized)
        let connection_score = (related.len() as f32 / 20.0).min(1.0);
        
        // Coherence = connections × diversity
        let coherence = connection_score * diversity;
        
        Ok(coherence)
    }

    /// Calculate reuse score
    async fn calculate_reuse(&self, grain_id: &[u8; 32]) -> Result<f32> {
        let tracker = self.reuse_tracker.read().await;
        Ok(tracker.calculate_reuse_score(grain_id))
    }

    /// Get reward weights
    pub fn weights(&self) -> &RewardWeights {
        &self.weights
    }
}

/// Reuse tracker for monitoring grain access
pub struct ReuseTracker {
    /// Access log: grain_id -> events
    access_log: HashMap<[u8; 32], Vec<AccessEvent>>,
}

impl ReuseTracker {
    /// Create new reuse tracker
    pub fn new() -> Self {
        Self {
            access_log: HashMap::new(),
        }
    }

    /// Record grain access
    pub fn record_access(&mut self, grain_id: [u8; 32], event: AccessEvent) {
        self.access_log
            .entry(grain_id)
            .or_insert_with(Vec::new)
            .push(event);
        
        debug!("Recorded {:?} access for grain {:?}", event.access_type, hex_encode(&grain_id[..8]));
    }

    /// Calculate reuse score for a grain
    pub fn calculate_reuse_score(&self, grain_id: &[u8; 32]) -> f32 {
        let events = match self.access_log.get(grain_id) {
            Some(e) => e,
            None => return 0.0,
        };
        
        if events.is_empty() {
            return 0.0;
        }
        
        // Count unique peers
        let unique_peers: HashSet<_> = events.iter().map(|e| &e.peer_id).collect();
        
        // Frequency score (log scale)
        let frequency_score = ((events.len() as f32).log10() / 3.0).min(1.0);
        
        // Diversity score (log scale)
        let diversity_score = ((unique_peers.len() as f32).log10() / 2.0).min(1.0);
        
        // Combined score
        (frequency_score + diversity_score).min(1.0)
    }

    /// Get access count for grain
    pub fn get_access_count(&self, grain_id: &[u8; 32]) -> usize {
        self.access_log.get(grain_id).map(|e| e.len()).unwrap_or(0)
    }

    /// Clean up old access events (older than duration)
    pub fn cleanup_old_events(&mut self, max_age_secs: u64) {
        let now = SystemTime::now();
        
        for events in self.access_log.values_mut() {
            events.retain(|event| {
                if let Ok(elapsed) = now.duration_since(event.timestamp) {
                    elapsed.as_secs() < max_age_secs
                } else {
                    true
                }
            });
        }
        
        // Remove empty entries
        self.access_log.retain(|_, events| !events.is_empty());
    }
}

impl Default for ReuseTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Access event for reuse tracking
#[derive(Debug, Clone)]
pub struct AccessEvent {
    /// When the access occurred
    pub timestamp: SystemTime,
    /// Which peer accessed it
    pub peer_id: String,
    /// Type of access
    pub access_type: AccessType,
}

/// Type of grain access
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessType {
    /// Found in search results
    Search,
    /// Explicitly retrieved
    Retrieve,
    /// Referenced by another grain
    Reference,
}

// Helper for hex encoding
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poe_score() {
        let weights = RewardWeights::default();
        let score = PoEScore::new(0.8, 0.6, 0.4, &weights);
        
        assert_eq!(score.novelty, 0.8);
        assert_eq!(score.coherence, 0.6);
        assert_eq!(score.reuse, 0.4);
        
        let reward = score.to_ngt_reward();
        assert!(reward >= 1.0 && reward <= 11.0);
    }

    #[test]
    fn test_reward_weights() {
        let weights = RewardWeights::default();
        assert!(weights.validate().is_ok());
        
        let invalid = RewardWeights {
            novelty: 0.5,
            coherence: 0.5,
            reuse: 0.5,
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_reuse_tracker() {
        let mut tracker = ReuseTracker::new();
        let grain_id = [0u8; 32];
        
        // Record some accesses
        tracker.record_access(
            grain_id,
            AccessEvent {
                timestamp: SystemTime::now(),
                peer_id: "peer1".to_string(),
                access_type: AccessType::Search,
            },
        );
        
        tracker.record_access(
            grain_id,
            AccessEvent {
                timestamp: SystemTime::now(),
                peer_id: "peer2".to_string(),
                access_type: AccessType::Retrieve,
            },
        );
        
        let score = tracker.calculate_reuse_score(&grain_id);
        assert!(score > 0.0);
        assert_eq!(tracker.get_access_count(&grain_id), 2);
    }

    #[test]
    fn test_access_type() {
        assert_eq!(AccessType::Search, AccessType::Search);
        assert_ne!(AccessType::Search, AccessType::Retrieve);
    }
}
