<script setup>
import { h, computed } from 'vue'
import { NDataTable } from 'naive-ui'
import { useI18n } from '../composables/useI18n.js'
import { useColumnSettings } from '../composables/useColumnSettings.js'
import { formatRubCompact } from '../utils.js'
const props = defineProps({
  postings: Array
})

const { t } = useI18n()

function resolveDotPath(row, path) {
  const keys = path.split('.')
  let val = row
  for (const k of keys) {
    if (val === undefined || val === null) break
    val = val[k]
  }
  return val
}

function numRenderFor(key) {
  return (row) => {
    const val = resolveDotPath(row, key)
    if (val === undefined || val === null) return '—'
    return formatRubCompact(val)
  }
}

function costRenderFor(key) {
  return (row) => {
    const val = resolveDotPath(row, key)
    if (val === undefined || val === null) return '—'
    return h('span', { class: 'amount-negative' }, formatRubCompact(val))
  }
}

function profitRenderFor(key) {
  return (row) => {
    const val = resolveDotPath(row, key)
    if (val === undefined || val === null) return '—'
    return h('span', {
      class: val >= 0 ? 'amount-positive' : 'amount-negative',
      style: { fontWeight: '600' }
    }, formatRubCompact(val))
  }
}

function servicesRender() {
  return (row) => {
    const services = row?.services
    if (!services || !Array.isArray(services) || services.length === 0) return '—'
    return services.map(s => `${s.name}: ${formatRubCompact(s.price)}`).join(', ')
  }
}

function mapPostingToRow(p) {
  const adCost = (p.ad_click_cost || 0) + (p.ad_order_cost || 0)
  return {
    name: p.posting_number,
    sku: p.sku,
    date: p.date,
    totalRevenue: p.seller_price_per_instance,
    netProfit: p.net - adCost,
    totalQuantity: 1,
    costs: {
      commission: p.commission_amount,
      delivery_to_pickup: p.delivery_charge,
      return_logistics: p.return_charge,
      pay_per_click: p.ad_click_cost || 0,
      pay_per_order: p.ad_order_cost || 0,
    },
    totalCosts: p.commission_amount + p.delivery_charge + p.return_charge + adCost,
    services: p.services,
  }
}

const sortBy = (key) => (a, b) => {
  const va = resolveDotPath(a, key)
  const vb = resolveDotPath(b, key)
  if (va == null) return 1
  if (vb == null) return -1
  return typeof va === 'string' ? va.localeCompare(vb) : va - vb
}

const RN = { resizable: true, minWidth: 60, sorter: 'default' }

const allColumns = computed(() => [
  // { title: t('table.name'), key: 'name', ...RN },
  // { title: t('table.sku'), key: 'sku', ...RN },
  { title: t('table.postingDate'), key: 'date', ...RN, render: (row) => row.date ? row.date.slice(0, 10) : '—' },
  { title: t('table.totalRevenue'), key: 'totalRevenue', ...RN, sorter: sortBy('totalRevenue'), render: numRenderFor('totalRevenue') },
  { title: t('table.netProfit'), key: 'netProfit', ...RN, sorter: sortBy('netProfit'), render: profitRenderFor('netProfit') },
  { title: t('table.commission'), key: 'costs.commission', ...RN, sorter: sortBy('costs.commission'), render: costRenderFor('costs.commission') },
  { title: t('table.deliveryToPickup'), key: 'costs.delivery_to_pickup', ...RN, sorter: sortBy('costs.delivery_to_pickup'), render: costRenderFor('costs.delivery_to_pickup') },
  { title: t('table.returnLogistics'), key: 'costs.return_logistics', ...RN, sorter: sortBy('costs.return_logistics'), render: costRenderFor('costs.return_logistics') },
  { title: t('table.payPerClick'), key: 'costs.pay_per_click', ...RN, sorter: sortBy('costs.pay_per_click'), render: costRenderFor('costs.pay_per_click') },
  { title: t('table.payPerOrder'), key: 'costs.pay_per_order', ...RN, sorter: sortBy('costs.pay_per_order'), render: costRenderFor('costs.pay_per_order') },
  { title: t('table.services'), key: 'services_list', ...RN, render: servicesRender() },
])

const { columnVisibility, showColumnSettings, visibleColumns, toggleColumn, settingsColumn } = useColumnSettings('postings', allColumns, () => t('table.columns'))

const displayColumns = computed(() => [...visibleColumns.value, settingsColumn.value])
const scrollX = computed(() =>
  displayColumns.value.reduce((sum, col) => sum + (col.width || 0), 0) + 20
)

const data = computed(() =>
  (props.postings || []).map((p, i) => ({ ...mapPostingToRow(p), _rk: `${p.posting_number}_${p.sku}_${i}` }))
)
</script>

<template>
  <div class="table-wrapper">
    <n-data-table
      :columns="displayColumns"
      :data="data"
      :bordered="false"
      :single-line="true"
      size="small"
      :scroll-x="scrollX"
      :row-key="(row) => row._rk"
      :theme-overrides="{
        tdPaddingLarge: 'unset !important',
        tdPaddingMedium: 'unset !important',
        tdPaddingSmall: 'unset !important',
      }"
    />
  </div>
</template>

<style scoped>
.table-wrapper {
  position: relative;
}
</style>
