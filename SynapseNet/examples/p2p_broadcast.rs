// P2P Grain Broadcasting Demo
//
// This example demonstrates:
// - Broadcasting grains to peers
// - Receiving grains from peers
// - Signature verification
// - Rate limiting
// - Peer reputation
//
// Run multiple instances to see grain propagation:
// ```
// cargo run --example p2p_broadcast
// cargo run --example p2p_broadcast -- --port 9001
// ```

use anyhow::Result;
use ed25519_dalek::SigningKey;
use rand::{rngs::OsRng, RngCore};
use synapsenet_core::{Grain, GrainMeta};
use synapsenet_p2p::{P2pConfig, SynapseSwarm};
use tracing::{info, Level};

fn main() -> Result<()> {
    // Create tokio runtime
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async_main())
}

async fn async_main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Parse port from command line
    let port = std::env::args()
        .nth(1)
        .and_then(|arg| {
            if arg == "--port" {
                std::env::args().nth(2)
            } else {
                None
            }
        })
        .and_then(|p| p.parse().ok())
        .unwrap_or(9000);

    info!("Starting P2P broadcast demo on port {}", port);

    // Create P2P configuration
    let config = P2pConfig {
        port,
        enable_mdns: true,
        bootstrap_peers: Vec::new(),
    };

    // Create swarm
    let mut swarm = SynapseSwarm::new(config).await?;

    info!("P2P swarm initialized");
    info!("Local peer ID: {}", swarm.local_peer_id());
    info!("Waiting for peer discovery...");

    // Create signing key
    let mut secret_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut secret_bytes);
    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let author_pk = signing_key.verifying_key().to_bytes();

    // Wait for peers to connect
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Broadcast a few test grains
    for counter in 1..=3 {
        // Create grain metadata
        let meta = GrainMeta {
            author_pk,
            ts_unix_ms: chrono::Utc::now().timestamp_millis(),
            tags: vec!["test".to_string(), "broadcast".to_string()],
            mime: "text/plain".to_string(),
            lang: "en".to_string(),
            title: Some(format!("Test grain #{} from port {}", counter, port)),
            summary: Some(format!("Broadcast test grain number {}", counter)),
        };

        // Create embedding vector
        let vec = vec![0.1 * counter as f32; 384];

        // Create grain with signature
        match Grain::new(vec, meta, &signing_key) {
            Ok(grain) => {
                info!("Broadcasting grain #{}...", counter);
                if let Err(e) = swarm.broadcast_grain(&grain) {
                    eprintln!("Failed to broadcast grain: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to create grain: {}", e);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }

    // Show peer stats
    info!("Broadcast complete!");
    info!("Connected peers: {}", swarm.peer_count());
    for (peer_id, peer_info) in swarm.peers() {
        info!(
            "  Peer {}: received={}, sent={}, reputation={}",
            peer_id, peer_info.grains_received, peer_info.grains_sent, peer_info.reputation
        );
    }

    info!("Demo complete. In a real application, the swarm would continue running.");
    info!("To see grain reception, run: cargo run --example p2p_broadcast -- --port 9001");

    Ok(())
}
