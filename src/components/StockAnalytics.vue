<script setup>
import { computed, onMounted } from 'vue'
import { useI18n } from '../composables/useI18n.js'
import { useStockAnalytics } from '../composables/useStockAnalytics.js'

const { t } = useI18n()
const { month, year, data, loading, error, stockByWarehouse, stockByProduct, totalFreeToSell, totalReserved, refresh } = useStockAnalytics()

onMounted(() => refresh())

const maxHeight = computed(() => {
    // Leave room for header + summary cards + padding (~250px total)
    return window.innerHeight - 280;
});

const columns = [
  { title: t('stocks.warehouse'), key: 'warehouse_name' },
  { title: t('stocks.stockType'), key: 'stock_type' },
  { title: t('stocks.freeStock'), key: 'free_to_sell' },
  { title: t('stocks.reservedStock'), key: 'reserved' },
  { title: t('stocks.totalStock'), key: 'total' },
]
</script>

<template>
  <div class="view-container">
    <div class="view-header">
      <h2>{{ t('stocks.title') }}</h2>
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
            <n-card><n-statistic :label="t('stocks.freeStock')" :value="totalFreeToSell" /></n-card>
            <n-card><n-statistic :label="t('stocks.reservedStock')" :value="totalReserved" /></n-card>
          </div>
          <!-- Stock data table -->
          <n-data-table :columns="columns" :data="stockByWarehouse" :bordered="false" :single-line="false" :max-height="maxHeight" />
        </template>
        <n-empty v-else :description="t('noData')" />
      </n-spin>
    </div>
  </div>
</template>
