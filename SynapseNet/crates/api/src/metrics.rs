// Prometheus Metrics for SynapseNet
// Exports metrics on /metrics endpoint

use axum::{response::IntoResponse, routing::get, Router};
use prometheus::{
    register_counter, register_gauge, register_histogram, Counter, Encoder, Gauge, Histogram,
    TextEncoder,
};
use std::sync::Arc;

lazy_static::lazy_static! {
    // Embedding metrics
    pub static ref EMBEDDING_DURATION: Histogram = register_histogram!(
        "syn_embedding_seconds",
        "Time spent generating embeddings"
    ).unwrap();

    pub static ref EMBEDDING_TOTAL: Counter = register_counter!(
        "syn_embedding_total",
        "Total number of embeddings generated"
    ).unwrap();

    // Query metrics
    pub static ref QUERY_DURATION: Histogram = register_histogram!(
        "syn_query_seconds",
        "Time spent processing queries"
    ).unwrap();

    pub static ref QUERY_TOTAL: Counter = register_counter!(
        "syn_query_total",
        "Total number of queries processed"
    ).unwrap();

    // Grain metrics
    pub static ref GRAINS_TOTAL: Gauge = register_gauge!(
        "syn_grains_total",
        "Total number of grains in storage"
    ).unwrap();

    pub static ref GRAINS_ADDED: Counter = register_counter!(
        "syn_grains_added_total",
        "Total number of grains added"
    ).unwrap();

    // P2P metrics
    pub static ref P2P_PEERS: Gauge = register_gauge!(
        "syn_p2p_peers",
        "Number of connected P2P peers"
    ).unwrap();

    pub static ref P2P_MESSAGES_SENT: Counter = register_counter!(
        "syn_p2p_messages_sent_total",
        "Total P2P messages sent"
    ).unwrap();

    pub static ref P2P_MESSAGES_RECEIVED: Counter = register_counter!(
        "syn_p2p_messages_received_total",
        "Total P2P messages received"
    ).unwrap();

    pub static ref P2P_DROPS: Counter = register_counter!(
        "syn_p2p_drops_total",
        "Total P2P message drops"
    ).unwrap();

    // PoE metrics
    pub static ref POE_REWARD_TOTAL: Counter = register_counter!(
        "syn_poe_reward_total",
        "Total PoE rewards distributed"
    ).unwrap();

    pub static ref POE_NOVELTY: Histogram = register_histogram!(
        "syn_poe_novelty",
        "PoE novelty scores"
    ).unwrap();

    pub static ref POE_COHERENCE: Histogram = register_histogram!(
        "syn_poe_coherence",
        "PoE coherence scores"
    ).unwrap();

    pub static ref POE_REUSE: Histogram = register_histogram!(
        "syn_poe_reuse",
        "PoE reuse scores"
    ).unwrap();

    pub static ref POE_CALCULATION_DURATION: Histogram = register_histogram!(
        "syn_poe_calculation_seconds",
        "Time spent calculating PoE scores"
    ).unwrap();

    // Batch processing metrics (NEW in v0.4)
    pub static ref BATCH_SIZE: Histogram = register_histogram!(
        "syn_batch_size",
        "Batch processing size"
    ).unwrap();

    pub static ref BATCH_DURATION: Histogram = register_histogram!(
        "syn_batch_seconds",
        "Time spent processing batches"
    ).unwrap();

    pub static ref BATCH_SUCCESS_RATE: Histogram = register_histogram!(
        "syn_batch_success_rate",
        "Batch processing success rate (0.0-1.0)"
    ).unwrap();

    pub static ref BATCH_TOTAL: Counter = register_counter!(
        "syn_batch_total",
        "Total number of batches processed"
    ).unwrap();

    // Multi-model metrics (NEW in v0.4)
    pub static ref MODELS_LOADED: Gauge = register_gauge!(
        "syn_models_loaded",
        "Number of embedding models loaded"
    ).unwrap();

    pub static ref MODEL_SWITCH_TOTAL: Counter = register_counter!(
        "syn_model_switch_total",
        "Total number of model switches"
    ).unwrap();

    // Network clustering metrics (NEW in v0.4)
    pub static ref CLUSTERS_TOTAL: Gauge = register_gauge!(
        "syn_clusters_total",
        "Number of peer clusters"
    ).unwrap();

    pub static ref CLUSTER_SIZE: Histogram = register_histogram!(
        "syn_cluster_size",
        "Size of peer clusters"
    ).unwrap();
}

/// Metrics endpoint handler
async fn metrics_handler() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];

    encoder.encode(&metric_families, &mut buffer).unwrap();

    (
        [("content-type", "text/plain; version=0.0.4")],
        buffer,
    )
}

/// Create metrics router
pub fn create_metrics_router() -> Router {
    Router::new().route("/metrics", get(metrics_handler))
}

/// Helper to record embedding time
pub fn record_embedding(duration_secs: f64) {
    EMBEDDING_DURATION.observe(duration_secs);
    EMBEDDING_TOTAL.inc();
}

/// Helper to record query time
pub fn record_query(duration_secs: f64) {
    QUERY_DURATION.observe(duration_secs);
    QUERY_TOTAL.inc();
}

/// Helper to record grain addition
pub fn record_grain_added() {
    GRAINS_ADDED.inc();
    GRAINS_TOTAL.inc();
}

/// Helper to update P2P peer count
pub fn update_peer_count(count: i64) {
    P2P_PEERS.set(count as f64);
}

/// Helper to record P2P message sent
pub fn record_p2p_sent() {
    P2P_MESSAGES_SENT.inc();
}

/// Helper to record P2P message received
pub fn record_p2p_received() {
    P2P_MESSAGES_RECEIVED.inc();
}

/// Helper to record P2P drop
pub fn record_p2p_drop() {
    P2P_DROPS.inc();
}

/// Helper to record PoE reward
pub fn record_poe_reward(amount: f64, novelty: f64, coherence: f64) {
    POE_REWARD_TOTAL.inc_by(amount);
    POE_NOVELTY.observe(novelty);
    POE_COHERENCE.observe(coherence);
}

/// Helper to record PoE v2 score (NEW in v0.4)
pub fn record_poe_v2_score(novelty: f64, coherence: f64, reuse: f64, duration_secs: f64) {
    POE_NOVELTY.observe(novelty);
    POE_COHERENCE.observe(coherence);
    POE_REUSE.observe(reuse);
    POE_CALCULATION_DURATION.observe(duration_secs);
}

/// Helper to record batch processing (NEW in v0.4)
pub fn record_batch(size: usize, duration_secs: f64, success_rate: f64) {
    BATCH_SIZE.observe(size as f64);
    BATCH_DURATION.observe(duration_secs);
    BATCH_SUCCESS_RATE.observe(success_rate);
    BATCH_TOTAL.inc();
}

/// Helper to update models loaded count (NEW in v0.4)
pub fn update_models_loaded(count: usize) {
    MODELS_LOADED.set(count as f64);
}

/// Helper to record model switch (NEW in v0.4)
pub fn record_model_switch() {
    MODEL_SWITCH_TOTAL.inc();
}

/// Helper to update cluster metrics (NEW in v0.4)
pub fn update_clusters(cluster_count: usize, avg_size: f64) {
    CLUSTERS_TOTAL.set(cluster_count as f64);
    CLUSTER_SIZE.observe(avg_size);
}
