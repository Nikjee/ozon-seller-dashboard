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
