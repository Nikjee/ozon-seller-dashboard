# Draft: posting-constructor

## Status
- [x] Explored codebase
- [x] Mapped Ozon API endpoints
- [x] Identified patterns (Rust commands, composables, i18n, tabs)
- [x] User scope confirmed: full multi-step wizard
- [x] Approval gate passed → wrote `.omo/plans/posting-constructor.md` on 2026-06-20

## Intent
**CLEAR** — outcome ("FBO posting constructor wizard") is known. One scope question asked and answered.

## Decisions
- **Supply type**: Direct supply (прямая поставка) — simplest, covers most common case. Crossdock and multi-cluster deferred.
- **Product search**: Use existing `/v3/product/list` + `/v3/product/info/list` for product lookup by name/offer_id.
- **Wizard UI**: Multi-step stepper (step 1-7), not a single form. Each step is a distinct "page" within the component.
- **i18n**: All user-facing strings in flat keys under `postingConstructor.*`.
- **Tests**: Vitest + jsdom, mock composable + naive-ui stubs (same pattern as `AnalyticsDashboard.spec.js`).
- **Backend module**: New `src-tauri/src/supply.rs` for all FBO supply request logic.
- **Pagination on supply list**: Cursor-based, same pattern as `get_fbo_posting_totals`.

## Ozon API Endpoints Used

| Step | Endpoint | Rust fn | Notes |
|------|----------|---------|-------|
| 1 | `POST /v1/supplier/available_warehouses` | `get_available_warehouses` | Check warehouse load |
| 2 | `POST /v1/cluster/list` | `get_cluster_list` | Get clusters + their warehouses |
| 3 | `POST /v1/draft/create` | `create_supply_draft` | Create draft with items (SKU + qty) |
| 4 | `POST /v1/draft/create/info` | `get_draft_info` | Poll for draft calc status |
| 5 | `POST /v1/draft/timeslot/info` | `get_timeslots` | Get available timeslots |
| 6 | `POST /v1/draft/supply/create` | `create_supply_from_draft` | Create supply from draft |
| 7 | `POST /v1/draft/supply/create/status` | `get_supply_create_status` | Verify result |
| Extra | `POST /v3/supply-order/list` | `get_supply_orders` | List created supply orders |
| Extra | `POST /v3/supply-order/get` | `get_supply_order_details` | Get order details |
| Extra | `POST /v1/supply-order/bundle` | `get_supply_order_bundle` | Get supply contents |
| Extra | `POST /v3/product/list` + `/v3/product/info/list` | reuse existing `ozon::get_products` + `ozon::get_product_info_list` | Product lookup for wizard |

## Files to Create/Modify

| File | Action | Purpose |
|------|--------|---------|
| `src-tauri/src/supply.rs` | **CREATE** | All FBO supply request API calls |
| `src-tauri/src/lib.rs` | **EDIT** | Add `mod supply`, register commands, add handler fns |
| `src/composables/usePostingConstructor.js` | **CREATE** | Composable: wizard state + API calls |
| `src/components/PostingConstructor.vue` | **CREATE** | Wizard UI component |
| `src/components/__tests__/PostingConstructor.spec.js` | **CREATE** | Vitest spec |
| `src/App.vue` | **EDIT** | Add `n-tab-pane` + lazy activation |
| `src/composables/useI18n.js` | **EDIT** | Add `postingConstructor.*` i18n keys |

## Must-NOT-Have (out of scope for this plan)
- Crossdock supply type
- Multi-cluster supply type
- Driver pass creation (`/v1/supply-order/pass/create`)
- Cargo space management (`/v1/cargoes/create`, labels)
- FBP endpoints (beta, different flow)
- Supply order cancellation
- Editing existing drafts

## Gates
- [x] Codebase explored
- [x] API mapped
- [x] Scope confirmed
- [x] Approval: user confirmed → wrote plan
- [x] Momus review: APPROVE-WITH-CHANGES — 3 issues fixed
  - Major: Added `list_products` Tauri command (Task 1 fn 8, Task 2 cmd 8). Updated Task 4 `searchProducts` to use it with client-side filtering.
  - Minor: Fixed Task 2 title (7→8 commands). Cut `get_supply_orders`/`get_supply_order_details` (YAGNI).
  - Nit: Added polling params (1s interval, 30 max retries).
