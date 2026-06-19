mod analytics;
mod config;
mod dashboard;
mod expenses;
mod ozon;
mod uniteconomy;

use log::info;
use serde_json::Value;
use std::sync::Mutex;
use tauri::State;

use crate::config::OzonConfig;

struct AppState {
    config: Mutex<Option<OzonConfig>>,
}

fn get_config(state: &State<'_, AppState>) -> Result<OzonConfig, String> {
    let guard = state.config.lock().map_err(|e| e.to_string())?;
    guard
        .as_ref()
        .ok_or_else(|| "Config not loaded. Please configure API keys first.".to_string())
        .cloned()
}

#[tauri::command]
async fn check_config(state: State<'_, AppState>) -> Result<Value, String> {
    match OzonConfig::load() {
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
    OzonConfig::save(&client_id, &api_key)?;
    let mut c = state.config.lock().map_err(|e| e.to_string())?;
    *c = Some(OzonConfig {
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
    let cfg = get_config(&state)?;
    info!("[CMD] get_dashboard_summary invoked month={} year={}", month, year);
    let result = dashboard::build_dashboard_summary(&cfg, month, year).await;
    match &result {
        Ok(v) => info!("[CMD] get_dashboard_summary OK: products={} totals_keys={:?}",
            v["products"].as_array().map(|a| a.len()).unwrap_or(0),
            v["totals"].as_object().map(|o| o.keys().collect::<Vec<_>>())),
        Err(e) => info!("[CMD] get_dashboard_summary FAIL: {}", e),
    }
    result
}

/// Returns the GitHub PAT for updater authentication, embedded at compile time.
#[tauri::command]
fn get_updater_token() -> Option<String> {
    option_env!("TAURI_UPDATER_TOKEN").map(|s| s.to_string())
}

#[tauri::command]
async fn get_stock_on_warehouses(
    limit: i64,
    offset: i64,
    warehouse_type: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    ozon::get_stock_on_warehouses(&cfg, limit, offset, &warehouse_type)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_analytics_stocks(
    skus: Vec<i64>,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    ozon::get_analytics_stocks(&cfg, skus)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_stocks_turnover(
    sku: i64,
    limit: i64,
    offset: i64,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    ozon::get_stocks_turnover(&cfg, sku, limit, offset)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_finance_totals(
    date_from: String,
    date_to: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    info!("[CMD] get_finance_totals invoked date_from={} date_to={}", date_from, date_to);
    let raw = ozon::get_finance_totals(&cfg, &date_from, &date_to)
        .await
        .map_err(|e| e.to_string())?;
    info!("[CMD] get_finance_totals raw keys={:?}", raw.as_object().map(|o| o.keys().collect::<Vec<_>>()));

    let result = &raw["result"];
    let extract = |key: &str| -> f64 { result[key].as_f64().unwrap_or(0.0) };

    Ok(serde_json::json!({
        "accruals_for_sale": extract("accruals_for_sale"),
        "total_compensation": extract("compensation_amount"),
        "sale_commission": extract("sale_commission"),
        "services_amount": extract("services_amount"),
        "processing_and_delivery": extract("processing_and_delivery"),
        "refunds_and_cancellations": extract("refunds_and_cancellations"),
        "money_transfer": extract("money_transfer"),
        "others_amount": extract("others_amount"),
    }))
}

#[tauri::command]
async fn get_stock_report(
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    info!("[CMD] get_stock_report invoked");
    let result = analytics::get_stock_report(&cfg).await;
    match &result {
        Ok(v) => info!("[CMD] get_stock_report OK: keys={:?}", v.as_object().map(|o| o.keys().collect::<Vec<_>>())),
        Err(e) => info!("[CMD] get_stock_report FAIL: {}", e),
    }
    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_stock_analytics(
    skus: Vec<i64>,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    info!("[CMD] get_stock_analytics invoked skus={:?}", skus);
    let result = analytics::get_stock_analytics(&cfg, &skus).await;
    match &result {
        Ok(v) => info!("[CMD] get_stock_analytics OK: keys={:?} products_len={}", v.as_object().map(|o| o.keys().collect::<Vec<_>>()), v["products"].as_array().map(|a| a.len()).unwrap_or(0)),
        Err(e) => info!("[CMD] get_stock_analytics FAIL: {}", e),
    }
    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_turnover_data(
    skus: Vec<i64>,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    analytics::get_turnover_data(&cfg, &skus)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_analytics_dashboard_data(
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    info!("[CMD] get_analytics_dashboard_data invoked");
    let result = analytics::get_analytics_dashboard_data(&cfg).await;
    match &result {
        Ok(v) => info!("[CMD] get_analytics_dashboard_data OK: products={} aggregates={:?}",
            v["products"].as_array().map(|a| a.len()).unwrap_or(0),
            v.as_object().map(|o| o.keys().collect::<Vec<_>>())),
        Err(e) => info!("[CMD] get_analytics_dashboard_data FAIL: {}", e),
    }
    result.map_err(|e| e.to_string())
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
            get_stock_on_warehouses,
            get_analytics_stocks,
            get_stocks_turnover,
            get_finance_totals,
            get_stock_report,
            get_stock_analytics,
            get_turnover_data,
            get_analytics_dashboard_data,
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
