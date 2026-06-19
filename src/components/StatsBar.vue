<script setup>
import { useI18n } from '../composables/useI18n.js'
import { formatRub, formatInt } from '../utils.js'

defineProps({ totals: Object, accountExpenses: Object })
const { t } = useI18n()
</script>

<template>
<div v-if="totals" class="stats-grid">
    <div class="stat-card">
      <div class="stat-card__label">{{ t('productsSold') }}</div>
      <div class="stat-card__value">{{ formatInt(totals.total_quantity) }}</div>
      <div class="stat-card__sub">{{ formatInt(totals.product_count) }} {{ t('positions') }}</div>
    </div>
    <div class="stat-card">
      <div class="stat-card__label">{{ t('revenue') }}</div>
      <div class="stat-card__value amount">{{ formatRub(totals.total_revenue) }}</div>
    </div>
    <div class="stat-card">
      <div class="stat-card__label">{{ t('expenses') }}</div>
      <div class="stat-card__value amount amount-negative">{{ formatRub(totals.total_expenses) }}</div>
      <div v-if="accountExpenses" class="stat-card__sub">
        <div class="expense-line">&#8226; {{ t('commissionLabel') }} {{ formatRub(totals.product_expenses) }}</div>
        <div class="expense-line">{{ t('accountExpenses') }} {{ formatRub(totals.account_expenses) }}</div>
        <div class="expense-line expense-line--indent">{{ t('accountAd') }} {{ formatRub(accountExpenses.cats.ad) }}</div>
        <div class="expense-line expense-line--indent">{{ t('accountLogistics') }} {{ formatRub(accountExpenses.cats.logistics) }}</div>
        <div class="expense-line expense-line--indent">{{ t('accountStorage') }} {{ formatRub(accountExpenses.cats.storage) }}</div>
      </div>
    </div>
    <div class="stat-card stat-card--highlight">
      <div class="stat-card__label">{{ t('netProfit') }}</div>
      <div class="stat-card__value amount" :class="totals.net_profit >= 0 ? 'amount-positive' : 'amount-negative'">{{ formatRub(totals.net_profit) }}</div>
    </div>
  </div>
</template>
