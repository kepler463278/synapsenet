// SynapseNet P2P - libp2p networking layer

pub mod clustering;
pub mod dht;
pub mod nat;
pub mod pqc_transport;
pub mod swarm;
pub mod topics;

pub use clustering::{ClusterStats, ClusteringManager, PeerCluster};
pub use dht::{DhtDiscovery, TopicDiscovery};
pub use nat::{ConnectionMethod, ConnectionStrategy, NatTraversal, NatType, RelayNode};
#[cfg(feature = "pqc-kyber")]
pub use pqc_transport::{KyberHandshake, KyberKem};
pub use swarm::{P2pConfig, PeerInfo, SynapseSwarm};
pub use topics::{GossipMessage, Topic};
