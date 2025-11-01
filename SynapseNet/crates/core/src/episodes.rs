//! Episode schema for reasoning memory chain
//! 
//! Episodes represent individual reasoning steps with their context,
//! retrieved information, and synthesis results.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A single episode in the reasoning chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    /// Unique episode identifier
    pub id: Uuid,
    /// Associated goal ID
    pub goal_id: Uuid,
    /// Step number in the reasoning sequence
    pub step: u32,
    /// Query or sub-question for this step
    pub query: String,
    /// Query embedding vector
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_vec: Option<Vec<f32>>,
    /// Retrieved grain IDs and their relevance scores
    pub retrieved_grains: Vec<RetrievedGrain>,
    /// Synthesized answer/insight for this step
    pub synthesis: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Verification signatures (for P2P validation)
    pub signatures: Vec<Signature>,
    /// Timestamp
    pub timestamp: i64,
    /// Metadata (sources, reasoning type, etc.)
    pub metadata: serde_json::Value,
}

/// Retrieved grain with relevance score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievedGrain {
    /// Grain ID
    pub grain_id: String,
    /// Relevance/similarity score
    pub score: f64,
    /// Source (local, peer ID, etc.)
    pub source: String,
    /// Optional snippet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}

/// Cryptographic signature for episode verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    /// Signer's peer ID or identity
    pub signer: String,
    /// Signature algorithm (ed25519, dilithium, etc.)
    pub algorithm: String,
    /// Signature bytes (hex or base64)
    pub signature: String,
    /// Timestamp of signing
    pub signed_at: i64,
}

impl Episode {
    /// Create a new episode
    pub fn new(
        goal_id: Uuid,
        step: u32,
        query: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            goal_id,
            step,
            query: query.into(),
            query_vec: None,
            retrieved_grains: Vec::new(),
            synthesis: String::new(),
            confidence: 0.0,
            signatures: Vec::new(),
            timestamp: chrono::Utc::now().timestamp(),
            metadata: serde_json::json!({}),
        }
    }

    /// Add retrieved grain
    pub fn add_grain(&mut self, grain: RetrievedGrain) {
        self.retrieved_grains.push(grain);
    }

    /// Set synthesis result
    pub fn set_synthesis(&mut self, synthesis: impl Into<String>, confidence: f64) {
        self.synthesis = synthesis.into();
        self.confidence = confidence.clamp(0.0, 1.0);
    }

    /// Add signature
    pub fn add_signature(&mut self, signature: Signature) {
        self.signatures.push(signature);
    }

    /// Check if episode is verified (has signatures)
    pub fn is_verified(&self) -> bool {
        !self.signatures.is_empty()
    }

    /// Get average grain score
    pub fn avg_grain_score(&self) -> f64 {
        if self.retrieved_grains.is_empty() {
            return 0.0;
        }
        
        let sum: f64 = self.retrieved_grains.iter().map(|g| g.score).sum();
        sum / self.retrieved_grains.len() as f64
    }

    /// Count grains from P2P sources
    pub fn p2p_grain_count(&self) -> usize {
        self.retrieved_grains
            .iter()
            .filter(|g| g.source != "local")
            .count()
    }
}

impl RetrievedGrain {
    /// Create a new retrieved grain
    pub fn new(
        grain_id: impl Into<String>,
        score: f64,
        source: impl Into<String>,
    ) -> Self {
        Self {
            grain_id: grain_id.into(),
            score: score.clamp(0.0, 1.0),
            source: source.into(),
            snippet: None,
        }
    }

    /// Add snippet
    pub fn with_snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = Some(snippet.into());
        self
    }
}

impl Signature {
    /// Create a new signature
    pub fn new(
        signer: impl Into<String>,
        algorithm: impl Into<String>,
        signature: impl Into<String>,
    ) -> Self {
        Self {
            signer: signer.into(),
            algorithm: algorithm.into(),
            signature: signature.into(),
            signed_at: chrono::Utc::now().timestamp(),
        }
    }
}

/// Memory chain - sequence of episodes
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MemoryChain {
    /// Episodes in chronological order
    pub episodes: Vec<Episode>,
}

impl MemoryChain {
    /// Create a new empty memory chain
    pub fn new() -> Self {
        Self {
            episodes: Vec::new(),
        }
    }

    /// Add episode to chain
    pub fn push(&mut self, episode: Episode) {
        self.episodes.push(episode);
    }

    /// Get episodes for a specific goal
    pub fn by_goal(&self, goal_id: &Uuid) -> Vec<&Episode> {
        self.episodes
            .iter()
            .filter(|e| e.goal_id == *goal_id)
            .collect()
    }

    /// Get episode by ID
    pub fn get(&self, id: &Uuid) -> Option<&Episode> {
        self.episodes.iter().find(|e| e.id == *id)
    }

    /// Get recent episodes (last N)
    pub fn recent(&self, n: usize) -> Vec<&Episode> {
        self.episodes.iter().rev().take(n).collect()
    }

    /// Get episodes with confidence above threshold
    pub fn high_confidence(&self, threshold: f64) -> Vec<&Episode> {
        self.episodes
            .iter()
            .filter(|e| e.confidence >= threshold)
            .collect()
    }

    /// Calculate average confidence
    pub fn avg_confidence(&self) -> f64 {
        if self.episodes.is_empty() {
            return 0.0;
        }
        
        let sum: f64 = self.episodes.iter().map(|e| e.confidence).sum();
        sum / self.episodes.len() as f64
    }

    /// Count total P2P grains used
    pub fn total_p2p_grains(&self) -> usize {
        self.episodes.iter().map(|e| e.p2p_grain_count()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_episode_creation() {
        let goal_id = Uuid::new_v4();
        let episode = Episode::new(goal_id, 1, "Test query");
        
        assert_eq!(episode.goal_id, goal_id);
        assert_eq!(episode.step, 1);
        assert_eq!(episode.query, "Test query");
        assert_eq!(episode.confidence, 0.0);
    }

    #[test]
    fn test_episode_synthesis() {
        let mut episode = Episode::new(Uuid::new_v4(), 1, "Query");
        episode.set_synthesis("Answer", 0.85);
        
        assert_eq!(episode.synthesis, "Answer");
        assert_eq!(episode.confidence, 0.85);
    }

    #[test]
    fn test_retrieved_grain() {
        let grain = RetrievedGrain::new("grain_123", 0.92, "local")
            .with_snippet("Test snippet");
        
        assert_eq!(grain.grain_id, "grain_123");
        assert_eq!(grain.score, 0.92);
        assert_eq!(grain.source, "local");
        assert_eq!(grain.snippet, Some("Test snippet".to_string()));
    }

    #[test]
    fn test_episode_grains() {
        let mut episode = Episode::new(Uuid::new_v4(), 1, "Query");
        
        episode.add_grain(RetrievedGrain::new("g1", 0.9, "local"));
        episode.add_grain(RetrievedGrain::new("g2", 0.8, "peer_123"));
        episode.add_grain(RetrievedGrain::new("g3", 0.7, "local"));
        
        assert_eq!(episode.retrieved_grains.len(), 3);
        assert_eq!(episode.avg_grain_score(), 0.8);
        assert_eq!(episode.p2p_grain_count(), 1);
    }

    #[test]
    fn test_memory_chain() {
        let mut chain = MemoryChain::new();
        let goal_id = Uuid::new_v4();
        
        let mut ep1 = Episode::new(goal_id, 1, "Query 1");
        ep1.set_synthesis("Answer 1", 0.9);
        
        let mut ep2 = Episode::new(goal_id, 2, "Query 2");
        ep2.set_synthesis("Answer 2", 0.7);
        
        chain.push(ep1);
        chain.push(ep2);
        
        assert_eq!(chain.episodes.len(), 2);
        assert_eq!(chain.by_goal(&goal_id).len(), 2);
        assert_eq!(chain.avg_confidence(), 0.8);
    }

    #[test]
    fn test_signature() {
        let sig = Signature::new("peer_123", "ed25519", "abc123");
        assert_eq!(sig.signer, "peer_123");
        assert_eq!(sig.algorithm, "ed25519");
        assert!(sig.signed_at > 0);
    }
}
