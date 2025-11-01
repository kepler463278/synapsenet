//! Mobile P2P networking with WebRTC and battery optimization

pub mod webrtc;
pub mod relay;
pub mod queue;
pub mod network_state;
pub mod manager;

pub use manager::{MobileP2PManager, MobileP2PConfig, NetworkState};
pub use queue::{P2POperation, OperationType};
pub use network_state::NetworkStateMonitor;
