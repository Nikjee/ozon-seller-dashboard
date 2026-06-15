use crate::config::OzonConfig;
use crate::expenses::{build_account_level_expenses, build_expense_categories};
use crate::ozon::{get_finance_transactions, get_products, get_realization_report};
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
    pub delivery_charge: f64,
    pub return_charge: f64,
    #[serde(rename = "net")]
    pub net: f64,
}

fn month_date_range(month: u32, year: i32) -> (String, String) {
    let padded = format!("{:02}", month);
    let from = format!("{}-{}-01T00:00:00.000Z", year, padded);
    let last_day = NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or(NaiveDate::from_ymd_opt(year, month, 28).unwrap())
        .pred_opt()
        .unwrap()
        .day();
    let to = format!(
        "{}-{}-{:02}T23:59:59.999Z",
        year, padded, last_day
    );
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

            let delivery_commission = row["delivery_commission"]["total"]
                .as_f64()
                .unwrap_or(0.0)
                .abs();
            let return_commission = row["return_commission"]["total"]
                .as_f64()
                .unwrap_or(0.0)
                .abs();
            let seller_price = row["seller_price_per_instance"]
                .as_f64()
                .unwrap_or(0.0);
            let commission_ratio = row["commission_ratio"].as_f64().unwrap_or(0.0);
            let commission_amount = seller_price * commission_ratio;

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
                delivery_charge: delivery_commission,
                return_charge: return_commission,
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

                let service_cost: f64 = (op["services"].as_array().unwrap_or(&vec![]))
                    .iter()
                    .map(|svc| (svc["price"].as_f64().unwrap_or(0.0)).abs())
                    .sum();
                let delivery_charge = field_delivery + service_cost;
                let return_charge = field_return;

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
                    delivery_charge,
                    return_charge,
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

    let mut tree: Vec<Value> = Vec::new();
    for (sku, postings) in &posting_map {
        let product = product_map.values().find(|p| {
            p["product_id"].as_i64().map_or(false, |pid| pid == *sku)
                || p["offer_id"].as_str().map_or(false, |oid| {
                    postings.first().map_or(false, |po| po.offer_id == oid)
                })
        });

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

        tree.push(serde_json::json!({
            "sku": sku,
            "name": name,
            "offer_id": product.and_then(|p| p["offer_id"].as_str()).unwrap_or_else(|| postings.first().map_or("", |po| &po.offer_id)),
            "product_id": product.and_then(|p| p["product_id"].as_i64()).unwrap_or(*sku),
            "has_fbo_stocks": product.map_or(false, |p| p["has_fbo_stocks"].as_bool().unwrap_or(false)),
            "has_fbs_stocks": product.map_or(false, |p| p["has_fbs_stocks"].as_bool().unwrap_or(false)),
            "archived": product.map_or(false, |p| p["archived"].as_bool().unwrap_or(false)),
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
            "postings": sorted_postings,
        }));
    }

    tree.sort_by(|a, b| {
        b["summary"]["net_profit"]
            .as_f64()
            .partial_cmp(&a["summary"]["net_profit"].as_f64())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

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
