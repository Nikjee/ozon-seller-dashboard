# Composables — Data Access Layer

## COMPOSABLE INDEX

| File | Tauri Command | Returns |
|------|--------------|---------|
| useDashboard | `get_dashboard_summary` | month/year picker, totals, products, accountExpenses |
| useAnalyticsDashboard | `get_analytics_dashboard_data` | products, overallAds, overallIdc, turnoverGrades, stockBalanceTotal |
| useStockAnalytics | `get_stock_report` | stockByWarehouse, stockByProduct, totalFreeToSell, totalReserved |
| useTransactionTotals | `get_finance_totals` | accrualsForSale, saleCommission, moneyTransfer, etc. (camelCase + snake_case fallback) |
| useConfig | `check_config`, `save_config` | configValid, configMessage, saving |
| useUpdater | `get_updater_token` + plugin-updater | checking, updateVersion, downloading |
| useI18n | none (local state) | locale, t(), toggle |
| useTheme | none (local state) | theme, toggle |

## PATTERNS

- **invoke→ref**: `const data = ref(null)` → `data.value = await invoke('cmd', args)` inside async `load()`/`refresh()`
- **Error normalization**: `typeof e === 'string' ? e : (e.message || 'Unknown error')` — useI18n and useUpdater skip this
- **Auto-fetch on param change**: `watch([month, year], load)` in useDashboard, useStockAnalytics, useTransactionTotals
- **Module-level singletons**: useI18n (`locale`) and useTheme (`theme`) use module-scoped refs, shared across all consumers
- **useI18n keys**: flat dot-notation (`'analytics.colAds'`), no nesting. `t(key)` returns key itself as fallback
- **useTransactionTotals**: dual-access computed props (`data.value?.accrualsForSale ?? data.value?.accruals_for_sale`) for snake/camel API variance

## WHERE TO LOOK

- Add new API data → create composable, follow invoke→ref pattern, return `{ data, loading, error, refresh }`
- Add i18n key → flat key in both `messages.ru` and `messages.en` objects in useI18n.js
- Change theme → useTheme.js, `data-theme` attribute on `<html>`