use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use synapsenet_ai::OnnxEmbedding;
use synapsenet_core::{Config, UnifiedSigningKey};
use synapsenet_storage::{HnswIndex, Store};
use tokio::sync::{Mutex, RwLock};

/// Application state shared across all Tauri commands
pub struct AppState {
    /// Configuration
    pub config: Arc<RwLock<Config>>,

    /// Storage layer (SQLite + Parquet) - uses Mutex because rusqlite::Connection is !Send
    pub store: Arc<Mutex<Store>>,

    /// HNSW vector index
    pub index: Arc<RwLock<HnswIndex<'static>>>,

    /// Embedding model
    pub embedding: Arc<RwLock<OnnxEmbedding>>,

    /// Signing key for creating grains
    pub signing_key: Arc<UnifiedSigningKey>,

    /// Data directory
    pub data_dir: PathBuf,
}

impl AppState {
    /// Create new application state
    pub async fn new(config: Config) -> Result<Self> {
        let data_dir = PathBuf::from(&config.node.data_dir);

        // Create data directory if it doesn't exist
        std::fs::create_dir_all(&data_dir)?;

        tracing::info!("Initializing storage at {:?}", data_dir);

        // Initialize storage
        let db_path = data_dir.join(&config.storage.db_file);
        let db_path_str = db_path.to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid database path"))?;
        let store = Store::new(db_path_str)?;

        tracing::info!("Initializing HNSW index");

        // Initialize HNSW index
        let mut index = HnswIndex::new(
            config.storage.hnsw_max_elements,
            config.ai.embedding_dim,
        );

        // Load existing grains into index
        let grains = store.get_all_grains()?;
        tracing::info!("Loading {} grains into index", grains.len());
        if !grains.is_empty() {
            index.rebuild(&grains)?;
        }

        tracing::info!("Initializing embedding model: {}", config.ai.model_name);

        // Initialize embedding model with GPU provider
        let provider = config.ai.provider.parse()
            .unwrap_or(synapsenet_ai::GpuProvider::Cpu);
        
        let embedding = synapsenet_ai::OnnxEmbedding::new_with_provider(
            data_dir.clone(),
            provider
        ).await?;

        tracing::info!("Generating signing key");

        // Generate signing key (using default backend)
        let signing_key = UnifiedSigningKey::generate(UnifiedSigningKey::default_backend());

        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            store: Arc::new(Mutex::new(store)),
            index: Arc::new(RwLock::new(index)),
            embedding: Arc::new(RwLock::new(embedding)),
            signing_key: Arc::new(signing_key),
            data_dir,
        })
    }
}
