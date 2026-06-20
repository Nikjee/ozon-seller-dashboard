use crate::config::OzonConfig;
use crate::ozon;
use serde_json::Value;

/// Get list of available warehouses for supply.
pub async fn get_available_warehouses(config: &OzonConfig) -> Result<Value, String> {
    ozon::ozon_request(config, "/v1/supplier/available_warehouses", "POST", None).await
}

/// Get list of Ozon clusters.
pub async fn get_cluster_list(config: &OzonConfig) -> Result<Value, String> {
    let body = serde_json::json!({
        "cluster_ids": [],
        "cluster_type": "CLUSTER_TYPE_OZON",
    });
    ozon::ozon_request(config, "/v1/cluster/list", "POST", Some(&body)).await
}

/// Create a supply draft with items, cluster, and optional drop-off warehouse.
pub async fn create_supply_draft(
    config: &OzonConfig,
    cluster_ids: Vec<i64>,
    items: Vec<Value>,
    drop_off_warehouse_id: Option<i64>,
    supply_type: String,
) -> Result<Value, String> {
    let mut body = serde_json::json!({
        "cluster_ids": cluster_ids,
        "items": items,
        "type": supply_type,
    });
    if let Some(id) = drop_off_warehouse_id {
        body["drop_off_point_warehouse_id"] = serde_json::json!(id);
    }
    ozon::ozon_request(config, "/v1/draft/create", "POST", Some(&body)).await
}

/// Get info for a created draft by operation ID.
pub async fn get_draft_info(config: &OzonConfig, operation_id: &str) -> Result<Value, String> {
    let body = serde_json::json!({ "operation_id": operation_id });
    ozon::ozon_request(config, "/v1/draft/create/info", "POST", Some(&body)).await
}

/// Get available timeslots for a draft.
pub async fn get_timeslots(
    config: &OzonConfig,
    draft_id: i64,
    warehouse_ids: Vec<i64>,
    date_from: &str,
    date_to: &str,
) -> Result<Value, String> {
    let body = serde_json::json!({
        "draft_id": draft_id,
        "warehouse_ids": warehouse_ids,
        "date_from": date_from,
        "date_to": date_to,
    });
    ozon::ozon_request(config, "/v1/draft/timeslot/info", "POST", Some(&body)).await
}

/// Create a real supply from a draft with the chosen timeslot.
pub async fn create_supply_from_draft(
    config: &OzonConfig,
    draft_id: i64,
    warehouse_id: i64,
    timeslot_from: &str,
    timeslot_to: &str,
) -> Result<Value, String> {
    let body = serde_json::json!({
        "draft_id": draft_id,
        "warehouse_id": warehouse_id,
        "timeslot": {
            "from_in_timezone": timeslot_from,
            "to_in_timezone": timeslot_to,
        },
    });
    ozon::ozon_request(config, "/v1/draft/supply/create", "POST", Some(&body)).await
}

/// Poll the status of a supply creation operation.
pub async fn get_supply_create_status(
    config: &OzonConfig,
    operation_id: &str,
) -> Result<Value, String> {
    let body = serde_json::json!({ "operation_id": operation_id });
    ozon::ozon_request(config, "/v1/draft/supply/create/status", "POST", Some(&body)).await
}

/// List products (reuses get_products, wraps result).
pub async fn list_products(config: &OzonConfig) -> Result<Value, String> {
    let (items, total) = crate::ozon::get_products(config, 1000).await?;
    Ok(serde_json::json!({ "products": items, "total": total }))
}
