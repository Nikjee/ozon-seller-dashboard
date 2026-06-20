# Learnings — posting-constructor

## Project Patterns
- Rust backend: `ozon_request()` helper in `ozon.rs`, `build_headers()` for auth, `serde_json::json!` for bodies
- Tauri commands: `#[tauri::command]` async fn, extract config via `get_config(&state)`, `.map_err(|e| e.to_string())`
- Composables: `ref()` state, `invoke('cmd', args)`, return `{ data, loading, error, refresh }`
- i18n: flat dot-notation keys, `t(key)` returns key as fallback
- Components: `<script setup>`, naive-ui imports, BEM-like classes, Catppuccin CSS vars
- Tests: Vitest + jsdom, vi.mock composables, stub naive-ui components
- Tab navigation: n-tabs with lazy activation via tracking refs + watch

## API Endpoints (FBO Supply)
- POST /v1/supplier/available_warehouses — no body
- POST /v1/cluster/list — body: {cluster_ids: [], cluster_type: "CLUSTER_TYPE_OZON"}
- POST /v1/draft/create — body: {cluster_ids, items: [{sku, quantity}], ...}
- POST /v1/draft/create/info — body: {operation_id}
- POST /v1/draft/timeslot/info — body: {draft_id, warehouse_ids, date_from, date_to}
- POST /v1/draft/supply/create — body: {draft_id, warehouse_id, timeslot}
- POST /v1/draft/supply/create/status — body: {operation_id}

## Safety Constraint
- NO real Ozon API calls during testing
- All tests mock the composable layer
- Rust verified by compilation only
