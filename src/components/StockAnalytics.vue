<script setup>
import { computed, onMounted } from 'vue'
import { NButton, NDataTable, NSpin, NAlert, NEmpty, NCard, NStatistic } from 'naive-ui'
import { useI18n } from '../composables/useI18n.js'
import { useColumnSettings } from '../composables/useColumnSettings.js'
import { useStockAnalytics } from '../composables/useStockAnalytics.js'
import { formatInt } from '../utils.js'

const { t } = useI18n()
const { month, year, data, loading, error, stockByWarehouse, stockByProduct, totalFreeToSell, totalReserved, refresh } = useStockAnalytics()

onMounted(() => refresh())

const maxHeight = computed(() => {
    return window.innerHeight - 280;
});

const R_ST = { resizable: true, minWidth: 60, sorter: 'default' }

const columns = computed(() => [
  { title: t('stocks.warehouse'), key: 'warehouse_name', ...R_ST },
  { title: t('stocks.stockType'), key: 'stock_type', ...R_ST },
  { title: t('stocks.freeStock'), key: 'free_to_sell', ...R_ST, render: (row) => formatInt(row.free_to_sell) },
  { title: t('stocks.reservedStock'), key: 'reserved', ...R_ST, render: (row) => formatInt(row.reserved) },
  { title: t('stocks.totalStock'), key: 'total', ...R_ST, render: (row) => formatInt(row.total) },
])

const { columnVisibility, showColumnSettings, visibleColumns, toggleColumn, settingsColumn } = useColumnSettings('stocks', columns, () => t('table.columns'))

const displayColumns = computed(() => [...visibleColumns.value, settingsColumn.value])
const scrollX = computed(() =>
  displayColumns.value.reduce((sum, col) => sum + (col.width || 80), 0) + 20
)
</script>

<template>
  <div class="view-container">
    <div class="view-header">
      <h2>{{ t('stocks.title') }}</h2>
      <details class="view-info">
        <summary>{{ t('info.stocks') }}</summary>
        <p>{{ t('info.stocksContent') }}</p>
      </details>
    </div>
    <div class="view-content">
      <n-spin :show="loading">
        <template v-if="error">
          <n-alert type="error" closable>
            <template #header>Error</template>
            {{ error }}
          </n-alert>
          <n-button @click="refresh" style="margin-top: 12px">Retry</n-button>
        </template>
        <template v-else-if="data">
          <!-- Summary cards -->
          <div class="stats-grid">
            <n-card><n-statistic :label="t('stocks.freeStock')" :value="formatInt(totalFreeToSell)" /></n-card>
            <n-card><n-statistic :label="t('stocks.reservedStock')" :value="formatInt(totalReserved)" /></n-card>
          </div>
          <!-- Stock data table -->
          <div class="table-wrapper">
            <n-data-table :columns="displayColumns" :data="stockByWarehouse" :bordered="false" :single-line="true" :max-height="maxHeight" :scroll-x="scrollX" />
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
