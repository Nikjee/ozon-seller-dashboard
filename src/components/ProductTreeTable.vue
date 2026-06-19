<script setup>
import { h, ref, computed, onMounted, onUnmounted } from 'vue'
import { NDataTable, NTag, NPopover, NButton, NCheckbox, NSpace } from 'naive-ui'
import { useI18n } from '../composables/useI18n.js'
import { formatRubCompact, formatInt } from '../utils.js'
import PostingTable from './PostingTable.vue'

const props = defineProps({
  products: Array
})

const { t } = useI18n()

const expandedRowKeys = ref([])

function onExpandedRowKeysChange(keys) {
  expandedRowKeys.value = keys
}

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

// Plain number renderer (no currency formatting — for stocks, counts)
function plainNumRenderFor(key) {
  return (row) => {
    const val = resolveDotPath(row, key)
    if (val === undefined || val === null) return '—'
    return formatInt(val)
  }
}

// Commission badge renderer — reads product_info.commissions array, shows NTag badges with popover
function commissionBadgeRender() {
  return (row) => {
    const commissions = row?.product_info?.commissions
    if (!commissions || !Array.isArray(commissions) || commissions.length === 0) return '—'
    const schemeTypeMap = { 'fbo': 'info', 'fbs': 'success', 'rfbs': 'warning', 'fbp': 'default' }
    const badges = commissions.map(c => {
      const scheme = (c.sale_schema || '').toLowerCase()
      const type = schemeTypeMap[scheme] || 'default'
      const label = (c.sale_schema || '').toUpperCase()
      const popoverText = t('commission.popover').replace('{scheme}', label).replace('{pct}', c.percent)
      return h(NPopover, {}, {
        trigger: () => h(NTag, { type, size: 'small' }, () => label),
        default: () => popoverText
      })
    })
    return h('div', { style: 'display: flex; gap: 4px; flex-wrap: wrap' }, badges)
  }
}

// Image thumbnail renderer — renders primary_image as small img
function imgRender() {
  return (row) => {
    const url = row?.product_info?.primary_image
    if (!url || url === '') return '—'
    return h('img', {
      src: url,
      style: 'width: 36px; height: 36px; object-fit: cover; border-radius: 4px; display: block;',
      alt: ''
    })
  }
}

// Profit share % — profitPerUnit / product_info.price * 100
function profitShareRender() {
  return (row) => {
    const price = row?.product_info?.price
    if (!price || price === 0) return '—'
    const share = (row.profitPerUnit / price) * 100
    return h('span', {
      class: share >= 0 ? 'amount-positive' : 'amount-negative',
    }, `${share >= 0 ? '+' : ''}${share.toFixed(1)}%`)
  }
}

// Color index renderer — shows NTag badge with color index type
function colorIndexRender() {
  return (row) => {
    const code = row?.product_info?.color_index
    if (code === null || code === undefined) return '—'
    const map = {
      'COLOR_INDEX_WITHOUT_INDEX': { label: t('colorIndex.withoutIndex'), type: 'default' },
      'COLOR_INDEX_RED': { label: t('colorIndex.red'), type: 'error' },
      'COLOR_INDEX_GREEN': { label: t('colorIndex.green'), type: 'success' },
      'COLOR_INDEX_SUPER': { label: t('colorIndex.super'), type: 'info' },
    }
    const entry = map[code]
    if (!entry) return h(NTag, { size: 'small', round: true, type: 'default' }, () => code)
    return h(NTag, { size: 'small', round: true, type: entry.type }, () => entry.label)
  }
}

const sortBy = (key) => (a, b) => {
  const va = resolveDotPath(a, key)
  const vb = resolveDotPath(b, key)
  if (va == null) return 1
  if (vb == null) return -1
  return typeof va === 'string' ? va.localeCompare(vb) : va - vb
}

const R = { resizable: true, minWidth: 60, sorter: 'default' }

const columns = computed(() => [
  { type: 'expand', fixed: 'left', renderExpand: expandedRowRender },
  {
    title: t('table.name'),
    key: 'name',
    fixed: 'left',
    width: 200,
    ellipsis: { tooltip: true },
    ...R,
    render(row) { return h('span', { style: 'font-weight: 600' }, row.name) }
  },
  { title: t('table.image'), key: 'product_info.primary_image', width: 56, ...R, sorter: false, render: imgRender() },
  { title: t('table.offerId'), key: 'offer_id', width: 120, ellipsis: { tooltip: true }, ...R, sorter: (a, b) => (Number(a.offer_id) || 0) - (Number(b.offer_id) || 0) },
  { title: t('table.sku'), key: 'sku', width: 100, ...R },
  { title: t('table.price'), key: 'product_info.price', width: 100, ...R, sorter: sortBy('product_info.price'), render: numRenderFor('product_info.price') },
  { title: t('table.oldPrice'), key: 'product_info.old_price', width: 100, ...R, sorter: sortBy('product_info.old_price'), render: numRenderFor('product_info.old_price') },
  { title: t('table.netPrice'), key: 'product_info.net_price', width: 100, ...R, sorter: sortBy('product_info.net_price'), render: numRenderFor('product_info.net_price') },
  { title: t('table.stocksPresent'), key: 'product_info.stocks_present', width: 90, ...R, sorter: sortBy('product_info.stocks_present'), render: plainNumRenderFor('product_info.stocks_present') },
  { title: t('table.stocksReserved'), key: 'product_info.stocks_reserved', width: 90, ...R, sorter: sortBy('product_info.stocks_reserved'), render: plainNumRenderFor('product_info.stocks_reserved') },
  { title: t('table.colorIndex'), key: 'product_info.color_index', width: 120, ...R, sorter: sortBy('product_info.color_index'), render: colorIndexRender() },
  { title: t('table.scheme'), key: 'product_info.scheme', width: 100, ...R, sorter: sortBy('product_info.scheme') },
  { title: t('table.totalRevenue'), key: 'totalRevenue', width: 120, ...R, render: numRenderFor('totalRevenue') },
  { title: t('table.commission'), key: 'costs.commission', width: 110, ...R, sorter: sortBy('costs.commission'), render: costRenderFor('costs.commission') },
  { title: t('table.acquiring'), key: 'costs.acquiring', width: 100, ...R, sorter: sortBy('costs.acquiring'), render: costRenderFor('costs.acquiring') },
  { title: t('table.orderProcessing'), key: 'costs.order_processing', width: 130, ...R, sorter: sortBy('costs.order_processing'), render: costRenderFor('costs.order_processing') },
  { title: t('table.logistics'), key: 'costs.logistics', width: 100, ...R, sorter: sortBy('costs.logistics'), render: costRenderFor('costs.logistics') },
  { title: t('table.deliveryToPickup'), key: 'costs.delivery_to_pickup', width: 140, ...R, sorter: sortBy('costs.delivery_to_pickup'), render: costRenderFor('costs.delivery_to_pickup') },
  { title: t('table.placement'), key: 'costs.placement', width: 120, ...R, sorter: sortBy('costs.placement'), render: costRenderFor('costs.placement') },
  { title: t('table.returnProcessing'), key: 'costs.return_processing', width: 130, ...R, sorter: sortBy('costs.return_processing'), render: costRenderFor('costs.return_processing') },
  { title: t('table.returnLogistics'), key: 'costs.return_logistics', width: 120, ...R, sorter: sortBy('costs.return_logistics'), render: costRenderFor('costs.return_logistics') },
  { title: t('table.disposal'), key: 'costs.disposal', width: 90, ...R, sorter: sortBy('costs.disposal'), render: costRenderFor('costs.disposal') },
  { title: t('table.ovhProcessing'), key: 'costs.ovh_processing', width: 130, ...R, sorter: sortBy('costs.ovh_processing'), render: costRenderFor('costs.ovh_processing') },
  { title: t('table.operationalErrors'), key: 'costs.operational_errors', width: 130, ...R, sorter: sortBy('costs.operational_errors'), render: costRenderFor('costs.operational_errors') },
  { title: t('table.payPerClick'), key: 'costs.pay_per_click', width: 110, ...R, sorter: sortBy('costs.pay_per_click'), render: costRenderFor('costs.pay_per_click') },
  { title: t('table.payPerOrder'), key: 'costs.pay_per_order', width: 110, ...R, sorter: sortBy('costs.pay_per_order'), render: costRenderFor('costs.pay_per_order') },
  { title: t('table.starProducts'), key: 'costs.star_products', width: 100, ...R, sorter: sortBy('costs.star_products'), render: costRenderFor('costs.star_products') },
  { title: t('table.paidBrand'), key: 'costs.paid_brand', width: 100, ...R, sorter: sortBy('costs.paid_brand'), render: costRenderFor('costs.paid_brand') },
  { title: t('table.reviewsCost'), key: 'costs.reviews_cost', width: 100, ...R, sorter: sortBy('costs.reviews_cost'), render: costRenderFor('costs.reviews_cost') },
  { title: t('table.discountPoints'), key: 'costs.discount_points', width: 110, ...R, sorter: sortBy('costs.discount_points'), render: costRenderFor('costs.discount_points') },
  { title: t('table.partnerPrograms'), key: 'costs.partner_programs', width: 140, ...R, sorter: sortBy('costs.partner_programs'), render: costRenderFor('costs.partner_programs') },
  { title: t('table.compensation'), key: 'costs.compensation', width: 120, ...R, sorter: sortBy('costs.compensation'), render: costRenderFor('costs.compensation') },
  { title: t('table.otherServices'), key: 'costs.other_services', width: 110, ...R, sorter: sortBy('costs.other_services'), render: costRenderFor('costs.other_services') },
  { title: t('table.totalCosts'), key: 'totalCosts', width: 120, ...R, render: costRenderFor('totalCosts'), cellProps: () => ({ style: 'font-weight: 700' }) },
  { title: t('table.netProfit'), key: 'netProfit', width: 120, ...R, render: profitRenderFor('netProfit') },
  { title: t('table.profitPerUnit'), key: 'profitPerUnit', width: 110, ...R, render: profitRenderFor('profitPerUnit') },
  { title: t('table.profitShare'), key: 'profitShare', width: 100, ...R, sorter: (a, b) => {
    const shareA = a.product_info?.price ? (a.profitPerUnit / a.product_info.price) * 100 : null
    const shareB = b.product_info?.price ? (b.profitPerUnit / b.product_info.price) * 100 : null
    if (shareA == null) return 1
    if (shareB == null) return -1
    return shareA - shareB
  }, render: profitShareRender() },
  { title: t('table.totalQuantity'), key: 'totalQuantity', width: 100, ...R, render: plainNumRenderFor('totalQuantity') },
])

// Column visibility
const columnVisibility = ref({})
const showColumnSettings = ref(false)

const visibleColumns = computed(() =>
  columns.value.filter(col => col.type === 'expand' || columnVisibility.value[col.key] !== false)
)

function toggleColumn(key) {
  const current = columnVisibility.value[key]
  columnVisibility.value = { ...columnVisibility.value, [key]: current === undefined ? false : !current }
}

const EXPAND_COL_WIDTH = 48

const scrollX = computed(() =>
  visibleColumns.value.reduce((sum, col) => sum + (col.width || 0), 0) + EXPAND_COL_WIDTH + 40
)

function expandedRowRender(row) {
  return h(PostingTable, { postings: row.postings || [] })
}

const rowKey = row => row.sku

// Reactive table height
const tableMaxHeight = ref(Math.max(400, window.innerHeight - 320))

function updateTableHeight() {
  tableMaxHeight.value = Math.max(400, window.innerHeight - 320)
}
onMounted(() => window.addEventListener('resize', updateTableHeight))
onUnmounted(() => window.removeEventListener('resize', updateTableHeight))
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
        <div style="min-width: 220px;">
          <div style="font-weight: 600; margin-bottom: 8px; font-size: 13px;">{{ t('table.columns') }}</div>
          <n-space vertical size="small">
            <template v-for="col in columns" :key="col.key">
              <div v-if="col.type !== 'expand'">
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
      :data="products"
      :bordered="false"
      :single-line="true"
      :max-height="tableMaxHeight"
      :scroll-x="scrollX"
      :row-key="rowKey"
      :expanded-row-keys="expandedRowKeys"
      @update:expanded-row-keys="onExpandedRowKeysChange"
    />
  </div>
</template>
<style>
.table-wrapper {
  position: relative;
}
.table-toolbar {
  position: absolute;
  top: -32px;
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
.n-data-table-tr.n-data-table-tr--expanded > .n-data-table-td.n-data-table-td--last-col{
  padding: 0 !important;
}
</style>
