<script setup>
import { h, ref, computed, onMounted, onUnmounted } from 'vue'
import { NDataTable, NTag, NTabs, NTabPane } from 'naive-ui'
import { useI18n } from '../composables/useI18n.js'
import { useColumnSettings } from '../composables/useColumnSettings.js'
import { formatRubCompact, formatInt } from '../utils.js'
import PostingTable from './PostingTable.vue'

const props = defineProps({
  products: Array,
  notDelivered: Array
})

const { t } = useI18n()
const activeTab = ref('delivered')

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

function plainNumRenderFor(key) {
  return (row) => {
    const val = resolveDotPath(row, key)
    if (val === undefined || val === null) return '—'
    return formatInt(val)
  }
}

function commissionBadgeRender() {
  return (row) => {
    const commissions = row?.product_info?.commissions
    if (!commissions || !Array.isArray(commissions) || commissions.length === 0) return '—'
    const schemeTypeMap = { 'fbo': 'info', 'fbs': 'success', 'rfbs': 'warning', 'fbp': 'default' }
    const badges = commissions.map(c => {
      const scheme = (c.sale_schema || '').toLowerCase()
      const type = schemeTypeMap[scheme] || 'default'
      const label = (c.sale_schema || '').toUpperCase()
      const delivery = formatRubCompact(c.delivery_amount || 0)
      const ret = formatRubCompact(c.return_amount || 0)
      const popoverText = t('commission.popover').replace('{scheme}', label).replace('{pct}', c.percent).replace('{delivery}', delivery).replace('{return}', ret)
      return h(NPopover, {}, {
        trigger: () => h(NTag, { type, size: 'small' }, () => label),
        default: () => popoverText
      })
    })
    return h('div', { style: 'display: flex; gap: 4px; flex-wrap: wrap' }, badges)
  }
}

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

function expandedRowRender(row) {
  return h(PostingTable, { postings: row.postings || [] })
}

// Delivered columns (existing, with expand + all cost/profit columns)
const deliveredColumns = computed(() => [
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
  { title: t('table.minPrice'), key: 'product_info.min_price', width: 90, ...R, sorter: sortBy('product_info.min_price'), render: numRenderFor('product_info.min_price') },
  { title: t('table.oldPrice'), key: 'product_info.old_price', width: 100, ...R, sorter: sortBy('product_info.old_price'), render: numRenderFor('product_info.old_price') },
  { title: t('table.netPrice'), key: 'product_info.net_price', width: 100, ...R, sorter: sortBy('product_info.net_price'), render: numRenderFor('product_info.net_price') },
  { title: t('table.stocksPresent'), key: 'product_info.stocks_present', width: 90, ...R, sorter: sortBy('product_info.stocks_present'), render: plainNumRenderFor('product_info.stocks_present') },
  { title: t('table.stocksReserved'), key: 'product_info.stocks_reserved', width: 90, ...R, sorter: sortBy('product_info.stocks_reserved'), render: plainNumRenderFor('product_info.stocks_reserved') },
  { title: t('table.colorIndex'), key: 'product_info.color_index', width: 120, ...R, sorter: sortBy('product_info.color_index'), render: colorIndexRender() },
  { title: t('table.scheme'), key: 'product_info.scheme', width: 100, ...R, sorter: sortBy('product_info.scheme') },
  { title: t('table.totalRevenue'), key: 'totalRevenue', width: 120, ...R, sorter: sortBy('totalRevenue'), render: numRenderFor('totalRevenue') },
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
  { title: t('table.totalCosts'), key: 'totalCosts', width: 120, ...R, sorter: sortBy('totalCosts'), render: costRenderFor('totalCosts'), cellProps: () => ({ style: 'font-weight: 700' }) },
  { title: t('table.netProfit'), key: 'netProfit', width: 120, ...R, sorter: sortBy('netProfit'), render: profitRenderFor('netProfit') },
  { title: t('table.profitPerUnit'), key: 'profitPerUnit', width: 110, ...R, sorter: sortBy('profitPerUnit'), render: profitRenderFor('profitPerUnit') },
  { title: t('table.profitShare'), key: 'profitShare', width: 100, ...R, sorter: (a, b) => {
    const shareA = a.product_info?.price ? (a.profitPerUnit / a.product_info.price) * 100 : null
    const shareB = b.product_info?.price ? (b.profitPerUnit / b.product_info.price) * 100 : null
    if (shareA == null) return 1
    if (shareB == null) return -1
    return shareA - shareB
  }, render: profitShareRender() },
  { title: t('table.totalQuantity'), key: 'totalQuantity', width: 100, ...R, sorter: sortBy('totalQuantity'), render: plainNumRenderFor('totalQuantity') },
])

// Not-delivered columns: product info only, no cost/profit columns
const INFO_KEYS = new Set([
  'name', 'product_info.primary_image', 'offer_id', 'sku',
  'product_info.price', 'product_info.min_price', 'product_info.old_price', 'product_info.net_price',
  'product_info.stocks_present', 'product_info.stocks_reserved',
  'product_info.color_index', 'product_info.scheme',
])

const notDeliveredColumns = computed(() =>
  deliveredColumns.value.filter(c => c.type !== 'expand' && INFO_KEYS.has(c.key))
)

const { columnVisibility, showColumnSettings, visibleColumns: visibleDeliveredColumns, toggleColumn, settingsColumn } = useColumnSettings('delivered', deliveredColumns, () => t('table.columns'))

const displayDeliveredColumns = computed(() => [...visibleDeliveredColumns.value, settingsColumn.value])
const EXPAND_COL_WIDTH = 48

const deliveredScrollX = computed(() =>
  displayDeliveredColumns.value.reduce((sum, col) => sum + (col.width || 0), 0) + EXPAND_COL_WIDTH + 40
)

const notDeliveredScrollX = computed(() =>
  notDeliveredColumns.value.reduce((sum, col) => sum + (col.width || 0), 0) + EXPAND_COL_WIDTH + 40
)

const rowKey = row => row.product_id ?? row.sku

const tableMaxHeight = ref(Math.max(400, window.innerHeight - 320))

function updateTableHeight() {
  tableMaxHeight.value = Math.max(400, window.innerHeight - 320)
}
onMounted(() => window.addEventListener('resize', updateTableHeight))
onUnmounted(() => window.removeEventListener('resize', updateTableHeight))
</script>

<template>
  <n-tabs v-model:value="activeTab" type="line" size="small">
    <n-tab-pane name="delivered" :tab="`${t('tabs.delivered')} (${(products || []).length})`">
      <div class="table-wrapper">
        <n-data-table
          :columns="displayDeliveredColumns"
          :data="products"
          :bordered="false"
          :single-line="true"
          :max-height="tableMaxHeight"
          :scroll-x="deliveredScrollX"
          :row-key="rowKey"
          :expanded-row-keys="expandedRowKeys"
          @update:expanded-row-keys="onExpandedRowKeysChange"
        />
      </div>
    </n-tab-pane>
    <n-tab-pane name="notDelivered" :tab="`${t('tabs.notDelivered')} (${(notDelivered || []).length})`">
      <n-data-table
        :columns="notDeliveredColumns"
        :data="notDelivered"
        :bordered="false"
        :single-line="true"
        :max-height="tableMaxHeight"
        :scroll-x="notDeliveredScrollX"
        :row-key="rowKey"
      />
    </n-tab-pane>
  </n-tabs>
</template>
<style>
.table-wrapper {
  position: relative;
}
.n-data-table-tr.n-data-table-tr--expanded > .n-data-table-td.n-data-table-td--last-col{
  padding: 0 !important;
}
</style>
