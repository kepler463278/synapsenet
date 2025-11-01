use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use synapsenet_ai::EmbeddingModel;
use synapsenet_core::{CryptoBackend, Grain, GrainMeta, SigningKeyTrait};
use tauri::State;

/// Error type for Tauri commands
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Embedding error: {0}")]
    Embedding(String),

    #[error("Index error: {0}")]
    Index(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<anyhow::Error> for CommandError {
    fn from(err: anyhow::Error) -> Self {
        CommandError::Internal(err.to_string())
    }
}

// Implement Serialize for CommandError so it can be sent to frontend
impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Response for add_grain command
#[derive(Debug, Serialize, Deserialize)]
pub struct AddGrainResponse {
    pub grain_id: String,
    pub embedding_time_ms: u64,
    pub storage_time_ms: u64,
}

/// Request for add_grain command
#[derive(Debug, Serialize, Deserialize)]
pub struct AddGrainRequest {
    pub text: String,
    pub tags: Vec<String>,
    pub title: Option<String>,
}

/// Search result
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub grain_id: String,
    pub similarity: f32,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub tags: Vec<String>,
    pub timestamp: i64,
}

/// Grain details
#[derive(Debug, Serialize, Deserialize)]
pub struct GrainDetails {
    pub grain_id: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub tags: Vec<String>,
    pub timestamp: i64,
    pub author_pk: String,
    pub crypto_backend: String,
    pub mime: String,
    pub lang: String,
}

/// Node statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStats {
    pub total_grains: usize,
    pub network_peers: usize,
    pub storage_path: String,
    pub embedding_model: String,
}

/// Peer information
#[derive(Debug, Serialize, Deserialize)]
pub struct PeerInfo {
    pub peer_id: String,
    pub addresses: Vec<String>,
    pub connected: bool,
}

/// Add a new grain to the network
#[tauri::command]
pub async fn add_grain(
    request: AddGrainRequest,
    state: State<'_, Arc<AppState>>,
) -> Result<AddGrainResponse, CommandError> {
    tracing::info!("Adding grain with {} tags", request.tags.len());

    // Validate input
    if request.text.trim().is_empty() {
        return Err(CommandError::InvalidInput("Text cannot be empty".to_string()));
    }

    let start = std::time::Instant::now();

    // Generate embedding
    let embedding = state
        .embedding
        .read()
        .await
        .embed(&request.text)
        .map_err(|e| CommandError::Embedding(e.to_string()))?;

    let embedding_time_ms = start.elapsed().as_millis() as u64;

    // Create grain metadata
    let config = state.config.read().await;
    let meta = GrainMeta {
        author_pk: state.signing_key.public_key(),
        crypto_backend: state.signing_key.backend(),
        ts_unix_ms: chrono::Utc::now().timestamp_millis(),
        tags: request.tags,
        mime: "text/plain".to_string(),
        lang: "en".to_string(),
        title: request.title,
        summary: Some(
            request
                .text
                .chars()
                .take(200)
                .collect::<String>()
                .trim()
                .to_string(),
        ),
        embedding_model: Some(config.ai.model_name.clone()),
        embedding_dimensions: Some(config.ai.embedding_dim),
    };

    // Create and sign grain
    let grain = Grain::new_with_unified_key(embedding, meta, &state.signing_key)
        .map_err(|e| CommandError::Internal(e.to_string()))?;

    let grain_id = hex::encode(grain.id);

    let storage_start = std::time::Instant::now();

    // Store grain
    state
        .store
        .lock()
        .await
        .insert_grain(&grain)
        .map_err(|e| CommandError::Storage(e.to_string()))?;

    // Add to index
    state
        .index
        .write()
        .await
        .add(&grain)
        .map_err(|e| CommandError::Index(e.to_string()))?;

    let storage_time_ms = storage_start.elapsed().as_millis() as u64;

    tracing::info!(
        "Grain {} added successfully (embed: {}ms, storage: {}ms)",
        grain_id,
        embedding_time_ms,
        storage_time_ms
    );

    Ok(AddGrainResponse {
        grain_id,
        embedding_time_ms,
        storage_time_ms,
    })
}

/// Search for grains using semantic search
#[tauri::command]
pub async fn search_grains(
    query: String,
    k: Option<usize>,
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<SearchResult>, CommandError> {
    tracing::info!("Searching for: {}", query);

    if query.trim().is_empty() {
        return Err(CommandError::InvalidInput("Query cannot be empty".to_string()));
    }

    let k = k.unwrap_or(10).min(100); // Default to 10, max 100

    // Generate query embedding
    let query_vec = state
        .embedding
        .read()
        .await
        .embed(&query)
        .map_err(|e| CommandError::Embedding(e.to_string()))?;

    // Search index
    let results = state
        .index
        .read()
        .await
        .search(&query_vec, k)
        .map_err(|e| CommandError::Index(e.to_string()))?;

    // Fetch grain details from storage
    let store = state.store.lock().await;
    let mut search_results = Vec::new();

    for result in results {
        if let Ok(Some(grain)) = store.get_grain(&result.grain_id) {
            search_results.push(SearchResult {
                grain_id: hex::encode(grain.id),
                similarity: result.similarity,
                title: grain.meta.title,
                summary: grain.meta.summary,
                tags: grain.meta.tags,
                timestamp: grain.meta.ts_unix_ms,
            });
        }
    }

    tracing::info!("Found {} results", search_results.len());

    Ok(search_results)
}

/// Get details for a specific grain
#[tauri::command]
pub async fn get_grain_details(
    grain_id: String,
    state: State<'_, Arc<AppState>>,
) -> Result<GrainDetails, CommandError> {
    tracing::info!("Getting details for grain: {}", grain_id);

    // Decode grain ID
    let id_bytes = hex::decode(&grain_id)
        .map_err(|_| CommandError::InvalidInput("Invalid grain ID format".to_string()))?;

    if id_bytes.len() != 32 {
        return Err(CommandError::InvalidInput(
            "Grain ID must be 32 bytes".to_string(),
        ));
    }

    let mut id = [0u8; 32];
    id.copy_from_slice(&id_bytes);

    // Fetch grain from storage
    let grain = state
        .store
        .lock()
        .await
        .get_grain(&id)
        .map_err(|e| CommandError::Storage(e.to_string()))?
        .ok_or_else(|| CommandError::InvalidInput("Grain not found".to_string()))?;

    Ok(GrainDetails {
        grain_id,
        title: grain.meta.title,
        summary: grain.meta.summary,
        tags: grain.meta.tags,
        timestamp: grain.meta.ts_unix_ms,
        author_pk: hex::encode(&grain.meta.author_pk),
        crypto_backend: format!("{:?}", grain.meta.crypto_backend),
        mime: grain.meta.mime,
        lang: grain.meta.lang,
    })
}

/// Get node statistics
#[tauri::command]
pub async fn get_stats(state: State<'_, Arc<AppState>>) -> Result<NodeStats, CommandError> {
    tracing::debug!("Getting node statistics");

    let total_grains = state
        .store
        .lock()
        .await
        .count_grains()
        .map_err(|e| CommandError::Storage(e.to_string()))?;

    let config = state.config.read().await;

    Ok(NodeStats {
        total_grains,
        network_peers: 0, // TODO: Implement P2P peer counting in future tasks
        storage_path: state.data_dir.display().to_string(),
        embedding_model: config.ai.model_name.clone(),
    })
}

/// Get network peers (placeholder for future P2P implementation)
#[tauri::command]
pub async fn get_network_peers(
    _state: State<'_, Arc<AppState>>,
) -> Result<Vec<PeerInfo>, CommandError> {
    tracing::debug!("Getting network peers");

    // TODO: Implement P2P peer listing in future tasks
    Ok(Vec::new())
}

/// Health check command
#[tauri::command]
pub async fn health_check() -> Result<String, CommandError> {
    Ok("OK".to_string())
}

// ===== Configuration Commands =====

/// Get current configuration
#[tauri::command]
pub async fn get_config(state: State<'_, Arc<AppState>>) -> Result<synapsenet_core::Config, CommandError> {
    tracing::debug!("Getting configuration");
    let config = state.config.read().await;
    Ok(config.clone())
}

/// Update configuration
#[tauri::command]
pub async fn update_config(
    new_config: synapsenet_core::Config,
    state: State<'_, Arc<AppState>>,
) -> Result<(), CommandError> {
    tracing::info!("Updating configuration");

    // Validate new configuration
    new_config.validate()
        .map_err(|e| CommandError::InvalidInput(format!("Invalid configuration: {}", e)))?;

    // Update in-memory config
    let mut config = state.config.write().await;
    *config = new_config.clone();

    // Save to file
    let config_path = state.data_dir.join("config.toml");
    new_config.save(&config_path)
        .map_err(|e| CommandError::Internal(format!("Failed to save config: {}", e)))?;

    tracing::info!("Configuration updated successfully");
    Ok(())
}

/// Reset configuration to defaults
#[tauri::command]
pub async fn reset_config(state: State<'_, Arc<AppState>>) -> Result<synapsenet_core::Config, CommandError> {
    tracing::info!("Resetting configuration to defaults");

    let default_config = synapsenet_core::Config::default();

    // Update in-memory config
    let mut config = state.config.write().await;
    *config = default_config.clone();

    // Save to file
    let config_path = state.data_dir.join("config.toml");
    default_config.save(&config_path)
        .map_err(|e| CommandError::Internal(format!("Failed to save config: {}", e)))?;

    tracing::info!("Configuration reset successfully");
    Ok(default_config)
}

/// Validate configuration without saving
#[tauri::command]
pub async fn validate_config(config: synapsenet_core::Config) -> Result<String, CommandError> {
    match config.validate() {
        Ok(_) => Ok("Configuration is valid".to_string()),
        Err(e) => Err(CommandError::InvalidInput(e.to_string())),
    }
}

/// Check for application updates
#[tauri::command]
pub async fn check_for_updates(app: tauri::AppHandle) -> Result<UpdateInfo, CommandError> {
    #[cfg(not(debug_assertions))]
    {
        match app.updater().check().await {
            Ok(update) => {
                if update.is_update_available() {
                    Ok(UpdateInfo {
                        available: true,
                        version: update.latest_version().to_string(),
                        current_version: app.package_info().version.to_string(),
                        download_url: update.download_url().map(|u| u.to_string()),
                        release_notes: update.body().map(|b| b.to_string()),
                    })
                } else {
                    Ok(UpdateInfo {
                        available: false,
                        version: app.package_info().version.to_string(),
                        current_version: app.package_info().version.to_string(),
                        download_url: None,
                        release_notes: None,
                    })
                }
            }
            Err(e) => Err(CommandError::Internal(format!("Update check failed: {}", e))),
        }
    }
    
    #[cfg(debug_assertions)]
    {
        // In development, return mock data
        Ok(UpdateInfo {
            available: false,
            version: app.package_info().version.to_string(),
            current_version: app.package_info().version.to_string(),
            download_url: None,
            release_notes: Some("Development mode - updates disabled".to_string()),
        })
    }
}

/// Install available update
#[tauri::command]
pub async fn install_update(app: tauri::AppHandle) -> Result<(), CommandError> {
    #[cfg(not(debug_assertions))]
    {
        match app.updater().check().await {
            Ok(update) => {
                if update.is_update_available() {
                    update
                        .download_and_install()
                        .await
                        .map_err(|e| CommandError::Internal(format!("Update installation failed: {}", e)))?;
                    Ok(())
                } else {
                    Err(CommandError::InvalidInput("No update available".to_string()))
                }
            }
            Err(e) => Err(CommandError::Internal(format!("Update check failed: {}", e))),
        }
    }
    
    #[cfg(debug_assertions)]
    {
        Err(CommandError::InvalidInput("Updates disabled in development mode".to_string()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub available: bool,
    pub version: String,
    pub current_version: String,
    pub download_url: Option<String>,
    pub release_notes: Option<String>,
}
