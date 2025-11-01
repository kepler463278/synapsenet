use anyhow::Result;
use libp2p::{
    kad::{self, store::MemoryStore, Behaviour as Kademlia, Config as KademliaConfig},
    Multiaddr, PeerId,
};
use std::time::Duration;
use tracing::{info, warn};

/// DHT-based peer discovery using Kademlia
pub struct DhtDiscovery {
    /// Kademlia DHT behaviour
    kademlia: Kademlia<MemoryStore>,
}

impl DhtDiscovery {
    /// Create new DHT discovery
    pub fn new(local_peer_id: PeerId) -> Result<Self> {
        // Create Kademlia config
        let mut config = KademliaConfig::default();
        config.set_query_timeout(Duration::from_secs(60));
        config.set_replication_factor(20.try_into().unwrap());
        
        // Create memory store
        let store = MemoryStore::new(local_peer_id);
        
        // Create Kademlia behaviour
        let kademlia = Kademlia::with_config(local_peer_id, store, config);
        
        info!("DHT discovery initialized with peer ID: {}", local_peer_id);
        
        Ok(Self { kademlia })
    }

    /// Bootstrap the DHT by connecting to known peers
    pub fn bootstrap(&mut self, bootstrap_peers: Vec<(PeerId, Multiaddr)>) -> Result<()> {
        info!("Bootstrapping DHT with {} peers", bootstrap_peers.len());

        for (peer_id, addr) in bootstrap_peers {
            // Add peer to routing table
            self.kademlia.add_address(&peer_id, addr.clone());
            info!("Added bootstrap peer {} at {}", peer_id, addr);
        }

        // Start bootstrap process
        match self.kademlia.bootstrap() {
            Ok(query_id) => {
                info!("DHT bootstrap started with query ID: {:?}", query_id);
                Ok(())
            }
            Err(e) => {
                warn!("DHT bootstrap failed: {}", e);
                Err(anyhow::anyhow!("Bootstrap failed: {}", e))
            }
        }
    }

    /// Announce a topic to the DHT
    pub fn announce_topic(&mut self, topic: &str) -> Result<()> {
        let key = kad::RecordKey::new(&topic.as_bytes());
        let record = kad::Record {
            key: key.clone(),
            value: vec![], // Empty value, just announcing presence
            publisher: None,
            expires: None,
        };

        match self.kademlia.put_record(record, kad::Quorum::One) {
            Ok(query_id) => {
                info!("Announced topic '{}' to DHT (query: {:?})", topic, query_id);
                Ok(())
            }
            Err(e) => {
                warn!("Failed to announce topic '{}': {}", topic, e);
                Err(anyhow::anyhow!("Failed to announce topic: {}", e))
            }
        }
    }

    /// Find peers interested in a topic
    pub fn find_peers_for_topic(&mut self, topic: &str) -> Result<kad::QueryId> {
        let key = kad::RecordKey::new(&topic.as_bytes());
        
        let query_id = self.kademlia.get_record(key);
        info!("Searching for peers interested in topic '{}' (query: {:?})", topic, query_id);
        
        Ok(query_id)
    }

    /// Get closest peers to a key
    pub fn get_closest_peers(&mut self, key: &[u8]) -> Result<kad::QueryId> {
        let peer_id = PeerId::from_bytes(key)
            .map_err(|e| anyhow::anyhow!("Invalid peer ID: {}", e))?;
        
        let query_id = self.kademlia.get_closest_peers(peer_id);
        info!("Finding closest peers to {:?} (query: {:?})", peer_id, query_id);
        
        Ok(query_id)
    }

    /// Add a peer address to the routing table
    pub fn add_peer(&mut self, peer_id: &PeerId, addr: Multiaddr) {
        self.kademlia.add_address(peer_id, addr.clone());
        info!("Added peer {} at {} to DHT routing table", peer_id, addr);
    }

    /// Remove a peer from the routing table
    pub fn remove_peer(&mut self, peer_id: &PeerId) {
        self.kademlia.remove_peer(peer_id);
        info!("Removed peer {} from DHT routing table", peer_id);
    }

    /// Get the Kademlia behaviour (for integration with swarm)
    pub fn behaviour_mut(&mut self) -> &mut Kademlia<MemoryStore> {
        &mut self.kademlia
    }

    /// Get the Kademlia behaviour (immutable)
    pub fn behaviour(&self) -> &Kademlia<MemoryStore> {
        &self.kademlia
    }
}

/// Topic-based peer discovery
pub struct TopicDiscovery {
    /// Topics this node is interested in
    topics: Vec<String>,
}

impl TopicDiscovery {
    /// Create new topic discovery
    pub fn new() -> Self {
        Self {
            topics: Vec::new(),
        }
    }

    /// Add a topic of interest
    pub fn add_topic(&mut self, topic: String) {
        if !self.topics.contains(&topic) {
            self.topics.push(topic);
            info!("Added topic of interest: {}", self.topics.last().unwrap());
        }
    }

    /// Remove a topic
    pub fn remove_topic(&mut self, topic: &str) {
        self.topics.retain(|t| t != topic);
        info!("Removed topic of interest: {}", topic);
    }

    /// Get all topics
    pub fn topics(&self) -> &[String] {
        &self.topics
    }

    /// Check if interested in a topic
    pub fn is_interested(&self, topic: &str) -> bool {
        self.topics.iter().any(|t| t == topic)
    }
}

impl Default for TopicDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic_discovery() {
        let mut discovery = TopicDiscovery::new();
        
        discovery.add_topic("ai".to_string());
        discovery.add_topic("rust".to_string());
        
        assert_eq!(discovery.topics().len(), 2);
        assert!(discovery.is_interested("ai"));
        assert!(discovery.is_interested("rust"));
        assert!(!discovery.is_interested("python"));
        
        discovery.remove_topic("ai");
        assert_eq!(discovery.topics().len(), 1);
        assert!(!discovery.is_interested("ai"));
    }
}
