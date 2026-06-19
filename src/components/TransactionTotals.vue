<template>
  <div class="view-container">
    <div class="view-header">
      <h2>{{ t('totals.title') }}</h2>
      <details class="view-info">
        <summary>{{ t('info.totals') }}</summary>
        <p>{{ t('info.totalsContent') }}</p>
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
          <div class="stats-grid">
            <n-card>
              <n-statistic :label="t('totals.accrualsForSale')" :value="formatRub(accrualsForSale)" />
            </n-card>
            <n-card>
              <n-statistic :label="t('totals.saleCommission')" :value="formatRub(saleCommission)" type="error" v-if="saleCommission < 0" />
              <n-statistic :label="t('totals.saleCommission')" :value="formatRub(saleCommission)" v-else />
            </n-card>
            <n-card>
              <n-statistic :label="t('totals.processingAndDelivery')" :value="formatRub(processingAndDelivery)" type="error" v-if="processingAndDelivery < 0" />
              <n-statistic :label="t('totals.processingAndDelivery')" :value="formatRub(processingAndDelivery)" v-else />
            </n-card>
            <n-card>
              <n-statistic :label="t('totals.servicesAmount')" :value="formatRub(servicesAmount)" type="error" v-if="servicesAmount < 0" />
              <n-statistic :label="t('totals.servicesAmount')" :value="formatRub(servicesAmount)" v-else />
            </n-card>
            <n-card>
              <n-statistic :label="t('totals.othersAmount')" :value="formatRub(othersAmount)" type="error" v-if="othersAmount < 0" />
              <n-statistic :label="t('totals.othersAmount')" :value="formatRub(othersAmount)" v-else />
            </n-card>
            <n-card>
              <n-statistic :label="t('totals.refundsAndCancellations')" :value="formatRub(refundsAndCancellations)" type="error" v-if="refundsAndCancellations < 0" />
              <n-statistic :label="t('totals.refundsAndCancellations')" :value="formatRub(refundsAndCancellations)" v-else />
            </n-card>
            <n-card>
              <n-statistic :label="t('totals.totalCompensation')" :value="formatRub(totalCompensation)" type="error" v-if="totalCompensation < 0" />
              <n-statistic :label="t('totals.totalCompensation')" :value="formatRub(totalCompensation)" v-else />
            </n-card>
            <n-card style="border: 2px solid var(--accent);">
              <n-statistic :label="t('totals.moneyTransfer')" :value="formatRub(moneyTransfer)" type="error" v-if="moneyTransfer < 0" />
              <n-statistic :label="t('totals.moneyTransfer')" :value="formatRub(moneyTransfer)" v-else />
            </n-card>
          </div>
        </template>
        <n-empty v-else :description="t('noData')" />
      </n-spin>
    </div>
  </div>
</template>

<script setup>
import { onMounted } from 'vue'
import { useI18n } from '../composables/useI18n.js'
import { useTransactionTotals } from '../composables/useTransactionTotals.js'
import { formatRub } from '../utils.js'

const { t } = useI18n()
const { 
  month, 
  year, 
  data, 
  loading, 
  error, 
  refresh,
  accrualsForSale,
  totalCompensation,
  saleCommission,
  servicesAmount,
  processingAndDelivery,
  refundsAndCancellations,
  moneyTransfer,
  othersAmount
} = useTransactionTotals()

onMounted(() => refresh())
</script>