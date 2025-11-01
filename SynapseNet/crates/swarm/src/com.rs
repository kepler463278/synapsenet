//! Consensus of Meaning (CoM) - Weighted aggregation

use crate::schema::*;
use std::collections::HashMap;

/// Consensus of Meaning aggregator
pub struct ConsensusOfMeaning {
    config: SwarmConfig,
    node_weights: HashMap<NodeId, NodeWeight>,
}

impl ConsensusOfMeaning {
    /// Create new CoM aggregator
    pub fn new(config: SwarmConfig) -> Self {
        Self {
            config,
            node_weights: HashMap::new(),
        }
    }

    /// Update node weight
    pub fn update_node_weight(&mut self, node_weight: NodeWeight) {
        self.node_weights.insert(node_weight.node_id.clone(), node_weight);
    }

    /// Get node weight
    fn get_node_weight(&self, node_id: &NodeId) -> f32 {
        self.node_weights
            .get(node_id)
            .map(|nw| nw.weight)
            .unwrap_or(1.0) // Default weight
    }

    /// Aggregate meaning from votes
    pub fn aggregate(&self, hyp: &Hypothesis, votes: &[Vote]) -> MeaningWeight {
        if votes.is_empty() {
            return MeaningWeight::new(hyp.id.clone(), 0.0, 0, 0);
        }

        let mut total_weight = 0.0;

        for vote in votes {
            let node_weight = self.get_node_weight(&vote.voter);
            
            let vote_score = self.config.alpha * vote.support
                + self.config.beta * vote.coherence
                + self.config.gamma * vote.novelty
                + self.config.delta * vote.reuse;

            total_weight += node_weight * vote_score;
        }

        MeaningWeight::new(hyp.id.clone(), total_weight, votes.len() as u32, 0)
    }

    /// Aggregate multiple hypotheses
    pub fn aggregate_all(
        &self,
        hypotheses: &[Hypothesis],
        votes_map: &HashMap<Hash, Vec<Vote>>,
        round: u32,
    ) -> Vec<MeaningWeight> {
        hypotheses
            .iter()
            .map(|hyp| {
                let votes = votes_map.get(&hyp.id).cloned().unwrap_or_default();
                let mut mw = self.aggregate(hyp, &votes);
                mw.round = round;
                mw
            })
            .collect()
    }

    /// Normalize weights using softmax
    pub fn normalize(&self, weights: &mut [MeaningWeight]) {
        if weights.is_empty() {
            return;
        }

        // Find max for numerical stability
        let max_weight = weights.iter().map(|w| w.weight).fold(f32::NEG_INFINITY, f32::max);

        // Compute exp and sum
        let exp_sum: f32 = weights
            .iter()
            .map(|w| (w.weight - max_weight).exp())
            .sum();

        // Normalize
        for w in weights.iter_mut() {
            w.weight = (w.weight - max_weight).exp() / exp_sum;
        }
    }

    /// Check if ready to commit
    pub fn can_commit(&self, weight: &MeaningWeight) -> bool {
        weight.weight >= self.config.tau_commit && weight.votes >= self.config.k_min
    }

    /// Find best hypothesis
    pub fn find_best(&self, weights: &[MeaningWeight]) -> Option<MeaningWeight> {
        weights
            .iter()
            .filter(|w| self.can_commit(w))
            .max_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap())
            .cloned()
    }

    /// Merge similar hypotheses
    pub fn merge_similar(&self, hypotheses: &mut Vec<Hypothesis>) {
        let threshold = 0.9;
        let mut to_remove = Vec::new();

        for i in 0..hypotheses.len() {
            if to_remove.contains(&i) {
                continue;
            }

            for j in (i + 1)..hypotheses.len() {
                if to_remove.contains(&j) {
                    continue;
                }

                let similarity = hypotheses[i].similarity(&hypotheses[j]);
                if similarity > threshold {
                    // Keep the one with more content or earlier timestamp
                    if hypotheses[i].content.len() >= hypotheses[j].content.len() {
                        to_remove.push(j);
                    } else {
                        to_remove.push(i);
                        break;
                    }
                }
            }
        }

        // Remove in reverse order to maintain indices
        to_remove.sort_unstable();
        to_remove.reverse();
        for idx in to_remove {
            hypotheses.remove(idx);
        }
    }

    /// Check convergence
    pub fn check_convergence(
        &self,
        current: &[MeaningWeight],
        previous: &[MeaningWeight],
    ) -> bool {
        if current.is_empty() || previous.is_empty() {
            return false;
        }

        // Find top hypotheses
        let mut current_sorted = current.to_vec();
        current_sorted.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

        let mut previous_sorted = previous.to_vec();
        previous_sorted.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

        // Check top 3 or all if less
        let n = current_sorted.len().min(previous_sorted.len()).min(3);

        for i in 0..n {
            let diff = (current_sorted[i].weight - previous_sorted[i].weight).abs();
            if diff >= self.config.epsilon {
                return false;
            }
        }

        true
    }
}

impl Default for ConsensusOfMeaning {
    fn default() -> Self {
        Self::new(SwarmConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_com_creation() {
        let com = ConsensusOfMeaning::default();
        assert_eq!(com.config.alpha, 0.35);
    }

    #[test]
    fn test_aggregate_single() {
        let com = ConsensusOfMeaning::default();
        
        let hyp = Hypothesis::new(
            Uuid::new_v4(),
            "Test".to_string(),
            vec![0.1; 384],
            "node1".to_string(),
        );

        let votes = vec![
            Vote::new(hyp.id.clone(), 0.8, 0.9, 0.7, 0.6, "voter1".to_string()),
            Vote::new(hyp.id.clone(), 0.7, 0.8, 0.6, 0.5, "voter2".to_string()),
        ];

        let mw = com.aggregate(&hyp, &votes);
        
        assert_eq!(mw.votes, 2);
        assert!(mw.weight > 0.0);
    }

    #[test]
    fn test_can_commit() {
        let com = ConsensusOfMeaning::default();
        
        let mw1 = MeaningWeight::new("h1".to_string(), 0.8, 10, 1);
        assert!(com.can_commit(&mw1));
        
        let mw2 = MeaningWeight::new("h2".to_string(), 0.5, 10, 1);
        assert!(!com.can_commit(&mw2));
        
        let mw3 = MeaningWeight::new("h3".to_string(), 0.8, 5, 1);
        assert!(!com.can_commit(&mw3));
    }

    #[test]
    fn test_normalize() {
        let com = ConsensusOfMeaning::default();
        
        let mut weights = vec![
            MeaningWeight::new("h1".to_string(), 2.0, 10, 1),
            MeaningWeight::new("h2".to_string(), 1.0, 8, 1),
            MeaningWeight::new("h3".to_string(), 0.5, 5, 1),
        ];

        com.normalize(&mut weights);
        
        let sum: f32 = weights.iter().map(|w| w.weight).sum();
        assert!((sum - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_merge_similar() {
        let com = ConsensusOfMeaning::default();
        
        let vec1 = vec![1.0; 384];
        let vec2 = vec![1.0; 384]; // Identical
        let vec3 = vec![0.0; 384]; // Different

        let mut hypotheses = vec![
            Hypothesis {
                id: "h1".to_string(),
                goal_id: Uuid::new_v4(),
                content: "Test 1".to_string(),
                vec: vec1,
                author: "node1".to_string(),
                sig: String::new(),
                timestamp: 0,
            },
            Hypothesis {
                id: "h2".to_string(),
                goal_id: Uuid::new_v4(),
                content: "Test 2".to_string(),
                vec: vec2,
                author: "node2".to_string(),
                sig: String::new(),
                timestamp: 0,
            },
            Hypothesis {
                id: "h3".to_string(),
                goal_id: Uuid::new_v4(),
                content: "Test 3".to_string(),
                vec: vec3,
                author: "node3".to_string(),
                sig: String::new(),
                timestamp: 0,
            },
        ];

        com.merge_similar(&mut hypotheses);
        
        // Should merge h1 and h2, keep h3
        assert_eq!(hypotheses.len(), 2);
    }

    #[test]
    fn test_check_convergence() {
        let com = ConsensusOfMeaning::default();
        
        let current = vec![
            MeaningWeight::new("h1".to_string(), 0.8, 10, 2),
            MeaningWeight::new("h2".to_string(), 0.6, 8, 2),
        ];

        let previous = vec![
            MeaningWeight::new("h1".to_string(), 0.79, 10, 1),
            MeaningWeight::new("h2".to_string(), 0.61, 8, 1),
        ];

        assert!(com.check_convergence(&current, &previous));
    }

    #[test]
    fn test_no_convergence() {
        let com = ConsensusOfMeaning::default();
        
        let current = vec![
            MeaningWeight::new("h1".to_string(), 0.8, 10, 2),
        ];

        let previous = vec![
            MeaningWeight::new("h1".to_string(), 0.5, 10, 1),
        ];

        assert!(!com.check_convergence(&current, &previous));
    }
}
