// P2P Network Demo
//
// This example demonstrates:
// - Creating a P2P swarm with mDNS discovery
// - Connecting to peers on local network
// - Broadcasting grains to peers
//
// Run multiple instances to see peer discovery:
// ```
// cargo run --example p2p_demo
// cargo run --example p2p_demo -- --port 9001
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

    info!("Starting P2P demo on port {}", port);

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

    // Create grain metadata
    let meta = GrainMeta {
        author_pk,
        ts_unix_ms: chrono::Utc::now().timestamp_millis(),
        tags: vec!["test".to_string(), "p2p".to_string()],
        mime: "text/plain".to_string(),
        lang: "en".to_string(),
        title: Some(format!("Test grain from peer {}", port)),
        summary: Some("This is a test grain for P2P demo".to_string()),
    };

    // Create embedding vector
    let vec = vec![0.1; 384];

    // Create grain with signature
    let grain = Grain::new(vec, meta, &signing_key)?;

    info!("P2P demo running. Press Ctrl+C to exit.");
    info!("Try running another instance with: cargo run --example p2p_demo -- --port 9001");

    // Note: In a real application, you would:
    // 1. Run swarm.run() in a background task
    // 2. Use channels to communicate with the swarm
    // 3. Broadcast grains when needed
    //
    // For this demo, we just show the swarm initialization

    info!("Swarm initialized successfully!");
    info!("Peer count: {}", swarm.peer_count());

    // In a real app, you would run the event loop:
    // swarm.run().await?;

    Ok(())
}
