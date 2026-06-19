<script setup>
import { computed, onMounted, ref } from 'vue'
import { NButton, NCheckbox, NSpace, NPopover, NDataTable, NSpin, NAlert, NEmpty, NCard, NStatistic } from 'naive-ui'
import { useI18n } from '../composables/useI18n.js'
import { useStockAnalytics } from '../composables/useStockAnalytics.js'
import { formatInt } from '../utils.js'

const { t } = useI18n()
const { month, year, data, loading, error, stockByWarehouse, stockByProduct, totalFreeToSell, totalReserved, refresh } = useStockAnalytics()

onMounted(() => refresh())

const maxHeight = computed(() => {
    return window.innerHeight - 280;
});

const R_ST = { resizable: true, minWidth: 60, sorter: 'default' }
const columnVisibility = ref({})
const showColumnSettings = ref(false)

function toggleColumn(key) {
  const current = columnVisibility.value[key]
  columnVisibility.value = { ...columnVisibility.value, [key]: current === undefined ? false : !current }
}

const columns = computed(() => [
  { title: t('stocks.warehouse'), key: 'warehouse_name', ...R_ST },
  { title: t('stocks.stockType'), key: 'stock_type', ...R_ST },
  { title: t('stocks.freeStock'), key: 'free_to_sell', ...R_ST, render: (row) => formatInt(row.free_to_sell) },
  { title: t('stocks.reservedStock'), key: 'reserved', ...R_ST, render: (row) => formatInt(row.reserved) },
  { title: t('stocks.totalStock'), key: 'total', ...R_ST, render: (row) => formatInt(row.total) },
])

const visibleColumns = computed(() =>
  columns.value.filter(col => columnVisibility.value[col.key] !== false)
)

const scrollX = computed(() =>
  visibleColumns.value.reduce((sum, col) => sum + (col.width || 80), 0) + 20
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
            <div class="table-toolbar">
              <n-popover
                placement="bottom-end"
                trigger="click"
                :show="showColumnSettings"
                @update:show="showColumnSettings = $event"
                style="max-height: 400px; overflow-y: auto;"
              >
                <template #trigger>
                  <n-button size="tiny" quaternary circle class="table-settings-btn">
                    <template #icon>
                      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <circle cx="12" cy="12" r="3"/>
                        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
                      </svg>
                    </template>
                  </n-button>
                </template>
                <div style="min-width: 200px;">
                  <div style="font-weight: 600; margin-bottom: 8px; font-size: 13px;">{{ t('table.columns') }}</div>
                  <n-space vertical size="small">
                    <template v-for="col in columns" :key="col.key">
                      <div v-if="col.type !== 'expand'">
                        <n-checkbox
                          :checked="columnVisibility[col.key] !== false"
                          @update:checked="toggleColumn(col.key)"
                        >
                          {{ col.title || col.key }}
                        </n-checkbox>
                      </div>
                    </template>
                  </n-space>
                </div>
              </n-popover>
            </div>
            <n-data-table :columns="visibleColumns" :data="stockByWarehouse" :bordered="false" :single-line="true" :max-height="maxHeight" :scroll-x="scrollX" />
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
.table-toolbar {
  position: absolute;
  top: -36px;
  right: 0;
  z-index: 10;
}
.table-settings-btn {
  opacity: 0.5;
  transition: opacity 0.2s;
}
.table-settings-btn:hover {
  opacity: 1;
}
</style>
