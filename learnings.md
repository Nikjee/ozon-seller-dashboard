# Learnings

- Added four Ozon API wrappers in `src-tauri/src/ozon.rs` using the existing `ozon_request` helper pattern.
- Enhancement plan confirms the backend should stay simple: add direct wrappers, no new client abstraction.
- `useAnalyticsDashboard` follows the `useDashboard` pattern exactly: local refs, `load()`, `refresh()`, and watch-driven reloads.
- Analytics command signatures in `src-tauri/src/lib.rs` accept `skus`, so the composable currently targets `get_stock_analytics` and derives safe computed aggregates from returned product data.
