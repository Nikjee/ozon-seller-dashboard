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
          <!-- Overview stat cards -->
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

          <!-- Expense breakdown section -->
          <div v-if="expenses && expenseCategories.length" class="expense-breakdown">
            <h3 class="section-title">{{ t('totals.expenseBreakdown') }}</h3>
            <div class="category-grid">
              <n-card
                v-for="cat in expenseCategories" :key="cat.key"
                class="category-card"
                hoverable
                @click="openCategory(cat)"
              >
                <div class="category-card__header">
                  <span class="category-card__label">{{ cat.label }}</span>
                  <span class="category-card__count">{{ cat.ops }} {{ t('entries') }}</span>
                </div>
                <div class="category-card__amount amount-negative">{{ formatRub(cat.total) }}</div>
              </n-card>
            </div>
          </div>
        </template>
        <n-empty v-else :description="t('noData')" />
      </n-spin>
    </div>

    <!-- Category detail dialog -->
    <n-modal v-model:show="showModal" :mask-closable="true" preset="card" :title="modalTitle" style="max-width: 720px">
      <n-data-table
        v-if="modalOperations.length"
        :columns="modalColumns"
        :data="modalOperations"
        :bordered="false"
        :single-line="true"
        size="small"
        :max-height="400"
      />
      <n-empty v-else :description="t('noData')" />
    </n-modal>
  </div>
</template>

<script setup>
import { h, computed, ref, onMounted } from 'vue'
import { useI18n } from '../composables/useI18n.js'
import { useTransactionTotals } from '../composables/useTransactionTotals.js'
import { formatRub } from '../utils.js'

const { t } = useI18n()
const { 
  month, year, data, loading, error, refresh,
  expenses, accrualsForSale, totalCompensation, saleCommission,
  servicesAmount, processingAndDelivery, refundsAndCancellations,
  moneyTransfer, othersAmount
} = useTransactionTotals()

onMounted(() => refresh())

const categoryLabels = {
  ad: () => t('totals.categoryAd'),
  storage: () => t('totals.categoryStorage'),
  logistics: () => t('totals.categoryLogistics'),
  compensation: () => t('totals.categoryCompensation'),
  other: () => t('totals.categoryOther'),
}

const expenseCategories = computed(() => {
  const cats = expenses.value?.categories
  const details = expenses.value?.details
  if (!cats) return []
  return Object.entries(cats)
    .filter(([, total]) => total > 0)
    .map(([key, total]) => ({
      key,
      label: categoryLabels[key]?.() || key,
      total,
      ops: (details?.[key] || []).length,
    }))
    .sort((a, b) => b.total - a.total)
})

// Dialog state
const showModal = ref(false)
const modalTitle = ref('')
const modalOperations = ref([])

function openCategory(cat) {
  modalTitle.value = cat.label
  modalOperations.value = expenses.value?.details?.[cat.key] || []
  showModal.value = true
}

const modalColumns = [
  { title: t('date'), key: 'date', width: 120, render(row) { return row.date ? row.date.slice(0, 10) : '—' } },
  { title: t('totals.operationName'), key: 'name', ellipsis: { tooltip: true } },
  { title: t('totals.totalAmount'), key: 'amount', width: 120,
    render(row) { return h('span', { class: 'amount-negative' }, formatRub(row.amount)) }
  },
]
</script>

<style scoped>
.expense-breakdown {
  margin-top: 32px;
}
.section-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 12px;
}
.category-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 12px;
}
.category-card {
  cursor: pointer;
  transition: transform 0.15s, box-shadow 0.15s;
}
.category-card:hover {
  transform: translateY(-2px);
}
.category-card__header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}
.category-card__label {
  font-weight: 600;
  font-size: 14px;
}
.category-card__count {
  font-size: 11px;
  opacity: 0.6;
}
.category-card__amount {
  font-size: 20px;
  font-weight: 700;
}
</style>