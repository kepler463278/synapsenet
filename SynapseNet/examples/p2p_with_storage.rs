// P2P with Storage Integration Demo
//
// This example demonstrates:
// - P2P swarm with grain storage
// - Automatic storage of received grains
// - Database and index integration
//
// Run multiple instances to see grain propagation and storage:
// ```
// cargo run --example p2p_with_storage
// cargo run --example p2p_with_storage -- --port 9001 --data-dir .synapsenet2
// ```

use anyhow::Result;
use ed25519_dalek::SigningKey;
use rand::{RngCore, rngs::OsRng};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use synapsenet_core::{Grain, GrainMeta};
use synapsenet_p2p::{P2pConfig, SynapseSwarm};
use synapsenet_storage::Store;
use tracing::{info, Level};

fn main() -> Result<()> {
    // Create tokio runtime
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async_main())
}

async fn async_main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Parse command line arguments
    let mut port = 9000;
    let mut data_dir = PathBuf::from(".synapsenet");
    
    let args: Vec<String> = std::env::args().collect();
    for i in 0..args.len() {
        if args[i] == "--port" && i + 1 < args.len() {
            port = args[i + 1].parse().unwrap_or(9000);
        }
        if args[i] == "--data-dir" && i + 1 < args.len() {
            data_dir = PathBuf::from(&args[i + 1]);
        }
    }

    info!("Starting P2P with storage demo");
    info!("Port: {}", port);
    info!("Data dir: {:?}", data_dir);

    // Create data directory
    std::fs::create_dir_all(&data_dir)?;

    // Open database
    let db_path = data_dir.join("synapsenet.db");
    let store = Arc::new(Mutex::new(Store::new(&db_path.to_string_lossy())?));
    
    info!("Database opened: {:?}", db_path);

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

    // Set up grain storage callback
    let store_clone = Arc::clone(&store);
    swarm.set_grain_callback(move |grain| {
        let store = store_clone.lock().unwrap();
        store.insert_grain(&grain)?;
        Ok(())
    });
    
    info!("✓ Grain storage callback configured");

    // Wait for peers
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // Create and broadcast a test grain
    let mut secret_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut secret_bytes);
    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let author_pk = signing_key.verifying_key().to_bytes();

    let meta = GrainMeta {
        author_pk,
        ts_unix_ms: chrono::Utc::now().timestamp_millis(),
        tags: vec!["p2p".to_string(), "storage".to_string()],
        mime: "text/plain".to_string(),
        lang: "en".to_string(),
        title: Some(format!("Test grain from port {}", port)),
        summary: Some("P2P storage integration test".to_string()),
    };

    let vec = vec![0.1 * port as f32; 384];
    let grain = Grain::new(vec, meta, &signing_key)?;

    info!("Broadcasting test grain...");
    match swarm.broadcast_grain(&grain) {
        Ok(()) => {
            info!("✓ Grain broadcasted: {:?}", hex_encode(&grain.id[..8]));
            
            // Also store locally
            let store = store.lock().unwrap();
            store.insert_grain(&grain)?;
            info!("✓ Grain stored locally");
        }
        Err(e) => {
            info!("Broadcast failed (no peers?): {}", e);
        }
    }

    // Show statistics
    let store = store.lock().unwrap();
    let grains = store.get_all_grains()?;
    info!("\n📊 Storage Statistics:");
    info!("  Total grains: {}", grains.len());
    info!("  Peer count: {}", swarm.peer_count());

    info!("\n✅ Demo complete!");
    info!("Grains are stored in: {:?}", db_path);
    info!("Run another instance to see P2P grain propagation and storage.");

    Ok(())
}

// Helper for hex encoding
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
