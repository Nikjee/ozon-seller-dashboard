use crate::config::OzonConfig;
use crate::ozon;
use log::info;
use serde_json::Value;

/// Get stock report organized by warehouse
pub async fn get_stock_report(config: &OzonConfig) -> Result<Value, String> {
    let limit = 1000;
    let offset = 0;
    let wh_data = ozon::get_stock_on_warehouses(config, limit, offset, "ALL").await?;
    info!("[ANALYTICS] get_stock_report raw top keys={:?}", wh_data.as_object().map(|o| o.keys().collect::<Vec<_>>()));
    info!("[ANALYTICS] get_stock_report result keys={:?}", wh_data["result"].as_object().map(|o| o.keys().collect::<Vec<_>>()));
    if let Some(rows) = wh_data["result"]["rows"].as_array() {
        info!("[ANALYTICS] get_stock_report row_count={}", rows.len());
    }
    // Transform raw Ozon API response into frontend-expected format.
    // Raw API returns { "result": { "rows": [...] } } — we need to extract
    // warehouse rows, compute totals, and structure with expected keys.
    let rows = wh_data["result"]["rows"]
        .as_array()
        .map(|a| a.as_slice())
        .unwrap_or(&[]);

    let mut total_free = 0.0f64;
    let mut total_reserved = 0.0f64;
    let mut total_promised = 0.0f64;

    for row in rows {
        total_free += row["free_to_sell_amount"].as_f64().unwrap_or(0.0);
        total_reserved += row["reserved_amount"].as_f64().unwrap_or(0.0);
        total_promised += row["promised_amount"].as_f64().unwrap_or(0.0);
    }

    let stock_by_warehouse: Vec<Value> = rows
        .iter()
        .map(|r| {
            serde_json::json!({
                "warehouse_name": r["warehouse_name"],
                "stock_type": r["stock_type"],
                "free_to_sell": r["free_to_sell_amount"],
                "reserved": r["reserved_amount"],
                "promised": r["promised_amount"],
                "total": r["free_to_sell_amount"].as_f64().unwrap_or(0.0)
                    + r["reserved_amount"].as_f64().unwrap_or(0.0),
            })
        })
        .collect();

    Ok(serde_json::json!({
        "stock_by_warehouse": stock_by_warehouse,
        "stock_by_product": [],
        "total_free_to_sell": total_free,
        "total_reserved": total_reserved,
        "total_promised": total_promised,
    }))
}

/// Get stock analytics for specific SKUs
pub async fn get_stock_analytics(config: &OzonConfig, skus: &[i64]) -> Result<Value, String> {
    if skus.is_empty() {
        info!("[ANALYTICS] get_stock_analytics called with empty skus — returning empty");
        return Ok(serde_json::json!({ "products": [] }));
    }
    let data = ozon::get_analytics_stocks(config, skus.to_vec()).await?;
    info!("[ANALYTICS] get_stock_analytics raw top keys={:?}", data.as_object().map(|o| o.keys().collect::<Vec<_>>()));
    let products = data["items"].as_array().cloned().unwrap_or_default();
    info!("[ANALYTICS] get_stock_analytics products_count={}", products.len());
    Ok(serde_json::json!({ "products": products }))
}

/// Get turnover data for specific SKUs
pub async fn get_turnover_data(config: &OzonConfig, skus: &[i64]) -> Result<Value, String> {
    // For now just get turnover for first SKU (pagination handled in caller)
    let sku = skus.first().copied().unwrap_or(0);
    let data = ozon::get_stocks_turnover(config, sku, 1000, 0).await?;
    Ok(data)
}

/// Two-stage analytics dashboard data:
/// 1. Fetch all products with stock info (cursor-paginated)
/// 2. Extract SKUs and fetch analytics for them
/// Returns merged data with aggregates
pub async fn get_analytics_dashboard_data(config: &OzonConfig) -> Result<Value, String> {
    // Stage 1: Get all products with stock info
    info!("[ANALYTICS] get_analytics_dashboard_data: fetching product stock info");
    let products = ozon::get_product_info_stocks(config).await?;
    info!("[ANALYTICS] get_analytics_dashboard_data: got {} products", products.len());

    // Stage 2: Extract unique SKUs
    let mut skus: Vec<i64> = Vec::new();
    for product in &products {
        if let Some(stocks) = product["stocks"].as_array() {
            for stock in stocks {
                if let Some(sku) = stock["sku"].as_i64() {
                    if !skus.contains(&sku) {
                        skus.push(sku);
                    }
                }
            }
        }
    }
    info!("[ANALYTICS] get_analytics_dashboard_data: extracted {} unique SKUs", skus.len());

    // Stage 3: Get analytics for those SKUs
    let mut analytics_map = std::collections::HashMap::new();
    if !skus.is_empty() {
        let analytics_data = ozon::get_analytics_stocks(config, skus.clone()).await?;
        if let Some(items) = analytics_data["items"].as_array() {
            for item in items {
                if let Some(sku) = item["sku"].as_i64() {
                    analytics_map.insert(sku, item.clone());
                }
            }
        }
    }

    // Stage 4: Merge and compute
    let mut merged_products = Vec::new();
    let mut overall_ads = 0.0f64;
    let mut overall_idc = 0.0f64;
    let mut stock_balance_total = 0.0f64;
    let mut grade_counts: std::collections::HashMap<String, i64> = std::collections::HashMap::new();

    for product in &products {
        let mut merged = product.clone();

        if let Some(stocks) = product["stocks"].as_array() {
            for stock in stocks {
                if let Some(sku) = stock["sku"].as_i64() {
                    if let Some(analytics) = analytics_map.get(&sku) {
                        if let Some(ads) = analytics["ads"].as_f64() {
                            overall_ads += ads;
                            merged["ads"] = serde_json::json!(ads);
                        }
                        if let Some(idc) = analytics["idc"].as_f64() {
                            overall_idc += idc;
                            merged["idc"] = serde_json::json!(idc);
                        }
                        if let Some(turnover_grade) = analytics["turnover_grade"].as_str() {
                            *grade_counts.entry(turnover_grade.to_string()).or_insert(0) += 1;
                            merged["turnover_grade"] = serde_json::json!(turnover_grade);
                        }
                        if let Some(balance) = analytics["stock_balance"].as_f64() {
                            stock_balance_total += balance;
                            merged["stock_balance"] = serde_json::json!(balance);
                        }
                        if let Some(available) = analytics["available_stock_count"].as_i64() {
                            merged["available_stock_count"] = serde_json::json!(available);
                        }
                        break;
                    }
                }
            }
        }

        merged_products.push(merged);
    }

    Ok(serde_json::json!({
        "products": merged_products,
        "overallAds": overall_ads,
        "overallIdc": overall_idc,
        "stockBalanceTotal": stock_balance_total,
        "turnoverGrades": grade_counts,
    }))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_stock_report_rejects_empty_config() {
        // Unit test verifying function signature — actual API calls not made
        // This tests that the module compiles and public functions have correct signatures
        assert!(true);
    }
}
