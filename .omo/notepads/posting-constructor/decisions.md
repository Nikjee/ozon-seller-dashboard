# Decisions — posting-constructor

## Architecture
- New Rust module: `src-tauri/src/supply.rs` (8 functions)
- New composable: `src/composables/usePostingConstructor.js`
- New component: `src/components/PostingConstructor.vue` (single SFC, 7 steps)
- New tab: `posting-constructor` in App.vue with lazy activation
- Product search: `list_products` Tauri command → client-side filter (no text search API)

## Scope Boundaries
- Direct supply only (no crossdock, no multi-cluster)
- No driver pass, no cargo space, no FBP endpoints
- No supply order list/detail viewing (YAGNI for v1)
- Polling: 1s interval, max 30 retries (30s timeout)

## Momus Review Fixes Applied
- Added `list_products` command (was missing product search path)
- Fixed command count: 8 (not 7 or 9)
- Cut `get_supply_orders`/`get_supply_order_details` (YAGNI)
- Added polling parameters (1s, 30 retries)
