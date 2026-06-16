<script setup>
import { computed, h, onMounted } from 'vue'
import { NTag, NCard, NStatistic, NDataTable, NSpin, NAlert, NEmpty, NButton, NPopover } from 'naive-ui'
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

const columns = computed(() => [
  { title: t('analytics.colName'), key: 'name', width: 200 },
  { title: t('analytics.colSku'), key: 'sku', width: 100 },
  { title: t('analytics.colAvailableStock'), key: 'available_stock_count' },
  { title: t('analytics.colAds'), key: 'ads' },
  { title: t('analytics.colIdc'), key: 'idc' },
  { 
    title: t('analytics.colTurnoverGrade'), 
    key: 'turnover_grade',
    render(row) {
      return h(NTag, { type: gradeTagType(row.turnover_grade) }, () => row.turnover_grade)
    }
  },
  { title: t('analytics.colDaysWithoutSales'), key: 'days_without_sales' },
])
</script>

<template>
  <div class="view-container">
    <div class="view-header">
      <h2 style="display: flex; align-items: center; gap: 8px;">
        {{ t('analytics.title') }}
        <n-popover trigger="hover" placement="right" :width="320">
          <template #trigger>
            <span class="help-icon" style="cursor: help; display: inline-flex; align-items: center; justify-content: center; width: 20px; height: 20px; border-radius: 50%; border: 1.5px solid var(--ctp-text); font-size: 12px; font-weight: 700; line-height: 1; opacity: 0.5; transition: opacity 0.15s;">?</span>
          </template>
          <div>
            <p><strong>{{ t('analytics.title') }}</strong></p>
            <p style="margin-top: 8px;">{{ t('analytics.helpIntro') }}</p>
            <ul style="margin-top: 8px; padding-left: 16px;">
              <li>{{ t('analytics.helpStockBalance') }}</li>
              <li>{{ t('analytics.helpAds') }}</li>
              <li>{{ t('analytics.helpIdc') }}</li>
              <li>{{ t('analytics.helpTurnover') }}</li>
              <li>{{ t('analytics.helpDataSource') }}</li>
            </ul>
          </div>
        </n-popover>
      </h2>
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
            <n-card><n-statistic :label="t('analytics.stockBalance')" :value="stockBalanceTotal" /></n-card>
            <n-card><n-statistic :label="t('analytics.overallAds')" :value="overallAds" /></n-card>
            <n-card><n-statistic :label="t('analytics.overallIdc')" :value="overallIdc" /></n-card>
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