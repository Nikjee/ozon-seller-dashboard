<script setup>
import { computed, h, onMounted } from 'vue'
import { NTag, NCard, NStatistic, NDataTable, NSpin, NAlert, NEmpty, NButton } from 'naive-ui'
import { useI18n } from '../composables/useI18n.js'
import { useColumnSettings } from '../composables/useColumnSettings.js'
import { useAnalyticsDashboard } from '../composables/useAnalyticsDashboard.js'
import { formatRub, formatRubCompact, formatInt } from '../utils.js'

const { t } = useI18n()
const { data, loading, error, products, overallAds, overallIdc, turnoverGrades, stockBalanceTotal, refresh } = useAnalyticsDashboard()

onMounted(() => refresh())

const maxHeight = computed(() => {
    return window.innerHeight - 320;
});

function gradeTagType(grade) {
  const map = { deficit: 'error', popular: 'success', actual: 'info', surplus: 'warning', no_sales: 'default', high: 'success', medium: 'warning', low: 'error' }
  return map[grade] || 'default'
}

const filteredGrades = computed(() => {
  const raw = turnoverGrades.value || {}
  return Object.fromEntries(Object.entries(raw).filter(([g]) => g !== 'COLLECTING_DATA'))
})

const R_AN = { resizable: true, minWidth: 60, sorter: 'default' }

const analyticsColumns = computed(() => [
  { title: t('analytics.colName'), key: 'name', width: 200, ...R_AN },
  { title: t('analytics.colSku'), key: 'sku', width: 100, ...R_AN },
  { title: t('analytics.colAvailableStock'), key: 'available_stock_count', ...R_AN, render: (row) => formatInt(row.available_stock_count) },
  { title: t('analytics.colAds'), key: 'ads', ...R_AN, render: (row) => formatRubCompact(row.ads) },
  { title: t('analytics.colIdc'), key: 'idc', ...R_AN, render: (row) => formatRubCompact(row.idc) },
  { 
    title: t('analytics.colTurnoverGrade'), 
    key: 'turnover_grade',
    ...R_AN,
    render(row) {
      return h(NTag, { type: gradeTagType(row.turnover_grade) }, () => row.turnover_grade)
    }
  },
  { title: t('analytics.colDaysWithoutSales'), key: 'days_without_sales', ...R_AN, render: (row) => formatInt(row.days_without_sales) },
])

const { columnVisibility, showColumnSettings, visibleColumns, toggleColumn, settingsColumn } = useColumnSettings('analytics', analyticsColumns, () => t('table.columns'))

const displayColumns = computed(() => [...visibleColumns.value, settingsColumn.value])
const scrollX = computed(() =>
  displayColumns.value.reduce((sum, col) => sum + (col.width || 80), 0) + 20
)
</script>

<template>
  <div class="view-container">
    <div class="view-header">
      <h2>{{ t('analytics.title') }}</h2>
      <details class="view-info">
        <summary>{{ t('info.analytics') }}</summary>
        <p>{{ t('info.analyticsContent') }}</p>
      </details>
    </div>
    <div class="view-content">
      <n-spin :show="loading">
        <template v-if="error">
          <n-alert type="error" closable>
            <template #header>{{ t('analytics.error') }}</template>
            {{ error }}
          </n-alert>
          <n-button @click="refresh" style="margin-top: 12px">{{ t('analytics.retry') }}</n-button>
        </template>
        <template v-else-if="data">
          <!-- Summary stats -->
          <div class="stats-grid">
            <n-card><n-statistic :label="t('analytics.stockBalance')" :value="formatInt(stockBalanceTotal)" /></n-card>
            <n-card><n-statistic :label="t('analytics.overallAds')" :value="formatRub(overallAds)" /></n-card>
            <n-card><n-statistic :label="t('analytics.overallIdc')" :value="formatRub(overallIdc)" /></n-card>
          </div>
          
          <!-- Grade distribution badges -->
          <div v-if="Object.keys(filteredGrades).length" class="grade-badges" style="margin-bottom: 16px; display: flex; gap: 8px; flex-wrap: wrap;">
            <n-tag v-for="(count, grade) in filteredGrades" :key="grade" :type="gradeTagType(grade)">
              {{ grade }}: {{ count }}
            </n-tag>
          </div>
          
          <!-- Product table -->
          <div class="table-wrapper">
            <n-data-table 
              :columns="displayColumns" 
              :data="products" 
              :bordered="false" 
              :single-line="true" 
              :max-height="maxHeight"
              :scroll-x="scrollX"
            />
          </div>
        </template>
        <n-empty v-else :description="t('noData')" />
      </n-spin>
    </div>
  </div>
</template>

<style scoped>
.table-wrapper {
  position: relative;
}
</style>