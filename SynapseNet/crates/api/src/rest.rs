// REST API Server for SynapseNet
// Provides HTTP endpoints for node operations

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

use synapsenet_ai::{EmbeddingModel, OnnxEmbedding};
use synapsenet_core::{Grain, GrainMeta, SigningKeyTrait, UnifiedSigningKey};
use synapsenet_storage::{HnswIndex, Store};

/// API Server state
pub struct ApiState {
    pub store: Arc<Mutex<Store>>,
    pub embedding: Arc<OnnxEmbedding>,
    pub signing_key: Arc<UnifiedSigningKey>,
    pub index: Arc<RwLock<HnswIndex<'static>>>,
}

/// API Error type
#[derive(Debug)]
pub struct ApiError(anyhow::Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        error!("API error: {}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": self.0.to_string()
            })),
        )
            .into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

/// Init request
#[derive(Debug, Deserialize)]
pub struct InitRequest {
    pub data_dir: Option<String>,
}

/// Init response
#[derive(Debug, Serialize)]
pub struct InitResponse {
    pub success: bool,
    pub public_key: String,
    pub data_dir: String,
}

/// Add grain request
#[derive(Debug, Deserialize)]
pub struct AddRequest {
    pub text: String,
    pub tags: Option<Vec<String>>,
}

/// Add grain response
#[derive(Debug, Serialize)]
pub struct AddResponse {
    pub grain_id: String,
    pub embedding_time_ms: u64,
}

/// Query request
#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    pub text: String,
    pub k: Option<usize>,
}

/// Query result
#[derive(Debug, Serialize)]
pub struct QueryResult {
    pub grain_id: String,
    pub similarity: f32,
    pub title: Option<String>,
}

/// Query response
#[derive(Debug, Serialize)]
pub struct QueryResponse {
    pub results: Vec<QueryResult>,
    pub query_time_ms: u64,
}

/// Stats response
#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub grains_total: usize,
    pub peers_connected: usize,
    pub uptime_seconds: u64,
}

/// Create REST API router with v1 and v2 endpoints
pub fn create_router(state: Arc<ApiState>) -> Router {
    // v2 API router
    let v2_router = crate::v2::create_v2_router();

    // v1 API router (legacy, with deprecation warnings)
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/init", post(init))
        .route("/add", post(add_grain))
        .route("/query", post(query))
        .route("/stats", get(stats))
        .route("/peers", get(peers))
        .nest("/v2", v2_router)
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Root endpoint
async fn root() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "SynapseNet API",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": [
            "GET  /",
            "GET  /health",
            "POST /init",
            "POST /add",
            "POST /query",
            "GET  /stats",
            "GET  /peers",
        ]
    }))
}

/// Health check endpoint
async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().timestamp()
    }))
}

/// Initialize node
async fn init(
    State(_state): State<Arc<ApiState>>,
    Json(req): Json<InitRequest>,
) -> Result<Json<InitResponse>, ApiError> {
    info!("POST /init: {:?}", req);

    // In a real implementation, this would initialize the node
    // For now, return mock response
    Ok(Json(InitResponse {
        success: true,
        public_key: "mock_public_key".to_string(),
        data_dir: req.data_dir.unwrap_or_else(|| ".synapsenet".to_string()),
    }))
}

/// Add grain (v1 - DEPRECATED, use POST /v2/batch/import instead)
async fn add_grain(
    State(state): State<Arc<ApiState>>,
    Json(req): Json<AddRequest>,
) -> Result<Json<AddResponse>, ApiError> {
    tracing::warn!("POST /add is deprecated, use POST /v2/batch/import instead");
    info!("POST /add: {} chars", req.text.len());

    let start = std::time::Instant::now();

    // Generate embedding
    let vec = state.embedding.embed(&req.text)?;

    // Create metadata
    let meta = GrainMeta {
        author_pk: state.signing_key.public_key(),
        crypto_backend: state.signing_key.backend(),
        ts_unix_ms: chrono::Utc::now().timestamp_millis(),
        tags: req.tags.unwrap_or_default(),
        mime: "text/plain".to_string(),
        lang: "en".to_string(),
        title: Some(req.text.chars().take(50).collect()),
        summary: None,
        embedding_model: Some("all-MiniLM-L6-v2".to_string()),
        embedding_dimensions: Some(vec.len()),
    };

    // Create grain
    let grain = Grain::new_with_unified_key(vec, meta, &state.signing_key)?;
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

    let embedding_time_ms = start.elapsed().as_millis() as u64;

    info!("✓ Grain added: {} ({}ms)", &grain_id[..8], embedding_time_ms);

    Ok(Json(AddResponse {
        grain_id,
        embedding_time_ms,
    }))
}

/// Query grains
async fn query(
    State(state): State<Arc<ApiState>>,
    Json(req): Json<QueryRequest>,
) -> Result<Json<QueryResponse>, ApiError> {
    info!("POST /query: {}", req.text);

    let start = std::time::Instant::now();
    let k = req.k.unwrap_or(5);

    // Generate query embedding
    let query_vec = state.embedding.embed(&req.text)?;

    // Search index
    let results = {
        let index = state.index.read().await;
        index.search(&query_vec, k)?
    };

    // Get grain details
    let store = state.store.lock().unwrap();
    let mut query_results = Vec::new();

    for result in results {
        if let Some(grain) = store.get_grain(&result.grain_id)? {
            query_results.push(QueryResult {
                grain_id: hex::encode(&result.grain_id),
                similarity: result.similarity,
                title: grain.meta.title,
            });
        }
    }

    let query_time_ms = start.elapsed().as_millis() as u64;

    info!("✓ Query complete: {} results ({}ms)", query_results.len(), query_time_ms);

    Ok(Json(QueryResponse {
        results: query_results,
        query_time_ms,
    }))
}

/// Get stats
async fn stats(State(state): State<Arc<ApiState>>) -> Result<Json<StatsResponse>, ApiError> {
    info!("GET /stats");

    let store = state.store.lock().unwrap();
    let grains_total = store.count_grains()?;

    Ok(Json(StatsResponse {
        grains_total,
        peers_connected: 0, // TODO: Get from P2P
        uptime_seconds: 0,  // TODO: Track uptime
    }))
}

/// Get peers
async fn peers(State(_state): State<Arc<ApiState>>) -> Json<serde_json::Value> {
    info!("GET /peers");

    // TODO: Get from P2P swarm
    Json(serde_json::json!({
        "peers": [],
        "count": 0
    }))
}


