<template>
  <div class="view-container">
    <div class="view-header">
      <h2>{{ t('totals.title') }}</h2>
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
              <n-statistic :label="t('totals.accrualsForSale')" :value="accrualsForSale" />
            </n-card>
            <n-card>
              <n-statistic :label="t('totals.saleCommission')" :value="saleCommission" type="error" v-if="saleCommission < 0" />
              <n-statistic :label="t('totals.saleCommission')" :value="saleCommission" v-else />
            </n-card>
            <n-card>
              <n-statistic :label="t('totals.processingAndDelivery')" :value="processingAndDelivery" type="error" v-if="processingAndDelivery < 0" />
              <n-statistic :label="t('totals.processingAndDelivery')" :value="processingAndDelivery" v-else />
            </n-card>
            <n-card>
              <n-statistic :label="t('totals.servicesAmount')" :value="servicesAmount" type="error" v-if="servicesAmount < 0" />
              <n-statistic :label="t('totals.servicesAmount')" :value="servicesAmount" v-else />
            </n-card>
            <n-card>
              <n-statistic :label="t('totals.othersAmount')" :value="othersAmount" type="error" v-if="othersAmount < 0" />
              <n-statistic :label="t('totals.othersAmount')" :value="othersAmount" v-else />
            </n-card>
            <n-card>
              <n-statistic :label="t('totals.refundsAndCancellations')" :value="refundsAndCancellations" type="error" v-if="refundsAndCancellations < 0" />
              <n-statistic :label="t('totals.refundsAndCancellations')" :value="refundsAndCancellations" v-else />
            </n-card>
            <n-card>
              <n-statistic :label="t('totals.totalCompensation')" :value="totalCompensation" type="error" v-if="totalCompensation < 0" />
              <n-statistic :label="t('totals.totalCompensation')" :value="totalCompensation" v-else />
            </n-card>
            <n-card style="border: 2px solid var(--accent);">
              <n-statistic :label="t('totals.moneyTransfer')" :value="moneyTransfer" type="error" v-if="moneyTransfer < 0" />
              <n-statistic :label="t('totals.moneyTransfer')" :value="moneyTransfer" v-else />
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