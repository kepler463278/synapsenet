//! Reinforcement of Value (RoV) - Reward calculation

use crate::schema::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Reward for hypothesis author
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorReward {
    pub hyp: Hash,
    pub author: NodeId,
    pub reward: f64,
    pub evidence_count: usize,
    pub final_weight: f32,
}

/// Reward for voter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoterReward {
    pub voter: NodeId,
    pub reward: f64,
    pub proximity: f32,
    pub node_weight: f32,
}

/// RoV configuration
#[derive(Debug, Clone)]
pub struct RoVConfig {
    pub kappa: f64,            // Author reward multiplier
    pub lambda: f64,           // Voter reward multiplier
}

impl Default for RoVConfig {
    fn default() -> Self {
        Self {
            kappa: 1.0,
            lambda: 0.5,
        }
    }
}

/// Reinforcement of Value calculator
pub struct ReinforcementOfValue {
    config: RoVConfig,
}

impl ReinforcementOfValue {
    /// Create new RoV calculator
    pub fn new(config: RoVConfig) -> Self {
        Self { config }
    }

    /// Calculate author reward
    pub fn calculate_author_reward(
        &self,
        hyp: &Hypothesis,
        final_weight: &MeaningWeight,
        evidence_count: usize,
    ) -> AuthorReward {
        let reward = self.config.kappa 
            * final_weight.weight as f64
            * (1.0 + (1.0 + evidence_count as f64).ln());

        AuthorReward {
            hyp: hyp.id.clone(),
            author: hyp.author.clone(),
            reward,
            evidence_count,
            final_weight: final_weight.weight,
        }
    }

    /// Calculate voter reward
    pub fn calculate_voter_reward(
        &self,
        vote: &Vote,
        final_weight: &MeaningWeight,
        node_weight: f32,
    ) -> VoterReward {
        let proximity = self.calculate_proximity(vote, final_weight);
        let reward = self.config.lambda * node_weight as f64 * proximity as f64;

        VoterReward {
            voter: vote.voter.clone(),
            reward,
            proximity,
            node_weight,
        }
    }

    /// Calculate proximity between vote and final weight
    fn calculate_proximity(&self, vote: &Vote, final_weight: &MeaningWeight) -> f32 {
        // Normalize final weight to 0..1 range for comparison
        let normalized_weight = final_weight.weight.clamp(0.0, 1.0);
        
        // Calculate weighted vote score
        let vote_score = 0.35 * vote.support
            + 0.35 * vote.coherence
            + 0.2 * vote.novelty
            + 0.1 * vote.reuse;

        // Proximity is inverse of distance
        let distance = (vote_score - normalized_weight).abs();
        1.0 - distance.clamp(0.0, 1.0)
    }

    /// Calculate all rewards for a committed hypothesis
    pub fn calculate_all_rewards(
        &self,
        hyp: &Hypothesis,
        final_weight: &MeaningWeight,
        votes: &[Vote],
        evidence_count: usize,
        node_weights: &HashMap<NodeId, f32>,
    ) -> (AuthorReward, Vec<VoterReward>) {
        let author_reward = self.calculate_author_reward(hyp, final_weight, evidence_count);

        let voter_rewards: Vec<VoterReward> = votes
            .iter()
            .map(|vote| {
                let node_weight = node_weights.get(&vote.voter).copied().unwrap_or(1.0);
                self.calculate_voter_reward(vote, final_weight, node_weight)
            })
            .collect();

        (author_reward, voter_rewards)
    }

    /// Calculate total rewards
    pub fn total_rewards(&self, author: &AuthorReward, voters: &[VoterReward]) -> f64 {
        author.reward + voters.iter().map(|v| v.reward).sum::<f64>()
    }
}

impl Default for ReinforcementOfValue {
    fn default() -> Self {
        Self::new(RoVConfig::default())
    }
}

/// Penalty calculator
pub struct PenaltyCalculator {
    spam_penalty: f64,
    inconsistent_penalty: f64,
}

impl PenaltyCalculator {
    pub fn new() -> Self {
        Self {
            spam_penalty: -0.5,
            inconsistent_penalty: -0.3,
        }
    }

    /// Calculate spam penalty
    pub fn spam_penalty(&self) -> f64 {
        self.spam_penalty
    }

    /// Calculate inconsistent evidence penalty
    pub fn inconsistent_evidence_penalty(&self) -> f64 {
        self.inconsistent_penalty
    }

    /// Calculate low quality vote penalty
    pub fn low_quality_vote_penalty(&self, vote: &Vote) -> f64 {
        // Penalize votes with very low scores
        if vote.coherence < 0.3 && vote.support < 0.0 {
            -0.2
        } else {
            0.0
        }
    }
}

impl Default for PenaltyCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_rov_creation() {
        let rov = ReinforcementOfValue::default();
        assert_eq!(rov.config.kappa, 1.0);
        assert_eq!(rov.config.lambda, 0.5);
    }

    #[test]
    fn test_author_reward() {
        let rov = ReinforcementOfValue::default();
        
        let hyp = Hypothesis::new(
            Uuid::new_v4(),
            "Test".to_string(),
            vec![0.1; 384],
            "author1".to_string(),
        );

        let mw = MeaningWeight::new(hyp.id.clone(), 0.8, 10, 1);
        
        let reward = rov.calculate_author_reward(&hyp, &mw, 5);
        
        assert!(reward.reward > 0.0);
        assert_eq!(reward.author, "author1");
        assert_eq!(reward.evidence_count, 5);
    }

    #[test]
    fn test_voter_reward() {
        let rov = ReinforcementOfValue::default();
        
        let vote = Vote::new(
            "hyp1".to_string(),
            0.8,
            0.9,
            0.7,
            0.6,
            "voter1".to_string(),
        );

        let mw = MeaningWeight::new("hyp1".to_string(), 0.8, 10, 1);
        
        let reward = rov.calculate_voter_reward(&vote, &mw, 1.5);
        
        assert!(reward.reward > 0.0);
        assert_eq!(reward.voter, "voter1");
        assert!(reward.proximity > 0.0);
    }

    #[test]
    fn test_proximity_calculation() {
        let rov = ReinforcementOfValue::default();
        
        let vote = Vote::new(
            "hyp1".to_string(),
            0.8,
            0.8,
            0.8,
            0.8,
            "voter1".to_string(),
        );

        let mw = MeaningWeight::new("hyp1".to_string(), 0.8, 10, 1);
        
        let proximity = rov.calculate_proximity(&vote, &mw);
        
        // Should be high since vote aligns with final weight
        assert!(proximity > 0.8);
    }

    #[test]
    fn test_total_rewards() {
        let rov = ReinforcementOfValue::default();
        
        let author = AuthorReward {
            hyp: "h1".to_string(),
            author: "author1".to_string(),
            reward: 1.0,
            evidence_count: 5,
            final_weight: 0.8,
        };

        let voters = vec![
            VoterReward {
                voter: "v1".to_string(),
                reward: 0.3,
                proximity: 0.9,
                node_weight: 1.0,
            },
            VoterReward {
                voter: "v2".to_string(),
                reward: 0.2,
                proximity: 0.8,
                node_weight: 1.0,
            },
        ];

        let total = rov.total_rewards(&author, &voters);
        assert!((total - 1.5).abs() < 0.01);
    }

    #[test]
    fn test_penalty_calculator() {
        let pc = PenaltyCalculator::default();
        
        assert_eq!(pc.spam_penalty(), -0.5);
        assert_eq!(pc.inconsistent_evidence_penalty(), -0.3);
    }

    #[test]
    fn test_low_quality_vote_penalty() {
        let pc = PenaltyCalculator::default();
        
        let bad_vote = Vote::new(
            "h1".to_string(),
            -0.5,
            0.2,
            0.1,
            0.1,
            "voter1".to_string(),
        );

        let penalty = pc.low_quality_vote_penalty(&bad_vote);
        assert!(penalty < 0.0);
        
        let good_vote = Vote::new(
            "h1".to_string(),
            0.8,
            0.9,
            0.7,
            0.6,
            "voter2".to_string(),
        );

        let penalty2 = pc.low_quality_vote_penalty(&good_vote);
        assert_eq!(penalty2, 0.0);
    }
}
