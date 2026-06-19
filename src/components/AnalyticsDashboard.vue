<script setup>
import { computed, h, onMounted, ref } from 'vue'
import { NTag, NCard, NStatistic, NDataTable, NSpin, NAlert, NEmpty, NButton, NCheckbox, NSpace, NPopover } from 'naive-ui'
import { useI18n } from '../composables/useI18n.js'
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
const columnVisibility = ref({})
const showColumnSettings = ref(false)

function toggleColumn(key) {
  const current = columnVisibility.value[key]
  columnVisibility.value = { ...columnVisibility.value, [key]: current === undefined ? false : !current }
}

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

const visibleColumns = computed(() =>
  analyticsColumns.value.filter(col => columnVisibility.value[col.key] !== false)
)

const scrollX = computed(() =>
  visibleColumns.value.reduce((sum, col) => sum + (col.width || 80), 0) + 20
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
                    <template v-for="col in analyticsColumns" :key="col.key">
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
            <n-data-table 
              :columns="visibleColumns" 
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