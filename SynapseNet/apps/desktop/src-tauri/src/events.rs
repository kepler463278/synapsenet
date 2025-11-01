//! Real-time event system for GUI updates

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AppEvent {
    NetworkUpdate {
        peers: u32,
        grains: u32,
        timestamp: i64,
    },
    RewardEarned {
        amount: f64,
        reason: String,
        timestamp: i64,
    },
    GrainValidated {
        grain_id: String,
        validators: u32,
    },
    NodeStatusChanged {
        running: bool,
    },
    SyncProgress {
        progress: f64,
    },
}

/// Background task that emits periodic network updates
pub async fn start_event_emitter(app: AppHandle, state: Arc<Mutex<AppState>>) {
    info!("Starting event emitter background task");

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));

    loop {
        interval.tick().await;

        let app_state = state.lock().await;

        if app_state.node_running {
            let event = AppEvent::NetworkUpdate {
                peers: app_state.peer_count,
                grains: app_state.grain_count,
                timestamp: chrono::Utc::now().timestamp(),
            };

            if let Err(e) = app.emit_all("app-event", &event) {
                error!("Failed to emit network update: {}", e);
            }
        }
    }
}

/// Emit a reward earned event
pub fn emit_reward_earned(app: &AppHandle, amount: f64, reason: String) {
    let event = AppEvent::RewardEarned {
        amount,
        reason,
        timestamp: chrono::Utc::now().timestamp(),
    };

    if let Err(e) = app.emit_all("app-event", &event) {
        error!("Failed to emit reward event: {}", e);
    }
}

/// Emit a grain validated event
pub fn emit_grain_validated(app: &AppHandle, grain_id: String, validators: u32) {
    let event = AppEvent::GrainValidated {
        grain_id,
        validators,
    };

    if let Err(e) = app.emit_all("app-event", &event) {
        error!("Failed to emit grain validated event: {}", e);
    }
}

/// Emit a node status changed event
pub fn emit_node_status_changed(app: &AppHandle, running: bool) {
    let event = AppEvent::NodeStatusChanged { running };

    if let Err(e) = app.emit_all("app-event", &event) {
        error!("Failed to emit node status event: {}", e);
    }
}

/// Emit a sync progress event
pub fn emit_sync_progress(app: &AppHandle, progress: f64) {
    let event = AppEvent::SyncProgress { progress };

    if let Err(e) = app.emit_all("app-event", &event) {
        error!("Failed to emit sync progress event: {}", e);
    }
}
