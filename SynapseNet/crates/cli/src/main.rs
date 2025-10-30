use anyhow::Result;
use clap::{Parser, Subcommand};
use ed25519_dalek::SigningKey;
use rand::{rngs::OsRng, RngCore};
use std::path::PathBuf;
use synapsenet_ai::{EmbeddingModel, OnnxEmbedding};
use synapsenet_core::{Grain, GrainMeta};
use synapsenet_storage::{HnswIndex, Store};
use tracing::{info, Level};

#[derive(Parser)]
#[command(name = "syn")]
#[command(about = "SynapseNet CLI - Decentralized semantic memory", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Data directory
    #[arg(short, long, default_value = ".synapsenet")]
    data_dir: PathBuf,

    /// Configuration file
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize local node (keys, DB, index)
    Init,

    /// Add text or file as grain
    Add {
        /// Text content or file path
        input: String,
    },

    /// Query semantic memory
    Query {
        /// Query text
        question: String,

        /// Number of results
        #[arg(short, long, default_value = "5")]
        k: usize,
    },

    /// Show peers and P2P status
    Peers,

    /// Export grains to Parquet
    Export {
        /// Output directory
        #[arg(short, long, default_value = "export")]
        output: PathBuf,
    },

    /// Import grains from Parquet
    Import {
        /// Input directory
        #[arg(short, long, default_value = "export")]
        input: PathBuf,
    },

    /// Generate default configuration file
    Config {
        /// Output path for config file
        #[arg(short, long, default_value = "config.toml")]
        output: PathBuf,
    },
    
    /// Show node statistics and metrics
    Stats,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_node(&cli.data_dir).await,
        Commands::Add { input } => add_grain(&cli.data_dir, &input).await,
        Commands::Query { question, k } => query_grains(&cli.data_dir, &question, k).await,
        Commands::Peers => show_peers(&cli.data_dir).await,
        Commands::Export { output } => export_grains(&cli.data_dir, &output).await,
        Commands::Import { input } => import_grains(&cli.data_dir, &input).await,
        Commands::Config { output } => generate_config(&output).await,
        Commands::Stats => show_stats(&cli.data_dir).await,
    }
}

async fn init_node(data_dir: &PathBuf) -> Result<()> {
    info!("Initializing SynapseNet node at {:?}", data_dir);

    // Create data directory
    std::fs::create_dir_all(data_dir)?;

    // Generate keypair
    let mut secret_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut secret_bytes);
    let signing_key = SigningKey::from_bytes(&secret_bytes);
    let public_key = signing_key.verifying_key();

    // Save keys
    let key_path = data_dir.join("node.key");
    std::fs::write(&key_path, signing_key.to_bytes())?;

    let pub_path = data_dir.join("node.pub");
    std::fs::write(&pub_path, public_key.to_bytes())?;

    // Initialize database
    let db_path = data_dir.join("synapsenet.db");
    let _store = Store::new(db_path.to_str().unwrap())?;

    info!("✓ Node initialized");
    info!("  Public key: {}", hex::encode(public_key.to_bytes()));
    info!("  Data dir: {:?}", data_dir);

    Ok(())
}

async fn add_grain(data_dir: &PathBuf, input: &str) -> Result<()> {
    info!("Adding grain: {}", input);

    // Load signing key
    let key_path = data_dir.join("node.key");
    let key_bytes = std::fs::read(&key_path)?;
    let signing_key = SigningKey::from_bytes(&key_bytes.try_into().unwrap());
    let author_pk = signing_key.verifying_key().to_bytes();

    // Read input (file or text)
    let content = if std::path::Path::new(input).exists() {
        std::fs::read_to_string(input)?
    } else {
        input.to_string()
    };

    // Create metadata
    let meta = GrainMeta {
        author_pk,
        ts_unix_ms: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as i64,
        tags: vec![],
        mime: "text/plain".to_string(),
        lang: "en".to_string(),
        title: Some(content.chars().take(50).collect()),
        summary: None,
    };

    // Generate embedding using ONNX model
    let embedding = OnnxEmbedding::new(data_dir.clone()).await?;
    let vec = embedding.embed(&content)?;

    // Create grain
    let grain = Grain::new(vec, meta, &signing_key)?;

    // Store grain
    let db_path = data_dir.join("synapsenet.db");
    let store = Store::new(db_path.to_str().unwrap())?;
    store.insert_grain(&grain)?;

    info!("✓ Grain added: {}", hex::encode(grain.id));

    Ok(())
}

async fn query_grains(data_dir: &PathBuf, question: &str, k: usize) -> Result<()> {
    info!("Querying: {}", question);

    // Load grains from DB
    let db_path = data_dir.join("synapsenet.db");
    let store = Store::new(db_path.to_str().unwrap())?;
    let grains = store.get_all_grains()?;

    if grains.is_empty() {
        info!("No grains in local memory. Use 'syn add' first.");
        return Ok(());
    }

    // Build index
    let dim = grains[0].vec.len();
    let mut index = HnswIndex::new(grains.len(), dim);

    for grain in &grains {
        index.add(grain)?;
    }

    // Generate query embedding using ONNX model
    let embedding = OnnxEmbedding::new(data_dir.clone()).await?;
    let query_vec = embedding.embed(question)?;

    // Search
    let results = index.search(&query_vec, k)?;

    info!("Found {} results:", results.len());
    for (i, result) in results.iter().enumerate() {
        let grain = store.get_grain(&result.grain_id)?.unwrap();
        println!("\n{}. Similarity: {:.3}", i + 1, result.similarity);
        println!("   ID: {}", hex::encode(result.grain_id));
        if let Some(title) = &grain.meta.title {
            println!("   Title: {}", title);
        }
    }

    Ok(())
}

async fn show_peers(_data_dir: &PathBuf) -> Result<()> {
    info!("P2P peer information");
    
    println!("\n🌐 P2P Network Status");
    println!("====================\n");
    println!("Status:       Local mode (P2P disabled)");
    println!("Peers:        0 connected");
    println!("\nTo enable P2P:");
    println!("  1. Generate config: syn config");
    println!("  2. Edit config.toml: set p2p.enabled = true");
    println!("  3. Restart node");
    println!("\nSee examples/p2p_*.rs for P2P demos");
    
    Ok(())
}

async fn export_grains(data_dir: &PathBuf, output: &PathBuf) -> Result<()> {
    use indicatif::{ProgressBar, ProgressStyle};
    use synapsenet_storage::ParquetExporter;

    info!("Exporting grains to Parquet: {:?}", output);

    // Open store
    let store = Store::new(&data_dir.join("synapsenet.db").to_string_lossy())?;

    // Get all grains
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Loading grains from database...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let grains = store.get_all_grains()?;
    pb.finish_with_message(format!("✓ Loaded {} grains", grains.len()));

    if grains.is_empty() {
        println!("No grains to export");
        return Ok(());
    }

    // Export to Parquet
    let exporter = ParquetExporter::new(output.to_string_lossy().to_string());

    let pb = ProgressBar::new(grains.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    let stats = exporter.export(grains)?;
    pb.finish_with_message("✓ Export complete");

    // Display statistics
    println!("\n📊 Export Statistics:");
    println!("  Total grains:   {}", stats.total_grains);
    println!("  Files created:  {}", stats.files_created);
    println!(
        "  Bytes written:  {} ({:.2} MB)",
        stats.bytes_written,
        stats.bytes_written as f64 / 1_048_576.0
    );
    println!("  Output dir:     {:?}", output);

    Ok(())
}

async fn import_grains(data_dir: &PathBuf, input: &PathBuf) -> Result<()> {
    use indicatif::{ProgressBar, ProgressStyle};
    use synapsenet_storage::ParquetImporter;

    info!("Importing grains from Parquet: {:?}", input);

    // Import from Parquet
    let importer = ParquetImporter::new(input.to_string_lossy().to_string());

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Importing grains...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let stats = importer.import()?;
    pb.finish_with_message("✓ Import complete");

    // Display statistics
    println!("\n📊 Import Statistics:");
    println!("  Total grains:   {}", stats.total_grains);
    println!("  Imported:       {}", stats.imported);
    println!("  Skipped:        {}", stats.skipped);
    println!("  Invalid:        {}", stats.invalid);

    if stats.imported > 0 {
        // Note about storage integration
        println!("\n📝 Note: Grains verified successfully.");
        println!("   Storage integration: Import validates grains.");
        println!("   To store grains, use 'syn add' command.");
        
        // Rebuild HNSW index from existing grains
        println!("\n🔄 Rebuilding HNSW index from database...");

        let db_path = data_dir.join("synapsenet.db");
        if db_path.exists() {
            let store = Store::new(&db_path.to_string_lossy())?;
            let grains = store.get_all_grains()?;

            if !grains.is_empty() {
                use synapsenet_storage::HnswIndex;
                let mut index = HnswIndex::new(grains.len(), 384);
                index.rebuild(&grains)?;
                println!("✓ Index rebuilt with {} grains", index.len());
            } else {
                println!("  No grains in database yet");
            }
        }

        println!("\n✅ Import validation complete!");
    }

    Ok(())
}

/// Dummy embedding function (fallback, deprecated)
#[allow(dead_code)]
fn dummy_embedding(text: &str) -> Vec<f32> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    let hash = hasher.finish();

    // Generate 384-dim vector from hash
    let mut vec = Vec::with_capacity(384);
    for i in 0..384 {
        let val = ((hash.wrapping_mul(i as u64 + 1)) % 1000) as f32 / 1000.0;
        vec.push(val);
    }

    // Normalize
    let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
    vec.iter_mut().for_each(|x| *x /= norm);

    vec
}

// Add hex crate for encoding
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes
            .as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}

async fn generate_config(output: &PathBuf) -> Result<()> {
    use synapsenet_core::Config;

    info!("Generating default configuration file: {:?}", output);

    // Check if file already exists
    if output.exists() {
        println!("⚠️  Configuration file already exists: {:?}", output);
        println!("   Delete it first or use a different path.");
        return Ok(());
    }

    // Create default config
    let config = Config::default();

    // Save to file
    config.save(output)?;

    println!("✓ Configuration file created: {:?}", output);
    println!("\nDefault configuration:");
    println!("  Node name:        {}", config.node.name);
    println!("  Data directory:   {}", config.node.data_dir);
    println!("  P2P enabled:      {}", config.p2p.enabled);
    println!("  P2P port:         {}", config.p2p.port);
    println!("  ONNX model:       {}", config.ai.model_name);
    println!("  Embedding dim:    {}", config.ai.embedding_dim);
    println!("\nEdit the file to customize settings.");

    Ok(())
}

async fn show_stats(data_dir: &PathBuf) -> Result<()> {
    use synapsenet_core::NodeMetrics;
    
    info!("Collecting node statistics...");
    
    // Open store
    let db_path = data_dir.join("synapsenet.db");
    let store = Store::new(&db_path.to_string_lossy())?;
    
    // Collect metrics
    let mut metrics = NodeMetrics::new();
    
    // Get grain count
    let grains = store.get_all_grains()?;
    metrics.grains_total = grains.len();
    metrics.grains_local = grains.len(); // For now, all are local
    metrics.grains_remote = 0;
    
    // Get database size
    if db_path.exists() {
        metrics.db_size_bytes = std::fs::metadata(&db_path)?.len();
    }
    
    // P2P stats (placeholder)
    metrics.peers_connected = 0;
    
    // Performance stats (placeholder - would need to track these)
    metrics.avg_embedding_time_ms = 0.0;
    metrics.avg_query_time_ms = 0.0;
    metrics.queries_total = 0;
    
    // Uptime (placeholder)
    metrics.uptime_seconds = 0;
    
    // Display metrics
    println!("{}", metrics.format());
    
    Ok(())
}
