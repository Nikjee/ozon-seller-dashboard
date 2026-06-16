use serde_json::Value;
use std::collections::HashMap;

/// Extract granular cost categories for a given SKU from finance transactions.
/// Returns a flat JSON object with all 20+ cost fields (0.0 if not found).
pub fn extract_product_costs(operations: &[Value], sku: i64) -> Value {
    let mut costs: HashMap<String, f64> = HashMap::new();

    for op in operations {
        let items = op["items"].as_array();
        if items.map_or(true, |i| i.is_empty()) {
            continue;
        }
        let has_sku = items.unwrap().iter().any(|item| {
            item["sku"].as_i64().map_or(false, |s| s == sku)
        });
        if !has_sku {
            continue;
        }

        let amount = op["amount"].as_f64().unwrap_or(0.0).abs();
        let op_type_name = op["operation_type_name"]
            .as_str()
            .unwrap_or("")
            .to_lowercase();
        let op_type = op["operation_type"].as_str().unwrap_or("").to_lowercase();

        // Extract based on keyword matching (follows expenses.rs pattern)
        if op_type_name.contains("комисс") || op_type == "sale_commission" {
            *costs.entry("commission".to_string()).or_insert(0.0) += amount;
        }
        if op_type_name.contains("эквайринг") || op_type == "acquiring" {
            *costs.entry("acquiring".to_string()).or_insert(0.0) += amount;
        }
        if op_type_name.contains("обработк") && op_type_name.contains("отправлен") {
            *costs
                .entry("order_processing".to_string())
                .or_insert(0.0) += amount;
        }
        if op_type_name.contains("логистик")
            || op_type_name.contains("кросс-док")
            || op_type_name.contains("crossdoc")
        {
            *costs.entry("logistics".to_string()).or_insert(0.0) += amount;
        }
        if op_type_name.contains("доставк")
            && (op_type_name.contains("выдач") || op_type_name.contains("pickup"))
        {
            *costs
                .entry("delivery_to_pickup".to_string())
                .or_insert(0.0) += amount;
        }
        if op_type_name.contains("размещ") || op_type_name.contains("хранени") {
            *costs.entry("placement".to_string()).or_insert(0.0) += amount;
        }
        if op_type_name.contains("обработк") && op_type_name.contains("возврат") {
            *costs
                .entry("return_processing".to_string())
                .or_insert(0.0) += amount;
        }
        if op_type_name.contains("обратн") && op_type_name.contains("логистик") {
            *costs
                .entry("return_logistics".to_string())
                .or_insert(0.0) += amount;
        }
        if op_type_name.contains("утилиз") {
            *costs.entry("disposal".to_string()).or_insert(0.0) += amount;
        }
        if op_type_name.contains("овх") || op_type_name.contains("дополнитель") {
            *costs
                .entry("ovh_processing".to_string())
                .or_insert(0.0) += amount;
        }
        if op_type_name.contains("операцион") && op_type_name.contains("ошибк") {
            *costs
                .entry("operational_errors".to_string())
                .or_insert(0.0) += amount;
        }
        if op_type_name.contains("клик") || op_type_name.contains("click") {
            *costs
                .entry("pay_per_click".to_string())
                .or_insert(0.0) += amount;
        }
        if op_type_name.contains("заказ") || op_type.contains("per_order") {
            *costs
                .entry("pay_per_order".to_string())
                .or_insert(0.0) += amount;
        }
        if op_type_name.contains("звёзд") || op_type_name.contains("star") {
            *costs
                .entry("star_products".to_string())
                .or_insert(0.0) += amount;
        }
        if op_type_name.contains("бренд") || op_type_name.contains("brand") {
            *costs
                .entry("paid_brand".to_string())
                .or_insert(0.0) += amount;
        }
        if op_type_name.contains("отзыв") || op_type_name.contains("review") {
            *costs
                .entry("reviews_cost".to_string())
                .or_insert(0.0) += amount;
        }
        if op_type_name.contains("балл") || op_type_name.contains("discount") {
            *costs
                .entry("discount_points".to_string())
                .or_insert(0.0) += amount;
        }
        if op_type_name.contains("партнёр") || op_type_name.contains("partner") {
            *costs
                .entry("partner_programs".to_string())
                .or_insert(0.0) += amount;
        }
        if op_type_name.contains("компенсац") || op_type.contains("compensation") {
            *costs
                .entry("compensation".to_string())
                .or_insert(0.0) += amount;
        }
    }

    // Return with all expected fields (0.0 as default)
    let all_fields = [
        "commission",
        "acquiring",
        "order_processing",
        "logistics",
        "delivery_to_pickup",
        "placement",
        "return_processing",
        "return_logistics",
        "disposal",
        "ovh_processing",
        "operational_errors",
        "pay_per_click",
        "pay_per_order",
        "star_products",
        "paid_brand",
        "reviews_cost",
        "discount_points",
        "partner_programs",
        "compensation",
        "other_services",
    ];

    let mut result = serde_json::Map::new();
    for field in &all_fields {
        let val = costs.get(*field).copied().unwrap_or(0.0);
        result.insert(field.to_string(), serde_json::json!(val));
    }

    serde_json::Value::Object(result)
}

/// Extract enriched product summary with computed fields.
pub fn extract_product_summary(operations: &[Value], sku: i64) -> Value {
    let costs = extract_product_costs(operations, sku);

    // Calculate total revenue
    let total_revenue: f64 = operations
        .iter()
        .filter(|op| {
            op["items"].as_array().map_or(false, |items| {
                items.iter().any(|item| item["sku"].as_i64() == Some(sku))
            })
        })
        .filter_map(|op| op["accruals_for_sale"].as_f64())
        .sum();

    let total_costs: f64 = costs
        .as_object()
        .map(|obj| obj.values().filter_map(|v| v.as_f64()).sum())
        .unwrap_or(0.0);

    let net_profit = total_revenue - total_costs;

    let qty: i64 = operations
        .iter()
        .filter(|op| {
            op["items"].as_array().map_or(false, |items| {
                items.iter().any(|item| item["sku"].as_i64() == Some(sku))
            })
        })
        .filter_map(|op| {
            op["items"].as_array().map(|items| {
                items
                    .iter()
                    .filter(|item| item["sku"].as_i64() == Some(sku))
                    .count() as i64
            })
        })
        .sum();

    serde_json::json!({
        "costs": costs,
        "total_revenue": total_revenue,
        "total_costs": total_costs,
        "net_profit": net_profit,
        "profit_per_unit": if qty > 0 { net_profit / qty as f64 } else { 0.0 },
        "total_quantity": qty,
    })
}
