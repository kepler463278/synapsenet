//! PoE hooks for reasoning episodes

use crate::episodes::Episode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// PoE reward for reasoning episode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningReward {
    pub episode_id: Uuid,
    pub goal_id: Uuid,
    pub novelty_score: f64,
    pub coherence_score: f64,
    pub reuse_count: u32,
    pub p2p_contribution: f64,
    pub total_ngt: f64,
    pub timestamp: i64,
}

/// PoE calculator for reasoning
pub struct ReasoningPoE {
    base_reward: f64,
    novelty_weight: f64,
    coherence_weight: f64,
    p2p_bonus: f64,
}

impl ReasoningPoE {
    pub fn new() -> Self {
        Self {
            base_reward: 0.1,
            novelty_weight: 0.6,
            coherence_weight: 0.4,
            p2p_bonus: 0.2,
        }
    }

    /// Calculate reward for episode
    pub fn calculate_reward(&self, episode: &Episode) -> ReasoningReward {
        let novelty_score = self.calculate_novelty(episode);
        let coherence_score = episode.confidence;
        let p2p_contribution = self.calculate_p2p_contribution(episode);
        
        let base_score = (novelty_score * self.novelty_weight) + 
                        (coherence_score * self.coherence_weight);
        
        let total_ngt = self.base_reward * base_score + 
                       (p2p_contribution * self.p2p_bonus);
        
        ReasoningReward {
            episode_id: episode.id,
            goal_id: episode.goal_id,
            novelty_score,
            coherence_score,
            reuse_count: 0,
            p2p_contribution,
            total_ngt,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    fn calculate_novelty(&self, _episode: &Episode) -> f64 {
        0.7
    }

    fn calculate_p2p_contribution(&self, episode: &Episode) -> f64 {
        let p2p_grains = episode.p2p_grain_count() as f64;
        let total_grains = episode.retrieved_grains.len() as f64;
        
        if total_grains == 0.0 {
            return 0.0;
        }
        
        p2p_grains / total_grains
    }
}

impl Default for ReasoningPoE {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poe_calculation() {
        let poe = ReasoningPoE::new();
        let mut episode = Episode::new(Uuid::new_v4(), 1, "Test");
        episode.set_synthesis("Answer", 0.8);
        
        let reward = poe.calculate_reward(&episode);
        assert!(reward.total_ngt > 0.0);
    }
}
