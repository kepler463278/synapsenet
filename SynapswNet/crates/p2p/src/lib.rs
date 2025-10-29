// SynapseNet P2P - libp2p networking layer

pub mod swarm;
pub mod topics;

pub use swarm::{P2pConfig, PeerInfo, SynapseSwarm};
pub use topics::{GossipMessage, Topic};
