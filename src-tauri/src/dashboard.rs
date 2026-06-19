use crate::config::OzonConfig;
use crate::expenses::{build_account_level_expenses, build_expense_categories};
use crate::ozon::{get_finance_transactions, get_product_info_list, get_product_info_prices, get_products, get_realization_report};
use crate::uniteconomy::extract_product_summary;
use chrono::{Datelike, NaiveDate};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct Posting {
    pub posting_number: String,
    pub date: String,
    pub sku: i64,
    pub name: String,
    pub offer_id: String,
    pub seller_price_per_instance: f64,
    pub commission_amount: f64,
    pub commission_ratio: f64,
    pub delivery_charge: f64,
    pub delivery_details: Value,
    pub return_charge: f64,
    pub return_details: Value,
    pub services: Vec<Value>,
    #[serde(rename = "net")]
    pub net: f64,
}

fn month_date_range(month: u32, year: i32) -> (String, String) {
    let padded = format!("{:02}", month);
    let from = format!("{}-{}-01", year, padded);
    let last_day = NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or(NaiveDate::from_ymd_opt(year, month, 28).unwrap())
        .pred_opt()
        .unwrap()
        .day();
    let to = format!("{}-{}-{:02}", year, padded, last_day);
    (from, to)
}

pub async fn build_dashboard_summary(
    config: &OzonConfig,
    month: u32,
    year: i32,
) -> Result<Value, String> {
    let (from, to) = month_date_range(month, year);

    let (products_result, realization_report, transactions_result) = tokio::try_join!(
        get_products(config, "", 1000),
        get_realization_report(config, month, year),
        get_finance_transactions(config, &from, &to),
    )?;

    let items = products_result["result"]["items"]
        .as_array()
        .map(|a| a.as_slice())
        .unwrap_or(&[]);
    let mut product_map: HashMap<i64, Value> = HashMap::new();
    for p in items {
        if let Some(pid) = p["product_id"].as_i64() {
            product_map.insert(pid, p.clone());
        }
    }

    // Collect all product_ids for rich data fetch
    let product_ids: Vec<i64> = product_map.keys().copied().collect();

    // Fetch rich product info (name, price, stocks, commissions, etc.)
    let product_info_items = match get_product_info_list(config, &product_ids).await {
        Ok(items) => items,
        Err(e) => {
            eprintln!("Warning: get_product_info_list failed: {}", e);
            Vec::new()
        }
    };

    // Build lookup map: product_id -> rich info
    let mut product_info_map: HashMap<i64, Value> = HashMap::new();
    let mut sku_to_product_id: HashMap<i64, i64> = HashMap::new();
    for item in &product_info_items {
        if let Some(id) = item["id"].as_i64() {
            product_info_map.insert(id, item.clone());
        }
        if let (Some(id), Some(sku)) = (item["id"].as_i64(), item["sku"].as_i64()) {
            sku_to_product_id.insert(sku, id);
        }
    }

    // Fetch pricing data (net_price, commission percents per scheme)
    let product_prices_items = match get_product_info_prices(config, &product_ids).await {
        Ok(items) => items,
        Err(e) => {
            eprintln!("Warning: get_product_info_prices failed: {}", e);
            Vec::new()
        }
    };

    // Build lookup map: product_id -> price info
    let mut product_prices_map: HashMap<i64, Value> = HashMap::new();
    for item in &product_prices_items {
        if let Some(id) = item["product_id"].as_i64() {
            product_prices_map.insert(id, item.clone());
        }
    }

    let mut posting_map: HashMap<i64, Vec<Posting>> = HashMap::new();
    let data_source;

    if let Some(realization) = realization_report {
        data_source = "realization".to_string();
        let rows = realization["rows"].as_array().map(|a| a.as_slice()).unwrap_or(&[]);
        for row in rows {
            let sku = match row["item"]["sku"].as_i64() {
                Some(s) => s,
                None => continue,
            };

            let seller_price = row["seller_price_per_instance"]
                .as_f64()
                .unwrap_or(0.0);
            let commission_ratio = row["commission_ratio"].as_f64().unwrap_or(0.0);
            let commission_amount = seller_price * commission_ratio;

            // Extract full delivery_commission breakdown
            let delivery_details = serde_json::json!({
                "amount": row["delivery_commission"]["amount"].as_f64().unwrap_or(0.0),
                "bonus": row["delivery_commission"]["bonus"].as_f64().unwrap_or(0.0),
                "standard_fee": row["delivery_commission"]["standard_fee"].as_f64().unwrap_or(0.0),
                "bank_coinvestment": row["delivery_commission"]["bank_coinvestment"].as_f64().unwrap_or(0.0),
                "stars": row["delivery_commission"]["stars"].as_f64().unwrap_or(0.0),
                "total": row["delivery_commission"]["total"].as_f64().unwrap_or(0.0).abs(),
            });
            let delivery_commission = row["delivery_commission"]["total"]
                .as_f64()
                .unwrap_or(0.0)
                .abs();

            // Extract full return_commission breakdown
            let return_details = serde_json::json!({
                "amount": row["return_commission"]["amount"].as_f64().unwrap_or(0.0),
                "bonus": row["return_commission"]["bonus"].as_f64().unwrap_or(0.0),
                "standard_fee": row["return_commission"]["standard_fee"].as_f64().unwrap_or(0.0),
                "bank_coinvestment": row["return_commission"]["bank_coinvestment"].as_f64().unwrap_or(0.0),
                "stars": row["return_commission"]["stars"].as_f64().unwrap_or(0.0),
                "total": row["return_commission"]["total"].as_f64().unwrap_or(0.0).abs(),
            });
            let return_commission = row["return_commission"]["total"]
                .as_f64()
                .unwrap_or(0.0)
                .abs();

            // No services in realization report — empty vec
            let services: Vec<Value> = Vec::new();

            let posting = Posting {
                posting_number: row["order"]["posting_number"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                date: row["order"]["created_date"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                sku,
                name: row["item"]["name"].as_str().unwrap_or("").to_string(),
                offer_id: row["item"]["offer_id"].as_str().unwrap_or("").to_string(),
                seller_price_per_instance: seller_price,
                commission_amount,
                commission_ratio,
                delivery_charge: delivery_commission,
                delivery_details,
                return_charge: return_commission,
                return_details,
                services,
                net: seller_price - commission_amount - delivery_commission - return_commission,
            };

            posting_map.entry(sku).or_default().push(posting);
        }
    } else {
        data_source = "transactions".to_string();
        let operations = transactions_result["result"]["operations"]
            .as_array()
            .map(|a| a.as_slice())
            .unwrap_or(&[]);

        for op in operations {
            let op_items = op["items"].as_array().map(|a| a.as_slice()).unwrap_or(&[]);
            for item in op_items {
                let sku = match item["sku"].as_i64() {
                    Some(s) => s,
                    None => continue,
                };

                let seller_price = op["accruals_for_sale"].as_f64().unwrap_or(0.0);
                let commission_amount = (op["sale_commission"].as_f64().unwrap_or(0.0)).abs();
                let field_delivery = (op["delivery_charge"].as_f64().unwrap_or(0.0)).abs();
                let field_return = (op["return_delivery_charge"].as_f64().unwrap_or(0.0)).abs();

                let services: Vec<Value> = (op["services"].as_array().unwrap_or(&vec![]))
                    .iter()
                    .map(|svc| serde_json::json!({
                        "name": svc["name"].as_str().unwrap_or(""),
                        "price": svc["price"].as_f64().unwrap_or(0.0).abs(),
                    }))
                    .collect();

                let service_cost: f64 = services.iter().map(|s| s["price"].as_f64().unwrap_or(0.0)).sum();

                let delivery_charge = field_delivery + service_cost;
                let return_charge = field_return;

                let delivery_details = serde_json::json!({
                    "amount": field_delivery,
                    "bonus": 0.0,
                    "standard_fee": 0.0,
                    "bank_coinvestment": 0.0,
                    "stars": 0.0,
                    "total": delivery_charge,
                });
                let return_details = serde_json::json!({
                    "amount": field_return,
                    "bonus": 0.0,
                    "standard_fee": 0.0,
                    "bank_coinvestment": 0.0,
                    "stars": 0.0,
                    "total": field_return,
                });

                let posting = Posting {
                    posting_number: op["posting"]["posting_number"]
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                    date: op["operation_date"].as_str().unwrap_or("").to_string(),
                    sku,
                    name: item["name"].as_str().unwrap_or("").to_string(),
                    offer_id: item["offer_id"].as_str().unwrap_or("").to_string(),
                    seller_price_per_instance: seller_price,
                    commission_amount,
                    commission_ratio: 0.0,
                    delivery_charge,
                    delivery_details,
                    return_charge,
                    return_details,
                    services,
                    net: seller_price - commission_amount - delivery_charge - return_charge,
                };

                posting_map.entry(sku).or_default().push(posting);
            }
        }
    }

    let all_ops = transactions_result["result"]["operations"]
        .as_array()
        .map(|a| a.as_slice())
        .unwrap_or(&[]);
    let account_level_expenses = build_account_level_expenses(all_ops);
    let account_expense_total: f64 = account_level_expenses
        .cats
        .iter()
        .map(|(_, v)| v)
        .sum();

    // Extract account-level ad costs (click + order) for per-product distribution
    let mut total_ad_clicks: f64 = 0.0;
    let mut total_ad_orders: f64 = 0.0;
    for op in all_ops {
        let items = op["items"].as_array();
        if items.map_or(false, |i| !i.is_empty()) { continue; }
        let amount = op["amount"].as_f64().unwrap_or(0.0).abs();
        if amount == 0.0 { continue; }
        let op_type_name = op["operation_type_name"].as_str().unwrap_or("").to_lowercase();
        let op_type = op["operation_type"].as_str().unwrap_or("").to_lowercase();
        if op_type_name.contains("клик") || op_type_name.contains("click") {
            total_ad_clicks += amount;
        }
        if op_type_name.contains("заказ") || op_type.contains("per_order") {
            total_ad_orders += amount;
        }
    }

    let mut tree: Vec<Value> = Vec::new();
    for (sku, postings) in &posting_map {
        // Resolve product_id from SKU using info_list mapping (which has both id and sku)
        // Falls back to direct product_id match, then offer_id match
        let resolved_product_id = sku_to_product_id.get(sku).copied()
            .or_else(|| product_map.keys().find(|pid| **pid == *sku).copied())
            .or_else(|| {
                postings.first().and_then(|po| {
                    let oid = &po.offer_id;
                    if oid.is_empty() { return None; }
                    product_map.iter().find(|(_, p)|
                        p["offer_id"].as_str().map_or(false, |o| o == *oid)
                    ).map(|(pid, _)| *pid)
                })
            });

        let product = resolved_product_id.and_then(|pid| product_map.get(&pid));
        let product_info = resolved_product_id.and_then(|pid| product_info_map.get(&pid));
        let product_prices = resolved_product_id.and_then(|pid| product_prices_map.get(&pid));

        let total_revenue: f64 = postings.iter().map(|p| p.seller_price_per_instance).sum();
        let total_commission: f64 = postings.iter().map(|p| p.commission_amount).sum();
        let total_delivery: f64 = postings.iter().map(|p| p.delivery_charge).sum();
        let total_returns: f64 = postings.iter().map(|p| p.return_charge).sum();

        let expenses_by_category = build_expense_categories(all_ops, *sku);
        let service_expenses: f64 = expenses_by_category
            .cats
            .iter()
            .map(|(_, v)| v)
            .sum();

        let total_expenses = total_commission + total_delivery + total_returns + service_expenses;
        let net_profit = total_revenue - total_expenses;

        let name = match product {
            Some(p) => {
                let offer_id = p["offer_id"].as_str().unwrap_or("");
                let posted_name = postings.first().map(|po| po.name.as_str()).unwrap_or("—");
                if !offer_id.is_empty() {
                    format!("{} ({})", posted_name, offer_id)
                } else {
                    posted_name.to_string()
                }
            }
            None => {
                let posted_name = postings.first().map(|po| po.name.as_str()).unwrap_or("—");
                if posted_name != "—" {
                    posted_name.to_string()
                } else {
                    format!("SKU {}", sku)
                }
            }
        };

        let mut sorted_postings: Vec<&Posting> = postings.iter().collect();
        sorted_postings.sort_by(|a, b| b.date.cmp(&a.date));

        let product_summary = extract_product_summary(all_ops, *sku);

        tree.push(serde_json::json!({
            "sku": sku,
            "name": name,
            "offer_id": product.and_then(|p| p["offer_id"].as_str()).unwrap_or_else(|| postings.first().map_or("", |po| &po.offer_id)),
            "product_id": product.and_then(|p| p["product_id"].as_i64()).unwrap_or(*sku),
            "has_fbo_stocks": product.map_or(false, |p| p["has_fbo_stocks"].as_bool().unwrap_or(false)),
            "has_fbs_stocks": product.map_or(false, |p| p["has_fbs_stocks"].as_bool().unwrap_or(false)),
            "archived": product.map_or(false, |p| p["archived"].as_bool().unwrap_or(false)),
            "product_info": match product_info {
                Some(info) => {
                    let stocks_arr = info["stocks"]["stocks"].as_array().map(|a| a.as_slice()).unwrap_or(&[]);
                    let stocks_present: i64 = stocks_arr.iter().filter_map(|s| s["present"].as_i64()).sum();
                    let stocks_reserved: i64 = stocks_arr.iter().filter_map(|s| s["reserved"].as_i64()).sum();

                    let commissions: Vec<Value> = info["commissions"].as_array()
                        .map(|arr| arr.iter().map(|c| serde_json::json!({
                            "sale_schema": c["sale_schema"].as_str().unwrap_or(""),
                            "percent": c["percent"].as_f64().unwrap_or(0.0),
                            "delivery_amount": c["delivery_amount"].as_f64().unwrap_or(0.0),
                            "return_amount": c["return_amount"].as_f64().unwrap_or(0.0),
                            "value": c["value"].as_f64().unwrap_or(0.0),
                        })).collect())
                        .unwrap_or_default();

                    serde_json::json!({
                        "name": info["name"].as_str().unwrap_or(""),
                        "offer_id": info["offer_id"].as_str().unwrap_or(""),
                        "price": info["price"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0),
                        "old_price": info["old_price"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0),
                        "min_price": info["min_price"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0),
                        "stocks_present": stocks_present,
                        "stocks_reserved": stocks_reserved,
                        "color_index": info["price_indexes"]["color_index"].as_str().unwrap_or(""),
                        "commissions": commissions,
                        "volume_weight": info["volume_weight"].as_f64().unwrap_or(0.0),
                        "is_archived": info["is_archived"].as_bool().unwrap_or(false),
                        "is_super": info["is_super"].as_bool().unwrap_or(false),
                        "status": info["statuses"]["status"].as_str().unwrap_or(""),
                        "net_price": product_prices.map_or(0.0, |pp| pp["price"]["net_price"].as_f64().unwrap_or(0.0)),
                        "images": info["images"].as_array()
                            .map(|arr| arr.iter().take(5).filter_map(|s| s.as_str().map(String::from)).collect::<Vec<_>>())
                            .unwrap_or_default(),
                        "primary_image": info["primary_image"].as_array()
                            .and_then(|arr| arr.first())
                            .and_then(|s| s.as_str())
                            .unwrap_or("")
                            .to_string(),
                        "scheme": (|| {
                            let fbo = product.map_or(false, |p| p["has_fbo_stocks"].as_bool().unwrap_or(false));
                            let fbs = product.map_or(false, |p| p["has_fbs_stocks"].as_bool().unwrap_or(false));
                            let rfbs = product_prices.map_or(false, |pp| {
                                pp["commissions"]["sales_percent_rfbs"].as_f64().unwrap_or(0.0) > 0.0
                            });
                            match (fbo, fbs, rfbs) {
                                (true, true, _) => "FBO+FBS",
                                (true, false, true) => "FBO+rFBS",
                                (true, false, false) => "FBO",
                                (false, true, _) => "FBS",
                                (false, false, true) => "rFBS",
                                _ => "",
                            }
                        })(),
                    })
                },
                None => serde_json::json!(null),
            },
            "summary": {
                "total_quantity": postings.len(),
                "total_revenue": total_revenue,
                "total_commission": total_commission,
                "total_delivery": total_delivery,
                "total_returns": total_returns,
                "service_expenses": service_expenses,
                "expenses_cats": expenses_by_category.cats,
                "expenses_details": expenses_by_category.details,
                "total_expenses": total_expenses,
                "net_profit": net_profit,
            },
            "costs": product_summary["costs"],
            "totalRevenue": product_summary["total_revenue"],
            "totalCosts": product_summary["total_costs"],
            "netProfit": product_summary["net_profit"],
            "profitPerUnit": product_summary["profit_per_unit"],
            "totalQuantity": product_summary["total_quantity"],
            "postings": sorted_postings,
        }));
    }

    tree.sort_by(|a, b| {
        b["summary"]["net_profit"]
            .as_f64()
            .partial_cmp(&a["summary"]["net_profit"].as_f64())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Distribute account-level ad costs (click/order) proportionally by product revenue,
    // then further split across each product's postings proportionally by posting revenue.
    if (total_ad_clicks > 0.0 || total_ad_orders > 0.0) && !tree.is_empty() {
        let total_rev: f64 = tree.iter()
            .map(|p| p["summary"]["total_revenue"].as_f64().unwrap_or(0.0))
            .sum();
        if total_rev > 0.0 {
            for entry in &mut tree {
                let rev = entry["summary"]["total_revenue"].as_f64().unwrap_or(0.0);
                if rev <= 0.0 { continue; }
                let share = rev / total_rev;
                let product_click = total_ad_clicks * share;
                let product_order = total_ad_orders * share;
                if let Some(costs_obj) = entry["costs"].as_object_mut() {
                    if total_ad_clicks > 0.0 {
                        costs_obj.insert("pay_per_click".to_string(), serde_json::json!(product_click));
                    }
                    if total_ad_orders > 0.0 {
                        costs_obj.insert("pay_per_order".to_string(), serde_json::json!(product_order));
                    }
                }
                // Split ad costs evenly across all postings of this product
                if let Some(postings) = entry["postings"].as_array_mut() {
                    let count = postings.len() as f64;
                    if count > 0.0 {
                        for posting in postings {
                            if total_ad_clicks > 0.0 {
                                posting["ad_click_cost"] = serde_json::json!(product_click / count);
                            }
                            if total_ad_orders > 0.0 {
                                posting["ad_order_cost"] = serde_json::json!(product_order / count);
                            }
                        }
                    }
                }
            }
        }
    }

    let product_revenue: f64 = tree.iter().map(|p| p["summary"]["total_revenue"].as_f64().unwrap_or(0.0)).sum();
    let product_expenses: f64 = tree.iter().map(|p| p["summary"]["total_expenses"].as_f64().unwrap_or(0.0)).sum();

    Ok(serde_json::json!({
        "month": month,
        "year": year,
        "period": { "from": from, "to": to },
        "data_source": data_source,
        "totals": {
            "total_revenue": product_revenue,
            "product_expenses": product_expenses,
            "account_expenses": account_expense_total,
            "total_expenses": product_expenses + account_expense_total,
            "net_profit": product_revenue - product_expenses - account_expense_total,
            "total_quantity": tree.iter().map(|p| p["summary"]["total_quantity"].as_u64().unwrap_or(0)).sum::<u64>(),
            "product_count": tree.len(),
        },
        "account_expenses": {
            "cats": account_level_expenses.cats,
            "details": account_level_expenses.details,
        },
        "products": tree,
    }))
}
