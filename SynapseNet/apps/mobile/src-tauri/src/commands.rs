//! Tauri commands for mobile app

use crate::state::MobileAppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct GrainData {
    pub text: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResult {
    pub id: String,
    pub text: String,
    pub similarity: f32,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsData {
    pub grain_count: usize,
    pub peer_count: usize,
    pub storage_mb: f32,
    pub battery_level: f32,
    pub network_state: String,
}

/// Initialize the app
#[tauri::command]
pub async fn syn_init(state: State<'_, MobileAppState>) -> Result<String, String> {
    tracing::info!("Initializing SynapseNet Mobile");
    
    let provider = state.ai_manager.read().await.provider_name().await;
    
    Ok(format!("SynapseNet Mobile initialized with {}", provider))
}

/// Add a new grain
#[tauri::command]
pub async fn syn_add(
    grain: GrainData,
    state: State<'_, MobileAppState>,
) -> Result<String, String> {
    tracing::info!("Adding grain: {} tags", grain.tags.len());
    
    // TODO: Generate embedding
    // TODO: Create grain
    // TODO: Store in capsule
    
    Ok("grain_id_placeholder".to_string())
}

/// Query for similar grains
#[tauri::command]
pub async fn syn_query(
    query: String,
    k: usize,
    state: State<'_, MobileAppState>,
) -> Result<Vec<QueryResult>, String> {
    tracing::info!("Querying: {} (k={})", query, k);
    
    // TODO: Generate query embedding
    // TODO: Search in capsule
    // TODO: Return results
    
    Ok(vec![])
}

/// Get app statistics
#[tauri::command]
pub async fn syn_stats(state: State<'_, MobileAppState>) -> Result<StatsData, String> {
    let capsule = state.capsule.read().await;
    let battery = state.battery.read().await;
    let p2p = state.p2p_manager.read().await;
    
    let grain_count = capsule.count_grains().map_err(|e| e.to_string())?;
    let peer_count = p2p.get_peers().len();
    
    Ok(StatsData {
        grain_count,
        peer_count,
        storage_mb: 0.0, // TODO: Calculate actual storage
        battery_level: battery.battery_level(),
        network_state: "WiFi".to_string(), // TODO: Get actual network state
    })
}

/// Get connected peers
#[tauri::command]
pub async fn syn_peers(state: State<'_, MobileAppState>) -> Result<Vec<String>, String> {
    let p2p = state.p2p_manager.read().await;
    let peers = p2p.get_peers();
    
    Ok(peers.iter().map(|p| p.to_string()).collect())
}

/// Get wallet info
#[tauri::command]
pub async fn syn_wallet(state: State<'_, MobileAppState>) -> Result<serde_json::Value, String> {
    // TODO: Implement wallet functionality
    
    Ok(serde_json::json!({
        "balance": 0.0,
        "rewards": []
    }))
}

/// Export data
#[tauri::command]
pub async fn syn_export(
    path: String,
    state: State<'_, MobileAppState>,
) -> Result<(), String> {
    tracing::info!("Exporting to: {}", path);
    
    let capsule = state.capsule.read().await;
    capsule.export_encrypted(&path).map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Import data
#[tauri::command]
pub async fn syn_import(
    path: String,
    recovery_phrase: String,
    state: State<'_, MobileAppState>,
) -> Result<(), String> {
    tracing::info!("Importing from: {}", path);
    
    let mut capsule = state.capsule.write().await;
    capsule.import_encrypted(&path, &recovery_phrase).map_err(|e| e.to_string())?;
    
    Ok(())
}
