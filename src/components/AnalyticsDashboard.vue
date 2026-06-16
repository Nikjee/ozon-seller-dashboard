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
            <n-card><n-statistic :label="t('analytics.revenue')" :value="totalRevenue" /></n-card>
            <n-card><n-statistic :label="t('analytics.expenses')" :value="totalExpenses" /></n-card>
            <n-card><n-statistic :label="t('analytics.profit')" :value="totalProfit" /></n-card>
            <n-card><n-statistic :label="t('analytics.margin')" :value="margin" /></n-card>
          </div>
          
          <!-- Grade distribution badges -->
          <div style="margin-bottom: 16px; display: flex; gap: 8px; flex-wrap: wrap;">
            <n-tag v-for="(count, grade) in turnoverGrades" :key="grade" :type="gradeTagType(grade)">
              {{ grade }}: {{ count }}
            </n-tag>
          </div>
          
          <!-- Product table -->
          <n-data-table :columns="columns" :data="products" :bordered="false" :single-line="false" />
        </template>
        <n-empty v-else :description="t('noData')" />
      </n-spin>
    </div>
  </div>
</template>

<script setup>
import { computed, h, onMounted } from 'vue'
import { NTag, NCard, NStatistic, NDataTable, NSpin, NAlert, NEmpty, NButton } from 'naive-ui'
import { useI18n } from '../composables/useI18n.js'
import { useAnalyticsDashboard } from '../composables/useAnalyticsDashboard.js'

const { t } = useI18n()
const { month, year, data, loading, error, products, overallAds, overallIdc, turnoverGrades, stockBalanceTotal, refresh } = useAnalyticsDashboard()

onMounted(() => refresh())

function gradeTagType(grade) {
  const map = { deficit: 'error', popular: 'success', actual: 'info', surplus: 'warning', no_sales: 'default' }
  return map[grade] || 'default'
}

const columns = [
  { title: 'Product', key: 'name', width: 200 },
  { title: 'SKU', key: 'sku', width: 100 },
  { title: 'Current Stock', key: 'current_stock' },
  { title: 'ADS', key: 'ads' },
  { title: 'IDC', key: 'idc' },
  { 
    title: 'Grade', 
    key: 'turnover_grade',
    render(row) {
      return h(NTag, { type: gradeTagType(row.turnover_grade) }, () => row.turnover_grade)
    }
  },
  { title: 'Days w/o Sales', key: 'days_without_sales' },
]

const totalRevenue = computed(() => data.value?.total_revenue || data.value?.totalRevenue || 0)
const totalExpenses = computed(() => data.value?.total_expenses || data.value?.totalExpenses || 0)
const totalProfit = computed(() => data.value?.total_profit || data.value?.totalProfit || 0)
const margin = computed(() => data.value?.margin || 0)
</script>