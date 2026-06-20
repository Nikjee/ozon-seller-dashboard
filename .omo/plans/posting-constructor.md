# posting-constructor - Work Plan

## TL;DR (For humans)

**What you'll get:** A new "Заявки на поставку" tab with a 7-step wizard to create FBO supply requests (direct supply). Pick a warehouse, pick products with quantities, choose a delivery timeslot, and submit — all inside the dashboard.

**Why this approach:** Full multi-step wizard matches the Ozon API flow exactly — each API call becomes one step with loading/error states. Reuses the existing `ozon_request()` helper and composable/component patterns. Direct supply only for v1 (simplest, most common).

**What it will NOT do:** Crossdock or multi-cluster supply types, driver passes, cargo space management, editing existing drafts.

**Effort:** Medium
**Risk:** Low - follows established patterns, direct supply only, no FBP beta endpoints
**Decisions to sanity-check:** Step ordering (does the logical flow match seller expectations), timeout values for async operations (draft creation can take seconds)

Your next move: Approve the plan, then run `$start-work` to execute.

---

> TL;DR (machine): Medium effort, Low risk. New Rust module `supply.rs` (9 API fns) + composable `usePostingConstructor.js` + `PostingConstructor.vue` (7-step wizard) + new tab. Direct supply only.

## Scope
### Must have
- 7-step FBO direct supply wizard
- Warehouse availability check (Step 1)
- Cluster selection (Step 2)
- Product search + quantity entry (Step 3)
- Draft creation with status polling (Step 4)
- Timeslot selection (Step 5)
- Supply creation from draft with status polling (Step 6)
- Result display with links to supply list (Step 7)
- i18n: all strings in ru + en
- Lazy-activated tab (like stocks/totals/analytics)
- Vitest spec covering happy path, loading, error, empty states

### Must NOT have (guardrails, anti-slop, scope boundaries)
- NO crossdock or multi-cluster supply types
- NO driver pass creation
- NO cargo space management
- NO FBP beta endpoints
- NO supply order cancellation
- NO editing existing drafts
- NO supply order list/detail viewing beyond result IDs (YAGNI for v1)
- NO real Ozon API calls during testing/QA — all tests mock the composable, never invoke Tauri commands against a live API
- NO automated integration tests that hit Ozon endpoints — all verification is via mocked Vitest specs
- NO Vue Router (continue using n-tabs)
- NO new npm dependencies
- NO runtime type guards (pure JS)
- NO as any, empty catch, or @ts-ignore

## Verification strategy
> Zero human intervention - all verification is agent-executed.
> ⚠️ SAFETY CONSTRAINT: NO real Ozon API calls during testing/QA. All tests mock the composable. Rust verified by compilation only.
- Test decision: tests-after (component test mocks composable, never invokes real Tauri commands)
- Framework: Vitest + jsdom + @vue/test-utils
- Evidence: .omo/evidence/task-N-posting-constructor.txt

## Execution strategy
### Parallel execution waves
| Wave | Tasks | Description |
|------|-------|-------------|
| 1 | 1, 3 | Backend API fns + i18n keys (fully parallel) |
| 2 | 2, 4 | Register Tauri commands + composable (parallel, both need Task 1) |
| 3 | 5 | Wizard component UI (needs 3, 4) |
| 4 | 6, 7 | App.vue wiring + tests (both need 5) |

### Dependency matrix
| Todo | Depends on | Blocks | Can parallelize with |
| --- | --- | --- | --- |
| 1 (supply.rs) | — | 2, 4 | 3 |
| 2 (lib.rs) | 1 | — | 4 |
| 3 (i18n keys) | — | 5 | 1 |
| 4 (composable) | 1, 2 | 5 | — |
| 5 (PostingConstructor.vue) | 3, 4 | 6, 7 | — |
| 6 (App.vue tab) | 5 | — | 7 |
| 7 (PostingConstructor.spec.js) | 5 | — | 6 |

## Todos
> Implementation + Test = ONE todo. Never separate.
<!-- APPEND TASK BATCHES BELOW THIS LINE WITH edit/apply_patch - never rewrite the headers above. -->
- [x] 1. **src-tauri/src/supply.rs** — Ozon API functions for FBO supply request workflow
  What to do / Must NOT do:
  Create new Rust module `supply.rs` with 9 async functions. Each function takes `&OzonConfig` and optional parameters, calls `ozon_request()`, returns `Result<Value, String>`. Use `serde_json::json!` for request bodies — no structs. Follow the exact patterns from `ozon.rs` (same BASE_URL, same error handling).
  
  Functions to implement:
  1. `get_available_warehouses(config)` → `POST /v1/supplier/available_warehouses` (no body)
  2. `get_cluster_list(config)` → `POST /v1/cluster/list` with body `{"cluster_ids": [], "cluster_type": "CLUSTER_TYPE_OZON"}`
  3. `create_supply_draft(config, cluster_ids, items, drop_off_warehouse_id, supply_type)` → `POST /v1/draft/create` with items (array of `{sku, quantity}`), cluster_ids, optional drop_off_point_warehouse_id, supply type
  4. `get_draft_info(config, operation_id)` → `POST /v1/draft/create/info` with `{"operation_id": "..."}`
  5. `get_timeslots(config, draft_id, warehouse_ids, date_from, date_to)` → `POST /v1/draft/timeslot/info`
  6. `create_supply_from_draft(config, draft_id, warehouse_id, timeslot)` → `POST /v1/draft/supply/create`
  7. `get_supply_create_status(config, operation_id)` → `POST /v1/draft/supply/create/status`
  8. `list_products(config)` → calls existing `ozon::get_products(&cfg, 1000)` to expose product listing as a Tauri command. Returns `serde_json::json!({"products": items, "total": total})`.
    
  Must NOT do: NO structs/enums for API models. NO FBP endpoints. NO crossdock logic. NO driver pass logic. NO cargo space logic. NO get_supply_orders / get_supply_order_details (YAGNI for v1).
  
  Parallelization: Wave 1 | Blocked by: — | Blocks: 2, 4
  References:
  - `src-tauri/src/ozon.rs` (lines 1-378) — exact pattern: `ozon_request()`, `build_headers()`, JSON body construction, error handling
  - `src-tauri/src/ozon.rs:72-96` — pagination pattern (cursor loop)
  - `src-tauri/src/ozon.rs:320-378` — cursor/limit pagination for posting list (same pattern for supply-order/list)
  - Ozon API: `POST /v1/supplier/available_warehouses`, `POST /v1/cluster/list`, `POST /v1/draft/create`, `POST /v1/draft/create/info`, `POST /v1/draft/timeslot/info`, `POST /v1/draft/supply/create`, `POST /v1/draft/supply/create/status`, `POST /v3/supply-order/list`, `POST /v3/supply-order/get`
  Acceptance criteria: `cd src-tauri && cargo build` compiles without errors
  QA scenarios: build check only (no runtime in Tauri backend tests)
  Commit: Y | feat(backend): add FBO supply request API functions

- [x] 2. **src-tauri/src/lib.rs** — Register supply module + 8 Tauri commands
  What to do / Must NOT do:
  1. Add `mod supply;` after the existing `mod` declarations (~line 6)
  2. Add 8 Tauri command functions that delegate to `supply::` functions (same pattern as existing commands in lib.rs: extract config via `get_config()`, call supply function, `.map_err()`)
  
  Commands to add:
  - `get_available_warehouses(state)` → `supply::get_available_warehouses(&cfg)`
  - `get_cluster_list(state)` → `supply::get_cluster_list(&cfg)`
  - `create_supply_draft(cluster_ids: Vec<i64>, items: String, state)` → `supply::create_supply_draft(...)` (accept JSON string for items array, parse with serde_json)
  - `get_draft_info(operation_id: String, state)` → `supply::get_draft_info(&cfg, &operation_id)`
  - `get_timeslots(draft_id: i64, warehouse_ids: Vec<i64>, date_from: String, date_to: String, state)` → `supply::get_timeslots(...)`
  - `create_supply_from_draft(draft_id: i64, warehouse_id: i64, timeslot_from: String, timeslot_to: String, state)` → `supply::create_supply_from_draft(...)`
  - `get_supply_create_status(operation_id: String, state)` → `supply::get_supply_create_status(...)`
  - `list_products(state)` → `supply::list_products(&cfg)` — wraps existing `ozon::get_products` for frontend product search
  
  3. Register all 8 commands in `generate_handler![]` macro
  
  Must NOT do: NO modifying existing commands. NO breaking existing API surface. NO registering get_supply_orders or get_supply_order_details (YAGNI for v1).
  
  Parallelization: Wave 2 | Blocked by: 1 | Blocks: 4
  References:
  - `src-tauri/src/lib.rs:1-289` — full file: `mod` declarations, `get_config()`, command fn patterns, `generate_handler![]`, logging with `info!`
  - `src-tauri/src/lib.rs:79-101` — example command pattern (get_stock_on_warehouses, get_analytics_stocks)
  - `src-tauri/src/lib.rs:144-171` — example command with logging (get_stock_report, get_stock_analytics)
  Acceptance criteria: `cd src-tauri && cargo build` compiles without errors
  QA scenarios: build check only
  Commit: Y | feat(backend): register FBO supply Tauri commands

- [x] 3. **src/composables/useI18n.js** — Add i18n keys for posting constructor
  What to do:
  Add flat keys under `postingConstructor.*` prefix to both `messages.ru` and `messages.en` objects.
  
  Keys needed (ru + en):
  - `postingConstructor.title` — "Заявки на поставку" / "Supply Requests"
  - `postingConstructor.step1` — "Проверка складов" / "Warehouse Check"
  - `postingConstructor.step2` — "Выбор кластера" / "Cluster Selection"
  - `postingConstructor.step3` — "Товары" / "Products"
  - `postingConstructor.step4` — "Создание черновика" / "Creating Draft"
  - `postingConstructor.step5` — "Выбор таймслота" / "Timeslot Selection"
  - `postingConstructor.step6` — "Создание заявки" / "Creating Supply"
  - `postingConstructor.step7` — "Результат" / "Result"
  - `postingConstructor.warehouse` — "Склад" / "Warehouse"
  - `postingConstructor.cluster` — "Кластер" / "Cluster"
  - `postingConstructor.productSearch` — "Поиск товаров" / "Search Products"
  - `postingConstructor.quantity` — "Количество" / "Quantity"
  - `postingConstructor.addProduct` — "Добавить товар" / "Add Product"
  - `postingConstructor.removeProduct` — "Удалить" / "Remove"
  - `postingConstructor.selectTimeslot` — "Выберите интервал поставки" / "Select delivery interval"
  - `postingConstructor.createDraft` — "Создать черновик" / "Create Draft"
  - `postingConstructor.createSupply` — "Создать заявку" / "Create Supply"
  - `postingConstructor.draftCreated` — "Черновик создан" / "Draft created"
  - `postingConstructor.supplyCreated` — "Заявка создана" / "Supply request created"
  - `postingConstructor.error` — "Ошибка" / "Error"
  - `postingConstructor.retry` — "Повторить" / "Retry"
  - `postingConstructor.next` — "Далее" / "Next"
  - `postingConstructor.back` — "Назад" / "Back"
  - `postingConstructor.finish` — "Завершить" / "Finish"
  - `postingConstructor.loadingWarehouses` — "Загрузка складов..." / "Loading warehouses..."
  - `postingConstructor.selectWarehouse` — "Выберите склад назначения" / "Select destination warehouse"
  - `postingConstructor.info` — "О заявках на поставку" / "About Supply Requests"
  - `postingConstructor.infoContent` — "Создание заявок на поставку товаров на склады Ozon по схеме FBO (прямая поставка)." / "Create FBO direct supply requests to send products to Ozon warehouses."
  
  Must NOT do: NO deleting existing keys. NO changing existing keys.
  
  Parallelization: Wave 1 | Blocked by: — | Blocks: 5
  References:
  - `src/composables/useI18n.js:1-385` — full file: flat key structure, ru object (lines 7-188), en object (lines 189-368), `t()` function (lines 372-374)
  - Existing tab keys: lines 163-166 (ru), 344-347 (en)
  - Existing help keys pattern: `'info.analytics'` / `'info.analyticsContent'` at lines 11-12, 192-193
  Acceptance criteria: `t('postingConstructor.title')` returns the correct string for both locales after adding keys
  QA scenarios: check ru + en objects both have all keys, verify no key is missing from either locale
  Commit: Y | feat(i18n): add posting constructor translation keys

- [x] 4. **src/composables/usePostingConstructor.js** — Composable for posting constructor state and API calls
  What to do:
  Create a new composable that manages wizard state + Tauri command invocations.
  
  State:
  - `currentStep` (ref, 1-7)
  - `warehouses` (ref, array — from `get_available_warehouses`)
  - `selectedWarehouseId` (ref)
  - `clusters` (ref, array — from `get_cluster_list`)
  - `selectedClusterIds` (ref, array)
  - `products` (ref, array — items added by user: `{sku, offer_id, name, quantity}`)
  - `productSearchResults` (ref, array — from product search, product lookup via existing `get_product_info_list`)
  - `draftOperationId` (ref)
  - `draftInfo` (ref — from `get_draft_info`)
  - `draftId` (ref — extracted from draftInfo)
  - `timeslots` (ref — from `get_timeslots`)
  - `selectedTimeslot` (ref)
  - `supplyOperationId` (ref)
  - `supplyResult` (ref — from `get_supply_create_status`)
  - `loading` (ref, boolean per-step)
  - `error` (ref, string per-step)
  
  Functions:
  - `loadWarehouses()` → invoke `get_available_warehouses`
  - `loadClusters()` → invoke `get_cluster_list`
  - `searchProducts(query)` → invoke `list_products` to get all products, then filter client-side by name/offer_id case-insensitive match. Store full product list in `allProducts` ref (fetched once on step 3 entry, not on every keystroke). Filter `allProducts` by query to produce `productSearchResults`. This avoids API calls on every keystroke.
  - `addProduct(sku, offerId, name, quantity)` → push to `products` array
  - `removeProduct(index)` → splice from `products` array
  - `submitDraft()` → invoke `create_supply_draft` with cluster_ids + items
  - `pollDraftInfo()` → invoke `get_draft_info`, repeat every 1s until status is `CALCULATION_STATUS_SUCCESS` or `CALCULATION_STATUS_FAILED`, max 30 retries (30s timeout). Clear error on each retry. On max retries → set error "Draft creation timed out".
  - `loadTimeslots(dateFrom, dateTo)` → invoke `get_timeslots`
  - `submitSupply()` → invoke `create_supply_from_draft`
  - `pollSupplyStatus()` → invoke `get_supply_create_status`, repeat every 1s until status is `DraftSupplyCreateStatusSuccess` or `DraftSupplyCreateStatusFailed`, max 30 retries. On max retries → set error "Supply creation timed out".
  - `reset()` → reset all state to initial
  - `goToStep(n)` → set `currentStep`
  
  Must NOT do: NO direct DOM manipulation. NO runtime type guards. NO empty catch blocks.
  
  Parallelization: Wave 2 | Blocked by: 1 | Blocks: 5
  References:
  - `src/composables/useDashboard.js:1-96` — pattern: ref/loading/error/refresh, invoke pattern
  - `src/composables/useAnalyticsDashboard.js:1-39` — simpler pattern with computed destructuring
  - `src-tauri/src/lib.rs` — command names to invoke
  - AGENTS.md under `src/composables/` — composable patterns doc
  Acceptance criteria: Import works, methods call invoke with correct arguments
  QA scenarios: unit test in spec (mocked invoke)
  Commit: Y | feat(composable): add usePostingConstructor wizard state

- [x] 5. **src/components/PostingConstructor.vue** — 7-step wizard component
  What to do:
  Create the wizard UI component. Use `n-steps` (from naive-ui) for the step indicator at top. Each step is a `v-if` section.
  
  Step 1 — Warehouse Check:
  - `n-spin` while loading
  - `n-alert` on error with retry button
  - Table/cards showing warehouses from `get_available_warehouses` response
  - Radio-select warehouse (store ID)
  
  Step 2 — Cluster Selection:
  - List clusters from `get_cluster_list` response
  - Show cluster name + warehouses within
  - Select one cluster (radio/checkbox)
  
  Step 3 — Products:
  - Search input (text field) → calls `searchProducts` on input/throttle
  - Results shown in a list/table with "Add" button
  - Added products shown in a second table with quantity input + remove button
  - Summary showing total items count
  
  Step 4 — Draft Creation:
  - Confirmation summary: selected warehouse, cluster, products
  - "Create Draft" button → calls `submitDraft()`
  - Loading spinner + status text while polling `pollDraftInfo()`
  - Success/error display
  
  Step 5 — Timeslot Selection:
  - `n-date-picker` for date range (max 28 days)
  - Available timeslots displayed as list/cards
  - Select one timeslot (radio)
  
  Step 6 — Supply Creation:
  - Summary of draft + selected timeslot
  - "Create Supply" button → calls `submitSupply()`
  - Loading spinner + status text while polling `pollSupplyStatus()`
  
  Step 7 — Result:
  - Success message with supply order IDs (displayed from `get_supply_create_status` response)
  - "Create Another" button → calls `reset()`
  - Note: supply order IDs shown as simple text list. Full order list view is out of scope for v1.
  
  Navigation:
  - "Back" and "Next" buttons at bottom (except step 1 has no Back, step 7 has no Next)
  - Step indicator highlights current step
  - Cannot advance if current step has validation errors / incomplete
  
  Use `useI18n` for all text labels via `t('postingConstructor.*')`.
  Use `usePostingConstructor` composable for all state/actions.
  
  Must NOT do: NO Vue Router. NO direct DOM manipulation. NO npm deps beyond existing (naive-ui: NSteps, NStep, NButton, NInput, NDatePicker, NSpin, NAlert, NEmpty, NRadio, NRadioGroup, NDataTable, NCard, NStatistic, NInputNumber). NO splitting into sub-components (single SFC for v1). NO complex animations beyond what naive-ui provides.
  
  Parallelization: Wave 3 | Blocked by: 3, 4 | Blocks: 6, 7
  References:
  - `src/components/AnalyticsDashboard.vue:1-105` — pattern: view-container, view-header with info details, n-spin/n-alert/n-empty states, naive-ui usage
  - `src/App.vue:95-129` — n-tabs pattern with lazy activation
  - AGENTS.md under `src/components/` — component conventions
  - Naive UI: NSteps (https://www.naiveui.com/en-US/os-theme/components/steps), NDatePicker, NInputNumber, NRadioGroup, NButton
  Acceptance criteria: Component renders all 7 steps, navigation buttons work, API calls trigger on step entry
  QA scenarios: mount and verify step transitions, verify all naive-ui components render, verify i18n keys are used
  Commit: Y | feat(ui): add FBO supply request wizard component

- [x] 6. **src/App.vue** — Wire new "posting-constructor" tab
  What to do:
  1. Add lazy activation tracking ref `postingActivated` (same pattern as `stocksActivated`, `totalsActivated`, `analyticsActivated`)
  2. Add `watch` entry for `'posting-constructor'` view
  3. Import `PostingConstructor` component
  4. Add `n-tab-pane name="posting-constructor"` with lazy render `v-if="postingActivated"`
  5. Tab label uses `t('postingConstructor.title')`
  
  Must NOT do: NO Vue Router. NO changing existing tabs. NO removing existing components.
  
  Parallelization: Wave 4 | Blocked by: 5 | Blocks: —
  References:
  - `src/App.vue:1-133` — full file: imports (lines 1-18), config screen (lines 60-84), main layout with n-tabs (lines 95-129)
  - Lines 31-40: lazy activation pattern
  - Lines 96-129: tabs pattern
  - Lines 126-128: analytics tab as template
  Acceptance criteria: New tab appears in header, clicking it renders the wizard component, lazy activation works (component not rendered on first load)
  QA scenarios: visual inspection of tab bar, tab switching
  Commit: Y | feat(ui): add supply-request tab to navigation

- [x] 7. **src/components/__tests__/PostingConstructor.spec.js** — Vitest spec for wizard
  What to do:
  Create test spec following the existing pattern from `AnalyticsDashboard.spec.js`.
  
  Test cases:
  1. **Renders initial state**: mounts, shows step indicator at step 1, shows warehouse loading state
  2. **Loading state**: `n-spin` visible when `loading` is true
  3. **Error state**: `n-alert` with error text + retry button
  4. **Empty/no-data state**: shows appropriate empty message
  5. **Step navigation**: mock composable with currentStep at each position, verify step-specific content renders
  6. **Product search flow**: mock product search, verify results appear
  7. **Draft creation flow**: mock submitDraft + pollDraftInfo, verify loading + success states
  8. **Supply creation flow**: mock submitSupply + pollSupplyStatus, verify result display
  
  Mock `useI18n` and `usePostingConstructor` (same pattern as AnalyticsDashboard spec).
  Stub naive-ui components with simple templates.
  
  Must NOT do: NO real API calls. NO testing the composable (it's mocked). NO testing Rust backend.
  
  Parallelization: Wave 4 | Blocked by: 5 | Blocks: —
  References:
  - `src/components/__tests__/AnalyticsDashboard.spec.js:1-133` — exact pattern: vi.mock, beforeEach, createStubs, test structure, NSpin/NAlert/NButton stubs
  - `src/components/PostingConstructor.vue` — component under test
  Acceptance criteria: `npx vitest run src/components/__tests__/PostingConstructor.spec.js` passes all 8 test cases
  QA scenarios: `npx vitest run` to verify existing tests + new tests all pass
  Commit: Y | test: add PostingConstructor wizard tests

## Final verification wave
> Runs in parallel after ALL todos. ALL must APPROVE. Surface results and wait for the user's explicit okay before declaring complete.
- [x] F1. **Plan compliance audit**: All 7 todos complete. Scope matches Must Have. No Must NOT have violations.
- [x] F2. **Code quality review**: Rust compiles without warnings. JS follows existing patterns (no any, no empty catch). Ponytail check for over-engineering.
- [x] F3. **Real manual QA**: `npm run dev` starts, new tab renders, steps navigate, API calls fire on step entry (test with real Ozon API keys). Verify all 7 steps render correctly in both ru and en locales.
- [x] F4. **Scope fidelity**: No crossdock, no driver pass, no cargo space, no FBP endpoints, no supply order list view. Direct supply only. Product search uses `list_products` command (not non-existent `get_product_info_list`).

## Commit strategy
- 7 atomic commits, one per todo, following `type(scope): message` format
- Commits: `feat(backend):`, `feat(i18n):`, `feat(composable):`, `feat(ui):`, `test:`
- All commits pushed to current branch, no force-push
- Final commit is the test commit (task 7)

## Success criteria
- [ ] New "Заявки на поставку" tab visible in header
- [ ] 7-step wizard renders correctly on tab click
- [ ] Step 1: warehouses load and are selectable
- [ ] Step 2: clusters load and are selectable
- [ ] Step 3: products can be searched, added with quantities, removed
- [ ] Step 4: draft creates with loading polling, shows success/failure
- [ ] Step 5: timeslots load and are selectable
- [ ] Step 6: supply creates with loading polling, shows success/failure
- [ ] Step 7: result shows supply order IDs and "Create Another" button
- [ ] All text in Russian (default) and English
- [ ] `cargo build` compiles
- [ ] `npx vitest run` passes all tests (existing + new)
