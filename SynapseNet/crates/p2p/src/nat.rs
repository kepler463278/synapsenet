use anyhow::Result;
use libp2p::PeerId;
use tracing::{info, warn};

/// NAT type detection result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NatType {
    /// No NAT (public IP)
    Public,
    /// Full cone NAT (easiest to traverse)
    FullCone,
    /// Restricted cone NAT
    RestrictedCone,
    /// Port-restricted cone NAT
    PortRestrictedCone,
    /// Symmetric NAT (hardest to traverse)
    Symmetric,
    /// Unknown NAT type
    Unknown,
}

impl NatType {
    /// Check if NAT traversal is possible without relay
    pub fn can_traverse(&self) -> bool {
        matches!(
            self,
            NatType::Public | NatType::FullCone | NatType::RestrictedCone | NatType::PortRestrictedCone
        )
    }

    /// Check if relay is required
    pub fn needs_relay(&self) -> bool {
        matches!(self, NatType::Symmetric | NatType::Unknown)
    }
}

impl std::fmt::Display for NatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NatType::Public => write!(f, "Public"),
            NatType::FullCone => write!(f, "Full Cone NAT"),
            NatType::RestrictedCone => write!(f, "Restricted Cone NAT"),
            NatType::PortRestrictedCone => write!(f, "Port-Restricted Cone NAT"),
            NatType::Symmetric => write!(f, "Symmetric NAT"),
            NatType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// NAT traversal system
pub struct NatTraversal {
    /// Detected NAT type
    nat_type: NatType,
    /// Whether relay is enabled
    relay_enabled: bool,
}

impl NatTraversal {
    /// Create new NAT traversal system
    pub fn new() -> Self {
        Self {
            nat_type: NatType::Unknown,
            relay_enabled: false,
        }
    }

    /// Detect NAT type
    /// 
    /// Note: This is a simplified implementation. Full NAT detection requires:
    /// - STUN server queries
    /// - Multiple connection attempts
    /// - Analysis of external/internal addresses
    /// 
    /// For v0.4 MVP, we'll use a basic heuristic
    pub async fn detect_nat_type(&mut self) -> Result<NatType> {
        info!("Detecting NAT type...");

        // TODO: Implement proper NAT detection using STUN
        // For now, assume we're behind NAT and need relay
        self.nat_type = NatType::Unknown;

        info!("NAT type detected: {}", self.nat_type);
        Ok(self.nat_type)
    }

    /// Get current NAT type
    pub fn nat_type(&self) -> NatType {
        self.nat_type
    }

    /// Enable relay for NAT traversal
    pub fn enable_relay(&mut self) {
        self.relay_enabled = true;
        info!("Relay enabled for NAT traversal");
    }

    /// Disable relay
    pub fn disable_relay(&mut self) {
        self.relay_enabled = false;
        info!("Relay disabled");
    }

    /// Check if relay is enabled
    pub fn is_relay_enabled(&self) -> bool {
        self.relay_enabled
    }

    /// Attempt to establish connection to peer
    /// 
    /// Strategy:
    /// 1. Try direct connection first
    /// 2. If behind NAT, attempt hole-punching
    /// 3. If hole-punching fails, use relay
    pub async fn establish_connection(&self, peer_id: &PeerId) -> Result<ConnectionMethod> {
        info!("Establishing connection to peer: {}", peer_id);

        // Check NAT type
        match self.nat_type {
            NatType::Public => {
                info!("Public IP detected, using direct connection");
                Ok(ConnectionMethod::Direct)
            }
            NatType::FullCone | NatType::RestrictedCone | NatType::PortRestrictedCone => {
                info!("NAT detected, attempting hole-punching");
                // TODO: Implement actual hole-punching
                // For now, fall back to relay
                if self.relay_enabled {
                    warn!("Hole-punching not implemented, using relay");
                    Ok(ConnectionMethod::Relay)
                } else {
                    Err(anyhow::anyhow!("Cannot establish connection: relay disabled"))
                }
            }
            NatType::Symmetric | NatType::Unknown => {
                if self.relay_enabled {
                    info!("Symmetric NAT or unknown type, using relay");
                    Ok(ConnectionMethod::Relay)
                } else {
                    Err(anyhow::anyhow!("Cannot establish connection: relay required but disabled"))
                }
            }
        }
    }

    /// Get connection strategy for peer
    pub fn get_connection_strategy(&self) -> ConnectionStrategy {
        if self.nat_type.can_traverse() {
            ConnectionStrategy::DirectOrHolePunch
        } else if self.relay_enabled {
            ConnectionStrategy::RelayOnly
        } else {
            ConnectionStrategy::None
        }
    }
}

impl Default for NatTraversal {
    fn default() -> Self {
        Self::new()
    }
}

/// Connection method used
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionMethod {
    /// Direct connection (no NAT)
    Direct,
    /// Hole-punching through NAT
    HolePunch,
    /// Relay connection
    Relay,
}

/// Connection strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStrategy {
    /// Try direct connection or hole-punching
    DirectOrHolePunch,
    /// Use relay only
    RelayOnly,
    /// No connection possible
    None,
}

/// Relay node information
#[derive(Debug, Clone)]
pub struct RelayNode {
    /// Relay peer ID
    pub peer_id: PeerId,
    /// Relay addresses
    pub addresses: Vec<libp2p::Multiaddr>,
    /// Relay capacity (max connections)
    pub capacity: usize,
    /// Current load (active connections)
    pub load: usize,
}

impl RelayNode {
    /// Check if relay has capacity
    pub fn has_capacity(&self) -> bool {
        self.load < self.capacity
    }

    /// Get load percentage
    pub fn load_percentage(&self) -> f32 {
        if self.capacity == 0 {
            return 100.0;
        }
        (self.load as f32 / self.capacity as f32) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nat_type() {
        assert!(NatType::Public.can_traverse());
        assert!(NatType::FullCone.can_traverse());
        assert!(!NatType::Symmetric.can_traverse());
        
        assert!(!NatType::Public.needs_relay());
        assert!(NatType::Symmetric.needs_relay());
    }

    #[tokio::test]
    async fn test_nat_traversal() {
        let mut nat = NatTraversal::new();
        
        assert_eq!(nat.nat_type(), NatType::Unknown);
        assert!(!nat.is_relay_enabled());
        
        nat.enable_relay();
        assert!(nat.is_relay_enabled());
        
        let nat_type = nat.detect_nat_type().await.unwrap();
        assert_eq!(nat_type, NatType::Unknown);
    }

    #[test]
    fn test_relay_node() {
        let relay = RelayNode {
            peer_id: PeerId::random(),
            addresses: vec![],
            capacity: 100,
            load: 50,
        };
        
        assert!(relay.has_capacity());
        assert_eq!(relay.load_percentage(), 50.0);
    }
}
