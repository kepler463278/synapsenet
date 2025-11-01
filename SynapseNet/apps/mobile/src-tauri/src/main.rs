// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;
mod voice;
mod file_import;
mod notifications;
mod poe;
mod accessibility;

use state::MobileAppState;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize app state
            let state = MobileAppState::new()?;
            app.manage(state);
            
            tracing::info!("SynapseNet Mobile initialized");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::syn_init,
            commands::syn_add,
            commands::syn_query,
            commands::syn_stats,
            commands::syn_peers,
            commands::syn_wallet,
            commands::syn_export,
            commands::syn_import,
            voice::start_voice_recognition,
            voice::stop_voice_recognition,
            voice::is_voice_available,
            file_import::pick_files,
            file_import::import_files,
            file_import::get_file_info,
            notifications::request_notification_permission,
            notifications::send_notification,
            notifications::schedule_notification,
            notifications::cancel_notification,
            notifications::get_notification_settings,
            notifications::update_notification_settings,
            poe::calculate_poe_score,
            poe::update_poe_reuse,
            poe::get_wallet_info,
            poe::get_reward_history,
            poe::get_poe_breakdown,
            poe::sync_rewards,
            poe::export_rewards,
            accessibility::is_screen_reader_running,
            accessibility::get_accessibility_info,
            accessibility::announce_for_accessibility,
            accessibility::post_accessibility_notification,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
