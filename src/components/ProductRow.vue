<script setup>
import { ref, computed } from 'vue'
import { useI18n } from '../composables/useI18n.js'
import { formatRub, formatRubCompact } from '../utils.js'
import PostingRow from './PostingRow.vue'

const props = defineProps({
  product: { type: Object, required: true },
  enriched: { type: Boolean, default: false }
})
const emit = defineEmits(['toggle'])

const expanded = ref(false)
const showExpenses = ref(false)
const { t } = useI18n()

const isPositive = computed(() => props.product.summary?.net_profit >= 0)

const expenseCats = computed(() => [
  { key: 'ad', label: t('adExpense'), color: 'var(--ctp-mauve)' },
  { key: 'storage', label: t('storageFees'), color: 'var(--ctp-peach)' },
  { key: 'logistics', label: t('logisticsFees'), color: 'var(--ctp-blue)' },
  { key: 'compensation', label: t('compensation'), color: 'var(--ctp-teal)' },
  { key: 'other', label: t('acquiring'), color: 'var(--ctp-muted)' }
])

const costItems = computed(() => {
  const costs = props.product.costs || {}
  const fields = [
    { key: 'commission', label: t('table.commission') },
    { key: 'acquiring', label: t('table.acquiring') },
    { key: 'order_processing', label: t('table.orderProcessing') },
    { key: 'logistics', label: t('table.logistics') },
    { key: 'delivery_to_pickup', label: t('table.deliveryToPickup') },
    { key: 'placement', label: t('table.placement') },
    { key: 'return_processing', label: t('table.returnProcessing') },
    { key: 'return_logistics', label: t('table.returnLogistics') },
    { key: 'disposal', label: t('table.disposal') },
    { key: 'ovh_processing', label: t('table.ovhProcessing') },
    { key: 'operational_errors', label: t('table.operationalErrors') },
    { key: 'pay_per_click', label: t('table.payPerClick') },
    { key: 'pay_per_order', label: t('table.payPerOrder') },
    { key: 'star_products', label: t('table.starProducts') },
    { key: 'paid_brand', label: t('table.paidBrand') },
    { key: 'reviews_cost', label: t('table.reviewsCost') },
    { key: 'discount_points', label: t('table.discountPoints') },
    { key: 'partner_programs', label: t('table.partnerPrograms') },
    { key: 'compensation', label: t('table.compensation') },
    { key: 'other_services', label: t('table.otherServices') },
  ]
  return fields.map(f => ({
    key: f.key,
    label: f.label,
    value: costs[f.key] ?? 0
  }))
})

const postings = computed(() => props.product.postings || [])

function formatCost(val) {
  if (!val || val === 0) return '—'
  const num = Number(val)
  return new Intl.NumberFormat('ru-RU', { style: 'currency', currency: 'RUB', minimumFractionDigits: 2 }).format(num)
}
</script>

<template>
  <template v-if="!enriched">
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
          <span class="product-row__children-h product-row__children-h--amount">{{ t('table.payPerClick') }}</span>
          <span class="product-row__children-h product-row__children-h--amount">{{ t('table.payPerOrder') }}</span>
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
  <template v-else>
    <div class="enriched-expanded">
      <!-- Cost breakdown grid -->
      <div class="enriched-costs">
        <div class="enriched-costs__title">{{ t('enriched.costBreakdown') }}</div>
        <div class="enriched-costs__grid">
          <div v-for="cost in costItems" :key="cost.key" class="enriched-costs__item">
            <span class="enriched-costs__label">{{ cost.label }}</span>
            <span class="enriched-costs__value">{{ formatCost(cost.value) }}</span>
          </div>
        </div>
      </div>
      
      <!-- Postings (existing pattern) -->
      <div class="product-row__children" v-if="postings.length">
        <div class="product-row__children-header">
          <span class="product-row__children-h">{{ t('enriched.posting') }}</span>
          <span class="product-row__children-h product-row__children-h--date">{{ t('enriched.date') }}</span>
          <span class="product-row__children-h product-row__children-h--amount">{{ t('enriched.price') }}</span>
          <span class="product-row__children-h product-row__children-h--amount">{{ t('commission') }}</span>
          <span class="product-row__children-h product-row__children-h--amount">{{ t('delivery') }}</span>
          <span class="product-row__children-h product-row__children-h--amount">{{ t('returns') }}</span>
          <span class="product-row__children-h product-row__children-h--amount">{{ t('table.payPerClick') }}</span>
          <span class="product-row__children-h product-row__children-h--amount">{{ t('table.payPerOrder') }}</span>
          <span class="product-row__children-h product-row__children-h--net">{{ t('enriched.net') }}</span>
        </div>
        <PostingRow v-for="p in postings" :key="p.posting_number" :posting="p" />
      </div>
    </div>
  </template>
</template>
