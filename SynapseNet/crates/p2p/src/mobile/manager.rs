//! Mobile P2P manager with battery awareness and background sync

use super::queue::{OperationQueue, P2POperation, OperationType};
use super::relay::{RelayManager, RelayConfig};
use super::webrtc::{WebRTCTransport, WebRTCConfig};
use anyhow::Result;
use libp2p::PeerId;
use std::time::{Duration, Instant};

/// Mobile P2P configuration
#[derive(Debug, Clone)]
pub struct MobileP2PConfig {
    /// Use WebRTC transport
    pub use_webrtc: bool,
    
    /// Use circuit relay
    pub use_relay: bool,
    
    /// Maximum peers to connect to
    pub max_peers: usize,
    
    /// Batch processing interval
    pub batch_interval: Duration,
    
    /// Cellular bandwidth limit (Mbps)
    pub cellular_limit_mbps: f32,
    
    /// Enable background sync
    pub background_sync: bool,
}

impl Default for MobileP2PConfig {
    fn default() -> Self {
        Self {
            use_webrtc: true,
            use_relay: true,
            max_peers: 10,
            batch_interval: Duration::from_secs(30),
            cellular_limit_mbps: 1.0,
            background_sync: true,
        }
    }
}

/// Network state for mobile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkState {
    Offline,
    WiFi,
    Cellular,
}

/// Mobile P2P manager
pub struct MobileP2PManager {
    config: MobileP2PConfig,
    webrtc: Option<WebRTCTransport>,
    relay: Option<RelayManager>,
    operation_queue: OperationQueue,
    network_state: NetworkState,
    last_sync: Instant,
    is_foreground: bool,
}

impl MobileP2PManager {
    /// Create a new mobile P2P manager
    pub fn new(config: MobileP2PConfig) -> Result<Self> {
        let webrtc = if config.use_webrtc {
            Some(WebRTCTransport::new(WebRTCConfig::default())?)
        } else {
            None
        };
        
        let relay = if config.use_relay {
            Some(RelayManager::new(RelayConfig::default()))
        } else {
            None
        };
        
        let operation_queue = OperationQueue::new(1000);
        
        Ok(Self {
            config,
            webrtc,
            relay,
            operation_queue,
            network_state: NetworkState::Offline,
            last_sync: Instant::now(),
            is_foreground: true,
        })
    }
    
    /// Connect to P2P network
    pub async fn connect(&mut self) -> Result<()> {
        tracing::info!("Connecting to P2P network (mobile)");
        
        // Connect to relays if enabled
        if let Some(relay) = &mut self.relay {
            relay.connect_bootstrap().await?;
        }
        
        // Start processing queue
        self.process_queue().await?;
        
        Ok(())
    }
    
    /// Disconnect from P2P network
    pub fn disconnect(&mut self) {
        tracing::info!("Disconnecting from P2P network (mobile)");
        
        // Save queue state
        let _ = self.operation_queue.save();
    }
    
    /// Queue an operation for later processing
    pub fn queue_operation(&mut self, operation: P2POperation) {
        if let Err(e) = self.operation_queue.enqueue(operation) {
            tracing::error!("Failed to queue operation: {}", e);
        }
    }
    
    /// Process queued operations
    pub async fn process_queue(&mut self) -> Result<()> {
        let wifi_available = self.network_state == NetworkState::WiFi;
        let processable = self.operation_queue.get_processable(wifi_available);
        
        tracing::info!("Processing {} queued operations", processable.len());
        
        for operation in processable {
            match self.process_operation(&operation).await {
                Ok(_) => {
                    tracing::debug!("Operation {} completed", operation.id);
                }
                Err(e) => {
                    tracing::warn!("Operation {} failed: {}", operation.id, e);
                    self.operation_queue.requeue(operation)?;
                }
            }
        }
        
        self.last_sync = Instant::now();
        Ok(())
    }
    
    /// Process a single operation
    async fn process_operation(&mut self, operation: &P2POperation) -> Result<()> {
        match &operation.op_type {
            OperationType::PublishGrain { grain_id, data } => {
                tracing::debug!("Publishing grain: {:?}", grain_id);
                // TODO: Implement grain publishing
                Ok(())
            }
            OperationType::QueryNetwork { query, k } => {
                tracing::debug!("Querying network for {} results", k);
                // TODO: Implement network query
                Ok(())
            }
            OperationType::SyncRewards => {
                tracing::debug!("Syncing rewards");
                // TODO: Implement reward sync
                Ok(())
            }
            OperationType::UpdatePeerList => {
                tracing::debug!("Updating peer list");
                // TODO: Implement peer list update
                Ok(())
            }
            OperationType::SyncGrain { grain_id, peer_id } => {
                tracing::debug!("Syncing grain {:?} with {}", grain_id, peer_id);
                // TODO: Implement grain sync
                Ok(())
            }
        }
    }
    
    /// Get connected peers
    pub fn get_peers(&self) -> Vec<PeerId> {
        let mut peers = Vec::new();
        
        if let Some(webrtc) = &self.webrtc {
            peers.extend(webrtc.active_connections());
        }
        
        if let Some(relay) = &self.relay {
            peers.extend(relay.active_relays());
        }
        
        peers
    }
    
    /// Sync grains with network
    pub async fn sync_grains(&mut self) -> Result<()> {
        tracing::info!("Syncing grains with network");
        
        // Queue sync operations
        let sync_op = P2POperation::new(OperationType::SyncRewards);
        self.queue_operation(sync_op);
        
        // Process immediately if in foreground
        if self.is_foreground {
            self.process_queue().await?;
        }
        
        Ok(())
    }
    
    /// Update network state
    pub fn update_network_state(&mut self, state: NetworkState) {
        if self.network_state != state {
            tracing::info!("Network state changed: {:?} -> {:?}", self.network_state, state);
            self.network_state = state;
            
            // Process queue if network became available
            if state != NetworkState::Offline {
                let manager = self.clone_for_async();
                tokio::spawn(async move {
                    let mut m = manager;
                    let _ = m.process_queue().await;
                });
            }
        }
    }
    
    /// Update foreground/background state
    pub fn set_foreground(&mut self, is_foreground: bool) {
        if self.is_foreground != is_foreground {
            tracing::info!("App state changed: {}", if is_foreground { "foreground" } else { "background" });
            self.is_foreground = is_foreground;
            
            if is_foreground {
                // Process queue when coming to foreground
                let manager = self.clone_for_async();
                tokio::spawn(async move {
                    let mut m = manager;
                    let _ = m.process_queue().await;
                });
            }
        }
    }
    
    /// Check if should sync now
    pub fn should_sync(&self) -> bool {
        self.last_sync.elapsed() > self.config.batch_interval
            && self.network_state != NetworkState::Offline
    }
    
    /// Get queue size
    pub fn queue_size(&self) -> usize {
        self.operation_queue.len()
    }
    
    /// Clone for async operations (placeholder)
    fn clone_for_async(&self) -> Self {
        // TODO: Implement proper cloning or use Arc
        Self {
            config: self.config.clone(),
            webrtc: None,
            relay: None,
            operation_queue: OperationQueue::new(1000),
            network_state: self.network_state,
            last_sync: self.last_sync,
            is_foreground: self.is_foreground,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_manager_creation() {
        let config = MobileP2PConfig::default();
        let manager = MobileP2PManager::new(config);
        assert!(manager.is_ok());
    }
    
    #[tokio::test]
    async fn test_queue_operation() {
        let config = MobileP2PConfig::default();
        let mut manager = MobileP2PManager::new(config).unwrap();
        
        let op = P2POperation::new(OperationType::SyncRewards);
        manager.queue_operation(op);
        
        assert_eq!(manager.queue_size(), 1);
    }
    
    #[test]
    fn test_network_state_update() {
        let config = MobileP2PConfig::default();
        let mut manager = MobileP2PManager::new(config).unwrap();
        
        assert_eq!(manager.network_state, NetworkState::Offline);
        
        manager.update_network_state(NetworkState::WiFi);
        assert_eq!(manager.network_state, NetworkState::WiFi);
    }
    
    #[test]
    fn test_foreground_background() {
        let config = MobileP2PConfig::default();
        let mut manager = MobileP2PManager::new(config).unwrap();
        
        assert!(manager.is_foreground);
        
        manager.set_foreground(false);
        assert!(!manager.is_foreground);
    }
}
