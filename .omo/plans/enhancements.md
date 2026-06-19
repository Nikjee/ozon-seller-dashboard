# Ozon Dashboard Enhancements

## TL;DR

> **Quick Summary**: Enrich the product table with granular uniteconomy-style cost columns (20+ per-product economics fields) and add 3 new API data views: Stock Analytics, Transaction Totals, and Analytics Dashboard.
>
> **Deliverables**:
> - Naive UI integration: `NConfigProvider`, `NDataTable`, `NTabs`, `NCard`, `NStatistic`, `NTag`, `NSpin`, `NEmpty`, `NAlert`
> - Product table with granular cost breakdown (matching uniteconomy.xlsx structure) via `NDataTable`
> - Tab-based navigation for 4 views (Dashboard | Stocks | Totals | Analytics) via `NTabs`
> - Stock Analytics view (warehouse stock levels, turnover grades)
> - Transaction Totals view (aggregated finance breakdown)
> - Analytics Dashboard view (stock balance, turnover trends)
> - Pagination fix for finance transactions (currently hardcoded to 1000)
>
> **Estimated Effort**: XL
> **Parallel Execution**: YES — 4 waves
> **Critical Path**: Fix pagination → API wrappers → Enriched table → New views

---

## Context

### Original Request
User wants to enhance their Ozon Seller Dashboard with richer per-product financial data and new analytics views powered by Ozon API.

### Interview Summary
**Key Discussions**:
- **Uniteconomy reference**: The `uniteconomy.xlsx` file from Ozon shows the exact granular cost columns user wants in the product table (20+ columns vs current 4)
- **Data source**: Uniteconomy data comes from Ozon APIs, not from importing the xlsx
- **Backend**: Extend Rust (Tauri) backend — add new `ozon_request`-based wrappers
- **Navigation**: Tab bar in header (Dashboard | Stocks | Totals | Analytics) — no vue-router
- **Architecture**: Split Tauri commands per view, NOT monolithic
- **Pagination**: Fix existing hardcoded limit (page:1, page_size:1000) on `get_finance_transactions`
- **Column accuracy**: Exact match from APIs — show what's available, mark unavailable as `—`
- **Component Library**: Naive UI — `npm i naive-ui`, auto-import via `unplugin-vue-components`, theme integration via `NConfigProvider`

**Metis Findings** (addressed):
- No vue-router/Pinia/TypeScript/chart libraries — keep simple tab-based navigation
- Keep existing 4 cost columns working — enrichment is additive
- No CSS architecture refactoring — add to existing App.css
- Split per-view Tauri commands for individual loading/error states

---

## Work Objectives

### Core Objective
Transform the product table from 4 simplified cost columns to 20+ granular uniteconomy-style columns, and add 3 new API-powered analytics views with tab-based navigation.

### Concrete Deliverables
- `src-tauri/src/ozon.rs` — New API wrappers: `get_stock_on_warehouses()`, `get_analytics_stocks()`, `get_stocks_turnover()`, `get_finance_totals()` + pagination fix
- `src-tauri/src/analytics.rs` (NEW) — Stock/analytics API orchestration module
- `src-tauri/src/uniteconomy.rs` (NEW) — Granular cost extraction from transaction data
- `src-tauri/src/lib.rs` — New Tauri commands: `get_stock_data()`, `get_finance_totals_data()`, `get_analytics_data()`
- `vite.config.js` — Updated with Naive UI auto-import plugins
- `src/main.js` — Updated with `NConfigProvider` wrapping the app
- `src/App.vue` — `NTabs`-based navigation between Dashboard, Stocks, Totals, Analytics views
- `src/components/ProductTreeTable.vue` — Refactored to use `NDataTable` with enriched columns
- `src/components/ProductRow.vue` — Refactored to use `NDataTable` expandable row render
- `src/components/StockAnalytics.vue` (NEW) — Uses `NDataTable`, `NCard`, `NStatistic`, `NTag`
- `src/components/TransactionTotals.vue` (NEW) — Uses `NCard`, `NStatistic`
- `src/components/AnalyticsDashboard.vue` (NEW) — Uses `NDataTable`, `NTag`, `NCard`
- `src/composables/useStockAnalytics.js`, `useTransactionTotals.js`, `useAnalyticsDashboard.js` (NEW)
- `src/composables/useI18n.js` — 40+ new translation keys (ru/en)

### Definition of Done
- [ ] All 3 new views load data via their own Tauri commands with individual loading/error states
- [ ] Product table shows 20+ granular cost columns with horizontal scroll
- [ ] Tab navigation works — switching tabs preserves other views' loaded data
- [ ] Pagination fix: transactions beyond 1000 are now fetched
- [ ] `cargo build` succeeds, `vite build` succeeds
- [ ] All QA scenarios pass with evidence

### Must Have
- Tab-based navigation working with 4 views
- Product table enriched with granular cost columns (at least 15+ columns populated from APIs)
- Individual loading/error states per view
- Stock Analytics: warehouse-level stock breakdown
- Transaction Totals: aggregated finance overview
- Analytics Dashboard: stock balance + turnover data
- Pagination fix for finance transactions
- All i18n keys added for both RU and EN locales

### Must NOT Have (Guardrails)
- NO vue-router, Pinia, TypeScript, or chart libraries
- NO CSS architecture refactoring (keep global CSS in App.css)
- NO changes to theme (useTheme), config (useConfig), or updater (useUpdater) systems
- NO changes to the 4 existing cost summary calculations
- NO breaking the existing dashboard view — enrichment is additive only
- NO abstraction layers beyond current patterns (no API client classes, no stores)
- NO full rewrite of existing custom components — Naive UI is for NEW views and the enriched table; existing LoadingOverlay/EmptyState/ErrorBanner stay as-is

---

## Verification Strategy (MANDATORY)

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed.
> Acceptance criteria requiring "user manually tests/confirms" are FORBIDDEN.

### Test Decision
- **Infrastructure exists**: NO (no test framework detected)
- **Automated tests**: Tests-after (basic testing with vitest or similar)
- **Framework**: To be determined (vitest recommended for Vue)

### QA Policy
Every task MUST include agent-executed QA scenarios. Evidence saved to `.omo/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Frontend/UI**: Use Playwright — navigate tabs, assert columns visible, verify data renders
- **Rust backend**: Use `cargo test` or curl equivalent — verify API responses
- **Build verification**: `cargo build` + `vite build` must succeed

---

## Execution Strategy

```
Wave 1 (Foundation — independent foundation work):
├── Task 1: Install Naive UI + configure auto-import + NConfigProvider [quick]
├── Task 2: Fix pagination in get_finance_transactions [quick]
├── Task 3: New API wrappers in ozon.rs (stock/analytics endpoints) [deep]
├── Task 4: Create analytics.rs module [deep]
├── Task 5: Create uniteconomy.rs module (granular cost extraction) [deep]
├── Task 6: Register new Tauri commands in lib.rs [quick]
├── Task 7: Add i18n keys (40+ entries, ru/en) [writing]
└── Task 8: Add CSS for view containers + responsive (Naive UI handles components) [visual-engineering]

Wave 2 (Core UI — depends on Wave 1):
├── Task 9: NTabs navigation + NConfigProvider in App.vue [visual-engineering]
├── Task 10: NDataTable for enriched product table [visual-engineering]
├── Task 11: NDataTable expandable row with granular costs [visual-engineering]
├── Task 12: Create useStockAnalytics.js composable [quick]
├── Task 13: Create useTransactionTotals.js composable [quick]
└── Task 14: Create useAnalyticsDashboard.js composable [quick]

Wave 3 (New Views — depends on Wave 2):
├── Task 15: Build StockAnalytics.vue (NDataTable + NCard + NStatistic + NTag) [visual-engineering]
├── Task 16: Build TransactionTotals.vue (NCard + NStatistic) [visual-engineering]
├── Task 17: Build AnalyticsDashboard.vue (NDataTable + NCard + NTag) [visual-engineering]
└── Task 18: Wire up App.vue with tab switching + per-view loading states [deep]

Wave 4 (Testing & Polish — depends on Wave 3):
├── Task 19: Add tests for Rust backend logic [deep]
├── Task 20: Add tests for Vue components [visual-engineering]
└── Task 21: Responsive polish for all views [visual-engineering]

Wave FINAL (Verification):
├── F1: Plan compliance audit (oracle)
├── F2: Code quality review (unspecified-high)
├── F3: Real manual QA (unspecified-high)
└── F4: Scope fidelity check (deep)

Critical Path: T1 → T2-T8 → T9 → T10-T11 → T15-T17 → T18 → T19-T21 → F1-F4 → user ok
```

---

## TODOs

- [x] 1. Install Naive UI + configure auto-import + integrate theme via NConfigProvider

  **What to do**:
  - Run `npm install naive-ui` in project root
  - Install auto-import plugins: `npm install -D unplugin-auto-import unplugin-vue-components`
  - Update `vite.config.js`:
    - Import `AutoImport` from `unplugin-auto-import/vite` and `Components` from `unplugin-vue-components/vite`
    - Import `NaiveUiResolver` from `unplugin-vue-components/resolvers`
    - Add both plugins to the Vite config
  - Update `src/main.js`:
    - Import and use `createApp(App)` with `NConfigProvider` wrapping the root
    - Configure `theme-overrides` to map Catppuccin CSS variables to Naive UI tokens:
      ```javascript
      const themeOverrides = {
        common: {
          primaryColor: 'var(--ctp-teal)',
          successColor: 'var(--ctp-green)',
          errorColor: 'var(--ctp-red)',
          warningColor: 'var(--ctp-yellow)',
          bodyColor: 'var(--bg)',
          cardColor: 'var(--bg-surface)',
          textColor1: 'var(--text)',
          textColor2: 'var(--text-subtle)',
          borderColor: 'var(--border)',
          borderRadius: 'var(--radius-md)'
        }
      }
      ```
    - Dynamically toggle `:theme` on `NConfigProvider` between `darkTheme` and `null` based on `useTheme().theme`
  - Verify `vite build` succeeds

  **Must NOT do**:
  - Do NOT refactor existing custom components (LoadingOverlay, EmptyState, ErrorBanner) — they stay as-is
  - Do NOT remove existing CSS variables — Naive UI overrides reference them

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Standard dependency install + config setup
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 1
  - **Can Run In Parallel**: YES (independent)
  - **Blocks**: Tasks 9, 10, 11, 15, 16, 17
  - **Blocked By**: None

  **References**:
  - `vite.config.js` — Current Vite config to modify
  - `src/main.js` — App entry point to wrap with NConfigProvider
  - `src/composables/useTheme.js` — To integrate darkTheme toggle
  - Naive UI docs: `https://www.naiveui.com/en-US/os-theme/docs/installation` — Install + auto-import setup
  - Naive UI theme customization: `NConfigProvider` + `theme-overrides` prop
  - Naive UI dark mode: `import { darkTheme } from 'naive-ui'`

  **Acceptance Criteria**:
  - [ ] `npm ls naive-ui` shows installed
  - [ ] `vite build` succeeds with auto-import configured
  - [ ] App renders with Naive UI theme applied (NConfigProvider wrapping the app)
  - [ ] Dark/light theme toggle still works (Naive UI theme follows)
  - [ ] No console errors about Naive UI components

  **QA Scenarios**:
  ```
  Scenario: Verify Naive UI installs and builds
    Tool: Bash
    Preconditions: None
    Steps:
      1. npm ls naive-ui — assert installed
      2. npm ls unplugin-auto-import unplugin-vue-components — assert dev deps installed
      3. vite build — assert success
    Expected Result: All dependencies installed, build succeeds
    Evidence: .omo/evidence/task-1-naive-ui-install.txt

  Scenario: Verify theme integration works
    Tool: Playwright
    Preconditions: App running
    Steps:
      1. Navigate to app
      2. Assert NConfigProvider wrapper in DOM (data-naive-ui-provider attribute)
      3. Toggle dark mode
      4. Assert Naive UI theme switches (card backgrounds change)
    Expected Result: Theme integration works
    Evidence: .omo/evidence/task-1-naive-ui-theme.mp4
  ```

  **Commit**: YES
  - Message: `feat(ui): add Naive UI with auto-import and Catppuccin theme integration`
  - Files: `package.json`, `vite.config.js`, `src/main.js`

- [x] 2. Fix pagination in `get_finance_transactions` (ozon.rs)

  **What to do**:
  - Modify `ozon.rs:get_finance_transactions()` to iterate through pages until all operations are fetched
  - Change `page: 1, page_size: 1000` to loop: send requests with `page: 1, 2, 3...` until response has fewer results than page_size
  - Collect ALL operations in a single `Vec<Value>` (or merge into the response)
  - Preserve the existing function signature `async fn get_finance_transactions(config: &OzonConfig, date_from: &str, date_to: &str) -> Result<Value, String>`
  - Test with a month where the seller has >1000 transactions

  **Must NOT do**:
  - Do NOT change the response schema — keep it compatible with existing callers in `dashboard.rs`
  - Do NOT add pagination to the UI — this is server-side only
  - Do NOT add new abstractions — just fix the loop

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Requires understanding pagination edge cases and data merging
  - **Skills**: N/A (generic Rust)

  **Parallelization**:
  - **Parallel Group**: Wave 1
  - **Can Run In Parallel**: YES (independent of all other tasks)
  - **Blocks**: Tasks 4, 18
  - **Blocked By**: None

  **References**:
  - `src-tauri/src/ozon.rs:127-149` — Current implementation with hardcoded page:1
  - `src-tauri/src/ozon.rs:23-58` — The existing `ozon_request` helper
  - Ozon API: `POST /v3/finance/transaction/list` — Response has `Page` field showing current page; if `result.operations` length < page_size, it's the last page

  **Acceptance Criteria**:
  - [ ] `get_finance_transactions` now returns ALL operations for the period, not just first 1000
  - [ ] Existing callers in `dashboard.rs` work unchanged

  **QA Scenarios**:
  ```
  Scenario: Verify pagination works for accounts with >1000 operations
    Tool: Bash (cargo test)
    Preconditions: Rust backend compiles
    Steps:
      1. Create a test that calls get_finance_transactions with a month that has >1000 ops
      2. Assert result["result"]["operations"] length > 1000
    Expected Result: More than 1000 operations returned
    Evidence: .omo/evidence/task-1-pagination.txt

  Scenario: Verifiy backward compatibility (month with <1000 ops)
    Tool: Bash (cargo test)
    Preconditions: Rust backend compiles
    Steps:
      1. Call get_finance_transactions with a quiet month
      2. Assert result["result"]["operations"] length <= 1000
    Expected Result: Works unchanged for small datasets
    Evidence: .omo/evidence/task-1-small.txt
  ```

  **Commit**: YES
  - Message: `fix(api): paginate get_finance_transactions to fetch all operations`
  - Files: `src-tauri/src/ozon.rs`

- [x] 3. Add new Ozon API wrappers in ozon.rs

  **What to do**:
  - Add these new async functions to `src-tauri/src/ozon.rs` following the existing `ozon_request` pattern:
    1. `get_stock_on_warehouses(config, limit, offset, warehouse_type)` — `POST /v2/analytics/stock_on_warehouses`
    2. `get_analytics_stocks(config, skus)` — `POST /v1/analytics/stocks`
    3. `get_stocks_turnover(config, skus)` — `POST /v1/analytics/turnover/stocks`
    4. `get_finance_totals(config, date_from, date_to)` — `POST /v3/finance/transaction/totals`
  - All functions should return `Result<Value, String>` matching the existing pattern
  - Use the existing `ozon_request` helper for all calls

  **Must NOT do**:
  - Do NOT create a new module — add to existing `ozon.rs`
  - Do NOT add generic abstractions — just new functions

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Straightforward addition of API wrappers following existing pattern
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 1
  - **Can Run In Parallel**: YES (independent)
  - **Blocks**: Tasks 3, 5, 11, 12, 13
  - **Blocked By**: None

  **References**:
  - `src-tauri/src/ozon.rs:60-71` — Existing `get_products` pattern (POST request via ozon_request)
  - `src-tauri/src/ozon.rs:127-149` — Existing `get_finance_transactions` pattern
  - Ozon API docs (from lookup):
    - `/v2/analytics/stock_on_warehouses` — Request: `{ limit, offset, warehouse_type }`
    - `/v1/analytics/stocks` — Request: `{ skus }`
    - `/v1/analytics/turnover/stocks` — Request: `{ sku, limit, offset }`
    - `/v3/finance/transaction/totals` — Request: `{ filter: { date: { from, to } } }`

  **Acceptance Criteria**:
  - [ ] 4 new functions added to ozon.rs
  - [ ] All return `Result<Value, String>` matching existing pattern
  - [ ] `cargo build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Verify new functions exist and compile
    Tool: Bash
    Preconditions: None
    Steps:
      1. cargo build
    Expected Result: Compilation succeeds, no warnings
    Evidence: .omo/evidence/task-2-build.txt
  ```

  **Commit**: NO (group with Task 3)

- [x] 4. Create `analytics.rs` module (stock/analytics orchestration)

  **What to do**:
  - Create `src-tauri/src/analytics.rs` as a new module
  - Implement these public async functions:
    1. `get_stock_report(config: &OzonConfig) -> Result<Value, String>` — Calls `get_stock_on_warehouses` and structures the response per warehouse with free_to_sell, reserved, promised amounts
    2. `get_stock_analytics(config: &OzonConfig, skus: &[i64]) -> Result<Value, String>` — Calls `get_analytics_stocks` and returns per-product stock balance data (available, transit, defect, turnover grade, days without sales, ads)
    3. `get_turnover_data(config: &OzonConfig, skus: &[i64]) -> Result<Value, String>` — Calls `get_stocks_turnover` and returns per-product turnover (current_stock, ads, idc, idc_grade)
  - Add `mod analytics;` to `src-tauri/src/lib.rs`

  **Must NOT do**:
  - Do NOT call these functions from `build_dashboard_summary` — they're for independent views
  - Do NOT add database/storage — pure API orchestration

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: New module requiring coordination of multiple API calls
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 1
  - **Can Run In Parallel**: YES (after Task 2)
  - **Blocks**: Tasks 5, 11, 13
  - **Blocked By**: Task 2 (needs the API wrappers)

  **References**:
  - `src-tauri/src/dashboard.rs:39-55` — Pattern: function that uses ozon helpers and returns `Result<Value, String>`
  - `src-tauri/src/ozon.rs` — For the `get_stock_on_warehouses`, `get_analytics_stocks`, `get_stocks_turnover` function signatures

  **Acceptance Criteria**:
  - [ ] `analytics.rs` created with 3 public functions
  - [ ] Module registered in `lib.rs`
  - [ ] `cargo build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Verify module compiles
    Tool: Bash
    Preconditions: Task 2 completed
    Steps:
      1. cargo build
    Expected Result: Compilation succeeds
    Evidence: .omo/evidence/task-3-build.txt
  ```

  **Commit**: YES (with Task 2)
  - Message: `feat(api): add analytics.rs module and 4 new Ozon API wrappers`
  - Files: `src-tauri/src/analytics.rs`, `src-tauri/src/ozon.rs`, `src-tauri/src/lib.rs`

- [x] 5. Create `uniteconomy.rs` module (granular cost extraction)

  **What to do**:
  - Create `src-tauri/src/uniteconomy.rs` as a new module
  - Implement these public functions:
    1. `extract_product_costs(operations: &[Value], sku: i64) -> Value` — Analyze finance transactions for a given SKU and extract all granular cost categories matching uniteconomy structure:
       - `commission` (sale_commission)
       - `acquiring` (эквайринг keyword)
       - `order_processing` (обработка отправления)
       - `logistics` (логистика, crossdoc)
       - `delivery_to_pickup` (доставка до места выдачи)
       - `placement` (стоимость размещения, хранение)
       - `return_processing` (обработка возврата)
       - `return_logistics` (обратная логистика)
       - `disposal` (утилизация)
       - `ovh_processing` (дополнительная обработка ОВХ)
       - `operational_errors` (операционные ошибки)
       - `pay_per_click` (оплата за клик)
       - `pay_per_order` (оплата за заказ)
       - `star_products` (звёздные товары)
       - `paid_brand` (платный бренд)
       - `reviews_cost` (отзывы)
       - `discount_points` (баллы за скидки)
       - `partner_programs` (программы партнёров)
       - `compensation` (компенсации)
       - `other_services` (прочие услуги)
    2. `extract_product_summary(operations: &[Value], sku: i64) -> Value` — Return the enriched summary with all cost fields + computed fields:
       - `total_revenue` — sum of accruals_for_sale
       - `total_costs` — sum of all cost categories
       - `net_profit` — total_revenue - total_costs
       - `profit_per_unit` — net_profit / quantity
       - `total_quantity` — count of operations
  - Use keyword-based categorization logic (follow the pattern from `expenses.rs`)
  - Return structured JSON for each SKU that can be merged into the product data
  - Add `mod uniteconomy;` to `src-tauri/src/lib.rs`

  **Must NOT do**:
  - Do NOT call this from existing `build_dashboard_summary` yet (will be wired in Task 9)
  - Do NOT remove the existing `expenses.rs` — the old summary still needs 4-column mode
  - Do NOT assume all 20+ columns will have data — return 0.0 for unfilled columns

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Complex extraction logic with many categorization rules
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 1
  - **Can Run In Parallel**: YES (after Task 1)
  - **Blocks**: Tasks 9, 10
  - **Blocked By**: Task 2 (needs proper pagination for complete data)

  **References**:
  - `src-tauri/src/expenses.rs:44-62` — Existing `categorize_expense` keyword matching pattern
  - `src-tauri/src/expenses.rs:84-140` — Existing `build_expense_categories` for the logic pattern
  - `uniteconomy.xlsx` columns reference (draft file) for the exact column structure expected
  - `src-tauri/src/dashboard.rs` lines 117-154 — How current code extracts costs from operations

  **Acceptance Criteria**:
  - [ ] `uniteconomy.rs` created with 2 public functions
  - [ ] All 20+ cost categories defined and extracted
  - [ ] Module registered in `lib.rs`
  - [ ] `cargo build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Verify cost extraction produces all fields
    Tool: Bash (cargo test)
    Preconditions: Task 1 completed
    Steps:
      1. Write test that feeds mock operations into extract_product_costs
      2. Assert response contains all expected keys (20+ categories)
      3. Assert numeric values sum correctly
    Expected Result: All expected fields present with correct sums
    Evidence: .omo/evidence/task-4-extraction.txt

  Scenario: Verify empty operations handled gracefully
    Tool: Bash (cargo test)
    Preconditions: Task 1 completed
    Steps:
      1. Call extract_product_costs with empty operations array
      2. Assert response has all keys with 0.0 values
    Expected Result: No crash, all fields return 0.0
    Evidence: .omo/evidence/task-4-empty.txt
  ```

  **Commit**: YES
  - Message: `feat(costs): add uniteconomy.rs with granular per-product cost extraction`
  - Files: `src-tauri/src/uniteconomy.rs`, `src-tauri/src/lib.rs`



- [x] 6. Register new Tauri commands in lib.rs

  **What to do**:
  - Add these new async Tauri commands to `src-tauri/src/lib.rs` following the existing `get_dashboard_summary` pattern:
    1. `get_stock_data(state, month, year)` → Calls `analytics::get_stock_report` + `analytics::get_stock_analytics`
    2. `get_finance_totals_data(state, month, year)` → Calls `ozon::get_finance_totals`
    3. `get_analytics_data(state, month, year)` → Calls `analytics::get_turnover_data` + `analytics::get_stock_analytics`
  - Each command should:
    - Lock state to get config (same pattern as existing commands)
    - Call the appropriate module functions
    - Return `Result<Value, String>`
  - Register each command in the `invoke_handler` macro
  - Import new modules: `mod analytics;`, `mod uniteconomy;`

  **Must NOT do**:
  - Do NOT change the existing `get_dashboard_summary` signature
  - Do NOT merge data — keep commands separate

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Mechanical work — add commands following existing template
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 1
  - **Can Run In Parallel**: YES (after Tasks 2, 3)
  - **Blocks**: Tasks 11, 12, 13
  - **Blocked By**: Tasks 2, 3

  **References**:
  - `src-tauri/src/lib.rs:41-55` — Existing `get_dashboard_summary` command pattern
  - `src-tauri/src/lib.rs:71-76` — Existing `invoke_handler` macro registration
  - `src-tauri/src/analytics.rs` — For the analytics functions
  - `src-tauri/src/ozon.rs` — For `get_finance_totals`

  **Acceptance Criteria**:
  - [ ] 3 new Tauri commands registered
  - [ ] `cargo build` succeeds
  - [ ] Frontend can call `invoke('get_stock_data', ...)`, `invoke('get_finance_totals_data', ...)`, `invoke('get_analytics_data', ...)`

  **QA Scenarios**:
  ```
  Scenario: Verify commands compile and register
    Tool: Bash
    Preconditions: Tasks 2, 3 completed
    Steps:
      1. cargo build
      2. cargo run (verify Tauri starts without crash)
    Expected Result: Build succeeds, app launches
    Evidence: .omo/evidence/task-5-commands.txt
  ```

  **Commit**: NO (group with Task 6)

- [x] 7. Add i18n keys for all new labels (40+ entries, ru/en)

  **What to do**:
  - Add new translation keys to `src/composables/useI18n.js` for:
    - **Tab labels**: `stocks`, `totals`, `analytics`, `dashboard`
    - **Column headers** (product table enrichment): `sku`, `offerId`, `scheme`, `costPrice`, `currentPrice`, `ordered`, `delivered`, `returned`, `revenue`, `discountPoints`, `partnerPrograms`, `commission`, `acquiring`, `orderProcessing`, `logistics`, `deliveryToPickup`, `placement`, `returnProcessing`, `returnLogistics`, `disposal`, `ovhProcessing`, `operationalErrors`, `payPerClick`, `payPerOrder`, `starProducts`, `paidBrand`, `reviewsCost`, `shareOfSales`, `profitPerUnit`, `priceIndex`, `availability`, `periodProfit`
    - **Stock Analytics view**: `warehouse`, `freeToSell`, `reserved`, `promised`, `turnoverGrade`, `daysWithoutSales`, `ads`
    - **Transaction Totals view**: `accrualsForSale`, `totalCompensation`, `saleCommission`, `servicesAmount`, `processingAndDelivery`, `refundsAndCancellations`, `moneyTransfer`, `othersAmount`
    - **Analytics Dashboard view**: `currentStock`, `avgDailySales`, `idcDays`, `stockGrade`, `deficit`, `popular`, `actual`, `surplus`, `noSales`
    - **General**: `loading`, `error`, `retry`, `noData` (may already exist)
  - Both RU and EN translations for ALL keys

  **Must NOT do**:
  - Do NOT refactor the i18n system to vue-i18n
  - Do NOT restructure existing keys
  - Keep flat key-value structure as currently used

  **Recommended Agent Profile**:
  - **Category**: `writing`
    - Reason: Translation work with many entries
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 1
  - **Can Run In Parallel**: YES (independent)
  - **Blocks**: Tasks 9, 10, 14, 15, 16
  - **Blocked By**: None

  **References**:
  - `src/composables/useI18n.js` — Entire file, follow the existing pattern
  - `uniteconomy.xlsx` column headers — Use the Russian column names as RU translations

  **Acceptance Criteria**:
  - [ ] 40+ new keys added in both RU and EN
  - [ ] `locale.value === 'ru'` shows Russian, `locale.value === 'en'` shows English
  - [ ] App still loads without errors

  **QA Scenarios**:
  ```
  Scenario: Verify i18n keys load without errors
    Tool: Bash
    Preconditions: None
    Steps:
      1. vite build
      2. Open app in browser
      3. Assert no console errors
    Expected Result: Build succeeds, no i18n errors
    Evidence: .omo/evidence/task-6-i18n.txt
  ```

  **Commit**: YES (with Task 5)
  - Message: `chore: register new Tauri commands and add i18n keys`
  - Files: `src-tauri/src/lib.rs`, `src/composables/useI18n.js`

- [x] 8. Add CSS for view containers + responsive layout (Naive UI handles component styling)

  **What to do**:
  - Naive UI components (NDataTable, NTabs, NCard, NStatistic, NTag, NSpin, NEmpty, NAlert) handle their own styling — NO custom CSS needed for component internals
  - Add minimal CSS to `src/App.css` for:
    - **View containers**: `.view-section`, `.view-content` — consistent padding/margin for each tab panel
    - **Period picker presentation**: Ensure month/year picker from DashboardHeader is visible above all views (Naive UI NTabs placed below header)
    - **Existing component compatibility**: Ensure existing custom components (LoadingOverlay, ErrorBanner, EmptyState) still look correct alongside Naive UI
    - **Naive UI overrides** (only if needed): Minor adjustments like table row height, card gap inside views
  - Follow existing CSS variable patterns

  **Must NOT do**:
  - Do NOT add custom CSS for component internals (Naive UI handles tables, cards, tabs, tags, spinners, alerts)
  - Do NOT refactor existing CSS or modify existing component classes
  - Do NOT add CSS framework dependencies

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Layout compatibility CSS
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 1
  - **Can Run In Parallel**: YES (independent)
  - **Blocks**: Tasks 9, 10, 11, 15, 16, 17
  - **Blocked By**: None

  **References**:
  - `src/App.css` — Add minimal view container styles
  - Naive UI NDataTable docs: `scroll-x`, `max-height` props for table layout
  - Naive UI NTabs docs: Tab pane layout

  **Acceptance Criteria**:
  - [ ] View container CSS added to App.css
  - [ ] No existing CSS broken
  - [ ] `vite build` succeeds

  **QA Scenarios**:
  ```
  Scenario: Verify CSS compiles without errors
    Tool: Bash
    Preconditions: None
    Steps:
      1. vite build
    Expected Result: Build succeeds
    Evidence: .omo/evidence/task-8-css.txt
  ```

  **Commit**: YES
  - Message: `style: add view container CSS for Naive UI compatibility`
  - Files: `src/App.css`

- [x] 9. Add tab navigation with `NTabs` in App.vue + NConfigProvider theme integration

  **What to do**:
  - In `src/App.vue`:
    - Import Naive UI's `NTabs`, `NTabPane` components (auto-imported via plugin, no manual import needed)
    - Add `activeView` ref (default: `'dashboard'`)
    - Replace the single-view layout with `NTabs`:
      ```vue
      <n-tabs v-model:value="activeView" type="line" animated>
        <n-tab-pane name="dashboard" tab="Dashboard">
          <div class="view-section">
            <!-- existing dashboard content -->
          </div>
        </n-tab-pane>
        <n-tab-pane name="stocks" tab="Stocks">
          <StockAnalytics v-if="activeView === 'stocks'" />
        </n-tab-pane>
        <n-tab-pane name="totals" tab="Totals">
          <TransactionTotals v-if="activeView === 'totals'" />
        </n-tab-pane>
        <n-tab-pane name="analytics" tab="Analytics">
          <AnalyticsDashboard v-if="activeView === 'analytics'" />
        </n-tab-pane>
      </n-tabs>
      ```
  - Keep the `v-if="activeView === 'stocks'"` pattern for lazy loading (only load view data when tab is first activated)
  - Place NTabs in the `<main>` section, after the DashboardHeader
  - Keep existing header elements (title, period picker, theme/locale toggles, refresh button) above the tabs
  - The period picker (month/year) in DashboardHeader should be shared with the active view's composable
  - Add placeholder imports for the 3 new view components

  **Must NOT do**:
  - Do NOT add vue-router
  - Do NOT remove existing header controls
  - Do NOT modify existing dashboard content
  - Do NOT hand-roll tab styling — NTabs handles all visual presentation

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: UI component with navigation
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 2
  - **Can Run In Parallel**: NO — core structural change
  - **Blocks**: Tasks 15, 16, 17
  - **Blocked By**: Tasks 1, 8 (needs Naive UI installed and view container CSS)

  **References**:
  - `src/App.vue` — Current App.vue with single dashboard view
  - `src/main.js` — After Task 1 (NConfigProvider setup)
  - Naive UI NTabs docs: `v-model:value`, `type="line"`, `animated`

  **Acceptance Criteria**:
  - [ ] 4 tabs visible: Dashboard, Stocks, Totals, Analytics
  - [ ] Clicking a tab switches visible content panel
  - [ ] Active tab is visually highlighted by NTabs
  - [ ] Existing header controls (period, theme, locale, refresh) still work
  - [ ] Dashboard view still works as before
  - [ ] Tab content lazy-loads (v-if pattern for non-dashboard tabs)

  **QA Scenarios**:
  ```
  Scenario: Verify tab switching works
    Tool: Playwright
    Preconditions: App is running
    Steps:
      1. Navigate to app
      2. Click "Stocks" tab
      3. Assert "Stocks" tab pane is visible
      4. Click "Dashboard" tab
      5. Assert "Dashboard" tab pane is visible
    Expected Result: NTabs switches visible sections
    Evidence: .omo/evidence/task-9-tabs.mp4

  Scenario: Verify existing dashboard still works
    Tool: Playwright
    Preconditions: App is running with valid config
    Steps:
      1. Navigate to app (default dashboard view)
      2. Assert dashboard data loads (stats bar, product table visible)
      3. Assert theme toggle works
      4. Assert locale toggle works
    Expected Result: Dashboard functions unchanged
    Evidence: .omo/evidence/task-9-dashboard.mp4
  ```

  **Commit**: YES
  - Message: `feat(nav): add NTabs navigation to App.vue`
  - Files: `src/App.vue`



- [x] 10. Refactor ProductTreeTable.vue with `NDataTable` enriched columns

  **What to do**:
  - Refactor `src/components/ProductTreeTable.vue` to use Naive UI's `NDataTable` for the enriched 20+ column view
  - Keep the existing custom grid layout as `compact` mode (preserve existing behavior for backward compatibility)
  - For enriched mode, use `NDataTable`:
    - Import (auto-imported): `NDataTable`, `NButton`, `NSpace`, `NTag`
    - Configure `columns` array with all 20+ column definitions:
      - **Fixed columns**: Product name (frozen left, `fixed: 'left'`)
      - **Numeric columns**: Revenue, Commission, Acquiring, Order Processing, Logistics, etc. with `formatRubCompact` render
      - **Column groups**: Use `title` with sub-headers for "Sales", "Fulfillment Costs", "Marketing", "KPIs"
      - **Badge columns**: Scheme (FBO/FBS) with `NTag`, profit with color coding
    - Set props: `scroll-x` for horizontal scroll, `max-height` for vertical scroll, `bordered`, `single-line`
    - Use `expanded-row-render` for posting details (passing data to PostingRow)
    - Row key: `row.sku`
  - Pass enriched `uniteconomy` data from the backend response as table `data`
  - Compact mode uses current custom template (via `v-if="!enriched"`)

  **Must NOT do**:
  - Do NOT remove compact mode — existing view uses it
  - Do NOT add custom CSS for table internals — NDataTable handles all styling
  - Do NOT add pagination or column toggles

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Complex table layout redesign
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 2
  - **Can Run In Parallel**: YES (with Task 11)
  - **Blocks**: Task 11, 18
  - **Blocked By**: Tasks 1, 4, 5, 8 (needs Naive UI, uniteconomy data, and CSS)

  **References**:
  - `src/components/ProductTreeTable.vue` — Current implementation
  - `src-tauri/src/uniteconomy.rs` — For the data structure returned
  - Naive UI NDataTable docs: `columns`, `scroll-x`, `expanded-row-render`

  **Acceptance Criteria**:
  - [ ] Enriched mode shows 20+ columns in NDataTable with horizontal scroll
  - [ ] Column group headers visible
  - [ ] Old compact mode still works with `enriched: false`
  - [ ] Data flows correctly

  **QA Scenarios**:
  ```
  Scenario: Verify enriched table renders all columns
    Tool: Playwright
    Preconditions: App running with valid config, data loaded
    Steps:
      1. Navigate to Dashboard tab
      2. Assert NDataTable has 20+ column headers
      3. Assert each column header has a label
      4. Assert product rows show numeric values in columns
    Expected Result: All 20+ columns visible with data
    Evidence: .omo/evidence/task-10-enriched-table.mp4
  ```

  **Commit**: NO (group with Task 11)

- [x] 11. Refactor ProductRow.vue with `NDataTable` expandable row + granular cost display

  **What to do**:
  - The enriched product table uses `NDataTable` (Task 10), so ProductRow.vue becomes the **expandable row content** for posting details
  - Refactor `src/components/ProductRow.vue`:
    - Keep the compact mode (8 columns, custom grid) as-is for backward compatibility
    - In enriched mode (when used inside NDataTable expanded row), display:
      - **Posting details**: The existing postings list layout (posting number, date, price, commission, delivery, returns, net) using the existing `PostingRow` component
      - **Expense breakdown**: Show the granular cost details for the product using `NDataTable` or `NDescriptions` inside the expanded area
    - The cost columns are rendered inline by `NDataTable` columns definition (not by ProductRow)
  - Postings are shown in the `expanded-row-render` of NDataTable
  - Keep `PostingRow.vue` unchanged

  **Must NOT do**:
  - Do NOT break the existing compact mode layout
  - Do NOT add inline expense bar changes — enriched table replaces the need
  - Do NOT add column toggles
  - Do NOT add custom CSS for the expanded area — Naive UI handles it
  - Columns that have 0.0 or no data should show `—` (em-dash)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Complex conditional rendering with many data fields
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 2
  - **Can Run In Parallel**: YES (with Task 10)
  - **Blocks**: Tasks 15, 18
  - **Blocked By**: Tasks 1, 5, 10 (needs Naive UI, uniteconomy data, NDataTable structure)

  **References**:
  - `src/components/ProductRow.vue` — Current implementation
  - `src/components/PostingRow.vue` — Keep unchanged
  - `src/utils.js` — For `formatRub` and `formatRubCompact`

  **Acceptance Criteria**:
  - [ ] Enriched mode shows all 20+ columns with formatted values
  - [ ] Compact mode unchanged
  - [ ] Zero/empty values show as `—`
  - [ ] Expandable postings still work
  - [ ] Profit values color-coded (green/red)

  **QA Scenarios**:
  ```
  Scenario: Verify enriched row display
    Tool: Playwright
    Preconditions: Task 9 complete, data loaded
    Steps:
      1. Navigate to Dashboard tab
      2. Assert product row shows all cost columns
      3. Assert numeric values are formatted (with ₽)
      4. Assert positive profit is green, negative is red
    Expected Result: Rows display correctly with all data
    Evidence: .omo/evidence/task-11-enriched-row.mp4
  ```

  **Commit**: YES (with Task 9)
  - Message: `feat(table): redesign product table and rows with enriched cost columns`
  - Files: `src/components/ProductTreeTable.vue`, `src/components/ProductRow.vue`

- [x] 12. Create `useStockAnalytics.js` composable

  **What to do**:
  - Create `src/composables/useStockAnalytics.js` following the pattern from `useDashboard.js`
  - Implement:
    - `month`, `year` refs
    - `data`, `loading`, `error` refs
    - `async function load()` that calls `invoke('get_stock_data', { month, year })`
    - `computed` values: `stockByWarehouse`, `stockByProduct`, `totalFreeToSell`, `totalReserved`
    - `watch([month, year], load)` for reactive reloading
    - Return: `{ month, year, data, loading, error, stockByWarehouse, stockByProduct, totalFreeToSell, totalReserved, refresh }`

  **Must NOT do**:
  - Do NOT share the same loading state with other composables (each view independent)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Standard composable pattern following existing code
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 2
  - **Can Run In Parallel**: YES (with Tasks 13, 14)
  - **Blocks**: Task 15
  - **Blocked By**: Tasks 3, 4 (needs Tauri commands for stock data)

  **References**:
  - `src/composables/useDashboard.js` — Full pattern to follow (refs, computed, watch, invoke)

  **Acceptance Criteria**:
  - [ ] Composable created with all expected refs/computed
  - [ ] `invoke('get_stock_data')` called correctly
  - [ ] `watch` on month/year triggers reload

  **QA Scenarios**:
  ```
  Scenario: Verify composable structure
    Tool: Bash
    Preconditions: Task 4 complete (analytics module exists)
    Steps:
      1. Read file — assert all required refs exist
      2. Check invoke name matches Task 2 command name
    Expected Result: File follows useDashboard.js pattern
    Evidence: .omo/evidence/task-12-exists.txt
  ```

  **Commit**: NO (group with Tasks 13, 14)

- [x] 13. Create `useTransactionTotals.js` composable

  **What to do**:
  - Create `src/composables/useTransactionTotals.js`
  - Same pattern as Task 12, but for `invoke('get_finance_totals_data', { month, year })`
  - Computed values: `accrualsForSale`, `totalCompensation`, `saleCommission`, `servicesAmount`, `processingAndDelivery`, `refundsAndCancellations`, `moneyTransfer`, `othersAmount`
  - Return: `{ month, year, data, loading, error, ...computedValues, refresh }`

  **Must NOT do**:
  - Same as Task 11

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Standard composable pattern
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 2
  - **Can Run In Parallel**: YES (with Tasks 12, 14)
  - **Blocks**: Task 16
  - **Blocked By**: Tasks 3, 4 (needs Tauri commands for finance data)

  **References**:
  - `src/composables/useDashboard.js` — Pattern to follow

  **Acceptance Criteria**:
  - [ ] Composable created with `invoke('get_finance_totals_data')`

  **Commit**: NO (group with Tasks 12, 14)

- [x] 14. Create `useAnalyticsDashboard.js` composable

  **What to do**:
  - Create `src/composables/useAnalyticsDashboard.js`
  - Same pattern as Task 12, but for `invoke('get_analytics_data', { month, year })`
  - Computed values: `products`, `overallAds`, `overallIdc`, `turnoverGrades` (dictionary of grade counts), `stockBalanceTotal`
  - Return: `{ month, year, data, loading, error, products, overallAds, overallIdc, turnoverGrades, stockBalanceTotal, refresh }`

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Standard composable pattern
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 2
  - **Can Run In Parallel**: YES (with Tasks 12, 13)
  - **Blocks**: Task 17
  - **Blocked By**: Tasks 3, 4 (needs Tauri commands for analytics data)

  **References**:
  - `src/composables/useDashboard.js` — Pattern to follow

  **Acceptance Criteria**:
  - [ ] Composable created with `invoke('get_analytics_data')`

  **Commit**: YES (with Tasks 12, 13)
  - Message: `feat(composables): add useStockAnalytics, useTransactionTotals, useAnalyticsDashboard`
  - Files: `src/composables/useStockAnalytics.js`, `src/composables/useTransactionTotals.js`, `src/composables/useAnalyticsDashboard.js`



- [x] 15. Build StockAnalytics.vue (NDataTable + NCard + NStatistic + NTag)

  **What to do**:
  - Create `src/components/StockAnalytics.vue` as a new view component using Naive UI
  - Use the `useStockAnalytics` composable (Task 12)
  - Layout:
    - **Summary cards** at top: use `NCard` + `NStatistic` for total free-to-sell, total reserved, total promised
    - **Data table**: use `NDataTable` with columns: product name, SKU, warehouse, free_to_sell, reserved, promised
    - **Loading state**: `NSpin` wrapping the content area
    - **Error state**: `NAlert` with `type="error"` and retry slot
    - **Empty state**: `NEmpty` with description prop
  - Use existing shared components (LoadingOverlay, ErrorBanner, EmptyState) only if Naive UI equivalents don't cover the use case
  - Apply view-container CSS classes from Task 8

  **Must NOT do**:
  - Do NOT add chart visualizations
  - Do NOT add warehouse filtering (beyond what API provides)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: New data-heavy view component
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 3
  - **Can Run In Parallel**: YES (with Tasks 16, 17)
  - **Blocks**: Task 18
  - **Blocked By**: Tasks 9, 12 (needs tab navigation in NTabs and composable)

  **References**:
  - `src/components/StatsBar.vue` — For the summary cards pattern
  - `src/components/ProductTreeTable.vue` — For table layout pattern
  - `src/composables/useStockAnalytics.js` — For data
  - CSS classes from Task 8

  **Acceptance Criteria**:
  - [ ] Stock Analytics view renders with summary cards and data table
  - [ ] Loading state shows spinner
  - [ ] Error state shows retryable error banner
  - [ ] Empty state shows appropriate message
  - [ ] Tab navigation switches to this view

  **QA Scenarios**:
  ```
  Scenario: Verify Stock Analytics loads and displays data
    Tool: Playwright
    Preconditions: App running, valid API config
    Steps:
      1. Navigate to Stocks tab
      2. Wait for data to load
      3. Assert summary cards visible at top
      4. Assert stock data table visible with rows
      5. Assert at least one warehouse column populated
    Expected Result: Stock data renders correctly
    Evidence: .omo/evidence/task-15-stocks.mp4

  Scenario: Verify error state
    Tool: Bash + Playwright
    Preconditions: Temporarily invalidate API key in config.json
    Steps:
      1. Invalidate API key
      2. Navigate to Stocks tab
      3. Assert error banner visible with retry button
    Expected Result: Error state shown gracefully
    Evidence: .omo/evidence/task-15-error.mp4
  ```

  **Commit**: NO (group with Tasks 16, 17)

- [x] 16. Build TransactionTotals.vue (NCard + NStatistic)

  **What to do**:
  - Create `src/components/TransactionTotals.vue` as a new view component using Naive UI
  - Use the `useTransactionTotals` composable (Task 13)
  - Layout:
    - **Metric cards grid**: use `NCard` containing `NStatistic` for each financial total
    - Grid: 4 columns (responsive), each card shows label + formatted value
    - Cards for: Accruals for sale, Compensation, Sale commission, Services, Processing & delivery, Refunds & cancellations, Other amounts, Net total
    - **Color coding**: Positive values use `NStatistic` with default color, negative use `type="error"`, net total uses `NCard` with `content-style` accent border
    - **Loading state**: `NSpin`
    - **Error state**: `NAlert`
    - **Empty state**: `NEmpty`

  **Must NOT do**:
  - Do NOT add charts
  - Do NOT add product-level breakdown (this is account-level totals only)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Dashboard-style metrics display
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 3
  - **Can Run In Parallel**: YES (with Tasks 15, 17)
  - **Blocks**: Task 18
  - **Blocked By**: Tasks 9, 13 (needs NTabs navigation and transaction composable)

  **References**:
  - `src/components/StatsBar.vue` — Card pattern (stat-card classes)
  - `src/composables/useTransactionTotals.js` — For data

  **Acceptance Criteria**:
  - [ ] Transaction Totals view renders with metric card grid
  - [ ] All 8 financial metrics displayed with proper formatting
  - [ ] Net total highlighted
  - [ ] Loading/error/empty states handled

  **QA Scenarios**:
  ```
  Scenario: Verify Transaction Totals displays
    Tool: Playwright
    Preconditions: App running, valid API config
    Steps:
      1. Navigate to Totals tab
      2. Wait for data to load
      3. Assert 8 metric cards visible
      4. Assert each card shows formatted ₽ value
    Expected Result: All financial metrics visible
    Evidence: .omo/evidence/task-16-totals.mp4
  ```

  **Commit**: NO (group with Tasks 15, 17)

- [x] 17. Build AnalyticsDashboard.vue (NDataTable + NCard + NTag for grade badges)

  **What to do**:
  - Create `src/components/AnalyticsDashboard.vue` as a new view component using Naive UI
  - Use the `useAnalyticsDashboard` composable (Task 14)
  - Layout:
    - **Summary stats**: `NCard` grid at top — total products (with desktop icon), average daily sales (ADS), average stock coverage (IDC)
    - **Grade distribution**: `NTag` badges per turnover grade — deficit (`type="error"`), popular (`type="success"`), actual (`type="info"`), surplus (`type="warning"`), no_sales (`type="default"`)
    - **Grade badge colors mapping**:
      - `deficit` → NTag `type="error"`
      - `popular` → NTag `type="success"`
      - `actual` → NTag `type="info"`
      - `surplus` → NTag `type="warning"`
      - `no_sales` → NTag `type="default"`
    - **Product table**: `NDataTable` with columns: product name, current stock, ADS, IDC days, turnover grade (with NTag badge), days without sales
    - **Loading**: `NSpin`
    - **Error**: `NAlert`
    - **Empty**: `NEmpty`

  **Must NOT do**:
  - Do NOT add chart visualizations (no chart library)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Mixed data view with stats and table
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 3
  - **Can Run In Parallel**: YES (with Tasks 15, 16)
  - **Blocks**: Task 18
  - **Blocked By**: Tasks 9, 14 (needs NTabs navigation and analytics composable)

  **References**:
  - `src/composables/useAnalyticsDashboard.js` — For data
  - `src/App.css` — CSS classes from Task 8

  **Acceptance Criteria**:
  - [ ] Analytics Dashboard renders with summary stats and product table
  - [ ] Turnover grade badges color-coded
  - [ ] Loading/error/empty states handled

  **QA Scenarios**:
  ```
  Scenario: Verify Analytics Dashboard displays
    Tool: Playwright
    Preconditions: App running, valid API config
    Steps:
      1. Navigate to Analytics tab
      2. Wait for data to load
      3. Assert summary stats visible
      4. Assert turnover grade badges visible
      5. Assert product table visible with rows
    Expected Result: Analytics data renders correctly
    Evidence: .omo/evidence/task-17-analytics.mp4
  ```

  **Commit**: YES (with Tasks 15, 16)
  - Message: `feat(views): add StockAnalytics, TransactionTotals, AnalyticsDashboard using Naive UI`
  - Files: `src/components/StockAnalytics.vue`, `src/components/TransactionTotals.vue`, `src/components/AnalyticsDashboard.vue`

- [x] 18. Wire up App.vue with NTabs tab switching + per-view loading states

  **What to do**:
  - Finalize `src/App.vue` integration:
    - Import all 4 view sections (Dashboard, Stocks, Totals, Analytics) and their composables
    - Each view section gets its own `v-if="activeView === 'viewname'"` block
    - Each view section has its own loading/error/data state (independent)
    - Dashboard still loads on mount via the existing `useDashboard()` composable
    - Other views load only when their tab is first activated (lazy-load pattern)
    - Use `v-show` for instant tab switching, `v-if` with `activated` tracking for lazy loading
    - Period picker (month/year) in DashboardHeader should affect the active view's composable
  - Handle edge cases:
    - Rapid tab switching: cancel in-flight requests for deactivated views
    - Concurrent month/year changes: use proper cleanup

  **Must NOT do**:
  - Do NOT add vue-router
  - Do NOT share state between views beyond the period picker

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Complex integration with multiple async data sources
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 3
  - **Can Run In Parallel**: NO — depends on all view components
  - **Blocks**: Tasks 19, 20, 21
  - **Blocked By**: Tasks 9, 15, 16, 17 (needs NTabs navigation and all 3 view components)

  **References**:
  - `src/App.vue` — Current app structure
  - `src/composables/useDashboard.js` — Existing composable pattern

  **Acceptance Criteria**:
  - [ ] All 4 views switch via tabs with independent loading/error states
  - [ ] Views lazy-load (no API calls for inactive views until first activation)
  - [ ] Period changes propagate to active view
  - [ ] No performance issues with rapid tab switching
  - [ ] Month/year picker works for all views

  **QA Scenarios**:
  ```
  Scenario: Verify independent loading states
    Tool: Playwright
    Preconditions: App running
    Steps:
      1. Observe Dashboard loading (should load on mount)
      2. Switch to Stocks tab before Dashboard finishes
      3. Assert Stocks shows its own loading state
      4. Assert Dashboard finishes loading in background
      5. Switch back to Dashboard — assert data is rendered
    Expected Result: Each view has independent state
    Evidence: .omo/evidence/task-18-loading.mp4

  Scenario: Verify lazy loading
    Tool: Playwright
    Preconditions: App running, monitor network
    Steps:
      1. Launch app
      2. Verify only get_dashboard_summary is called (not stock/totals/analytics)
      3. Navigate to Stocks tab
      4. Verify get_stock_data is called
      5. Navigate to Totals tab — previous calls not repeated
    Expected Result: Only active view loads data
    Evidence: .omo/evidence/task-18-lazy.mp4
  ```

  **Commit**: YES
  - Message: `feat(nav): wire up tab switching with per-view loading states`
  - Files: `src/App.vue`



- [x] 19. Add tests for Rust backend logic

  **What to do**:
  - Add Rust integration/unit tests to the key modules:
    - `uniteconomy.rs`: Add `#[cfg(test)] mod tests` with test functions for `extract_product_costs` and `extract_product_summary` using mock data
    - `analytics.rs`: Add tests for response parsing with mock API responses
    - `ozon.rs`: Add test for pagination logic (mock multiple pages)
  - Mock data: Create sample JSON responses that match the Ozon API schema to test extraction logic
  - Run `cargo test` to verify

  **Must NOT do**:
  - Do NOT add integration tests that call real Ozon API
  - Do NOT add end-to-end tests

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Test design with mock data
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 4
  - **Can Run In Parallel**: YES (with Task 20)
  - **Blocks**: None
  - **Blocked By**: Tasks 2, 4, 5, 6 (needs pagination fix, analytics module, uniteconomy, and command registration)

  **References**:
  - Ozon API response schemas (from lookup-operation.js calls)

  **Acceptance Criteria**:
  - [ ] `cargo test` passes with all new tests
  - [ ] Mock data tests cover: pagination, cost extraction, analytics response parsing

  **QA Scenarios**:
  ```
  Scenario: Rust tests pass
    Tool: Bash
    Preconditions: All Rust code changes complete
    Steps:
      1. cd src-tauri && cargo test
    Expected Result: All tests pass
    Evidence: .omo/evidence/task-19-cargo-test.txt
  ```

  **Commit**: NO (group with Task 20)

- [x] 20. Add tests for Vue components (with Naive UI mounted)

  **What to do**:
  - Set up basic test infrastructure (vitest + @vue/test-utils) in the project
  - Add test configuration to `package.json` scripts
  - Add basic component tests:
    - `ProductRow.spec.js`: Test both compact and enriched modes render
    - `ProductTreeTable.spec.js`: Test header renders for both modes
    - `StockAnalytics.spec.js`: Test rendering with mock data
    - `TransactionTotals.spec.js`: Test metric cards render
    - `AnalyticsDashboard.spec.js`: Test grade badges render
  - Each test: mount component with props, assert key elements exist

  **Must NOT do**:
  - Do NOT add full integration tests (use Playwright for e2e)
  - Do NOT test composables in isolation (test through components)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Vue component testing
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 4
  - **Can Run In Parallel**: YES (with Task 19)
  - **Blocks**: None
  - **Blocked By**: Tasks 10, 11, 15, 16, 17, 18 (needs all view components and their NDataTable/NCard implementations)

  **References**:
  - `src/components/*.vue` — All new/modified components

  **Acceptance Criteria**:
  - [ ] `vite build` succeeds
  - [ ] Component tests pass for all new/modified components

  **QA Scenarios**:
  ```
  Scenario: Vue tests pass
    Tool: Bash
    Preconditions: All Vue code changes complete
    Steps:
      1. npx vitest run
    Expected Result: All tests pass
    Evidence: .omo/evidence/task-20-vitest.txt
  ```

  **Commit**: YES (with Task 19)
  - Message: `test: add Rust backend and Vue component tests`
  - Files: Multiple test files

- [x] 21. Responsive polish for enriched table + new views

  **What to do**:
  - Review and polish responsive behavior:
    - Enriched product table: Ensure horizontal scroll works on narrow screens, sticky first column (product name)
    - Stock Analytics: Responsive card layout (2-col on medium, 1-col on small)
    - Transaction Totals: Responsive metric card grid
    - Analytics Dashboard: Responsive stats + table layout
    - Tab bar: Handle overflow on narrow screens (scrollable tabs or wrap)
  - Add responsive CSS breakpoints following existing patterns (900px, 640px)
  - Test all views at 1440px, 1024px, 768px, 375px widths

  **Must NOT do**:
  - Do NOT add mobile-specific navigation patterns (no hamburger menus)
  - Do NOT add CSS framework

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Responsive design polish
  - **Skills**: N/A

  **Parallelization**:
  - **Parallel Group**: Wave 4
  - **Can Run In Parallel**: NO — depends on all views being built
  - **Blocks**: None
  - **Blocked By**: Tasks 15, 16, 17, 18 (needs all views wired up with Naive UI)

  **References**:
  - `src/App.css` lines 572-604 — Existing responsive breakpoints
  - `src/styles/base.css` — CSS variables

  **Acceptance Criteria**:
  - [ ] All views render acceptably at 1440px, 1024px, 768px widths
  - [ ] Enriched table scrolls horizontally on narrow screens
  - [ ] Tab bar handles overflow (scrolls or wraps)
  - [ ] No layout breakage at any tested width

  **QA Scenarios**:
  ```
  Scenario: Verify responsive behavior
    Tool: Playwright
    Preconditions: App running
    Steps:
      1. Set viewport to 1440px
      2. Navigate through all 4 tabs — assert no layout breakage
      3. Set viewport to 768px
      4. Navigate through all 4 tabs — assert horizontal scroll on tables
      5. Set viewport to 375px
      6. Assert tab bar adapts (scrollable or wrapped)
    Expected Result: No layout breakage at any breakpoint
    Evidence: .omo/evidence/task-21-responsive.mp4
  ```

  **Commit**: YES
  - Message: `style: add responsive polish for enriched table and new views`
  - Files: `src/App.css`

---

## Final Verification Wave (MANDATORY)

- [x] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, curl endpoint, run command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .omo/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [x] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo build` and `vite build`. Review all changed files for: type suppression, empty catches, debug logging in prod, commented-out code, unused imports. Check AI slop: excessive comments, over-abstraction, generic names.
  Output: `Build [PASS/FAIL] | Lint [PASS/FAIL] | Files [N clean/N issues] | VERDICT`

- [x] F3. **Real Manual QA** — `unspecified-high` (+ `playwright` skill) [RESOLVED: Playwright v1.60.0 with cached Chromium browsers, mocked Tauri invoke via context.addInitScript] ✅
  Start from clean state. Execute EVERY QA scenario from EVERY task. Test cross-task integration. Test edge cases: empty state, invalid input, rapid tab switching. Save to `.omo/evidence/final-qa/`.
  Output: `Scenarios [N/N pass] | Integration [N/N] | Edge Cases [N tested] | VERDICT`

- [x] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff. Verify 1:1 — everything in spec was built, nothing beyond was added. Check "Must NOT do" compliance. Detect cross-task contamination.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **1-2**: `feat(ui): add Naive UI with auto-import, NConfigProvider theme, and fix pagination`
- **3-4**: `feat(api): add new Ozon API wrappers and analytics module`
- **5-7**: `feat(costs): add uniteconomy module, register commands, add i18n keys`
- **8**: `style: add view container CSS for Naive UI compatibility`
- **9-11**: `feat(tabs): add NTabs navigation and NDataTable enriched product table`
- **12-14**: `feat(composables): add useStockAnalytics, useTransactionTotals, useAnalyticsDashboard`
- **15-17**: `feat(views): add StockAnalytics, TransactionTotals, AnalyticsDashboard using Naive UI`
- **18**: `feat(nav): wire up tab switching with per-view loading states`
- **19-20**: `test: add Rust backend and Vue component tests`
- **21**: `style: add responsive polish for enriched table and new views`
- **F1-F4**: `chore: final verification cleanup`

---

## Success Criteria

### Verification Commands
```bash
cargo build  # Expected: Compiling src-tauri... Finished
vite build   # Expected: ✓ built in Xs
```

### Final Checklist
- [ ] All "Must Have" present and verified
- [ ] All "Must NOT Have" absent (search for forbidden patterns)
- [ ] All tests pass
- [ ] All QA scenarios pass with evidence in `.omo/evidence/`
