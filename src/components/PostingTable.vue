<script setup>
import { h, ref, computed } from 'vue'
import { NDataTable, NPopover, NButton, NCheckbox, NSpace } from 'naive-ui'
import { useI18n } from '../composables/useI18n.js'
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
  { title: t('table.name'), key: 'name', ...RN },
  { title: t('table.sku'), key: 'sku', ...RN },
  { title: t('table.postingDate'), key: 'date', ...RN },
  { title: t('table.totalRevenue'), key: 'totalRevenue', ...RN, render: numRenderFor('totalRevenue') },
  { title: t('table.netProfit'), key: 'netProfit', ...RN, render: profitRenderFor('netProfit') },
  { title: t('table.commission'), key: 'costs.commission', ...RN, sorter: sortBy('costs.commission'), render: costRenderFor('costs.commission') },
  { title: t('table.deliveryToPickup'), key: 'costs.delivery_to_pickup', ...RN, sorter: sortBy('costs.delivery_to_pickup'), render: costRenderFor('costs.delivery_to_pickup') },
  { title: t('table.returnLogistics'), key: 'costs.return_logistics', ...RN, sorter: sortBy('costs.return_logistics'), render: costRenderFor('costs.return_logistics') },
  { title: t('table.payPerClick'), key: 'costs.pay_per_click', ...RN, sorter: sortBy('costs.pay_per_click'), render: costRenderFor('costs.pay_per_click') },
  { title: t('table.payPerOrder'), key: 'costs.pay_per_order', ...RN, sorter: sortBy('costs.pay_per_order'), render: costRenderFor('costs.pay_per_order') },
  { title: t('table.services'), key: 'services_list', ...RN, render: servicesRender() },
])

const columnVisibility = ref({})
const showColumnSettings = ref(false)

const visibleColumns = computed(() =>
  allColumns.value.filter(col => columnVisibility.value[col.key] !== false)
)

function toggleColumn(key) {
  const current = columnVisibility.value[key]
  columnVisibility.value = { ...columnVisibility.value, [key]: current === undefined ? false : !current }
}

const scrollX = computed(() =>
  visibleColumns.value.reduce((sum, col) => sum + (col.width || 0), 0) + 20
)

const data = computed(() => (props.postings || []).map(mapPostingToRow))
</script>

<template>
  <div class="table-wrapper">
    <div class="table-toolbar">
      <n-popover
        placement="bottom-end"
        trigger="click"
        :show="showColumnSettings"
        @update:show="showColumnSettings = $event"
        style="max-height: 400px; overflow-y: auto;"
      >
        <template #trigger>
          <n-button size="tiny" quaternary circle class="table-settings-btn">
            <template #icon>
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="3"/>
                <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
              </svg>
            </template>
          </n-button>
        </template>
        <div style="min-width: 200px;">
          <div style="font-weight: 600; margin-bottom: 8px; font-size: 13px;">{{ t('table.columns') }}</div>
          <n-space vertical size="small">
            <template v-for="col in allColumns" :key="col.key">
              <div>
                <n-checkbox
                  :checked="columnVisibility[col.key] !== false"
                  @update:checked="toggleColumn(col.key)"
                >
                  {{ col.title || col.key }}
                </n-checkbox>
              </div>
            </template>
          </n-space>
        </div>
      </n-popover>
    </div>
    <n-data-table
      :columns="visibleColumns"
      :data="data"
      :bordered="false"
      :single-line="true"
      size="small"
      :scroll-x="scrollX"
      :row-key="(row) => row.name"
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
.table-toolbar {
  position: absolute;
  top: -4px;
  right: 0;
  z-index: 10;
}
.table-settings-btn {
  opacity: 0.5;
  transition: opacity 0.2s;
}
.table-settings-btn:hover {
  opacity: 1;
}
</style>
