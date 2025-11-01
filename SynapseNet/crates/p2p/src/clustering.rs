use libp2p::PeerId;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime};
use tracing::{debug, info};

/// Peer cluster based on shared topics/interests
#[derive(Debug, Clone)]
pub struct PeerCluster {
    /// Cluster topic/theme
    pub topic: String,
    /// Peers in this cluster
    pub peers: HashSet<PeerId>,
    /// Last time cluster was updated
    pub last_updated: SystemTime,
    /// Topic tags for similarity calculation
    pub tags: Vec<String>,
}

impl PeerCluster {
    /// Create new cluster
    pub fn new(topic: String, tags: Vec<String>) -> Self {
        Self {
            topic,
            peers: HashSet::new(),
            last_updated: SystemTime::now(),
            tags,
        }
    }

    /// Add peer to cluster
    pub fn add_peer(&mut self, peer_id: PeerId) {
        if self.peers.insert(peer_id) {
            self.last_updated = SystemTime::now();
            debug!("Added peer {} to cluster '{}'", peer_id, self.topic);
        }
    }

    /// Remove peer from cluster
    pub fn remove_peer(&mut self, peer_id: &PeerId) -> bool {
        if self.peers.remove(peer_id) {
            self.last_updated = SystemTime::now();
            debug!("Removed peer {} from cluster '{}'", peer_id, self.topic);
            true
        } else {
            false
        }
    }

    /// Remove inactive peers (not seen for timeout duration)
    pub fn remove_inactive_peers(&mut self, peer_last_seen: &HashMap<PeerId, SystemTime>, timeout: Duration) -> usize {
        let now = SystemTime::now();
        let mut removed = 0;

        self.peers.retain(|peer_id| {
            if let Some(last_seen) = peer_last_seen.get(peer_id) {
                if let Ok(elapsed) = now.duration_since(*last_seen) {
                    if elapsed > timeout {
                        debug!("Removing inactive peer {} from cluster '{}'", peer_id, self.topic);
                        removed += 1;
                        return false;
                    }
                }
            }
            true
        });

        if removed > 0 {
            self.last_updated = SystemTime::now();
            info!("Removed {} inactive peers from cluster '{}'", removed, self.topic);
        }

        removed
    }

    /// Get best peers from cluster (by some metric)
    pub fn get_best_peers(&self, count: usize) -> Vec<PeerId> {
        // For MVP, just return first N peers
        // In production, this would rank by reputation, latency, etc.
        self.peers.iter().take(count).copied().collect()
    }

    /// Get cluster size
    pub fn size(&self) -> usize {
        self.peers.len()
    }

    /// Check if cluster is empty
    pub fn is_empty(&self) -> bool {
        self.peers.is_empty()
    }

    /// Check if peer is in cluster
    pub fn contains(&self, peer_id: &PeerId) -> bool {
        self.peers.contains(peer_id)
    }

    /// Calculate similarity with another cluster (based on tags)
    pub fn similarity(&self, other: &PeerCluster) -> f32 {
        if self.tags.is_empty() || other.tags.is_empty() {
            return 0.0;
        }

        let self_tags: HashSet<_> = self.tags.iter().collect();
        let other_tags: HashSet<_> = other.tags.iter().collect();

        let intersection = self_tags.intersection(&other_tags).count();
        let union = self_tags.union(&other_tags).count();

        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }
}

/// Peer clustering manager
pub struct ClusteringManager {
    /// All clusters
    clusters: HashMap<String, PeerCluster>,
    /// Peer to clusters mapping
    peer_clusters: HashMap<PeerId, HashSet<String>>,
    /// Peer last seen times
    peer_last_seen: HashMap<PeerId, SystemTime>,
    /// Similarity threshold for clustering (0.0 - 1.0)
    similarity_threshold: f32,
}

impl ClusteringManager {
    /// Create new clustering manager
    pub fn new(similarity_threshold: f32) -> Self {
        Self {
            clusters: HashMap::new(),
            peer_clusters: HashMap::new(),
            peer_last_seen: HashMap::new(),
            similarity_threshold: similarity_threshold.clamp(0.0, 1.0),
        }
    }

    /// Add or update cluster
    pub fn add_cluster(&mut self, cluster: PeerCluster) {
        info!("Adding cluster: {}", cluster.topic);
        self.clusters.insert(cluster.topic.clone(), cluster);
    }

    /// Remove cluster
    pub fn remove_cluster(&mut self, topic: &str) -> Option<PeerCluster> {
        if let Some(cluster) = self.clusters.remove(topic) {
            // Remove from peer mappings
            for peer_id in &cluster.peers {
                if let Some(topics) = self.peer_clusters.get_mut(peer_id) {
                    topics.remove(topic);
                }
            }
            info!("Removed cluster: {}", topic);
            Some(cluster)
        } else {
            None
        }
    }

    /// Add peer to cluster
    pub fn add_peer_to_cluster(&mut self, peer_id: PeerId, topic: &str) {
        // Update last seen
        self.peer_last_seen.insert(peer_id, SystemTime::now());

        // Add to cluster
        if let Some(cluster) = self.clusters.get_mut(topic) {
            cluster.add_peer(peer_id);

            // Update peer mapping
            self.peer_clusters
                .entry(peer_id)
                .or_insert_with(HashSet::new)
                .insert(topic.to_string());
        }
    }

    /// Remove peer from cluster
    pub fn remove_peer_from_cluster(&mut self, peer_id: &PeerId, topic: &str) {
        if let Some(cluster) = self.clusters.get_mut(topic) {
            cluster.remove_peer(peer_id);

            // Update peer mapping
            if let Some(topics) = self.peer_clusters.get_mut(peer_id) {
                topics.remove(topic);
            }
        }
    }

    /// Remove peer from all clusters
    pub fn remove_peer(&mut self, peer_id: &PeerId) {
        if let Some(topics) = self.peer_clusters.remove(peer_id) {
            for topic in topics {
                if let Some(cluster) = self.clusters.get_mut(&topic) {
                    cluster.remove_peer(peer_id);
                }
            }
        }
        self.peer_last_seen.remove(peer_id);
        info!("Removed peer {} from all clusters", peer_id);
    }

    /// Update peer last seen time
    pub fn update_peer_activity(&mut self, peer_id: PeerId) {
        self.peer_last_seen.insert(peer_id, SystemTime::now());
    }

    /// Clean up inactive peers from all clusters
    pub fn cleanup_inactive_peers(&mut self, timeout: Duration) {
        let mut total_removed = 0;

        for cluster in self.clusters.values_mut() {
            total_removed += cluster.remove_inactive_peers(&self.peer_last_seen, timeout);
        }

        if total_removed > 0 {
            info!("Cleaned up {} inactive peers from all clusters", total_removed);
        }
    }

    /// Get clusters for a peer
    pub fn get_peer_clusters(&self, peer_id: &PeerId) -> Vec<String> {
        self.peer_clusters
            .get(peer_id)
            .map(|topics| topics.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get cluster by topic
    pub fn get_cluster(&self, topic: &str) -> Option<&PeerCluster> {
        self.clusters.get(topic)
    }

    /// Get all clusters
    pub fn clusters(&self) -> &HashMap<String, PeerCluster> {
        &self.clusters
    }

    /// Find similar clusters
    pub fn find_similar_clusters(&self, topic: &str) -> Vec<(String, f32)> {
        if let Some(cluster) = self.clusters.get(topic) {
            let mut similar: Vec<_> = self
                .clusters
                .iter()
                .filter(|(t, _)| *t != topic)
                .map(|(t, c)| (t.clone(), cluster.similarity(c)))
                .filter(|(_, sim)| *sim >= self.similarity_threshold)
                .collect();

            similar.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            similar
        } else {
            Vec::new()
        }
    }

    /// Get best peers across all clusters
    pub fn get_best_peers(&self, count: usize) -> Vec<PeerId> {
        let mut all_peers: Vec<_> = self.peer_last_seen.keys().copied().collect();
        all_peers.truncate(count);
        all_peers
    }

    /// Get cluster statistics
    pub fn stats(&self) -> ClusterStats {
        let total_clusters = self.clusters.len();
        let total_peers = self.peer_last_seen.len();
        let avg_cluster_size = if total_clusters > 0 {
            self.clusters.values().map(|c| c.size()).sum::<usize>() as f32 / total_clusters as f32
        } else {
            0.0
        };

        ClusterStats {
            total_clusters,
            total_peers,
            avg_cluster_size,
        }
    }
}

impl Default for ClusteringManager {
    fn default() -> Self {
        Self::new(0.7) // Default similarity threshold
    }
}

/// Cluster statistics
#[derive(Debug, Clone)]
pub struct ClusterStats {
    pub total_clusters: usize,
    pub total_peers: usize,
    pub avg_cluster_size: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_cluster() {
        let mut cluster = PeerCluster::new(
            "ai".to_string(),
            vec!["machine-learning".to_string(), "neural-networks".to_string()],
        );

        let peer1 = PeerId::random();
        let peer2 = PeerId::random();

        cluster.add_peer(peer1);
        cluster.add_peer(peer2);

        assert_eq!(cluster.size(), 2);
        assert!(cluster.contains(&peer1));

        cluster.remove_peer(&peer1);
        assert_eq!(cluster.size(), 1);
        assert!(!cluster.contains(&peer1));
    }

    #[test]
    fn test_cluster_similarity() {
        let cluster1 = PeerCluster::new(
            "ai".to_string(),
            vec!["ml".to_string(), "nn".to_string(), "dl".to_string()],
        );

        let cluster2 = PeerCluster::new(
            "ml".to_string(),
            vec!["ml".to_string(), "nn".to_string()],
        );

        let similarity = cluster1.similarity(&cluster2);
        assert!(similarity > 0.5); // Should have high similarity
    }

    #[test]
    fn test_clustering_manager() {
        let mut manager = ClusteringManager::new(0.7);

        let cluster = PeerCluster::new("ai".to_string(), vec!["ml".to_string()]);
        manager.add_cluster(cluster);

        let peer = PeerId::random();
        manager.add_peer_to_cluster(peer, "ai");

        assert_eq!(manager.get_peer_clusters(&peer), vec!["ai".to_string()]);

        let stats = manager.stats();
        assert_eq!(stats.total_clusters, 1);
        assert_eq!(stats.total_peers, 1);
    }
}
