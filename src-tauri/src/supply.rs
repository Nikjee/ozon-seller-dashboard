use crate::config::OzonConfig;
use crate::ozon;
use serde_json::Value;

/// Get list of available warehouses for supply.
pub async fn get_available_warehouses(config: &OzonConfig) -> Result<Value, String> {
    let raw = ozon::ozon_request(config, "/v1/supplier/available_warehouses", "GET", None).await?;
    let mut warehouses = Vec::new();
    if let Some(result) = raw["result"].as_array() {
        for item in result {
            if let Some(wh) = item["warehouse"].as_object() {
                let id_str = wh["id"].as_str().unwrap_or("0");
                let id: i64 = id_str.parse().unwrap_or(0);
                let name = wh["name"].as_str().unwrap_or("");
                warehouses.push(serde_json::json!({
                    "warehouse_id": id,
                    "name": name,
                }));
            }
        }
    }
    Ok(serde_json::json!(warehouses))
}

/// Get list of Ozon clusters with their warehouses.
pub async fn get_cluster_list(config: &OzonConfig) -> Result<Value, String> {
    let body = serde_json::json!({
        "cluster_ids": [],
        "cluster_type": "CLUSTER_TYPE_OZON",
    });
    let raw = ozon::ozon_request(config, "/v1/cluster/list", "POST", Some(&body)).await?;
    let mut clusters = Vec::new();
    if let Some(items) = raw["clusters"].as_array() {
        for item in items {
            let id = item["id"].as_i64().unwrap_or(0);
            let name = item["name"].as_str().unwrap_or("");

            // Flatten warehouses from all logistic_clusters
            let mut warehouses = Vec::new();
            if let Some(logistic_clusters) = item["logistic_clusters"].as_array() {
                for lc in logistic_clusters {
                    if let Some(whs) = lc["warehouses"].as_array() {
                        for wh in whs {
                            warehouses.push(serde_json::json!({
                                "warehouse_id": wh["warehouse_id"],
                                "name": wh["name"],
                                "type": wh["type"],
                            }));
                        }
                    }
                }
            }

            clusters.push(serde_json::json!({
                "cluster_id": id,
                "name": name,
                "warehouses": warehouses,
            }));
        }
    }
    Ok(serde_json::json!(clusters))
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

/// List products enriched with name and sku from product info.
pub async fn list_products(config: &OzonConfig) -> Result<Value, String> {
    let (items, total) = crate::ozon::get_products(config, 1000).await?;

    // Extract product IDs for enrichment
    let product_ids: Vec<i64> = items.iter()
        .filter_map(|item| item["product_id"].as_i64())
        .collect();

    if product_ids.is_empty() {
        return Ok(serde_json::json!({ "products": [], "total": 0 }));
    }

    // Get enriched product info (name, sku, etc.)
    let enriched = crate::ozon::get_product_info_list(config, &product_ids).await?;

    // Build merged response with only needed fields
    let mut merged: Vec<Value> = Vec::new();
    for item in &enriched {
        let sku = item["sources"]
            .as_array()
            .and_then(|s| s.first())
            .and_then(|s| s["sku"].as_i64())
            .unwrap_or(0);

        merged.push(serde_json::json!({
            "id": item["id"],
            "offer_id": item["offer_id"],
            "name": item["name"],
            "sku": sku,
        }));
    }

    Ok(serde_json::json!({ "products": merged, "total": total }))
}
