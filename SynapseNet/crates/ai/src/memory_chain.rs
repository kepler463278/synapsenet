//! Memory Chain - RAG across local and P2P knowledge
//! 
//! Provides unified access to episodes and grains from both
//! local storage and P2P network for reasoning context.

use crate::episodes::{Episode, MemoryChain as EpisodeChain, RetrievedGrain};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Memory source configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemorySource {
    /// Local storage only
    Local,
    /// P2P network only
    P2P,
    /// Both local and P2P
    LocalAndP2P,
}

/// Memory chain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Source to query
    pub source: MemorySource,
    /// Maximum results to retrieve
    pub max_results: usize,
    /// Minimum similarity threshold
    pub min_similarity: f64,
    /// Include P2P timeout (ms)
    pub p2p_timeout_ms: u64,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            source: MemorySource::LocalAndP2P,
            max_results: 10,
            min_similarity: 0.7,
            p2p_timeout_ms: 5000,
        }
    }
}

/// Memory Chain manager
pub struct MemoryChainManager {
    /// Configuration
    config: MemoryConfig,
    /// Local episode storage
    local_episodes: EpisodeChain,
    /// Cache of recent queries
    query_cache: std::collections::HashMap<String, Vec<RetrievedGrain>>,
}

impl MemoryChainManager {
    /// Create a new memory chain manager
    pub fn new(config: MemoryConfig) -> Self {
        Self {
            config,
            local_episodes: EpisodeChain::new(),
            query_cache: std::collections::HashMap::new(),
        }
    }

    /// Add episode to local chain
    pub fn add_episode(&mut self, episode: Episode) {
        self.local_episodes.push(episode);
    }

    /// Retrieve relevant grains for a query
    pub async fn retrieve(
        &mut self,
        query: &str,
        query_vec: &[f32],
    ) -> Result<Vec<RetrievedGrain>, String> {
        // Check cache first
        if let Some(cached) = self.query_cache.get(query) {
            return Ok(cached.clone());
        }

        let mut results = Vec::new();

        // Retrieve from local
        if matches!(
            self.config.source,
            MemorySource::Local | MemorySource::LocalAndP2P
        ) {
            let local_results = self.retrieve_local(query_vec).await?;
            results.extend(local_results);
        }

        // Retrieve from P2P
        if matches!(
            self.config.source,
            MemorySource::P2P | MemorySource::LocalAndP2P
        ) {
            let p2p_results = self.retrieve_p2p(query_vec).await?;
            results.extend(p2p_results);
        }

        // Sort by score and limit
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(self.config.max_results);

        // Filter by threshold
        results.retain(|g| g.score >= self.config.min_similarity);

        // Cache results
        self.query_cache.insert(query.to_string(), results.clone());

        Ok(results)
    }

    /// Retrieve from local storage
    async fn retrieve_local(&self, query_vec: &[f32]) -> Result<Vec<RetrievedGrain>, String> {
        // TODO: Implement actual vector search
        // For now, return mock results
        
        let mut results = Vec::new();
        
        // Search through local episodes
        for episode in &self.local_episodes.episodes {
            if let Some(ref ep_vec) = episode.query_vec {
                let similarity = cosine_similarity(query_vec, ep_vec);
                
                if similarity >= self.config.min_similarity {
                    // Add grains from this episode
                    for grain in &episode.retrieved_grains {
                        results.push(RetrievedGrain::new(
                            &grain.grain_id,
                            similarity,
                            "local",
                        ));
                    }
                }
            }
        }

        Ok(results)
    }

    /// Retrieve from P2P network
    async fn retrieve_p2p(&self, _query_vec: &[f32]) -> Result<Vec<RetrievedGrain>, String> {
        // TODO: Implement P2P query protocol
        // For now, return empty results
        Ok(Vec::new())
    }

    /// Get episode by ID
    pub fn get_episode(&self, id: &Uuid) -> Option<&Episode> {
        self.local_episodes.get(id)
    }

    /// Get episodes for goal
    pub fn episodes_for_goal(&self, goal_id: &Uuid) -> Vec<&Episode> {
        self.local_episodes.by_goal(goal_id)
    }

    /// Get recent episodes
    pub fn recent_episodes(&self, n: usize) -> Vec<&Episode> {
        self.local_episodes.recent(n)
    }

    /// Clear query cache
    pub fn clear_cache(&mut self) {
        self.query_cache.clear();
    }

    /// Get statistics
    pub fn stats(&self) -> MemoryStats {
        MemoryStats {
            total_episodes: self.local_episodes.episodes.len(),
            avg_confidence: self.local_episodes.avg_confidence(),
            total_p2p_grains: self.local_episodes.total_p2p_grains(),
            cache_size: self.query_cache.len(),
        }
    }
}

/// Memory statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_episodes: usize,
    pub avg_confidence: f64,
    pub total_p2p_grains: usize,
    pub cache_size: usize,
}

/// Calculate cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
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
    fn test_memory_config_default() {
        let config = MemoryConfig::default();
        assert_eq!(config.source, MemorySource::LocalAndP2P);
        assert_eq!(config.max_results, 10);
        assert_eq!(config.min_similarity, 0.7);
    }

    #[test]
    fn test_memory_chain_manager() {
        let config = MemoryConfig::default();
        let manager = MemoryChainManager::new(config);
        
        let stats = manager.stats();
        assert_eq!(stats.total_episodes, 0);
    }

    #[test]
    fn test_add_episode() {
        let config = MemoryConfig::default();
        let mut manager = MemoryChainManager::new(config);
        
        let episode = Episode::new(Uuid::new_v4(), 1, "Test query");
        manager.add_episode(episode);
        
        let stats = manager.stats();
        assert_eq!(stats.total_episodes, 1);
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);
        
        let c = vec![1.0, 0.0, 0.0];
        let d = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&c, &d) - 0.0).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_retrieve_empty() {
        let config = MemoryConfig::default();
        let mut manager = MemoryChainManager::new(config);
        
        let query_vec = vec![1.0, 0.0, 0.0];
        let results = manager.retrieve("test", &query_vec).await.unwrap();
        
        assert_eq!(results.len(), 0);
    }
}
