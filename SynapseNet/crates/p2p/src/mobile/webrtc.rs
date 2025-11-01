//! WebRTC transport for mobile P2P

use anyhow::Result;
use libp2p::{PeerId, Multiaddr};
use std::time::Duration;

/// WebRTC transport configuration
#[derive(Debug, Clone)]
pub struct WebRTCConfig {
    /// STUN server URLs
    pub stun_servers: Vec<String>,
    
    /// TURN server URLs (optional)
    pub turn_servers: Vec<TurnServer>,
    
    /// Connection timeout
    pub connection_timeout: Duration,
    
    /// Keep-alive interval
    pub keep_alive_interval: Duration,
}

impl Default for WebRTCConfig {
    fn default() -> Self {
        Self {
            stun_servers: vec![
                "stun:stun.l.google.com:19302".to_string(),
                "stun:stun1.l.google.com:19302".to_string(),
            ],
            turn_servers: Vec::new(),
            connection_timeout: Duration::from_secs(30),
            keep_alive_interval: Duration::from_secs(15),
        }
    }
}

/// TURN server configuration
#[derive(Debug, Clone)]
pub struct TurnServer {
    pub url: String,
    pub username: Option<String>,
    pub credential: Option<String>,
}

/// WebRTC transport manager
pub struct WebRTCTransport {
    config: WebRTCConfig,
    // TODO: Add libp2p-webrtc transport
}

impl WebRTCTransport {
    /// Create a new WebRTC transport
    pub fn new(config: WebRTCConfig) -> Result<Self> {
        tracing::info!("Initializing WebRTC transport");
        tracing::debug!("STUN servers: {:?}", config.stun_servers);
        
        Ok(Self {
            config,
        })
    }
    
    /// Establish WebRTC connection to peer
    pub async fn connect(&mut self, peer_id: PeerId, addr: Multiaddr) -> Result<()> {
        tracing::info!("Connecting to peer {} via WebRTC", peer_id);
        
        // TODO: Implement WebRTC connection establishment
        // 1. Create offer
        // 2. Exchange SDP via signaling
        // 3. Establish ICE connection
        // 4. Create data channel
        
        Ok(())
    }
    
    /// Check connection quality
    pub fn connection_quality(&self, _peer_id: &PeerId) -> ConnectionQuality {
        // TODO: Implement connection quality monitoring
        // - RTT (Round Trip Time)
        // - Packet loss
        // - Bandwidth
        
        ConnectionQuality {
            rtt_ms: 50,
            packet_loss: 0.0,
            bandwidth_kbps: 1000,
        }
    }
    
    /// Close connection to peer
    pub async fn disconnect(&mut self, peer_id: PeerId) -> Result<()> {
        tracing::info!("Disconnecting from peer {} (WebRTC)", peer_id);
        
        // TODO: Implement graceful disconnect
        
        Ok(())
    }
    
    /// Get active connections
    pub fn active_connections(&self) -> Vec<PeerId> {
        // TODO: Return list of active WebRTC connections
        Vec::new()
    }
}

/// Connection quality metrics
#[derive(Debug, Clone)]
pub struct ConnectionQuality {
    /// Round-trip time in milliseconds
    pub rtt_ms: u32,
    
    /// Packet loss percentage (0.0 - 1.0)
    pub packet_loss: f32,
    
    /// Available bandwidth in kbps
    pub bandwidth_kbps: u32,
}

impl ConnectionQuality {
    /// Check if connection is good
    pub fn is_good(&self) -> bool {
        self.rtt_ms < 200 && self.packet_loss < 0.05
    }
    
    /// Check if connection is acceptable
    pub fn is_acceptable(&self) -> bool {
        self.rtt_ms < 500 && self.packet_loss < 0.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_webrtc_config_default() {
        let config = WebRTCConfig::default();
        assert!(!config.stun_servers.is_empty());
        assert_eq!(config.connection_timeout, Duration::from_secs(30));
    }
    
    #[test]
    fn test_connection_quality() {
        let good = ConnectionQuality {
            rtt_ms: 50,
            packet_loss: 0.01,
            bandwidth_kbps: 1000,
        };
        assert!(good.is_good());
        assert!(good.is_acceptable());
        
        let poor = ConnectionQuality {
            rtt_ms: 600,
            packet_loss: 0.15,
            bandwidth_kbps: 100,
        };
        assert!(!poor.is_good());
        assert!(!poor.is_acceptable());
    }
    
    #[tokio::test]
    async fn test_webrtc_transport_creation() {
        let config = WebRTCConfig::default();
        let transport = WebRTCTransport::new(config);
        assert!(transport.is_ok());
    }
}
