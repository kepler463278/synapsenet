// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

use state::AppState;
use std::sync::Arc;
use synapsenet_core::Config;
use tauri::Manager;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "synapsenet_tauri=info,synapsenet=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Load or create default configuration
            let config_path = app
                .path()
                .app_config_dir()
                .expect("Failed to get config dir")
                .join("config.toml");

            let config = Config::load_or_default(&config_path)
                .expect("Failed to load configuration");

            tracing::info!("Loaded configuration from {:?}", config_path);
            tracing::info!("Data directory: {}", config.node.data_dir);

            // Initialize application state asynchronously
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match AppState::new(config).await {
                    Ok(app_state) => {
                        app_handle.manage(Arc::new(app_state));
                        tracing::info!("SynapseNet Tauri application initialized");
                    }
                    Err(e) => {
                        tracing::error!("Failed to initialize application state: {}", e);
                        std::process::exit(1);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::add_grain,
            commands::search_grains,
            commands::get_grain_details,
            commands::get_stats,
            commands::get_network_peers,
            commands::health_check,
            commands::get_config,
            commands::update_config,
            commands::reset_config,
            commands::validate_config,
            commands::check_for_updates,
            commands::install_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
