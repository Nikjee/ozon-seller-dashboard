use crate::config::OzonConfig;
use serde_json::Value;

const BASE_URL: &str = "https://api-seller.ozon.ru";

pub(crate) fn build_headers(config: &OzonConfig) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Client-Id",
        config.ozon_client_id.parse().unwrap(),
    );
    headers.insert(
        "Api-Key",
        config.ozon_api_key.parse().unwrap(),
    );
    headers.insert(
        "Content-Type",
        "application/json".parse().unwrap(),
    );
    headers
}

pub(crate) async fn ozon_request(
    config: &OzonConfig,
    path: &str,
    method: &str,
    body: Option<&Value>,
) -> Result<Value, String> {
    let client = reqwest::Client::new();
    let url = format!("{}{}", BASE_URL, path);
    let headers = build_headers(config);

    let mut req = match method {
        "GET" => client.get(&url),
        _ => client.post(&url),
    };
    req = req.headers(headers);

    if let Some(b) = body {
        req = req.json(b);
    }

    let mut res = req
        .send()
        .await
        .map_err(|e| format!("Ozon API request failed: {}", e))?;
    if res.status() == 429 {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        let mut retry = match method {
            "GET" => client.get(&url),
            _ => client.post(&url),
        };
        retry = retry.headers(build_headers(config));
        if let Some(b) = body {
            retry = retry.json(b);
        }
        res = retry.send().await.map_err(|e| format!("Ozon API request failed: {}", e))?;
    }
    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        return Err(format!(
            "Ozon API {} returned {}: {}",
            path, status, body
        ));
    }
    res.json()
        .await
        .map_err(|e| format!("Ozon API deserialize failed: {}", e))
}

pub async fn get_products(
    config: &OzonConfig,
    limit: i64,
) -> Result<(Vec<Value>, i64), String> {
    let mut all_items: Vec<Value> = Vec::new();
    let mut total: i64 = 0;
    let mut last_id = String::new();
    loop {
        let body = serde_json::json!({
            "filter": { "visibility": "ALL" },
            "last_id": last_id,
            "limit": limit,
        });
        let response = ozon_request(config, "/v3/product/list", "POST", Some(&body)).await?;
        if let Some(items) = response["result"]["items"].as_array() {
            all_items.extend(items.clone());
        }
        if let Some(t) = response["result"]["total"].as_i64() {
            total = t;
        }
        last_id = response["result"]["last_id"].as_str().unwrap_or("").to_string();
        if last_id.is_empty() { break; }
    }
    Ok((all_items, total))
}

/// Fetch rich product data for a batch of product IDs via /v3/product/info/list.
/// API limit: max 1000 product_ids per request. We batch in chunks of 1000.
pub async fn get_product_info_list(
    config: &OzonConfig,
    product_ids: &[i64],
) -> Result<Vec<Value>, String> {
    let mut all_items: Vec<Value> = Vec::new();

    for chunk in product_ids.chunks(1000) {
        let body = serde_json::json!({
            "product_id": chunk,
        });
        let response = ozon_request(config, "/v3/product/info/list", "POST", Some(&body)).await?;
        if let Some(items) = response["items"].as_array() {
            all_items.extend(items.clone());
        }
    }

    Ok(all_items)
}

/// Fetch pricing info (cost price, commission %) for a batch of product IDs via /v5/product/info/prices.
/// API limit: max 1000 product_ids per request. Uses cursor pagination.
pub async fn get_product_info_prices(
    config: &OzonConfig,
    product_ids: &[i64],
) -> Result<Vec<Value>, String> {
    let mut all_items: Vec<Value> = Vec::new();

    for chunk in product_ids.chunks(1000) {
        let mut cursor = String::new();
        loop {
            let body = serde_json::json!({
                "filter": { "product_id": chunk },
                "limit": 1000,
                "cursor": cursor,
            });
            let response = ozon_request(config, "/v5/product/info/prices", "POST", Some(&body)).await?;
            if let Some(items) = response["items"].as_array() {
                all_items.extend(items.clone());
            }
            cursor = response["cursor"].as_str().unwrap_or("").to_string();
            if cursor.is_empty() {
                break;
            }
        }
    }

    Ok(all_items)
}

pub async fn get_realization_report(
    config: &OzonConfig,
    month: u32,
    year: i32,
) -> Result<Option<Value>, String> {
    let body = serde_json::json!({ "month": month, "year": year });
    let res = reqwest::Client::new()
        .post(&format!("{}/v1/finance/realization/posting", BASE_URL))
        .headers(build_headers(config))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ozon API request failed: {}", e))?;

    let status = res.status();
    if !status.is_success() {
        let text = res.text().await.unwrap_or_default();
        if status == reqwest::StatusCode::NOT_FOUND && text.contains("Report was not found") {
            return Ok(None);
        }
        return Err(format!("Ozon API /v1/finance/realization/posting returned {}: {}", status, text));
    }

    res.json().await.map(Some).map_err(|e| format!("Ozon API deserialize failed: {}", e))
}

pub async fn get_finance_transactions(
    config: &OzonConfig,
    date_from: &str,
    date_to: &str,
) -> Result<Value, String> {
    let page_size = 1000;
    let mut page = 1;
    let mut all_operations: Vec<Value> = Vec::new();

    let from_iso = format!("{}T00:00:00.000Z", date_from);
    let to_iso = format!("{}T23:59:59.999Z", date_to);
    loop {
        let body = serde_json::json!({
            "filter": {
                "date": {
                    "from": &from_iso,
                    "to": &to_iso,
                }
            },
            "page": page,
            "page_size": page_size,
        });

        let response = ozon_request(
            config,
            "/v3/finance/transaction/list",
            "POST",
            Some(&body),
        )
        .await?;

        if let Some(ops) = response["result"]["operations"].as_array() {
            all_operations.extend(ops.clone());
            if ops.len() < page_size {
                break;
            }
        } else {
            break;
        }

        page += 1;
    }

    Ok(serde_json::json!({
        "result": {
            "operations": all_operations,
        }
    }))
}

pub async fn get_stock_on_warehouses(
    config: &OzonConfig,
    limit: i64,
    offset: i64,
    warehouse_type: &str,
) -> Result<Value, String> {
    let body = serde_json::json!({
        "limit": limit,
        "offset": offset,
        "warehouse_type": warehouse_type,
    });
    ozon_request(config, "/v2/analytics/stock_on_warehouses", "POST", Some(&body)).await
}

pub async fn get_analytics_stocks(
    config: &OzonConfig,
    skus: Vec<i64>,
) -> Result<Value, String> {
    let body = serde_json::json!({
        "skus": skus,
    });
    ozon_request(config, "/v1/analytics/stocks", "POST", Some(&body)).await
}

/// Get product stock info with cursor pagination from /v4/product/info/stocks
pub async fn get_product_info_stocks(config: &OzonConfig) -> Result<Vec<Value>, String> {
    let mut all_items = Vec::new();
    let mut cursor = String::new();

    loop {
        let body = serde_json::json!({
            "limit": 100,
            "cursor": cursor,
            "filter": {"visibility": "ALL"},
        });

        let response = ozon_request(
            config,
            "/v4/product/info/stocks",
            "POST",
            Some(&body),
        )
        .await?;

        if let Some(items) = response["items"].as_array() {
            all_items.extend(items.clone());
        }

        // Check if there's a next cursor — API returns cursor in response
        cursor = response["cursor"]
            .as_str()
            .unwrap_or("")
            .to_string();

        if cursor.is_empty() {
            break;
        }
    }

    Ok(all_items)
}

pub async fn get_stocks_turnover(
    config: &OzonConfig,
    sku: i64,
    limit: i64,
    offset: i64,
) -> Result<Value, String> {
    let body = serde_json::json!({
        "sku": sku,
        "limit": limit,
        "offset": offset,
    });
    ozon_request(config, "/v1/analytics/turnover/stocks", "POST", Some(&body)).await
}

pub async fn get_finance_totals(
    config: &OzonConfig,
    date_from: &str,
    date_to: &str,
) -> Result<Value, String> {
    // Ozon API requires ISO 8601 format with time component:
    // "2019-11-25T00:00:00.000Z" — and the "date" object must be
    // at the top level (not wrapped in a "filter" key).
    let from_iso = format!("{}T00:00:00.000Z", date_from);
    let to_iso = format!("{}T23:59:59.999Z", date_to);
    let body = serde_json::json!({
        "date": {
            "from": from_iso,
            "to": to_iso,
        }
    });
    ozon_request(config, "/v3/finance/transaction/totals", "POST", Some(&body)).await
}

pub async fn get_fbo_posting_totals(
    config: &OzonConfig,
    month: u32,
    year: i32,
) -> Result<Value, String> {
    let padded = format!("{:02}", month);
    let from_iso = format!("{}-{}-01T00:00:00.000Z", year, padded);
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => { if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 } }
        _ => 30,
    };
    let to_iso = format!("{}-{}-{:02}T23:59:59.999Z", year, padded, days_in_month);

    let mut total = 0u64;
    let mut delivered = 0u64;
    let mut total_products = 0u64;
    let mut cursor = String::new();

    loop {
        let body = serde_json::json!({
            "cursor": cursor,
            "filter": {
                "since": from_iso,
                "to": to_iso,
            },
            "limit": 100,
            "dir": "ASC",
        });
        let resp = ozon_request(config, "/v3/posting/fbo/list", "POST", Some(&body)).await?;
        if let Some(postings) = resp["postings"].as_array() {
            for p in postings {
                total += 1;
                if p["status"].as_str() == Some("delivered") {
                    delivered += 1;
                }
                if let Some(products) = p["products"].as_array() {
                    for pr in products {
                        total_products += pr["quantity"].as_u64().unwrap_or(1);
                    }
                }
            }
        }
        let has_next = resp["has_next"].as_bool().unwrap_or(false);
        if has_next {
            cursor = resp["cursor"].as_str().unwrap_or("").to_string();
            if cursor.is_empty() { break; }
        } else {
            break;
        }
    }

    Ok(serde_json::json!({
        "total_postings": total,
        "delivered_postings": delivered,
        "total_products": total_products,
    }))
}

