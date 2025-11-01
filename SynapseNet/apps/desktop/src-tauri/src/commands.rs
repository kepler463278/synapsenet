//! Tauri commands for SynapseNet GUI

use crate::node::SharedNodeManager;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatus {
    pub running: bool,
    pub peers: u32,
    pub grains: u32,
    pub uptime: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStats {
    pub total_peers: u32,
    pub total_grains: u64,
    pub network_health: f64,
    pub sync_progress: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Grain {
    pub id: String,
    pub content: String,
    pub author: String,
    pub timestamp: i64,
    pub confidence: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reward {
    pub id: String,
    pub amount: f64,
    pub reason: String,
    pub timestamp: i64,
}

// ============================================================================
// Node Control Commands
// ============================================================================

#[tauri::command]
pub async fn start_node(
    node_manager: State<'_, SharedNodeManager>,
    app_state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<NodeStatus, String> {
    let mut node = node_manager.lock().await;
    node.start().await?;

    let mut state = app_state.lock().await;
    state.node_running = true;
    state.start_time = Some(chrono::Utc::now().timestamp());

    Ok(NodeStatus {
        running: true,
        peers: state.peer_count,
        grains: state.grain_count,
        uptime: 0,
    })
}

#[tauri::command]
pub async fn stop_node(
    node_manager: State<'_, SharedNodeManager>,
    app_state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), String> {
    let mut node = node_manager.lock().await;
    node.stop().await?;

    let mut state = app_state.lock().await;
    state.node_running = false;
    state.start_time = None;

    Ok(())
}

#[tauri::command]
pub async fn get_node_status(
    app_state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<NodeStatus, String> {
    let state = app_state.lock().await;

    Ok(NodeStatus {
        running: state.node_running,
        peers: state.peer_count,
        grains: state.grain_count,
        uptime: state.uptime(),
    })
}

// ============================================================================
// Network Statistics Commands
// ============================================================================

#[tauri::command]
pub async fn get_network_stats(
    app_state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<NetworkStats, String> {
    let state = app_state.lock().await;

    // TODO: Get real network statistics
    Ok(NetworkStats {
        total_peers: state.peer_count,
        total_grains: state.grain_count as u64,
        network_health: if state.node_running { 0.95 } else { 0.0 },
        sync_progress: if state.node_running { 1.0 } else { 0.0 },
    })
}

// ============================================================================
// Knowledge Operations Commands
// ============================================================================

#[tauri::command]
pub async fn search_grains(
    query: String,
    limit: Option<usize>,
) -> Result<Vec<Grain>, String> {
    tracing::info!("Searching grains: {}", query);

    // TODO: Implement actual search using storage layer
    let results = vec![
        Grain {
            id: Uuid::new_v4().to_string(),
            content: format!("Result for '{}' - Decentralized intelligence emerges from collective wisdom...", query),
            author: "local".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            confidence: 0.87,
        },
        Grain {
            id: Uuid::new_v4().to_string(),
            content: format!("Another insight about '{}' - Knowledge flows freely in peer-to-peer networks...", query),
            author: "peer_abc123".to_string(),
            timestamp: chrono::Utc::now().timestamp() - 3600,
            confidence: 0.92,
        },
    ];

    let limit = limit.unwrap_or(10);
    Ok(results.into_iter().take(limit).collect())
}

#[tauri::command]
pub async fn add_grain(
    content: String,
    app_state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<String, String> {
    if content.trim().is_empty() {
        return Err("Content cannot be empty".to_string());
    }

    let grain_id = Uuid::new_v4().to_string();

    tracing::info!("Adding grain: {} ({} bytes)", grain_id, content.len());

    // TODO: Add to actual storage
    // TODO: Broadcast to network
    // TODO: Calculate novelty score

    // Simulate reward for novel grain
    let mut state = app_state.lock().await;
    let novelty_score = 0.7; // TODO: Calculate from embeddings
    let reward_amount = 0.1 * novelty_score * 10.0; // base * novelty * multiplier
    state.add_reward(reward_amount, "Novel grain added".to_string());
    state.grain_count += 1;

    Ok(grain_id)
}

#[tauri::command]
pub async fn get_grain_details(grain_id: String) -> Result<Grain, String> {
    tracing::info!("Getting grain details: {}", grain_id);

    // TODO: Fetch from storage
    Ok(Grain {
        id: grain_id,
        content: "Detailed grain content...".to_string(),
        author: "local".to_string(),
        timestamp: chrono::Utc::now().timestamp(),
        confidence: 0.85,
    })
}

// ============================================================================
// Reward Tracking Commands
// ============================================================================

#[tauri::command]
pub async fn get_balance(
    app_state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<f64, String> {
    let state = app_state.lock().await;
    Ok(state.balance)
}

#[tauri::command]
pub async fn get_rewards(
    app_state: State<'_, Arc<Mutex<AppState>>>,
    limit: Option<usize>,
) -> Result<Vec<Reward>, String> {
    let state = app_state.lock().await;
    let limit = limit.unwrap_or(50);

    let rewards: Vec<Reward> = state
        .rewards
        .iter()
        .rev()
        .take(limit)
        .map(|r| Reward {
            id: r.id.clone(),
            amount: r.amount,
            reason: r.reason.clone(),
            timestamp: r.timestamp,
        })
        .collect();

    Ok(rewards)
}

#[tauri::command]
pub async fn get_today_earnings(
    app_state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<f64, String> {
    let state = app_state.lock().await;
    Ok(state.today_earnings())
}

// ============================================================================
// Data Export Commands
// ============================================================================

#[tauri::command]
pub async fn export_data(format: String) -> Result<String, String> {
    match format.as_str() {
        "json" => {
            // TODO: Export to JSON
            tracing::info!("Exporting data to JSON");
            Ok("data.json".to_string())
        }
        "csv" => {
            // TODO: Export to CSV
            tracing::info!("Exporting data to CSV");
            Ok("data.csv".to_string())
        }
        _ => Err(format!("Unsupported format: {}", format)),
    }
}
