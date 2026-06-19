# Components

Flat UI layer: 14 SFCs, naive-ui + Catppuccin CSS vars, pure JS Composition API.

## Component Index

| Component | Role | Key naive-ui |
|-----------|------|-------------|
| ProductTreeTable | Dual-mode product table (compact list / n-data-table enriched) | `n-data-table` |
| ProductRow | Compact row with chevron expand + enriched cost-breakdown grid | `h()` render |
| AnalyticsDashboard | Analytics table with NTag turnover grades, stat cards, help popover | `n-data-table` `NTag` `NStatistic` `NPopover` |
| StockAnalytics | Stock-by-warehouse table with summary cards | `n-data-table` |
| TransactionTotals | Finance totals in n-statistic cards grid | `NStatistic` `NCard` |
| PostingRow | Single posting sub-row (number, date, amounts) | none |
| StatsBar | Summary stat cards (revenue, expenses, profit) | none |
| AccountExpensesPanel | Collapsible expense breakdown by category (ad/logistics/storage) | none |
| DashboardHeader | App header: title, MonthPicker, lang toggle, DarkModeToggle, refresh | none |
| MonthPicker | Month/year selector with arrow navigation and year wrap | none |
| DarkModeToggle | Theme toggle button (sun/moon) | none |
| ErrorBanner | Error display with retry button | none |
| EmptyState | No-data placeholder | none |
| LoadingOverlay | Spinner overlay | none |

## Key Patterns

- **Dual-mode rendering**: ProductTreeTable/ProductRow switch on `enriched` prop. `!enriched` = custom HTML table with chevron expand. `enriched` = `n-data-table` with `expandedRowRender` returning `h(ProductRow)`. ProductRow also branches: compact list vs cost-breakdown grid.
- **Locale-reactive columns**: Column definitions use `t()` inside `computed(() => [...])` so headers retranslate on locale switch. Dot-path keys (`costs.commission`) with `numRender`/`profitRender` traverse nested objects.
- **Custom renderers**: `numRender` does dot-path traversal + `formatRubCompact`. `profitRender` adds green/red class. AnalyticsDashboard uses `h(NTag, { type: gradeTagType(grade) })` for turnover badges.
- **Cost items as data**: ProductRow defines `costItems` as a computed array of `{ key, label, value }` objects, not inline template. Same pattern in AccountExpensesPanel for `cats`.
- **BEM-like classes**: `product-row__main`, `product-row--expanded`, `enriched-costs__grid`, `account-expenses__cat-dot`. Catppuccin CSS vars: `--ctp-mauve`, `--ctp-peach`, `--ctp-blue`, `--ctp-teal`, `--ctp-muted`.
- **Error/loading pattern**: Most views use `n-spin` + `n-alert` + `n-button(retry)` + `n-empty`. ErrorBanner is standalone alternative.

## Where to Look

| Task | File |
|------|------|
| Add/remove cost columns | `ProductTreeTable.vue` columns array + `ProductRow.vue` costItems computed |
| Change turnover grade colors | `AnalyticsDashboard.vue` gradeTagType map |
| Add new stat card | `StatsBar.vue` or `AnalyticsDashboard.vue` stats-grid section |
| Add new view/tab | New component + wire in `App.vue` n-tabs |
| Change expense categories | `ProductRow.vue` expenseCats + `AccountExpensesPanel.vue` cats |
| i18n key for any label | `useI18n.js` flat key map, `t('key')` in templates |