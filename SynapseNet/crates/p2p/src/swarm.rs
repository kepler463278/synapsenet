use anyhow::Result;
use futures::StreamExt;
use libp2p::{
    gossipsub, identify, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, Swarm, Transport,
};
use std::collections::{hash_map::DefaultHasher, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::topics::{GossipMessage, QueryResult, Topic};

/// P2P configuration
#[derive(Clone, Debug)]
pub struct P2pConfig {
    pub port: u16,
    pub enable_mdns: bool,
    pub bootstrap_peers: Vec<Multiaddr>,
}

impl Default for P2pConfig {
    fn default() -> Self {
        Self {
            port: 9000,
            enable_mdns: true,
            bootstrap_peers: Vec::new(),
        }
    }
}

/// Peer information
#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub peer_id: PeerId,
    pub addresses: Vec<Multiaddr>,
    pub connected_at: i64,
    pub grains_received: u64,
    pub grains_sent: u64,
    pub reputation: f64,
    pub last_seen: i64,
    /// Timestamp of last grain received (for rate limiting)
    pub last_grain_time: i64,
    /// Number of grains received in current minute
    pub grains_this_minute: u32,
}

/// Query state for tracking distributed queries
#[derive(Debug)]
struct QueryState {
    /// Query vector
    vector: Vec<f32>,
    /// Number of results requested
    k: usize,
    /// Collected results from peers
    results: Vec<QueryResult>,
    /// Channel to send final results
    response_tx: mpsc::Sender<Vec<QueryResult>>,
    /// Timestamp when query was created
    created_at: i64,
}

/// Callback for handling received grains
pub type GrainCallback = Box<dyn Fn(synapsenet_core::Grain) -> Result<()> + Send + Sync>;

/// SynapseNet P2P swarm
pub struct SynapseSwarm {
    swarm: Swarm<SynapseBehaviour>,
    local_peer_id: PeerId,
    connected_peers: HashMap<PeerId, PeerInfo>,
    config: P2pConfig,
    /// Track sent grains to avoid duplicates
    sent_grains: HashSet<[u8; 32]>,
    /// Track received grains to avoid duplicates
    received_grains: HashSet<[u8; 32]>,
    /// Track active queries
    active_queries: HashMap<String, QueryState>,
    /// Callback for storing received grains
    grain_callback: Option<GrainCallback>,
}

#[derive(NetworkBehaviour)]
struct SynapseBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
    identify: identify::Behaviour,
}

impl SynapseSwarm {
    /// Create new swarm with mDNS discovery
    pub async fn new(config: P2pConfig) -> Result<Self> {
        // Generate keypair
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        info!("Initializing P2P swarm with peer ID: {}", local_peer_id);

        // Create transport with Noise encryption
        let transport = tcp::tokio::Transport::default()
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::Config::new(&local_key)?)
            .multiplex(yamux::Config::default())
            .boxed();

        // Configure GossipSub
        let message_id_fn = |message: &gossipsub::Message| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            gossipsub::MessageId::from(s.finish().to_string())
        };

        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .message_id_fn(message_id_fn)
            .max_transmit_size(1024 * 1024) // 1MB max message size
            .build()
            .map_err(|msg| anyhow::anyhow!("GossipSub config error: {}", msg))?;

        let mut gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )
        .map_err(|e| anyhow::anyhow!("Failed to create GossipSub: {}", e))?;

        // Subscribe to core topics
        let topics = [
            Topic::GrainsPut.as_str(),
            Topic::GrainsAck.as_str(),
            Topic::QueryKnn.as_str(),
            Topic::QueryResp.as_str(),
        ];

        for topic_str in &topics {
            let topic = gossipsub::IdentTopic::new(*topic_str);
            gossipsub.subscribe(&topic)?;
            info!("Subscribed to topic: {}", topic_str);
        }

        // Configure mDNS
        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)?;

        // Configure Identify protocol
        let identify = identify::Behaviour::new(identify::Config::new(
            "/synapsenet/1.0.0".to_string(),
            local_key.public(),
        ));

        let behaviour = SynapseBehaviour {
            gossipsub,
            mdns,
            identify,
        };

        // Create swarm
        let swarm_config = libp2p::swarm::Config::with_tokio_executor();
        let mut swarm = Swarm::new(transport, behaviour, local_peer_id, swarm_config);

        // Listen on all interfaces
        let listen_addr: Multiaddr = format!("/ip4/0.0.0.0/tcp/{}", config.port).parse()?;
        swarm.listen_on(listen_addr)?;

        info!("P2P swarm listening on port {}", config.port);

        Ok(Self {
            swarm,
            local_peer_id,
            connected_peers: HashMap::new(),
            config,
            sent_grains: HashSet::new(),
            received_grains: HashSet::new(),
            active_queries: HashMap::new(),
            grain_callback: None,
        })
    }

    /// Start swarm event loop
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting P2P swarm event loop");

        // Connect to bootstrap peers
        for addr in &self.config.bootstrap_peers {
            info!("Dialing bootstrap peer: {}", addr);
            if let Err(e) = self.swarm.dial(addr.clone()) {
                warn!("Failed to dial bootstrap peer {}: {}", addr, e);
            }
        }

        // Set up timeout for peer discovery
        let mut discovery_timeout = tokio::time::interval(Duration::from_secs(30));

        // Set up reputation check interval
        let mut reputation_check = tokio::time::interval(Duration::from_secs(60));

        loop {
            select! {
                event = self.swarm.select_next_some() => {
                    if let Err(e) = self.handle_swarm_event(event).await {
                        error!("Error handling swarm event: {}", e);
                    }
                }
                _ = discovery_timeout.tick() => {
                    if self.connected_peers.is_empty() {
                        warn!("No peers connected after 30 seconds");
                    } else {
                        debug!("Connected to {} peers", self.connected_peers.len());
                    }
                }
                _ = reputation_check.tick() => {
                    self.check_peer_reputation();
                }
            }
        }
    }

    /// Handle swarm events
    async fn handle_swarm_event(&mut self, event: SwarmEvent<SynapseBehaviourEvent>) -> Result<()> {
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                info!("Listening on {}", address);
            }
            SwarmEvent::Behaviour(SynapseBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                for (peer_id, multiaddr) in list {
                    info!("Discovered peer via mDNS: {} at {}", peer_id, multiaddr);
                    if let Err(e) = self.swarm.dial(multiaddr) {
                        warn!("Failed to dial discovered peer: {}", e);
                    }
                }
            }
            SwarmEvent::Behaviour(SynapseBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                for (peer_id, _) in list {
                    debug!("mDNS record expired for peer: {}", peer_id);
                }
            }
            SwarmEvent::Behaviour(SynapseBehaviourEvent::Identify(identify::Event::Received {
                peer_id,
                info,
            })) => {
                debug!(
                    "Received identify from {}: protocol {}",
                    peer_id, info.protocol_version
                );
            }
            SwarmEvent::Behaviour(SynapseBehaviourEvent::Gossipsub(
                gossipsub::Event::Message {
                    propagation_source: _,
                    message_id: _,
                    message,
                },
            )) => {
                self.handle_gossip_message(message).await?;
            }
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                info!(
                    "Connection established with peer: {} at {}",
                    peer_id,
                    endpoint.get_remote_address()
                );

                let peer_info = PeerInfo {
                    peer_id,
                    addresses: vec![endpoint.get_remote_address().clone()],
                    connected_at: chrono::Utc::now().timestamp_millis(),
                    grains_received: 0,
                    grains_sent: 0,
                    reputation: 0.0,
                    last_seen: chrono::Utc::now().timestamp_millis(),
                    last_grain_time: 0,
                    grains_this_minute: 0,
                };

                self.connected_peers.insert(peer_id, peer_info);
            }
            SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                info!(
                    "Connection closed with peer: {} (cause: {:?})",
                    peer_id, cause
                );
                self.connected_peers.remove(&peer_id);
            }
            SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                if let Some(peer_id) = peer_id {
                    warn!("Outgoing connection error to {}: {}", peer_id, error);
                } else {
                    warn!("Outgoing connection error: {}", error);
                }
            }
            SwarmEvent::IncomingConnectionError {
                local_addr,
                send_back_addr,
                error,
                ..
            } => {
                warn!(
                    "Incoming connection error from {} to {}: {}",
                    send_back_addr, local_addr, error
                );
            }
            _ => {}
        }

        Ok(())
    }

    /// Handle received GossipSub message
    async fn handle_gossip_message(&mut self, message: gossipsub::Message) -> Result<()> {
        debug!("Received message on topic: {}", message.topic);

        // Deserialize message
        let gossip_msg: GossipMessage = bincode::deserialize(&message.data)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize message: {}", e))?;

        // Update peer stats
        if let Some(source) = message.source {
            if let Some(peer_info) = self.connected_peers.get_mut(&source) {
                peer_info.last_seen = chrono::Utc::now().timestamp_millis();
            }
        }

        // Handle message based on type
        match gossip_msg {
            GossipMessage::GrainPut { grain, links: _ } => {
                info!("Received grain: {:?}", hex_encode(&grain.id[..8]));

                // Check if already received
                if self.received_grains.contains(&grain.id) {
                    debug!(
                        "Grain {:?} already received, skipping",
                        hex_encode(&grain.id[..8])
                    );
                    return Ok(());
                }

                // Rate limiting: 100 grains per minute per peer
                if let Some(source) = message.source {
                    if let Some(peer_info) = self.connected_peers.get_mut(&source) {
                        let now = chrono::Utc::now().timestamp_millis();
                        let time_diff = now - peer_info.last_grain_time;

                        // Reset counter if more than 1 minute passed
                        if time_diff > 60_000 {
                            peer_info.grains_this_minute = 0;
                            peer_info.last_grain_time = now;
                        }

                        // Check rate limit
                        if peer_info.grains_this_minute >= 100 {
                            warn!(
                                "Rate limit exceeded for peer {}: {} grains/min",
                                source, peer_info.grains_this_minute
                            );
                            peer_info.reputation -= 0.5;
                            return Ok(());
                        }

                        peer_info.grains_this_minute += 1;
                    }
                }

                // Verify grain signature
                match grain.verify() {
                    Ok(true) => {
                        info!("Grain signature verified: {:?}", hex_encode(&grain.id[..8]));

                        // Track received grain
                        self.received_grains.insert(grain.id);

                        // Update peer stats
                        if let Some(source) = message.source {
                            if let Some(peer_info) = self.connected_peers.get_mut(&source) {
                                peer_info.grains_received += 1;
                            }
                        }

                        // Store grain using callback if available
                        if let Some(ref callback) = self.grain_callback {
                            match callback(grain.clone()) {
                                Ok(()) => {
                                    info!("Grain stored successfully: {:?}", hex_encode(&grain.id[..8]));
                                }
                                Err(e) => {
                                    warn!("Failed to store grain: {}", e);
                                }
                            }
                        }

                        // Send acknowledgment
                        let ack_msg = GossipMessage::GrainAck {
                            grain_id: grain.id,
                            peer_id: self.local_peer_id.to_string(),
                        };

                        let ack_data = bincode::serialize(&ack_msg)?;
                        let ack_topic = gossipsub::IdentTopic::new(Topic::GrainsAck.as_str());

                        if let Err(e) = self
                            .swarm
                            .behaviour_mut()
                            .gossipsub
                            .publish(ack_topic, ack_data)
                        {
                            warn!("Failed to send grain ack: {}", e);
                        }
                    }
                    Ok(false) => {
                        warn!("Invalid grain signature: {:?}", hex_encode(&grain.id[..8]));

                        // Decrease peer reputation
                        if let Some(source) = message.source {
                            if let Some(peer_info) = self.connected_peers.get_mut(&source) {
                                peer_info.reputation -= 1.0;
                                warn!(
                                    "Decreased reputation for peer {} to {}",
                                    source, peer_info.reputation
                                );
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error verifying grain signature: {}", e);
                    }
                }
            }
            GossipMessage::GrainAck { grain_id, peer_id } => {
                debug!(
                    "Received grain ack for {:?} from {}",
                    hex_encode(&grain_id[..8]),
                    peer_id
                );
            }
            GossipMessage::QueryKnn {
                query_id,
                vector,
                k,
            } => {
                debug!("Received KNN query {} (k={})", query_id, k);

                // TODO: Perform local KNN search
                // For now, we'll send empty results as placeholder
                // In real implementation, this would query local HNSW index

                let results = Vec::new(); // Placeholder

                // Send response
                let response_msg = GossipMessage::QueryResp {
                    query_id: query_id.clone(),
                    results,
                };

                let response_data = bincode::serialize(&response_msg)?;
                let response_topic = gossipsub::IdentTopic::new(Topic::QueryResp.as_str());

                if let Err(e) = self
                    .swarm
                    .behaviour_mut()
                    .gossipsub
                    .publish(response_topic, response_data)
                {
                    warn!("Failed to send query response: {}", e);
                } else {
                    debug!("Sent response for query {}", query_id);
                }
            }
            GossipMessage::QueryResp { query_id, results } => {
                debug!(
                    "Received query response for {} ({} results)",
                    query_id,
                    results.len()
                );

                // Find active query
                if let Some(query_state) = self.active_queries.get_mut(&query_id) {
                    // Send results through channel
                    if let Err(e) = query_state.response_tx.try_send(results) {
                        warn!("Failed to send query results to channel: {}", e);
                    }
                } else {
                    debug!("Received response for unknown query: {}", query_id);
                }
            }
        }

        Ok(())
    }

    /// Broadcast grain to all peers
    pub fn broadcast_grain(&mut self, grain: &synapsenet_core::Grain) -> Result<()> {
        // Check if already sent
        if self.sent_grains.contains(&grain.id) {
            debug!(
                "Grain {:?} already sent, skipping",
                hex_encode(&grain.id[..8])
            );
            return Ok(());
        }

        let message = GossipMessage::GrainPut {
            grain: grain.clone(),
            links: Vec::new(), // TODO: Include relevant links
        };

        let data = bincode::serialize(&message)?;
        let topic = gossipsub::IdentTopic::new(Topic::GrainsPut.as_str());

        self.swarm.behaviour_mut().gossipsub.publish(topic, data)?;

        // Track sent grain
        self.sent_grains.insert(grain.id);

        info!("Broadcasted grain: {:?}", hex_encode(&grain.id[..8]));

        Ok(())
    }

    /// Get connected peer count
    pub fn peer_count(&self) -> usize {
        self.connected_peers.len()
    }

    /// Get peer information
    pub fn peers(&self) -> &HashMap<PeerId, PeerInfo> {
        &self.connected_peers
    }

    /// Get local peer ID
    pub fn local_peer_id(&self) -> PeerId {
        self.local_peer_id
    }
    
    /// Set callback for storing received grains
    pub fn set_grain_callback<F>(&mut self, callback: F)
    where
        F: Fn(synapsenet_core::Grain) -> Result<()> + Send + Sync + 'static,
    {
        self.grain_callback = Some(Box::new(callback));
    }

    /// Query peers for similar grains (distributed KNN search)
    pub async fn query_peers(
        &mut self,
        query_vector: Vec<f32>,
        k: usize,
        timeout_secs: u64,
    ) -> Result<Vec<QueryResult>> {
        // Generate unique query ID
        let query_id = uuid::Uuid::new_v4().to_string();

        info!("Starting distributed query {} (k={})", query_id, k);

        // Create channel for collecting results
        let (tx, mut rx) = mpsc::channel(100);

        // Store query state
        let query_state = QueryState {
            vector: query_vector.clone(),
            k,
            results: Vec::new(),
            response_tx: tx,
            created_at: chrono::Utc::now().timestamp_millis(),
        };

        self.active_queries.insert(query_id.clone(), query_state);

        // Broadcast query to peers
        let message = GossipMessage::QueryKnn {
            query_id: query_id.clone(),
            vector: query_vector,
            k,
        };

        let data = bincode::serialize(&message)?;
        let topic = gossipsub::IdentTopic::new(Topic::QueryKnn.as_str());

        self.swarm.behaviour_mut().gossipsub.publish(topic, data)?;

        info!("Query {} broadcasted to peers", query_id);

        // Wait for responses with timeout
        let timeout = tokio::time::sleep(Duration::from_secs(timeout_secs));
        tokio::pin!(timeout);

        let mut all_results = Vec::new();

        loop {
            select! {
                Some(results) = rx.recv() => {
                    all_results.extend(results);
                }
                _ = &mut timeout => {
                    info!("Query {} timeout reached", query_id);
                    break;
                }
            }
        }

        // Remove query from active queries
        self.active_queries.remove(&query_id);

        // Sort by similarity descending and take top k
        all_results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        all_results.truncate(k);

        info!("Query {} complete: {} results", query_id, all_results.len());

        Ok(all_results)
    }

    /// Check and disconnect peers with bad reputation
    pub fn check_peer_reputation(&mut self) {
        let mut peers_to_disconnect = Vec::new();

        for (peer_id, peer_info) in &self.connected_peers {
            if peer_info.reputation < -10.0 {
                warn!(
                    "Disconnecting peer {} due to bad reputation: {}",
                    peer_id, peer_info.reputation
                );
                peers_to_disconnect.push(*peer_id);
            }
        }

        for peer_id in peers_to_disconnect {
            self.swarm.disconnect_peer_id(peer_id);
            self.connected_peers.remove(&peer_id);
            info!("Disconnected peer {} due to bad reputation", peer_id);
        }
    }

    /// Increase peer reputation (for good behavior)
    pub fn increase_peer_reputation(&mut self, peer_id: &PeerId, amount: f64) {
        if let Some(peer_info) = self.connected_peers.get_mut(peer_id) {
            peer_info.reputation += amount;
            debug!(
                "Increased reputation for peer {} to {}",
                peer_id, peer_info.reputation
            );
        }
    }

    /// Decrease peer reputation (for bad behavior)
    pub fn decrease_peer_reputation(&mut self, peer_id: &PeerId, amount: f64) {
        if let Some(peer_info) = self.connected_peers.get_mut(peer_id) {
            peer_info.reputation -= amount;
            warn!(
                "Decreased reputation for peer {} to {}",
                peer_id, peer_info.reputation
            );

            // Check if should disconnect immediately
            if peer_info.reputation < -10.0 {
                warn!("Peer {} reputation too low, disconnecting", peer_id);
                self.swarm.disconnect_peer_id(*peer_id);
                self.connected_peers.remove(peer_id);
            }
        }
    }

    /// Get peer statistics
    pub fn get_peer_stats(&self) -> Vec<(PeerId, f64, u64, u64)> {
        self.connected_peers
            .iter()
            .map(|(peer_id, info)| {
                (
                    *peer_id,
                    info.reputation,
                    info.grains_received,
                    info.grains_sent,
                )
            })
            .collect()
    }
}

// Helper for hex encoding
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
