<script setup>
import { ref, computed } from 'vue'
import { useI18n } from '../composables/useI18n.js'
import { formatRub, formatRubCompact } from '../utils.js'
import PostingRow from './PostingRow.vue'

const props = defineProps({ product: Object })
const expanded = ref(false)
const showExpenses = ref(false)
const { t } = useI18n()

const isPositive = computed(() => props.product.summary.net_profit >= 0)

const expenseCats = computed(() => [
  { key: 'ad', label: t('adExpense'), color: 'var(--ctp-mauve)' },
  { key: 'storage', label: t('storageFees'), color: 'var(--ctp-peach)' },
  { key: 'logistics', label: t('logisticsFees'), color: 'var(--ctp-blue)' },
  { key: 'compensation', label: t('compensation'), color: 'var(--ctp-teal)' },
  { key: 'other', label: t('acquiring'), color: 'var(--ctp-muted)' }
])
</script>

<template>
<div class="product-row">
    <div class="product-row__main" :class="{ 'product-row--expanded': expanded }" @click="expanded = !expanded">
      <span class="product-row__expand">
        <span class="product-row__chevron" :class="{ 'product-row__chevron--open': expanded }">&#9658;</span>
      </span>
      <span class="product-row__name">
        <span>{{ product.name }}</span>
        <span class="product-row__sku">
          <span v-if="product.has_fbo_stocks" class="tag tag-fbo">FBO</span>
          <span v-if="product.has_fbs_stocks" class="tag tag-fbs">FBS</span>
        </span>
      </span>
      <span class="product-row__num">{{ product.summary.total_quantity }}</span>
      <span class="product-row__amount" :title="formatRub(product.summary.total_revenue)">{{ formatRubCompact(product.summary.total_revenue) }}</span>
      <span class="product-row__amount amount-negative" :title="formatRub(product.summary.total_commission)">{{ formatRubCompact(product.summary.total_commission) }}</span>
      <span class="product-row__amount amount-negative" :title="formatRub(product.summary.total_delivery)">{{ formatRubCompact(product.summary.total_delivery) }}</span>
      <span class="product-row__amount amount-negative" :title="formatRub(product.summary.total_returns)">{{ formatRubCompact(product.summary.total_returns) }}</span>
      <span class="product-row__amount amount-negative">
        <span :title="formatRub(product.summary.service_expenses)">{{ formatRubCompact(product.summary.service_expenses) }}</span>
        <span
          v-if="product.summary.service_expenses > 0"
          class="product-row__info-icon"
          @click.stop="showExpenses = !showExpenses"
          :title="t('expenseBreakdown')"
        >&#9432;</span>
      </span>
      <span class="product-row__amount product-row__profit" :class="isPositive ? 'amount-positive' : 'amount-negative'" :title="formatRub(product.summary.net_profit)">
        {{ formatRubCompact(product.summary.net_profit) }}
      </span>
    </div>

    <div v-if="showExpenses && product.summary.service_expenses > 0" class="product-row__expenses">
      <div class="expense-bar">
        <div v-for="cat in expenseCats" :key="cat.key" class="expense-bar__item" v-show="product.summary.expenses_cats[cat.key] > 0">
          <span class="expense-bar__color" :style="{ background: cat.color }"></span>
          <span class="expense-bar__label">{{ cat.label }}</span>
          <span class="expense-bar__amount">{{ formatRub(product.summary.expenses_cats[cat.key]) }}</span>
        </div>
      </div>
    </div>

    <div v-if="expanded" class="product-row__children">
      <div class="product-row__children-header">
        <span class="product-row__children-h product-row__children-h--num">{{ t('posting') }}</span>
        <span class="product-row__children-h product-row__children-h--date">{{ t('date') }}</span>
        <span class="product-row__children-h product-row__children-h--amount">{{ t('price') }}</span>
        <span class="product-row__children-h product-row__children-h--amount">{{ t('commission') }}</span>
        <span class="product-row__children-h product-row__children-h--amount">{{ t('delivery') }}</span>
        <span class="product-row__children-h product-row__children-h--amount">{{ t('returns') }}</span>
        <span class="product-row__children-h product-row__children-h--net">{{ t('net') }}</span>
      </div>
      <PostingRow
        v-for="p in product.postings"
        :key="p.posting_number + '_' + p.sku"
        :posting="p"
      />
    </div>
  </div>
</template>
