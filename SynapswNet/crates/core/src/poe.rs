use serde::{Deserialize, Serialize};

/// Proof of Emergence - reward calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfEmergence {
    /// Novelty weight (default: 0.5)
    pub alpha: f64,
    /// Coherence weight (default: 0.3)
    pub beta: f64,
    /// Reuse weight (default: 0.2)
    pub gamma: f64,
    /// Novelty threshold
    pub tau_novelty: f32,
    /// Coherence threshold
    pub tau_coherence: f32,
}

impl Default for ProofOfEmergence {
    fn default() -> Self {
        Self {
            alpha: 0.5,
            beta: 0.3,
            gamma: 0.2,
            tau_novelty: 0.1,
            tau_coherence: 0.1,
        }
    }
}

impl ProofOfEmergence {
    /// Calculate NGT reward for a grain
    ///
    /// NGT(g) = α * N(g) + β * C(g) + γ * log(1 + R(g))
    ///
    /// where:
    /// - N(g) = novelty (1 - max_cos_sim with existing grains)
    /// - C(g) = coherence (avg cos_sim to relevant clusters)
    /// - R(g) = reuse count (how often grain appears in top-k results)
    pub fn calculate_ngt(&self, novelty: f32, coherence: f32, reuse_count: u32) -> f64 {
        // Anti-spam: if both novelty and coherence are below threshold, no reward
        if novelty < self.tau_novelty && coherence < self.tau_coherence {
            return 0.0;
        }

        let n = novelty as f64;
        let c = coherence as f64;
        let r = (1.0 + reuse_count as f64).ln();

        self.alpha * n + self.beta * c + self.gamma * r
    }

    /// Calculate novelty: 1 - max_cos_sim(grain, top_k_local)
    pub fn calculate_novelty(&self, max_similarity: f32) -> f32 {
        (1.0 - max_similarity).max(0.0)
    }

    /// Calculate coherence: average similarity to relevant cluster
    pub fn calculate_coherence(&self, similarities: &[f32]) -> f32 {
        if similarities.is_empty() {
            return 0.0;
        }

        let sum: f32 = similarities.iter().sum();
        sum / similarities.len() as f32
    }
}

/// Credit record for NGT allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credit {
    /// Grain ID
    pub grain_id: [u8; 32],
    /// Node public key
    pub node_pk: [u8; 32],
    /// NGT amount
    pub ngt: f64,
    /// Reason for credit
    pub reason: String,
    /// Timestamp (unix ms)
    pub ts_unix_ms: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poe_calculation() {
        let poe = ProofOfEmergence::default();

        // High novelty, medium coherence, no reuse
        let ngt = poe.calculate_ngt(0.8, 0.5, 0);
        assert!(ngt > 0.0);

        // Low novelty and coherence (spam)
        let ngt_spam = poe.calculate_ngt(0.05, 0.05, 0);
        assert_eq!(ngt_spam, 0.0);

        // With reuse
        let ngt_reuse = poe.calculate_ngt(0.6, 0.4, 10);
        assert!(ngt_reuse > poe.calculate_ngt(0.6, 0.4, 0));
    }

    #[test]
    fn test_novelty_calculation() {
        let poe = ProofOfEmergence::default();

        assert_eq!(poe.calculate_novelty(0.0), 1.0);
        assert_eq!(poe.calculate_novelty(1.0), 0.0);
        assert_eq!(poe.calculate_novelty(0.5), 0.5);
    }
}
