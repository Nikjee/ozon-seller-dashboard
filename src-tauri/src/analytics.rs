use crate::config::OzonConfig;
use crate::ozon;
use serde_json::Value;

/// Get stock report organized by warehouse
pub async fn get_stock_report(config: &OzonConfig) -> Result<Value, String> {
    let limit = 1000;
    let offset = 0;
    let wh_data = ozon::get_stock_on_warehouses(config, limit, offset, "ALL").await?;
    // Structure response with warehouse-level breakdown
    Ok(wh_data)
}

/// Get stock analytics for specific SKUs
pub async fn get_stock_analytics(config: &OzonConfig, skus: &[i64]) -> Result<Value, String> {
    let data = ozon::get_analytics_stocks(config, skus.to_vec()).await?;
    Ok(data)
}

/// Get turnover data for specific SKUs
pub async fn get_turnover_data(config: &OzonConfig, skus: &[i64]) -> Result<Value, String> {
    // For now just get turnover for first SKU (pagination handled in caller)
    let sku = skus.first().copied().unwrap_or(0);
    let data = ozon::get_stocks_turnover(config, sku, 1000, 0).await?;
    Ok(data)
}
