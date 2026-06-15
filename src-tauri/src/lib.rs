mod config;
mod dashboard;
mod expenses;
mod ozon;

use serde_json::Value;
use std::sync::Mutex;
use tauri::State;

struct AppState {
    config: Mutex<Option<config::OzonConfig>>,
}

#[tauri::command]
async fn check_config(state: State<'_, AppState>) -> Result<Value, String> {
    match config::OzonConfig::load() {
        Ok(cfg) => {
            let mut c = state.config.lock().map_err(|e| e.to_string())?;
            *c = Some(cfg);
            Ok(serde_json::json!({ "valid": true }))
        }
        Err(e) => Ok(serde_json::json!({ "valid": false, "message": e })),
    }
}

#[tauri::command]
async fn save_config(
    client_id: String,
    api_key: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    config::OzonConfig::save(&client_id, &api_key)?;
    let mut c = state.config.lock().map_err(|e| e.to_string())?;
    *c = Some(config::OzonConfig {
        ozon_client_id: client_id,
        ozon_api_key: api_key,
    });
    Ok(serde_json::json!({ "valid": true }))
}

#[tauri::command]
async fn get_dashboard_summary(
    month: u32,
    year: i32,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = {
        let guard = state.config.lock().map_err(|e| e.to_string())?;
        guard
            .as_ref()
            .ok_or_else(|| "Config not loaded. Please configure API keys first.".to_string())?
            .clone()
    };
    dashboard::build_dashboard_summary(&cfg, month, year).await
}

/// Returns the GitHub PAT for updater authentication, embedded at compile time.
/// This allows the updater to authenticate with GitHub's API (Bearer token)
/// instead of Basic Auth, which is required for private repo release assets.
#[tauri::command]
fn get_updater_token() -> Option<String> {
    option_env!("TAURI_UPDATER_TOKEN").map(|s| s.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            config: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            check_config,
            save_config,
            get_dashboard_summary,
            get_updater_token,
        ])
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            app.handle().plugin(
                tauri_plugin_updater::Builder::new().build(),
            )?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
