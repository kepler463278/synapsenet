// REST API v2 - New endpoints for v0.4 features

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, info};

use synapsenet_ai::EmbeddingModel;
use synapsenet_core::SigningKeyTrait;
use crate::rest::{ApiError, ApiState};

/// Create v2 API router
pub fn create_v2_router() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/models", get(list_models))
        .route("/models/:name", get(get_model_info))
        .route("/batch/import", post(batch_import))
        .route("/poe/scores", get(get_poe_scores))
        .route("/poe/scores/:grain_id", get(get_grain_poe_score))
        .route("/network/peers", get(get_network_peers))
        .route("/network/clusters", get(get_peer_clusters))
}

// ===== Models Endpoints =====

/// Model information
#[derive(Debug, Serialize)]
pub struct ModelInfo {
    pub name: String,
    pub dimensions: usize,
    pub file_size_mb: f64,
    pub loaded_at: i64,
    pub status: String,
}

/// List all available embedding models
async fn list_models(State(state): State<Arc<ApiState>>) -> Result<Json<Vec<ModelInfo>>, ApiError> {
    info!("GET /v2/models - Listing available models");

    let store = state.store.lock().unwrap();
    let models = store.get_all_embedding_models()?;

    let model_infos: Vec<ModelInfo> = models
        .into_iter()
        .map(|(name, dimensions, file_size_mb, loaded_at)| ModelInfo {
            name,
            dimensions,
            file_size_mb,
            loaded_at,
            status: "loaded".to_string(),
        })
        .collect();

    debug!("Found {} models", model_infos.len());
    Ok(Json(model_infos))
}

/// Get specific model information
async fn get_model_info(
    State(state): State<Arc<ApiState>>,
    Path(name): Path<String>,
) -> Result<Json<ModelInfo>, ApiError> {
    info!("GET /v2/models/{} - Getting model info", name);

    let store = state.store.lock().unwrap();
    let model = store
        .get_embedding_model(&name)?
        .ok_or_else(|| anyhow::anyhow!("Model not found: {}", name))?;

    Ok(Json(ModelInfo {
        name,
        dimensions: model.0,
        file_size_mb: model.1,
        loaded_at: model.2,
        status: "loaded".to_string(),
    }))
}

// ===== Batch Operations =====

/// Batch import request
#[derive(Debug, Deserialize)]
pub struct BatchImportRequest {
    pub items: Vec<BatchItem>,
    pub model: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BatchItem {
    pub text: String,
    pub tags: Option<Vec<String>>,
    pub title: Option<String>,
}

/// Batch import response
#[derive(Debug, Serialize)]
pub struct BatchImportResponse {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub grain_ids: Vec<String>,
    pub errors: Vec<String>,
    pub processing_time_ms: u64,
}

/// Import multiple grains in batch
async fn batch_import(
    State(state): State<Arc<ApiState>>,
    Json(request): Json<BatchImportRequest>,
) -> Result<Json<BatchImportResponse>, ApiError> {
    info!("POST /v2/batch/import - Importing {} items", request.items.len());

    let start = std::time::Instant::now();
    let mut grain_ids = Vec::new();
    let mut errors = Vec::new();
    let mut succeeded = 0;

    for (idx, item) in request.items.iter().enumerate() {
        match process_batch_item(item, &state).await {
            Ok(grain_id) => {
                grain_ids.push(grain_id);
                succeeded += 1;
            }
            Err(e) => {
                errors.push(format!("Item {}: {}", idx, e));
            }
        }
    }

    let processing_time_ms = start.elapsed().as_millis() as u64;
    let failed = request.items.len() - succeeded;

    info!(
        "Batch import complete: {}/{} succeeded in {}ms",
        succeeded,
        request.items.len(),
        processing_time_ms
    );

    Ok(Json(BatchImportResponse {
        total: request.items.len(),
        succeeded,
        failed,
        grain_ids,
        errors,
        processing_time_ms,
    }))
}

async fn process_batch_item(item: &BatchItem, state: &ApiState) -> anyhow::Result<String> {
    // Generate embedding
    let vec = state.embedding.embed(&item.text)?;

    // Create metadata
    let meta = synapsenet_core::GrainMeta {
        author_pk: state.signing_key.public_key(),
        crypto_backend: state.signing_key.backend(),
        ts_unix_ms: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as i64,
        tags: item.tags.clone().unwrap_or_default(),
        mime: "text/plain".to_string(),
        lang: "en".to_string(),
        title: item.title.clone(),
        summary: None,
        embedding_model: Some("all-MiniLM-L6-v2".to_string()),
        embedding_dimensions: Some(vec.len()),
    };

    // Create and sign grain
    let grain = synapsenet_core::Grain::new_with_unified_key(vec, meta, &state.signing_key)?;
    let grain_id = hex::encode(&grain.id);

    // Store grain
    {
        let mut store = state.store.lock().unwrap();
        store.insert_grain(&grain)?;
    }

    // Add to index
    {
        let mut index = state.index.write().await;
        index.add(&grain)?;
    }

    Ok(grain_id)
}

// ===== PoE Endpoints =====

/// PoE score information
#[derive(Debug, Serialize)]
pub struct PoEScoreInfo {
    pub grain_id: String,
    pub novelty: f32,
    pub coherence: f32,
    pub reuse: f32,
    pub total: f32,
    pub ngt_reward: f32,
    pub calculated_at: i64,
}

/// Query parameters for PoE scores
#[derive(Debug, Deserialize)]
pub struct PoEScoresQuery {
    pub limit: Option<usize>,
    pub min_score: Option<f32>,
}

/// Get PoE scores for all grains
async fn get_poe_scores(
    State(_state): State<Arc<ApiState>>,
    Query(query): Query<PoEScoresQuery>,
) -> Result<Json<Vec<PoEScoreInfo>>, ApiError> {
    info!("GET /v2/poe/scores - Getting PoE scores");

    // TODO: Implement actual PoE score retrieval from storage
    // For now, return empty list as placeholder
    let _limit = query.limit.unwrap_or(100);
    let _min_score = query.min_score.unwrap_or(0.0);

    debug!("PoE scores endpoint - implementation pending");
    Ok(Json(Vec::new()))
}

/// Get PoE score for specific grain
async fn get_grain_poe_score(
    State(_state): State<Arc<ApiState>>,
    Path(grain_id): Path<String>,
) -> Result<Json<PoEScoreInfo>, ApiError> {
    info!("GET /v2/poe/scores/{} - Getting grain PoE score", grain_id);

    // TODO: Implement actual PoE score retrieval
    Err(anyhow::anyhow!("PoE scores not yet implemented").into())
}

// ===== Network Endpoints =====

/// Peer information
#[derive(Debug, Serialize)]
pub struct PeerInfo {
    pub peer_id: String,
    pub topics: Vec<String>,
    pub similarity: f32,
    pub last_seen: i64,
    pub connection_status: String,
}

/// Cluster information
#[derive(Debug, Serialize)]
pub struct ClusterInfo {
    pub topic: String,
    pub peer_count: usize,
    pub avg_similarity: f32,
    pub peers: Vec<String>,
}

/// Get network peers with cluster information
async fn get_network_peers(
    State(state): State<Arc<ApiState>>,
) -> Result<Json<Vec<PeerInfo>>, ApiError> {
    info!("GET /v2/network/peers - Getting network peers");

    let store = state.store.lock().unwrap();

    // Get all unique peer IDs from clusters
    let stats = store.get_cluster_stats()?;
    debug!("Found {} topics, {} peers", stats.0, stats.1);

    // TODO: Implement full peer information retrieval
    // For now, return empty list as placeholder
    Ok(Json(Vec::new()))
}

/// Get peer clusters by topic
async fn get_peer_clusters(
    State(state): State<Arc<ApiState>>,
) -> Result<Json<Vec<ClusterInfo>>, ApiError> {
    info!("GET /v2/network/clusters - Getting peer clusters");

    let store = state.store.lock().unwrap();
    let stats = store.get_cluster_stats()?;

    debug!("Cluster stats: {} topics, {} peers", stats.0, stats.1);

    // TODO: Implement full cluster information retrieval
    // For now, return empty list as placeholder
    Ok(Json(Vec::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_import_request() {
        let json = r#"{
            "items": [
                {"text": "test1", "tags": ["tag1"]},
                {"text": "test2", "title": "Test 2"}
            ]
        }"#;

        let req: BatchImportRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.items.len(), 2);
        assert_eq!(req.items[0].text, "test1");
    }

    #[test]
    fn test_poe_scores_query() {
        let query = PoEScoresQuery {
            limit: Some(50),
            min_score: Some(0.5),
        };

        assert_eq!(query.limit.unwrap(), 50);
        assert_eq!(query.min_score.unwrap(), 0.5);
    }
}
