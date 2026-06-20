mod analytics;
mod config;
mod dashboard;
mod expenses;
mod ozon;
mod supply;
mod uniteconomy;

use log::info;
use serde_json::Value;
use std::sync::Mutex;
use tauri::State;
use tauri_plugin_dialog;

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
async fn get_finance_detailed(
    date_from: String,
    date_to: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    info!("[CMD] get_finance_detailed date_from={} date_to={}", date_from, date_to);

    // Get aggregate totals
    let totals = get_finance_totals(date_from.clone(), date_to.clone(), state.clone()).await?;

    // Get all transactions and categorize
    let transactions_raw = ozon::get_finance_transactions(&cfg, &date_from, &date_to).await?;
    let operations = transactions_raw["result"]["operations"]
        .as_array()
        .cloned()
        .unwrap_or_default();

    let expenses = expenses::build_all_expenses(&operations);

    let total_expenses: f64 = expenses.cats.values().sum();

    Ok(serde_json::json!({
        "totals": totals,
        "expenses": {
            "categories": expenses.cats,
            "details": expenses.details,
            "total": total_expenses,
        }
    }))
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

#[tauri::command]
async fn get_fbo_totals(
    month: u32,
    year: i32,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    ozon::get_fbo_posting_totals(&cfg, month, year).await
}

#[tauri::command]
async fn get_available_warehouses(
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    info!("[CMD] get_available_warehouses invoked");
    let result = supply::get_available_warehouses(&cfg).await;
    match &result {
        Ok(v) => info!("[CMD] get_available_warehouses OK: keys={:?}", v.as_object().map(|o| o.keys().collect::<Vec<_>>())),
        Err(e) => info!("[CMD] get_available_warehouses FAIL: {}", e),
    }
    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_cluster_list(
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    info!("[CMD] get_cluster_list invoked");
    let result = supply::get_cluster_list(&cfg).await;
    match &result {
        Ok(v) => info!("[CMD] get_cluster_list OK: keys={:?}", v.as_object().map(|o| o.keys().collect::<Vec<_>>())),
        Err(e) => info!("[CMD] get_cluster_list FAIL: {}", e),
    }
    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_supply_draft(
    cluster_ids: Vec<i64>,
    items: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    let items_vec: Vec<Value> = serde_json::from_str(&items).map_err(|e| e.to_string())?;
    info!("[CMD] create_supply_draft invoked cluster_ids={:?} items_count={}", cluster_ids, items_vec.len());
    let result = supply::create_supply_draft(&cfg, cluster_ids, items_vec, None, "CREATE_TYPE_DIRECT".to_string()).await;
    match &result {
        Ok(v) => info!("[CMD] create_supply_draft OK: keys={:?}", v.as_object().map(|o| o.keys().collect::<Vec<_>>())),
        Err(e) => info!("[CMD] create_supply_draft FAIL: {}", e),
    }
    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_draft_info(
    operation_id: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    info!("[CMD] get_draft_info invoked operation_id={}", operation_id);
    let result = supply::get_draft_info(&cfg, &operation_id).await;
    match &result {
        Ok(v) => info!("[CMD] get_draft_info OK: keys={:?}", v.as_object().map(|o| o.keys().collect::<Vec<_>>())),
        Err(e) => info!("[CMD] get_draft_info FAIL: {}", e),
    }
    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_timeslots(
    draft_id: i64,
    warehouse_ids: Vec<i64>,
    date_from: String,
    date_to: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    info!("[CMD] get_timeslots invoked draft_id={} warehouse_ids={:?}", draft_id, warehouse_ids);
    let result = supply::get_timeslots(&cfg, draft_id, warehouse_ids, &date_from, &date_to).await;
    match &result {
        Ok(v) => info!("[CMD] get_timeslots OK: keys={:?}", v.as_object().map(|o| o.keys().collect::<Vec<_>>())),
        Err(e) => info!("[CMD] get_timeslots FAIL: {}", e),
    }
    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_supply_from_draft(
    draft_id: i64,
    warehouse_id: i64,
    timeslot_from: String,
    timeslot_to: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    info!("[CMD] create_supply_from_draft invoked draft_id={} warehouse_id={}", draft_id, warehouse_id);
    let result = supply::create_supply_from_draft(&cfg, draft_id, warehouse_id, &timeslot_from, &timeslot_to).await;
    match &result {
        Ok(v) => info!("[CMD] create_supply_from_draft OK: keys={:?}", v.as_object().map(|o| o.keys().collect::<Vec<_>>())),
        Err(e) => info!("[CMD] create_supply_from_draft FAIL: {}", e),
    }
    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_supply_create_status(
    operation_id: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    info!("[CMD] get_supply_create_status invoked operation_id={}", operation_id);
    let result = supply::get_supply_create_status(&cfg, &operation_id).await;
    match &result {
        Ok(v) => info!("[CMD] get_supply_create_status OK: keys={:?}", v.as_object().map(|o| o.keys().collect::<Vec<_>>())),
        Err(e) => info!("[CMD] get_supply_create_status FAIL: {}", e),
    }
    result.map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_products(
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let cfg = get_config(&state)?;
    info!("[CMD] list_products invoked");
    let result = supply::list_products(&cfg).await;
    match &result {
        Ok(v) => info!("[CMD] list_products OK: products={} total={}", 
            v["products"].as_array().map(|a| a.len()).unwrap_or(0),
            v["total"]),
        Err(e) => info!("[CMD] list_products FAIL: {}", e),
    }
    result.map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
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
            get_finance_detailed,
            get_analytics_dashboard_data,
            get_fbo_totals,
            get_available_warehouses,
            get_cluster_list,
            create_supply_draft,
            get_draft_info,
            get_timeslots,
            create_supply_from_draft,
            get_supply_create_status,
            list_products,
        ])
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init());

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(tauri_plugin_mcp_bridge::init());
    }

    builder.setup(|app| {
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
