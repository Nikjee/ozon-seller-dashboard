<script setup>
import { computed, h, onMounted } from 'vue'
import { NTag, NCard, NStatistic, NDataTable, NSpin, NAlert, NEmpty, NButton } from 'naive-ui'
import { useI18n } from '../composables/useI18n.js'
import { useAnalyticsDashboard } from '../composables/useAnalyticsDashboard.js'

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

const columns = [
  { title: 'Name', key: 'name', width: 200 },
  { title: 'SKU', key: 'sku', width: 100 },
  { title: 'Available Stock', key: 'available_stock_count' },
  { title: 'ADS', key: 'ads' },
  { title: 'IDC', key: 'idc' },
  { 
    title: 'Turnover Grade', 
    key: 'turnover_grade',
    render(row) {
      return h(NTag, { type: gradeTagType(row.turnover_grade) }, () => row.turnover_grade)
    }
  },
  { title: 'Days w/o Sales', key: 'days_without_sales' },
]
</script>

<template>
  <div class="view-container">
    <div class="view-header">
      <h2>{{ t('analytics.title') }}</h2>
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
          <!-- Summary stats -->
          <div class="stats-grid">
            <n-card><n-statistic label="Stock Balance" :value="stockBalanceTotal" /></n-card>
            <n-card><n-statistic label="Overall ADS" :value="overallAds" /></n-card>
            <n-card><n-statistic label="Overall IDC" :value="overallIdc" /></n-card>
          </div>
          
          <!-- Grade distribution badges -->
          <div v-if="Object.keys(turnoverGrades || {}).length" class="grade-badges" style="margin-bottom: 16px; display: flex; gap: 8px; flex-wrap: wrap;">
            <n-tag v-for="(count, grade) in turnoverGrades" :key="grade" :type="gradeTagType(grade)">
              {{ grade }}: {{ count }}
            </n-tag>
          </div>
          
          <!-- Product table -->
          <n-data-table 
            :columns="columns" 
            :data="products" 
            :bordered="false" 
            :single-line="false" 
            :max-height="maxHeight"
          />
        </template>
        <n-empty v-else :description="t('noData')" />
      </n-spin>
    </div>
  </div>
</template>