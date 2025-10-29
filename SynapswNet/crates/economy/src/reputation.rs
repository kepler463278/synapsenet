use std::collections::HashMap;

/// Node reputation system
#[derive(Debug, Default)]
pub struct ReputationSystem {
    scores: HashMap<String, f64>,
}

impl ReputationSystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update reputation score
    pub fn update(&mut self, peer_id: &str, delta: f64) {
        let score = self.scores.entry(peer_id.to_string()).or_insert(0.0);
        *score = (*score + delta).clamp(-100.0, 100.0);
    }

    /// Get reputation score
    pub fn score(&self, peer_id: &str) -> f64 {
        self.scores.get(peer_id).copied().unwrap_or(0.0)
    }

    /// Check if peer is trusted
    pub fn is_trusted(&self, peer_id: &str, threshold: f64) -> bool {
        self.score(peer_id) >= threshold
    }
}
