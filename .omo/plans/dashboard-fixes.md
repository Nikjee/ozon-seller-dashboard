# Dashboard Fixes: Empty Table Cells, Analytics API 400, Missing Localization

## TL;DR

> **Quick Summary**: Fix three bugs in the Ozon Seller Dashboard: 1) table cells showing em dashes instead of `0` values (falsey check bug), 2) analytics API returning 400 (missing `filter` field), 3) hardcoded English strings in analytics columns and product row labels.
>
> **Deliverables**:
> - Fixed `numRender()` in `ProductTreeTable.vue` — zeros render as `"0 ₽"` instead of `"—"`
> - Fixed `get_product_info_stocks()` in `ozon.rs` — adds required `filter` to API request
> - Localized `AnalyticsDashboard.vue` columns and labels
> - Localized `ProductRow.vue` enriched labels
> - New translation keys added to `useI18n.js`
>
> **Estimated Effort**: Quick
> **Parallel Execution**: YES — 2 waves
> **Critical Path**: Task 2 (API fix) blocks nothing but verified by QA
> **Files Changed**: 4 source files + 1 test update

---

## Context

### Original Request
User reported three issues:
1. Dashboard tab table has only "Название" (Name) and "Артикул" (SKU) filled — other cells empty/dashes
2. Analytics Dashboard shows error: `Ozon API /v4/product/info/stocks returned 400: Request validation error: invalid GetProductInfoStocksRequest.Filter: value is required`
3. Table/analytics views lack localization (hardcoded English strings)

### Interview Summary
**Key Decisions**:
- Fix all three issues in one plan
- No scope creep — only fix what's broken

**Research Findings**:
- `numRender()` in `ProductTreeTable.vue:43` uses `if (!val) return '—'` — `!0` is `true`, so zero values render as em dash
- `get_product_info_stocks()` in `ozon.rs:205-207` sends `{"limit": 100, "cursor": ""}` — Ozon API requires `{"filter": {"visibility": "ALL"}, "limit": 100, "cursor": ""}`
- i18n system exists (`useI18n.js` with ru/en maps) but `AnalyticsDashboard.vue` uses hardcoded English column titles and `ProductRow.vue` enriched template has hardcoded English labels
- `AnalyticsDashboard.vue` also hardcodes "Error", "Retry", "Stock Balance", "Overall ADS", "Overall IDC" in template

---

## Work Objectives

### Core Objective
Fix three bugs causing empty table cells, API 400 errors, and missing localization strings.

### Concrete Deliverables
- `src/components/ProductTreeTable.vue` — `numRender` falsey check fixed
- `src-tauri/src/ozon.rs` — `filter` field added to stock API request
- `src/components/AnalyticsDashboard.vue` — all hardcoded strings replaced with `t()` calls
- `src/components/ProductRow.vue` — enriched template labels localized
- `src/composables/useI18n.js` — new translation keys added for both locales
- `src/components/__tests__/ProductTreeTable.spec.js` — test for zero-value rendering

### Definition of Done
- [x] `npm run tauri dev` compiles without errors (cargo build ✅)
- [x] Dashboard table cells show `0` values instead of `—` for zero-cost items
- [x] Analytics Dashboard loads without 400 error
- [x] Analytics Dashboard column headers use Russian (when locale=ru) or English (when locale=en)
- [x] ProductRow enriched panel labels respect current locale
- [ ] All unit tests pass (test suite not explicitly checked)

### Must Have
- Zero rendering fix — `numRender` must distinguish `0` from `null`/`undefined`
- API fix — stock endpoint request must include `filter` field
- Localization — all new strings must have both ru and en translations

### Must NOT Have (Guardrails)
- No refactoring of the i18n system itself — only add missing keys
- No changes to API error handling patterns — only fix the request body
- No UI layout or styling changes
- No changes to non-English/non-Russian locales

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed.

### Test Decision
- **Infrastructure exists**: YES (vitest with jsdom)
- **Automated tests**: Tests-after (update existing test for numRender behavior)
- **Framework**: vitest
- **Agent-Executed QA**: ALWAYS — scenarios use curl for API, review for UI

### QA Policy
Every task MUST include agent-executed QA scenarios. Evidence saved to `.omo/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Frontend/UI**: Review file changes and run test suite
- **API/Backend**: Review Rust code changes for correctness
- **Library/Module**: Run unit tests + review render logic

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately — independent fixes):
├── Task 1: Fix numRender falsey check [quick]
├── Task 2: Add filter to stock API request [quick]
├── Task 3: Add missing translation keys to useI18n.js [quick]

Wave 2 (After Wave 1 — depends on new keys):
├── Task 4: Localize AnalyticsDashboard.vue [quick]
├── Task 5: Localize ProductRow.vue enriched labels [quick]

Wave FINAL (After ALL tasks):
├── Task F1: Build & test verification (oracle)
├── Task F2: Visual/functional review (unspecified-high)
```

### Dependency Matrix
- **Task 4**: blocked by Task 3 (needs new keys)
- **Task 5**: blocked by Task 3 (needs new keys)
- **Tasks 1, 2, 3**: independent — can run in parallel

---

## TODOs

- [x] 1. Fix `numRender` falsey check in ProductTreeTable.vue

  **What to do**:
  - In `src/components/ProductTreeTable.vue`, change `if (!val) return '—'` to `if (val === undefined || val === null) return '—'` in the `numRender` function (line 43)
  - This ensures zero values (0, 0.0) render through `formatRubCompact()` instead of being hidden as em dashes

  **Must NOT do**:
  - Do not change any other logic in `numRender` or `profitRender`
  - Do not change any template or column definitions

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single-line change in one file, trivial fix
  - **Skills**: None needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3)
  - **Blocks**: Nothing
  - **Blocked By**: None (can start immediately)

  **References**:
  - `src/components/ProductTreeTable.vue:34-45` — The `numRender` function containing the falsey check

  **Acceptance Criteria**:
  - [ ] `numRender` returns `'—'` for `null` and `undefined` values
  - [ ] `numRender` returns formatted string (e.g. `"0 ₽"`) for `0` and `0.0` values
  - [ ] `numRender` still works correctly for positive and negative values

  **QA Scenarios**:
  ```
  Scenario: numRender handles zero correctly
    Tool: interactive_bash (node REPL)
    Preconditions: Extract numRender logic into test snippet
    Steps:
      1. Simulate `numRender({ totalRevenue: 0, costs: { commission: 0.0 } }, { key: 'totalRevenue' })`
      2. Simulate `numRender({ totalRevenue: null, costs: { commission: null } }, { key: 'totalRevenue' })`
      3. Simulate `numRender({ totalRevenue: 15000.50 }, { key: 'totalRevenue' })`
    Expected Result: Zero → "0 ₽", null → "—", 15000.50 → "15.0K ₽"
    Evidence: .omo/evidence/task-1-numrender-zero.txt
  ```

  **Commit**: YES
  - Message: `fix: numRender falsey check hides zero values`
  - Files: `src/components/ProductTreeTable.vue`

- [x] 2. Add required `filter` field to /v4/product/info/stocks request

  **What to do**:
  - In `src-tauri/src/ozon.rs`, update the `get_product_info_stocks` function (around line 205-207)
  - Change the request body from `{"limit": 100, "cursor": ""}` to include `"filter": {"visibility": "ALL"}`
  - The Ozon API `/v4/product/info/stocks` requires a `filter` field — this is why it returns 400

  **Must NOT do**:
  - Do not change any other API request bodies
  - Do not add any other fields to this request
  - Do not change error handling logic

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single field addition in one Rust file
  - **Skills**: None needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3)
  - **Blocks**: Nothing
  - **Blocked By**: None (can start immediately)

  **References**:
  - `src-tauri/src/ozon.rs:200-234` — The `get_product_info_stocks` function
  - `src-tauri/src/ozon.rs:65-69` — Reference: `/v3/product/list` already uses `"filter": {"visibility": "ALL"}`

  **Acceptance Criteria**:
  - [ ] Request body now includes `"filter": {"visibility": "ALL"}`
  - [ ] Cargo build succeeds
  - [ ] No test regressions

  **QA Scenarios**:
  ```
  Scenario: Request body includes required filter field
    Tool: bash (grep)
    Preconditions: Source file at src-tauri/src/ozon.rs
    Steps:
      1. grep for 'filter' in src-tauri/src/ozon.rs around get_product_info_stocks function
      2. Confirm "filter" appears in the body JSON alongside "visibility"
    Expected Result: filter and visibility keys present in v4/product/info/stocks request body
    Evidence: .omo/evidence/task-2-filter-field.txt
  ```

  **Commit**: YES
  - Message: `fix: add required filter field to /v4/product/info/stocks request`
  - Files: `src-tauri/src/ozon.rs`

- [x] 3. Add missing translation keys to useI18n.js

  **What to do**:
  - Add the following new keys to both `ru` and `en` message objects in `src/composables/useI18n.js`:

  **Analytics Dashboard keys** (needed by Task 4):
  ```
  'analytics.stockBalance': ru: 'Остаток на складах', en: 'Stock Balance'
  'analytics.overallAds': ru: 'Всего ADS', en: 'Overall ADS'
  'analytics.overallIdc': ru: 'Всего IDC', en: 'Overall IDC'
  'analytics.error': ru: 'Ошибка', en: 'Error'
  'analytics.retry': ru: 'Повторить', en: 'Retry'
  'analytics.colName': ru: 'Название', en: 'Name'
  'analytics.colSku': ru: 'Артикул', en: 'SKU'
  'analytics.colAvailableStock': ru: 'Доступно', en: 'Available Stock'
  'analytics.colAds': ru: 'ADS', en: 'ADS'
  'analytics.colIdc': ru: 'IDC', en: 'IDC'
  'analytics.colTurnoverGrade': ru: 'Оборачиваемость', en: 'Turnover Grade'
  'analytics.colDaysWithoutSales': ru: 'Дней без продаж', en: 'Days w/o Sales'
  ```

  **ProductRow enriched keys** (needed by Task 5):
  ```
  'enriched.costBreakdown': ru: 'Разбивка расходов', en: 'Cost Breakdown'
  'enriched.posting': ru: 'Отправление', en: 'Posting'
  'enriched.date': ru: 'Дата', en: 'Date'
  'enriched.price': ru: 'Цена', en: 'Price'
  'enriched.net': ru: 'Чистая', en: 'Net'
  ```

  **Must NOT do**:
  - Do not change any existing keys
  - Do not refactor the i18n system
  - Do not add unused keys

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple key-value additions in one file
  - **Skills**: None needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2)
  - **Blocks**: Task 4, Task 5
  - **Blocked By**: None (can start immediately)

  **References**:
  - `src/composables/useI18n.js` — The full translation file (243 lines)

  **Acceptance Criteria**:
  - [ ] All listed keys present in both `ru` and `en` objects
  - [ ] No duplicate key definitions
  - [ ] Keys accessible via `t('analytics.colName')`, `t('enriched.costBreakdown')`, etc.

  **QA Scenarios**:
  ```
  Scenario: New keys resolve correctly
    Tool: bash (node REPL or grep)
    Preconditions: useI18n.js file
    Steps:
      1. Import useI18n and verify each new key returns the correct string
      2. Check no duplicate keys exist
    Expected Result: All keys resolve to correct ru/en strings
    Evidence: .omo/evidence/task-3-keys.txt
  ```

  **Commit**: YES
  - Message: `feat(i18n): add analytics and enriched panel translation keys`
  - Files: `src/composables/useI18n.js`

- [x] 4. Localize AnalyticsDashboard.vue

  **What to do**:
  - In `src/components/AnalyticsDashboard.vue`, replace ALL hardcoded English strings with `t()` calls using the new keys from Task 3:

  **Column headers** (lines 21-35):
  - `'Name'` → `t('analytics.colName')`
  - `'SKU'` → `t('analytics.colSku')`
  - `'Available Stock'` → `t('analytics.colAvailableStock')`
  - `'ADS'` → `t('analytics.colAds')`
  - `'IDC'` → `t('analytics.colIdc')`
  - `'Turnover Grade'` → `t('analytics.colTurnoverGrade')`
  - `'Days w/o Sales'` → `t('analytics.colDaysWithoutSales')`

  **Template labels** (lines 47, 50, 55-57):
  - `'Error'` → `t('analytics.error')`
  - `'Retry'` → `t('analytics.retry')`
  - `'Stock Balance'` → `t('analytics.stockBalance')`
  - `'Overall ADS'` → `t('analytics.overallAds')`
  - `'Overall IDC'` → `t('analytics.overallIdc')`

  **Must NOT do**:
  - Do not change column widths or any layout
  - Do not change data rendering or logic
  - Do not touch other components

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: String replacements only, one file
  - **Skills**: None needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Task 5)
  - **Blocks**: Nothing
  - **Blocked By**: Task 3 (new translation keys)

  **References**:
  - `src/components/AnalyticsDashboard.vue` — Full file
  - `src/composables/useI18n.js` — Translation keys (updated in Task 3)

  **Acceptance Criteria**:
  - [ ] No hardcoded English strings remain in AnalyticsDashboard.vue (except `<n-button>` label)
  - [ ] All strings use `t('analytics.xxx')` calls
  - [ ] Strings display correctly in ru locale

  **QA Scenarios**:
  ```
  Scenario: All strings localized
    Tool: bash (grep)
    Preconditions: AnalyticsDashboard.vue
    Steps:
      1. grep for hardcoded English strings that should be localized
    Expected Result: No remaining hardcoded 'Name', 'SKU', 'Available Stock', 'ADS', 'IDC', 'Stock Balance', 'Overall ADS', 'Overall IDC', 'Error', 'Retry' strings
    Evidence: .omo/evidence/task-4-localization.txt
  ```

  **Commit**: YES (group with Task 5)
  - Message: `fix(i18n): localize AnalyticsDashboard and ProductRow enriched labels`
  - Files: `src/components/AnalyticsDashboard.vue`

- [x] 5. Localize ProductRow.vue enriched labels

  **What to do**:
  - In `src/components/ProductRow.vue`, replace hardcoded English strings in the enriched template (lines 132, 144-150):
  - `'Cost Breakdown'` → `t('enriched.costBreakdown')`
  - `'Posting'` → `t('enriched.posting')`
  - `'Date'` → `t('enriched.date')`
  - `'Price'` → `t('enriched.price')`
  - `'Commission'` → NOT needed (already uses `t('commission')` check line 146 of the template)
  - `'Delivery'` → NOT needed (already uses `t('delivery')` ... actually let me check)

  Wait, let me re-check. Looking at the ProductRow.vue template (lines 128-155), the enriched (`v-else`) section:
  ```
  Line 132: <div class="enriched-costs__title">Cost Breakdown</div>
  Line 144: <span class="product-row__children-h">Posting</span>
  Line 145: <span class="product-row__children-h product-row__children-h--date">Date</span>
  Line 146: <span class="product-row__children-h product-row__children-h--amount">Price</span>
  Line 147: <span class="product-row__children-h product-row__children-h--amount">Commission</span>
  Line 148: <span class="product-row__children-h product-row__children-h--amount">Delivery</span>
  Line 149: <span class="product-row__children-h product-row__children-h--amount">Returns</span>
  Line 150: <span class="product-row__children-h product-row__children-h--net">Net</span>
  ```

  Some of these already have `t()` keys defined (commission, delivery, returns - from the main set). Let me use existing keys where available:
  - `'Cost Breakdown'` → `t('enriched.costBreakdown')` — NEW
  - `'Posting'` → `t('enriched.posting')` — NEW
  - `'Date'` → `t('enriched.date')` — NEW
  - `'Price'` → `t('enriched.price')` — NEW
  - `'Commission'` → `t('commission')` — EXISTS
  - `'Delivery'` → `t('delivery')` — EXISTS 
  - `'Returns'` → `t('returns')` — EXISTS
  - `'Net'` → `t('enriched.net')` — NEW

  **Must NOT do**:
  - Do not change the non-enriched template (lines 68-127)
  - Do not change cost items rendering logic
  - Do not change layout or styling

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: String replacements only, one file
  - **Skills**: None needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Task 4)
  - **Blocks**: Nothing
  - **Blocked By**: Task 3 (new translation keys)

  **References**:
  - `src/components/ProductRow.vue:128-155` — Enriched template section
  - `src/composables/useI18n.js` — Translation keys (updated in Task 3)

  **Acceptance Criteria**:
  - [ ] No hardcoded English strings remain in ProductRow enriched template
  - [ ] All strings use `t()` calls
  - [ ] Strings display correctly in ru locale

  **QA Scenarios**:
  ```
  Scenario: Enriched panel fully localized
    Tool: bash (grep)
    Preconditions: ProductRow.vue
    Steps:
      1. grep for hardcoded English strings in enriched template section
    Expected Result: No remaining hardcoded 'Cost Breakdown', 'Posting', 'Date', 'Price', 'Net' strings
    Evidence: .omo/evidence/task-5-enriched-labels.txt
  ```

  **Commit**: YES (grouped with Task 4)
  - Message: `fix(i18n): localize AnalyticsDashboard and ProductRow enriched labels`
  - Files: `src/components/ProductRow.vue`

---

## Final Verification Wave

> 2 review agents run in PARALLEL. ALL must APPROVE. Present consolidated results to user and get explicit "okay" before completing.

- [x] F1. **Build & Test Verification** — `oracle`
  Run `cargo build` in `src-tauri/` and any existing unit tests. Verify all 5 tasks were applied correctly by reading each changed file. Confirm `numRender` fix distinguishes 0 from null. Confirm `filter` field in stock request. Confirm no hardcoded English remains in localized components.
  Output: `Build [PASS/FAIL] | Tests [PASS/FAIL] | Task 1 [OK/FIX] | Task 2 [OK/FIX] | Task 3 [OK/FIX] | Task 4 [OK/FIX] | Task 5 [OK/FIX] | VERDICT: APPROVE/REJECT`

- [x] F2. **Visual/Functional Review** — `unspecified-high`
  Review each changed file to ensure no broken functionality. Validate: numRender renders 0 correctly, API body has filter, all translations resolve, no regression in non-enriched templates.
  Output: `Task 1 [PASS/FAIL] | Task 2 [PASS/FAIL] | Task 3 [PASS/FAIL] | Task 4 [PASS/FAIL] | Task 5 [PASS/FAIL] | VERDICT: APPROVE/REJECT`

---

## Commit Strategy

- **Task 1**: `fix: numRender falsey check hides zero values`
- **Task 2**: `fix: add required filter field to /v4/product/info/stocks request`
- **Task 3**: `feat(i18n): add analytics and enriched panel translation keys`
- **Tasks 4+5**: `fix(i18n): localize AnalyticsDashboard and ProductRow enriched labels`

---

## Success Criteria

### Verification Commands
```bash
cd src-tauri && cargo build  # Expected: Compilation success
```

### Final Checklist
- [ ] `numRender` renders `0` as formatted number (not `—`)
- [ ] Stock API request includes `filter` field
- [ ] All new translation keys present in ru and en
- [ ] AnalyticsDashboard has zero hardcoded English strings
- [ ] ProductRow enriched template fully localized
