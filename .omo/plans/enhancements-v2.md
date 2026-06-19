# Ozon Dashboard V2 — UX Fixes & Consistent Redesign

## TL;DR

> **Quick Summary**: Fix all data-display bugs across the 4-tab dashboard (broken expandable rows, empty analytics, warehouse stock stuck at 0), give all tables consistent viewport-filling heights, and redesign all 4 tabs to follow a consistent layout: header → summary cards → constrained data table.
>
> **Deliverables**:
> - Rust backend fixes: corrected field names in 3 analytics/ozon functions
> - New `get_analytics_dashboard_data` Tauri command (two-stage loading)
> - Dashboard tab: expandable NDataTable rows with row-click + expand column
> - Stock Analytics: real data (fixed field names) + constrained height
> - Analytics Dashboard: populated with real product data
> - All 4 views: consistent `view-container → view-header → stats-grid → n-data-table` layout
>
> **Estimated Effort**: Short
> **Parallel Execution**: YES — 3 waves + final verification
> **Critical Path**: Rust fixes → Frontend → UX consistency → Verification

---

## Context

### Original Request
User reports the app from the previous planning session has several broken/missing features: Dashboard tab table has no expandable rows and cells don't display info, Stock Analytics has no height constraint, Analytics Dashboard is empty. User wants a comprehensive UX rethink and provides API credentials to test real data.

### Interview Summary
**Key Discussions**:
- **Dashboard table expand**: Row click toggles expand (recommended) — user chose row-click + expand arrow column
- **Analytics Dashboard data source**: Separate API call (two-stage: get products → extract SKUs → query analytics)
- **Table heights**: Flexible viewport-filling height for all data tables
- **UX Scope**: Consistent redesign across ALL 4 tabs
- **Test strategy**: No automated tests — agent-executed QA scenarios only
- **API credentials**: Client ID: 4926019, Token: 6c04f667-92de-4d16-8b0a-971c89c4c0eb (in config.json)

**Research Findings**:
- Real Ozon API tested with 5 endpoints — response shapes confirmed
- `/v2/analytics/stock_on_warehouses` returns `free_to_sell_amount`, `reserved_amount`, `promised_amount` (NOT `free_to_sell`, `reserved` as current code reads)
- `/v1/analytics/stocks` returns `{ "items": [...] }` at top level (NOT `{ "result": [...] }`)
- `/v1/analytics/stocks` expects field `skus` (plural) not `sku` (singular)
- `/v4/product/info/stocks` returns complete product list with cursor pagination — includes SKUs in nested `stocks[].sku`
- `/v3/finance/transaction/totals` works correctly with current field mapping

### Metis Review
**Identified Gaps** (addressed):
- **Bug 6**: `analytics.rs:63` reads `data["result"]` but API returns `{ "items": [...] }` — fixed
- **Bug 7**: `promised_amount` from warehouse API dropped — will add as `promised` in output
- **Analytics SKU strategy**: Use `/v4/product/info/stocks` to get all products with SKUs, then call `/v1/analytics/stocks`
- **Scope boundaries**: IN/OUT explicitly defined

---

## Work Objectives

### Core Objective
Fix all data-display bugs across the 4 tab-views (broken expandable rows, empty analytics, warehouse stock numbers stuck at 0), give all data tables consistent viewport-filling height constraints, and redesign all 4 tabs to follow a consistent layout pattern (header → summary cards → constrained data table) using Naive UI components.

### Concrete Deliverables
- `src-tauri/src/ozon.rs` — Fix `get_analytics_stocks` request field: `"sku"` → `"skus"`
- `src-tauri/src/analytics.rs` — Fix field names: `free_to_sell` → `free_to_sell_amount`, `reserved` → `reserved_amount`, add `promised` field
- `src-tauri/src/analytics.rs` — Fix response key: `data["result"]` → `data["items"]` in `get_stock_analytics`
- `src-tauri/src/analytics.rs` — New `get_analytics_dashboard_data()` function (two-stage: product fetch → SKU extraction → analytics query)
- `src-tauri/src/lib.rs` — New `get_analytics_dashboard_data` Tauri command + handler registration
- `src/components/ProductTreeTable.vue` — Add `type: 'expand'` column + `@row-click` handler for expandable rows
- `src/components/StockAnalytics.vue` — Add height constraint to NDataTable + verify real data display
- `src/composables/useAnalyticsDashboard.js` — Rewrite for two-stage loading pipeline
- `src/components/AnalyticsDashboard.vue` — Populate with real data + height constraint
- `src/App.css` — Add flexible table height CSS and layout consistency classes

### Definition of Done
- [ ] `cargo build` succeeds with all Rust fixes
- [ ] Dashboard table rows expand on click with visible expand arrow
- [ ] Stock Analytics shows non-zero `free_to_sell` and `reserved` values
- [ ] Analytics Dashboard shows product data (not empty)
- [ ] All data tables have viewport-filling height with internal scroll
- [ ] All 4 tabs follow consistent `view-container → view-header → (stats-grid?) → n-data-table` pattern
- [ ] All QA scenarios pass with evidence

### Must Have
- Fix Rust field name bugs (free_to_sell_amount, reserved_amount, skus plural, items vs result)
- NDataTable expandable rows with row-click trigger + expand column
- Stock Analytics table height constraint
- Analytics Dashboard populated with real product data
- Consistent layout pattern across all 4 views
- Flexible viewport-filling table heights

### Must NOT Have (Guardrails)
- NO chart libraries or visualizations
- NO vue-router, Pinia, or TypeScript
- NO changes to App.vue tab structure or lazy-load activation
- NO refactoring of non-enriched views (custom tree-table CSS, ProductRow compact mode)
- NO changes to `dashboard.rs` data pipeline or `get_dashboard_summary`
- NO wiring up dead endpoints (`get_turnover_data`, `get_stocks_turnover`)
- NO adding `promised_amount` to StockAnalytics (not requested)
- NO changes to TransactionTotals.vue layout (it's the reference pattern)
- NO CSS architecture refactoring

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed.

### Test Decision
- **Automated tests**: NONE (confirmed with user — no TDD, no tests-after)
- **Verification method**: Agent-executed QA scenarios only (cargo build + curl API tests + Playwright visual)
- **Applicable to**: Both Rust backend fixes and Vue frontend changes

### QA Policy
Every task MUST include agent-executed QA scenarios. Evidence saved to `.omo/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Rust backend**: `cargo build` to verify compilation; `cargo test` if applicable
- **API verification**: curl against real Ozon API endpoints with test SKUs
- **Frontend/UI**: Playwright — navigate tabs, assert columns visible, verify data renders, assert heights
- **Build**: `vite build` must succeed

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Foundation — Rust backend, ALL PARALLEL):
├── Task 1: Fix ozon.rs — request field "sku" → "skus" [quick]
├── Task 2: Fix analytics.rs — warehouse field names + add "promised" [quick]
├── Task 3: Fix analytics.rs — response key "result" → "items" [quick]
├── Task 4: Create get_analytics_dashboard_data in analytics.rs [deep]
├── Task 5: Register new Tauri command + update lib.rs [quick]
└── Task 6: Full Rust compilation check of all fixes [quick]

Wave 2 (Frontend — each view independently, MAX PARALLEL):
├── Task 7: Fix ProductTreeTable.vue — expand column + row-click [visual-engineering]
├── Task 8: Fix StockAnalytics.vue — height constraint + verify data [visual-engineering]
├── Task 9: Rewrite useAnalyticsDashboard.js — two-stage loading [quick]
└── Task 10: Fix AnalyticsDashboard.vue — populate data + height [visual-engineering]

Wave 3 (UX Consistency — depends on Wave 2):
├── Task 11: UX consistency pass across all 4 views [visual-engineering]
└── Task 12: CSS for flexible table heights + layout polish [visual-engineering]

Wave FINAL (Verification — 4 parallel reviews):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Real manual QA with Playwright (unspecified-high)
└── Task F4: Scope fidelity check (deep)
-> Present results -> Get explicit user okay

Critical Path: T1-T3 → T4 → T5 → T6 → T9 → T10 → T11 → T12 → F1-F4 → user ok
Parallel Speedup: ~60% faster than sequential
Max Concurrent: 6 (Wave 1)
```

---

## TODOs

- [x] 1. Fix `ozon.rs` — change `"sku"` to `"skus"` in `get_analytics_stocks` request body

  **What to do**:
  - In `src-tauri/src/ozon.rs`, function `get_analytics_stocks` (line ~193), change the request body from `"sku": skus` to `"skus": skus`
  - The Ozon API `POST /v1/analytics/stocks` expects the field `skus` (plural) not `sku` (singular)
  - Also ensure the `skus` vector is passed correctly — it's already `Vec<i64>`, so the JSON representation will be `[4549460466, 4552355450, ...]`

  **Must NOT do**:
  - Do NOT change the function signature — keep `async fn get_analytics_stocks(config: &OzonConfig, skus: Vec<i64>) -> Result<Value, String>`

  **Recommended Agent Profile**:
  - **Category**: `quick` — 1-line field name change
  - **Skills**: N/A

  **Parallelization**:
  - **Can Run In Parallel**: YES — Wave 1
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 4, 5, 6)
  - **Blocks**: Task 6 (full compilation)
  - **Blocked By**: None

  **References**:
  - `src-tauri/src/ozon.rs:189-197` — Current `get_analytics_stocks` function body
  - Real API test: `curl -X POST ... -d '{"skus":[4549460466,4552355450,4552889112]}'` returned `{"items": [...]}` (verified)

  **Acceptance Criteria**:
  - [ ] `cargo build` succeeds
  - [ ] Request body sends `"skus"` field (not `"sku"`)

  **QA Scenarios**:
  ```
  Scenario: Verify compilation
    Tool: bash
    Preconditions: None
    Steps: cargo build in src-tauri/
    Expected: Build succeeds
    Evidence: .omo/evidence/task-1-build.txt

  Scenario: Verify field name in source
    Tool: bash (grep)
    Preconditions: None
    Steps: grep -n '"skus"' src-tauri/src/ozon.rs
    Expected: Line containing `"skus": skus` found (not `"sku": skus`)
    Evidence: .omo/evidence/task-1-field.txt
  ```

  **Commit**: NO (group with Tasks 2, 3, 6)

- [x] 2. Fix `analytics.rs` — warehouse field names: `free_to_sell` → `free_to_sell_amount`, `reserved` → `reserved_amount`, add `promised`

  **What to do**:
  - In `src-tauri/src/analytics.rs`, function `get_stock_report` (lines ~28-29):
    - Change `row["free_to_sell"]` → `row["free_to_sell_amount"]` (2 occurrences)
    - Change `row["reserved"]` → `row["reserved_amount"]` (2 occurrences)
    - Add `"promised": r["promised_amount"]` to the warehouse row JSON output
  - The Ozon API `POST /v2/analytics/stock_on_warehouses` returns these fields with `_amount` suffix

  **Must NOT do**:
  - Do NOT change the function signature

  **Recommended Agent Profile**:
  - **Category**: `quick` — 3 field name changes
  - **Skills**: N/A

  **Parallelization**:
  - **Can Run In Parallel**: YES — Wave 1
  - **Blocks**: Task 6
  - **Blocked By**: None

  **References**:
  - `src-tauri/src/analytics.rs:24-44` — Current `get_stock_report` transform logic
  - Real API test: `curl -X POST /v2/analytics/stock_on_warehouses` returns `{"free_to_sell_amount": 40, "reserved_amount": 0, "promised_amount": 0}`

  **Acceptance Criteria**:
  - [ ] `cargo build` succeeds
  - [ ] Warehouse stocks now show non-zero values in frontend

  **QA Scenarios**:
  ```
  Scenario: Verify compilation
    Tool: bash
    Preconditions: None
    Steps: cargo build in src-tauri/
    Expected: Build succeeds
    Evidence: .omo/evidence/task-2-build.txt
  ```

  **Commit**: NO (group with Tasks 1, 3, 6)

- [x] 3. Fix `analytics.rs` — response key: `data["result"]` → `data["items"]` in `get_stock_analytics`

  **What to do**:
  - In `src-tauri/src/analytics.rs`, function `get_stock_analytics` (line ~63):
    - Change `data["result"]` to `data["items"]`
  - The Ozon API `POST /v1/analytics/stocks` returns `{ "items": [...] }` at the top level (no `result` wrapper)

  **Must NOT do**:
  - Do NOT change the function signature or other logic

  **Recommended Agent Profile**:
  - **Category**: `quick` — 1-line fix
  - **Skills**: N/A

  **Parallelization**:
  - **Can Run In Parallel**: YES — Wave 1
  - **Blocks**: Task 6
  - **Blocked By**: None

  **References**:
  - `src-tauri/src/analytics.rs:55-66` — Current `get_stock_analytics`
  - Real API test: curl response confirmed `{ "items": [...] }` at top level

  **Acceptance Criteria**:
  - [ ] `cargo build` succeeds
  - [ ] Correct field name in source

  **QA Scenarios**:
  ```
  Scenario: Verify compilation
    Tool: bash
    Steps: cargo build
    Expected: Succeeds
    Evidence: .omo/evidence/task-3-build.txt
  ```

  **Commit**: NO (group with Tasks 1, 2, 6)

- [x] 4. Create `get_analytics_dashboard_data` function in `analytics.rs`

  **What to do**:
  - Add a new public async function `get_analytics_dashboard_data(config: &OzonConfig) -> Result<Value, String>` to `src-tauri/src/analytics.rs`
  - Implementation:
    1. Call `/v4/product/info/stocks` (via new `ozon::get_product_info_stocks` wrapper) with `visibility: ALL`, paginate with cursor to get ALL products
    2. Extract SKUs from `items[].stocks[].sku` — collect unique SKUs into a `Vec<i64>`
    3. If SKUs are not empty, call `ozon::get_analytics_stocks(config, skus)` to get analytics data
    4. Merge product stock info (from step 1) with analytics data (from step 3)
    5. Compute aggregate values: `overallAds`, `overallIdc`, `turnoverGrades`, `stockBalanceTotal`
    6. Return structured JSON with `products` array + aggregate fields
  - Also add a new function `get_product_info_stocks` to `ozon.rs` (follows existing pattern)
    - POST to `/v4/product/info/stocks`
    - Handle cursor-based pagination (loop while `cursor` is non-empty)
    - Return all items

  **Must NOT do**:
  - Do NOT change existing function signatures
  - Do NOT add external dependencies

  **Recommended Agent Profile**:
  - **Category**: `deep` — new function with pagination and data merging
  - **Skills**: N/A

  **Parallelization**:
  - **Can Run In Parallel**: YES — Wave 1 (independent but needs `ozon.rs` updated)
  - **Blocks**: Task 5, 6, 9, 10
  - **Blocked By**: Task 1 (needs correct `skus` field in analytics call)

  **References**:
  - `src-tauri/src/ozon.rs:189-197` — Pattern for `get_analytics_stocks`
  - `src-tauri/src/analytics.rs:6-52` — Pattern for `get_stock_report`
  - Real API test: `/v4/product/info/stocks` returns 55 products with cursor pagination
  - Real API test: `/v1/analytics/stocks` with `skus=[4549460466,...]` returns rich analytics

  **Acceptance Criteria**:
  - [ ] `cargo build` succeeds
  - [ ] Function returns `Result<Value, String>` matching existing pattern
  - [ ] Output includes `products` array, `overallAds`, `overallIdc`, `turnoverGrades`, `stockBalanceTotal`

  **QA Scenarios**:
  ```
  Scenario: Verify compilation
    Tool: bash
    Steps: cargo build
    Expected: Succeeds
    Evidence: .omo/evidence/task-4-build.txt
  ```

  **Commit**: NO (group with Tasks 5, 6)

- [x] 5. Register new Tauri command + update `lib.rs`

  **What to do**:
  - Add new async Tauri command `get_analytics_dashboard_data` to `src-tauri/src/lib.rs` following existing pattern
  - Register command in `invoke_handler` macro
  - Add `ozon::get_product_info_stocks` module call (the new function from Task 4)

  **Must NOT do**:
  - Do NOT change existing command signatures
  - Do NOT break existing invoke handlers

  **Recommended Agent Profile**:
  - **Category**: `quick` — mechanical registration
  - **Skills**: N/A

  **Parallelization**:
  - **Can Run In Parallel**: YES — Wave 1 (after Task 4)
  - **Blocks**: Task 6, 9
  - **Blocked By**: Task 4

  **References**:
  - `src-tauri/src/lib.rs:17-27` — Existing `check_config` command pattern
  - `src-tauri/src/lib.rs:233-245` — Existing `invoke_handler` macro

  **Acceptance Criteria**:
  - [ ] `cargo build` succeeds
  - [ ] Command registered in invoke_handler

  **QA Scenarios**:
  ```
  Scenario: Verify compilation
    Tool: bash
    Steps: cargo build
    Expected: Succeeds
    Evidence: .omo/evidence/task-5-build.txt
  ```

  **Commit**: NO (group with Tasks 4, 6)

- [x] 6. Full Rust compilation check

  **What to do**:
  - Run `cargo build` in `src-tauri/` to verify ALL Rust changes compile correctly
  - Run `cargo test` if any tests exist
  - Fix any compilation errors

  **Must NOT do**:
  - Do NOT change any code logic

  **Recommended Agent Profile**:
  - **Category**: `quick` — build verification
  - **Skills**: N/A

  **Parallelization**:
  - **Can Run In Parallel**: NO — depends on Tasks 1-5
  - **Blocks**: Wave 2 (all frontend tasks)
  - **Blocked By**: Tasks 1, 2, 3, 4, 5

  **References**:
  - `src-tauri/Cargo.toml` — If any dependencies need updating

  **Acceptance Criteria**:
  - [ ] `cargo build` succeeds with zero errors
  - [ ] `cargo test` passes (if tests exist)

  **QA Scenarios**:
  ```
  Scenario: Full Rust build
    Tool: bash
    Preconditions: Tasks 1-5 complete
    Steps: cargo build in src-tauri/
    Expected: Compilation succeeds
    Evidence: .omo/evidence/task-6-build.txt
  ```

  **Commit**: YES
  - Message: `fix(api): correct Ozon API field names and add analytics dashboard command`
  - Files: `src-tauri/src/ozon.rs`, `src-tauri/src/analytics.rs`, `src-tauri/src/lib.rs`

---

## Wave 2 (Frontend — each view independently)

- [x] 7. Fix `ProductTreeTable.vue` — add expand column + row-click handler

  **What to do**:
  - Add a `type: 'expand'` column as the FIRST column in the enriched `columns` array:
    ```javascript
    { type: 'expand', width: 48, renderExpandIcon(row) { ... } }
    ```
  - Add `onRowClick(row)` function that toggles `expandedRowKeys`
  - Wire to NDataTable: `@row-click="onRowClick"`
  - Keep existing `expanded-row-render`, `expanded-row-keys`, `@update:expanded-row-keys`
  - Keep existing columns unchanged

  **Must NOT do**:
  - Do NOT change non-enriched template path
  - Do NOT change ProductRow.vue or PostingRow.vue

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering` — Naive UI component interaction
  - **Skills**: N/A

  **Parallelization**:
  - **Can Run In Parallel**: YES — Wave 2 (with Tasks 8, 9, 10)
  - **Blocks**: Task 11
  - **Blocked By**: None

  **References**:
  - `src/components/ProductTreeTable.vue:17-34` — Existing expand plumbing
  - `src/components/ProductTreeTable.vue:53-92` — Columns array structure
  - Naive UI NDataTable: `type: 'expand'` column, `renderExpandIcon`

  **Acceptance Criteria**:
  - [ ] Expand arrow visible as first column
  - [ ] Clicking row toggles expand/collapse

  **QA Scenarios**:
  ```
  Scenario: Verify expand works
    Tool: Playwright
    Preconditions: App running, dashboard tab active
    Steps:
      1. Assert expand arrow column exists (▶ character)
      2. Click product row
      3. Assert expanded content visible
    Expected: Rows expand on click
    Evidence: .omo/evidence/task-7-expand.mp4
  ```

  **Commit**: NO (group with Task 11)

- [x] 8. Fix `StockAnalytics.vue` — add height constraint to NDataTable

  **What to do**:
  - Add `max-height` and/or `flex-height` prop to `n-data-table` so it fills remaining viewport
  - Use CSS approach: make `.view-content` flex container with `flex: 1; overflow: hidden`
  - Add `:max-height` computed that adjusts on resize

  **Must NOT do**:
  - Do NOT change summary cards or loading/error states

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: N/A

  **Parallelization**:
  - **Can Run In Parallel**: YES — Wave 2
  - **Blocks**: Task 11
  - **Blocked By**: Task 6

  **References**:
  - `src/components/StockAnalytics.vue:41` — Current NDataTable template
  - `src/App.css:908-910` — `.view-content` CSS

  **Acceptance Criteria**:
  - [ ] Table has height constraint, does not extend beyond viewport
  - [ ] Summary cards visible above table
  - [ ] Data shows non-zero values (after Rust fix)

  **QA Scenarios**:
  ```
  Scenario: Verify height constraint
    Tool: Playwright
    Preconditions: App running, Stocks tab
    Steps: Assert table constrained within viewport with scroll
    Expected: Table respects viewport
    Evidence: .omo/evidence/task-8-height.mp4
  ```

  **Commit**: NO (group with Task 11)

- [x] 9. Rewrite `useAnalyticsDashboard.js` composable

  **What to do**:
  - Rewrite to call `invoke('get_analytics_dashboard_data')` (the new Tauri command)
  - Standard composable pattern: `data`, `loading`, `error` refs + computed values
  - Computed: `products`, `overallAds`, `overallIdc`, `turnoverGrades`, `stockBalanceTotal`
  - Add `month`, `year` refs with `watch` for reactive reloading

  **Must NOT do**:
  - Do NOT depend on dashboard data or other composables

  **Recommended Agent Profile**:
  - **Category**: `quick` — standard composable pattern
  - **Skills**: N/A

  **Parallelization**:
  - **Can Run In Parallel**: YES — Wave 2
  - **Blocks**: Task 10, 11
  - **Blocked By**: Tasks 4, 5, 6

  **References**:
  - `src/composables/useStockAnalytics.js` — Reference pattern

  **Acceptance Criteria**:
  - [ ] Calls `invoke('get_analytics_dashboard_data')`
  - [ ] All computed values defined

  **QA Scenarios**:
  ```
  Scenario: Verify composable structure
    Tool: bash
    Steps: grep for invoke and computed values
    Expected: All present
    Evidence: .omo/evidence/task-9-structure.txt
  ```

  **Commit**: NO (group with Task 10)

- [x] 10. Fix `AnalyticsDashboard.vue` — populate with real data + height

  **What to do**:
  - Update template: summary cards for `stockBalanceTotal`, `overallAds`, `overallIdc`
  - Keep grade distribution badges (NTag per grade)
  - Update NDataTable columns: name, sku, available_stock_count, ads, idc, turnover_grade, days_without_sales
  - Remove old revenue/expenses/profit/margin cards
  - Add same height constraint approach as Task 8

  **Must NOT do**:
  - Do NOT add chart visualizations
  - Do NOT remove loading/error/empty states

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: N/A

  **Parallelization**:
  - **Can Run In Parallel**: YES — Wave 2 (with Tasks 7, 8, 9)
  - **Blocks**: Task 11
  - **Blocked By**: Tasks 6, 9

  **References**:
  - `src/components/StockAnalytics.vue` — Reference layout
  - Real API test: `/v1/analytics/stocks` response fields

  **Acceptance Criteria**:
  - [ ] Analytics Dashboard shows product data
  - [ ] Summary cards, grade badges, and table all visible

  **QA Scenarios**:
  ```
  Scenario: Verify analytics shows data
    Tool: Playwright
    Preconditions: App running
    Steps: Navigate to Analytics tab, wait for data, assert products, badges, cards
    Expected: Real data displayed
    Evidence: .omo/evidence/task-10-analytics.mp4
  ```

  **Commit**: YES (with Task 9)
  - Message: `feat(analytics): populate Analytics Dashboard with two-stage loading`
  - Files: `src/composables/useAnalyticsDashboard.js`, `src/components/AnalyticsDashboard.vue`

---

## Wave 3 (UX Consistency)

- [x] 11. UX consistency pass across all 4 views

  **What to do**:
  - Ensure ALL 4 tabs follow: `view-container → view-header → view-content → (stats-grid? → n-data-table)`
  - Dashboard tab: enriched NDataTable sits in view-content
  - Stock Analytics: already follows pattern
  - Transaction Totals: reference pattern — no changes
  - Analytics Dashboard: after Task 10, ensure pattern match
  - Consistent spacing between sections

  **Must NOT do**:
  - Do NOT change TransactionTotals layout
  - Do NOT refactor existing component CSS classes

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: N/A

  **Parallelization**:
  - **Can Run In Parallel**: NO — depends on Wave 2 completion
  - **Blocks**: Task 12
  - **Blocked By**: Tasks 7, 8, 10

  **References**:
  - `src/App.css:883-950` — Existing view container CSS
  - `src/components/TransactionTotals.vue` — Reference pattern

  **Acceptance Criteria**:
  - [ ] All 4 tabs have consistent padding, spacing, section order
  - [ ] No visual regressions

  **QA Scenarios**:
  ```
  Scenario: Verify consistent layout
    Tool: Playwright
    Steps: Navigate all tabs, assert .view-container, .view-header, consistent spacing
    Expected: All tabs consistent
    Evidence: .omo/evidence/task-11-layout.mp4
  ```

  **Commit**: NO (group with Task 12)

- [x] 12. CSS for flexible table heights + layout polish

  **What to do**:
  - Add CSS to `src/App.css` for flexible viewport-filling table heights
  - `.view-content` gets `display: flex; flex-direction: column; flex: 1`
  - `.view-content .n-data-table` gets `flex: 1; min-height: 200px`
  - Ensure `stats-grid` sits above flex-growing table

  **Must NOT do**:
  - Do NOT override Naive UI internals with !important unnecessarily

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
  - **Skills**: N/A

  **Parallelization**:
  - **Can Run In Parallel**: NO — depends on Task 11
  - **Blocked By**: Task 11

  **References**:
  - `src/App.css:883-950` — Current view container CSS

  **Acceptance Criteria**:
  - [ ] Tables fill remaining viewport after header + cards
  - [ ] `vite build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Verify flexible heights
    Tool: Playwright
    Steps: Navigate tabs at 1280x800, assert table fills remaining space
    Expected: Tables adapt to viewport
    Evidence: .omo/evidence/task-12-flex.mp4
  ```

  **Commit**: YES (with Task 11)
  - Message: `style(ux): consistent layout and flexible table heights across views`
  - Files: `src/App.css`, `src/components/StockAnalytics.vue`, `src/components/AnalyticsDashboard.vue`

---

## Final Verification Wave (MANDATORY)

> 4 review agents run in PARALLEL. ALL must APPROVE. Present consolidated results to user and get explicit "okay" before completing.

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, curl endpoint, run command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in `.omo/evidence/`. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo build` and `vite build`. Review all changed files for: type suppression, empty catches, debug logging in prod, commented-out code, unused imports. Check AI slop: excessive comments, over-abstraction, generic names.
  Output: `Build [PASS/FAIL] | Files [N clean/N issues] | VERDICT`

- [x] F3. **Real Manual QA** — `unspecified-high` (+ `playwright` skill)
  Start from clean state (checkout + `cargo build` + `vite build`). Execute EVERY QA scenario from EVERY task — follow exact steps, capture evidence. Test cross-task integration: navigate all 4 tabs, verify features work together. Check edge cases: empty data, error states.
  Save to `.omo/evidence/final-qa/`.
  Output: `Scenarios [N/N pass] | Integration [N/N] | VERDICT`

- [x] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git log/diff). Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance. Detect cross-task contamination: Task N touching Task M's files.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | VERDICT`

---

## Commit Strategy

| Commit | Tasks | Message |
|--------|-------|---------|
| 1 | 1, 2, 3, 6 | `fix(api): correct Ozon API field names and add analytics dashboard command` |
| 2 | 4, 5, 6 | (included in commit 1 — sequential merge) |
| 3 | 9, 10 | `feat(analytics): populate Analytics Dashboard with two-stage loading` |
| 4 | 7, 8, 11, 12 | `style(ux): consistent layout and flexible table heights across views` |

---

## Success Criteria

### Verification Commands
```bash
cargo build --manifest-path src-tauri/Cargo.toml  # Expected: Compilation succeeds
vite build  # Expected: Build succeeds, 0 errors
```

### Final Checklist
- [ ] All "Must Have" present and verified by F1
- [ ] All "Must NOT Have" absent (F1 check)
- [ ] `cargo build` passes
- [ ] `vite build` passes
- [ ] All QA scenarios pass (F3 verification)
- [ ] Scope fidelity confirmed (F4 check)
- [ ] User explicitly approves presented results
