//! Swarm data structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Node identifier
pub type NodeId = String;

/// Content hash
pub type Hash = String;

/// Cryptographic signature
pub type Signature = String;

/// Hypothesis - a proposed understanding/meaning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hypothesis {
    pub id: Hash,
    pub goal_id: Uuid,
    pub content: String,       // <= 512 chars
    pub vec: Vec<f32>,         // 384-dim embedding
    pub author: NodeId,
    pub sig: Signature,
    pub timestamp: i64,
}

impl Hypothesis {
    /// Create new hypothesis
    pub fn new(goal_id: Uuid, content: String, vec: Vec<f32>, author: NodeId) -> Self {
        let id = Self::compute_hash(&goal_id, &content);
        Self {
            id,
            goal_id,
            content,
            vec,
            author,
            sig: String::new(), // TODO: Sign
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    /// Compute hypothesis hash
    fn compute_hash(goal_id: &Uuid, content: &str) -> Hash {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash as StdHash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        goal_id.to_string().hash(&mut hasher);
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Check if content is valid
    pub fn is_valid(&self) -> bool {
        !self.content.is_empty() 
            && self.content.len() <= 512
            && self.vec.len() == 384
    }

    /// Cosine similarity with another hypothesis
    pub fn similarity(&self, other: &Self) -> f32 {
        if self.vec.len() != other.vec.len() {
            return 0.0;
        }

        let dot: f32 = self.vec.iter().zip(&other.vec).map(|(a, b)| a * b).sum();
        let norm_a: f32 = self.vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = other.vec.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot / (norm_a * norm_b)
    }
}

/// Evidence supporting a hypothesis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub hyp: Hash,
    pub refs: Vec<Hash>,       // grain/episode IDs
    pub confidence: f32,       // 0..1
    pub summary: String,
    pub sig: Signature,
    pub timestamp: i64,
}

impl Evidence {
    pub fn new(hyp: Hash, refs: Vec<Hash>, confidence: f32, summary: String) -> Self {
        Self {
            hyp,
            refs,
            confidence: confidence.clamp(0.0, 1.0),
            summary,
            sig: String::new(),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.refs.is_empty() 
            && self.confidence >= 0.0 
            && self.confidence <= 1.0
            && !self.summary.is_empty()
    }
}

/// Vote on a hypothesis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub hyp: Hash,
    pub support: f32,          // -1..+1
    pub coherence: f32,        // 0..1
    pub novelty: f32,          // 0..1
    pub reuse: f32,            // 0..1
    pub voter: NodeId,
    pub sig: Signature,
    pub timestamp: i64,
}

impl Vote {
    pub fn new(
        hyp: Hash,
        support: f32,
        coherence: f32,
        novelty: f32,
        reuse: f32,
        voter: NodeId,
    ) -> Self {
        Self {
            hyp,
            support: support.clamp(-1.0, 1.0),
            coherence: coherence.clamp(0.0, 1.0),
            novelty: novelty.clamp(0.0, 1.0),
            reuse: reuse.clamp(0.0, 1.0),
            voter,
            sig: String::new(),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.support >= -1.0 && self.support <= 1.0
            && self.coherence >= 0.0 && self.coherence <= 1.0
            && self.novelty >= 0.0 && self.novelty <= 1.0
            && self.reuse >= 0.0 && self.reuse <= 1.0
    }
}

/// Aggregated meaning weight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeaningWeight {
    pub hyp: Hash,
    pub weight: f32,
    pub votes: u32,
    pub round: u32,
    pub committed: bool,
}

impl MeaningWeight {
    pub fn new(hyp: Hash, weight: f32, votes: u32, round: u32) -> Self {
        Self {
            hyp,
            weight,
            votes,
            round,
            committed: false,
        }
    }

    pub fn commit(&mut self) {
        self.committed = true;
    }
}

/// Swarm configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmConfig {
    pub alpha: f32,            // support weight
    pub beta: f32,             // coherence weight
    pub gamma: f32,            // novelty weight
    pub delta: f32,            // reuse weight
    pub tau_commit: f32,       // commit threshold
    pub k_min: u32,            // minimum votes
    pub epsilon: f32,          // convergence threshold
    pub max_rounds: u32,
}

impl Default for SwarmConfig {
    fn default() -> Self {
        Self {
            alpha: 0.35,
            beta: 0.35,
            gamma: 0.2,
            delta: 0.1,
            tau_commit: 0.72,
            k_min: 7,
            epsilon: 0.02,
            max_rounds: 3,
        }
    }
}

/// Node reputation and weight
#[derive(Debug, Clone)]
pub struct NodeWeight {
    pub node_id: NodeId,
    pub reputation: f32,       // 0..1
    pub reuse_score: f32,      // 0..1
    pub weight: f32,           // 0.1..3.0
}

impl NodeWeight {
    pub fn new(node_id: NodeId, reputation: f32, reuse_score: f32) -> Self {
        let weight = (reputation * (1.0 + reuse_score)).clamp(0.1, 3.0);
        Self {
            node_id,
            reputation,
            reuse_score,
            weight,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypothesis_creation() {
        let hyp = Hypothesis::new(
            Uuid::new_v4(),
            "Test hypothesis".to_string(),
            vec![0.1; 384],
            "node_123".to_string(),
        );

        assert!(hyp.is_valid());
        assert_eq!(hyp.vec.len(), 384);
    }

    #[test]
    fn test_hypothesis_similarity() {
        let vec1 = vec![1.0, 0.0, 0.0];
        let vec2 = vec![1.0, 0.0, 0.0];
        let vec3 = vec![0.0, 1.0, 0.0];

        let hyp1 = Hypothesis {
            id: "h1".to_string(),
            goal_id: Uuid::new_v4(),
            content: "Test".to_string(),
            vec: vec1,
            author: "node1".to_string(),
            sig: String::new(),
            timestamp: 0,
        };

        let hyp2 = Hypothesis {
            id: "h2".to_string(),
            goal_id: Uuid::new_v4(),
            content: "Test".to_string(),
            vec: vec2,
            author: "node2".to_string(),
            sig: String::new(),
            timestamp: 0,
        };

        let hyp3 = Hypothesis {
            id: "h3".to_string(),
            goal_id: Uuid::new_v4(),
            content: "Test".to_string(),
            vec: vec3,
            author: "node3".to_string(),
            sig: String::new(),
            timestamp: 0,
        };

        assert!((hyp1.similarity(&hyp2) - 1.0).abs() < 0.01);
        assert!((hyp1.similarity(&hyp3) - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_vote_validation() {
        let vote = Vote::new(
            "hyp_123".to_string(),
            0.8,
            0.9,
            0.7,
            0.6,
            "voter_1".to_string(),
        );

        assert!(vote.is_valid());
    }

    #[test]
    fn test_vote_clamping() {
        let vote = Vote::new(
            "hyp_123".to_string(),
            2.0,  // Should clamp to 1.0
            -0.5, // Should clamp to 0.0
            0.5,
            0.5,
            "voter_1".to_string(),
        );

        assert_eq!(vote.support, 1.0);
        assert_eq!(vote.coherence, 0.0);
    }

    #[test]
    fn test_node_weight_calculation() {
        let nw = NodeWeight::new("node_1".to_string(), 0.8, 0.5);
        
        // weight = 0.8 * (1 + 0.5) = 1.2
        assert!((nw.weight - 1.2).abs() < 0.01);
    }

    #[test]
    fn test_node_weight_clamping() {
        let nw1 = NodeWeight::new("node_1".to_string(), 0.01, 0.0);
        assert_eq!(nw1.weight, 0.1); // Clamped to min
        
        let nw2 = NodeWeight::new("node_2".to_string(), 2.0, 2.0);
        assert_eq!(nw2.weight, 3.0); // Clamped to max
    }

    #[test]
    fn test_swarm_config_default() {
        let config = SwarmConfig::default();
        
        assert_eq!(config.alpha, 0.35);
        assert_eq!(config.beta, 0.35);
        assert_eq!(config.gamma, 0.2);
        assert_eq!(config.delta, 0.1);
        assert_eq!(config.tau_commit, 0.72);
        assert_eq!(config.k_min, 7);
    }
}
