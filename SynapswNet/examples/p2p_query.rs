// P2P Distributed Query Demo
//
// This example demonstrates:
// - Broadcasting KNN queries to peers
// - Receiving and responding to queries
// - Merging results from multiple peers
// - Timeout handling
//
// Run multiple instances to see distributed search:
// ```
// cargo run --example p2p_query
// cargo run --example p2p_query -- --port 9001
// ```

use anyhow::Result;
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

    info!("Starting P2P query demo on port {}", port);

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

    // Wait for peers to connect
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    info!("Connected peers: {}", swarm.peer_count());

    if swarm.peer_count() == 0 {
        info!("No peers connected. Start another instance with:");
        info!("  cargo run --example p2p_query -- --port 9001");
    } else {
        // Perform distributed query
        info!("Performing distributed KNN query...");

        // Create query vector (384 dimensions for all-MiniLM-L6-v2)
        let query_vector = vec![0.5; 384];

        // Query with k=10, timeout=2 seconds
        match swarm.query_peers(query_vector, 10, 2).await {
            Ok(results) => {
                info!("Query complete! Received {} results:", results.len());
                for (i, result) in results.iter().enumerate() {
                    info!(
                        "  {}. Grain {:?} - similarity: {:.3}",
                        i + 1,
                        hex_encode(&result.grain_id[..8]),
                        result.similarity
                    );
                    if let Some(summary) = &result.summary {
                        info!("     Summary: {}", summary);
                    }
                }
            }
            Err(e) => {
                eprintln!("Query failed: {}", e);
            }
        }
    }

    info!("Demo complete.");
    info!("In a real application, queries would be integrated with local HNSW index.");

    Ok(())
}

// Helper for hex encoding
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
