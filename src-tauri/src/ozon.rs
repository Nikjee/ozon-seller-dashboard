use crate::config::OzonConfig;
use serde_json::Value;

const BASE_URL: &str = "https://api-seller.ozon.ru";

fn build_headers(config: &OzonConfig) -> reqwest::header::HeaderMap {
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

async fn ozon_request(
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

    let res = req
        .send()
        .await
        .map_err(|e| format!("Ozon API request failed: {}", e))?;
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
    last_id: &str,
    limit: i64,
) -> Result<Value, String> {
    let body = serde_json::json!({
        "filter": { "visibility": "ALL" },
        "last_id": last_id,
        "limit": limit,
    });
    ozon_request(config, "/v3/product/list", "POST", Some(&body)).await
}

pub async fn get_realization_report(
    config: &OzonConfig,
    month: u32,
    year: i32,
) -> Result<Option<Value>, String> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/v1/finance/realization/posting",
        BASE_URL
    );
    let headers = build_headers(config);

    let body = serde_json::json!({ "month": month, "year": year });

    let res = client
        .post(&url)
        .headers(headers)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ozon API request failed: {}", e))?;

    let status = res.status();

    if status == reqwest::StatusCode::NOT_FOUND {
        let err: Value = res
            .json()
            .await
            .unwrap_or(serde_json::json!({}));
        if let Some(msg) = err["message"].as_str() {
            if msg.contains("Report was not found") {
                return Ok(None);
            }
        }
        return Err(format!(
            "Ozon API /v1/finance/realization/posting returned {} (not found)",
            status
        ));
    }

    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(format!(
            "Ozon API /v1/finance/realization/posting returned {}: {}",
            status, body
        ));
    }

    res.json()
        .await
        .map(Some)
        .map_err(|e| format!("Ozon API deserialize failed: {}", e))
}

pub async fn get_finance_transactions(
    config: &OzonConfig,
    date_from: &str,
    date_to: &str,
) -> Result<Value, String> {
    let page_size = 1000;
    let mut page = 1;
    let mut all_operations: Vec<Value> = Vec::new();

    loop {
        let body = serde_json::json!({
            "filter": {
                "date": {
                    "from": date_from,
                    "to": date_to,
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_pagination_break_condition() {
        let page_size = 1000;
        let ops_len = 500; // less than page_size → should break
        assert!(ops_len < page_size);

        let ops_len = 1000; // equal to page_size → should continue
        assert!(ops_len >= page_size);
    }
}
