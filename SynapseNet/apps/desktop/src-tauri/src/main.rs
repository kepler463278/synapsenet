//! SynapseNet Desktop GUI - Tauri Backend

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod events;
mod node;
mod state;

use commands::*;
use events::start_event_emitter;
use node::create_node_manager;
use state::AppState;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting SynapseNet Desktop v1.0");

    // Create app state
    let app_state = Arc::new(Mutex::new(AppState::new()));
    let node_manager = create_node_manager();

    let app_state_for_setup = app_state.clone();
    
    tauri::Builder::default()
        .manage(app_state)
        .manage(node_manager)
        .invoke_handler(tauri::generate_handler![
            start_node,
            stop_node,
            get_node_status,
            get_network_stats,
            search_grains,
            add_grain,
            get_grain_details,
            get_balance,
            get_rewards,
            get_today_earnings,
            export_data
        ])
        .setup(move |app| {
            // Setup app window
            let window = app.get_window("main").unwrap();
            window.set_title("SynapseNet v1.0 - Decentralized Intelligence")?;

            // Start background event emitter
            let app_handle = app.handle();
            let state_clone = app_state_for_setup.clone();
            tokio::spawn(async move {
                start_event_emitter(app_handle, state_clone).await;
            });

            tracing::info!("SynapseNet Desktop initialized successfully");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
