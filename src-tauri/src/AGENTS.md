# Rust Backend (src-tauri/src/)

Tauri 2.0 async command layer over Ozon Seller API. All API calls, data processing, and business logic live here.

## MODULE INDEX

| Module | Role |
|--------|------|
| `lib.rs` | App bootstrap, `AppState` (Mutex<Option<OzonConfig>>), all `#[tauri::command]` handlers, `invoke_handler` registration |
| `config.rs` | `OzonConfig` load/save from `config.json` next to executable |
| `ozon.rs` | HTTP client: `ozon_request()` helper, all Ozon API calls |
| `analytics.rs` | Stock analytics: warehouse reports, SKU analytics, turnover, merged dashboard data |
| `dashboard.rs` | Monthly dashboard: realization report + transactions, product summaries, expense aggregation |
| `uniteconomy.rs` | Cost extraction: `extract_product_costs()` (20+ keyword-matched categories), `extract_product_summary()` |
| `expenses.rs` | Expense categorization: `ExpenseCategory` enum (ad/storage/logistics/compensation/other), `build_expense_categories()`, `build_account_level_expenses()` |

## KEY PATTERNS

- **Tauri command**: `async fn cmd_name(args, State<'_, AppState>) -> Result<Value, String>` — extract config from Mutex, delegate to module fn, return serde_json::Value
- **Config extraction**: every command clones `OzonConfig` from `state.config.lock()` before async work (Mutex not Send across await)
- **Error handling**: `Result<T, String>` everywhere; `map_err(|e| format!(...))` converts reqwest/serde errors
- **Dynamic shapes**: `serde_json::Value` for all API responses, no typed Rust structs for Ozon data
- **Data pipeline**: fetch raw → merge/process (HashMap, keyword matching) → return `serde_json::json!({})` to frontend

## API INTEGRATION

| Endpoint | Module | Notes |
|----------|--------|-------|
| `/v3/product/list` | `ozon::get_products` | Paginated product list |
| `/v4/product/info/stocks` | `ozon::get_product_info_stocks` | Cursor-paginated stock info |
| `/v1/analytics/stocks` | `ozon::get_analytics_stocks` | SKU-based analytics |
| `/v2/analytics/stock_on_warehouses` | `ozon::get_stock_on_warehouses` | Warehouse stock |
| `/v1/analytics/turnover/stocks` | `ozon::get_stocks_turnover` | Turnover per SKU |
| `/v1/finance/realization/posting` | `ozon::get_realization_report` | Returns `Option<Value>` (404 = no report) |
| `/v3/finance/transaction/list` | `ozon::get_finance_transactions` | Auto-paginated (page_size=1000) |
| `/v3/finance/transaction/totals` | `ozon::get_finance_totals` | ISO 8601 dates required |

Auth: `Client-Id` + `Api-Key` headers (not OAuth). Base URL: `https://api-seller.ozon.ru`.

## COMMANDS

`check_config`, `save_config`, `get_dashboard_summary`, `get_updater_token`, `get_stock_on_warehouses`, `get_analytics_stocks`, `get_stocks_turnover`, `get_finance_totals`, `get_stock_report`, `get_stock_analytics`, `get_turnover_data`, `get_analytics_dashboard_data`

## NOTES

- `dashboard.rs` uses `tokio::try_join!` for parallel API calls (products + realization + transactions)
- `uniteconomy.rs` matches Russian keywords (`комисс`, `логистик`, `хранени`) for cost categorization
- `expenses.rs` splits product-level vs account-level expenses by checking `items` array emptiness
- `analytics.rs` two-stage fetch: product stocks → extract SKUs → analytics merge
- `main.rs` is a thin entrypoint calling `app_lib::run()`