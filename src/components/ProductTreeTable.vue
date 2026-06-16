<script setup>
import { h, ref, computed, onMounted, onUnmounted } from 'vue'
import { NDataTable, NTag, NPopover } from 'naive-ui'
import { useI18n } from '../composables/useI18n.js'
import { formatRubCompact } from '../utils.js'

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
    return String(val)
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

// Delivery breakdown renderer — shows sub-fields from delivery_details object
function deliveryBreakdownRender() {
  return (row) => {
    const d = row?.delivery_details
    if (!d) return '—'
    const parts = []
    if (d.amount) parts.push(`${formatRubCompact(d.amount)}`)
    if (d.bonus) parts.push(`bonus: ${formatRubCompact(d.bonus)}`)
    if (d.standard_fee) parts.push(`fee: ${formatRubCompact(d.standard_fee)}`)
    if (d.bank_coinvestment) parts.push(`bank: ${formatRubCompact(d.bank_coinvestment)}`)
    if (d.stars) parts.push(`stars: ${formatRubCompact(d.stars)}`)
    return parts.length > 0 ? parts.join(' | ') : '—'
  }
}

// Return breakdown renderer — shows sub-fields from return_details object
function returnBreakdownRender() {
  return (row) => {
    const d = row?.return_details
    if (!d) return '—'
    const parts = []
    if (d.amount) parts.push(`${formatRubCompact(d.amount)}`)
    if (d.bonus) parts.push(`bonus: ${formatRubCompact(d.bonus)}`)
    if (d.standard_fee) parts.push(`fee: ${formatRubCompact(d.standard_fee)}`)
    if (d.bank_coinvestment) parts.push(`bank: ${formatRubCompact(d.bank_coinvestment)}`)
    if (d.stars) parts.push(`stars: ${formatRubCompact(d.stars)}`)
    return parts.length > 0 ? parts.join(' | ') : '—'
  }
}

// Services renderer — shows services array as "name: price" list
function servicesRender() {
  return (row) => {
    const services = row?.services
    if (!services || !Array.isArray(services) || services.length === 0) return '—'
    return services
      .map(s => `${s.name}: ${formatRubCompact(s.price)}`)
      .join(', ')
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

const columns = computed(() => [
  { type: 'expand', renderExpand: expandedRowRender },
  {
    title: t('table.name'),
    key: 'name',
    fixed: 'left',
    width: 200,
    ellipsis: { tooltip: true },
    render(row) { return h('span', { style: 'font-weight: 600' }, row.name) }
  },
  { title: t('table.image'), key: 'product_info.primary_image', width: 56, render: imgRender() },
  { title: t('table.offerId'), key: 'offer_id', width: 120, ellipsis: { tooltip: true } },
  { title: t('table.sku'), key: 'sku', width: 100 },
  { title: t('table.price'), key: 'product_info.price', width: 100, render: numRenderFor('product_info.price') },
  { title: t('table.oldPrice'), key: 'product_info.old_price', width: 100, render: numRenderFor('product_info.old_price') },
  { title: t('table.netPrice'), key: 'product_info.net_price', width: 100, render: numRenderFor('product_info.net_price') },
  { title: t('table.stocksPresent'), key: 'product_info.stocks_present', width: 90, render: plainNumRenderFor('product_info.stocks_present') },
  { title: t('table.stocksReserved'), key: 'product_info.stocks_reserved', width: 90, render: plainNumRenderFor('product_info.stocks_reserved') },
  { title: t('table.colorIndex'), key: 'product_info.color_index', width: 120, render: colorIndexRender() },
  { title: t('table.commissionPct'), key: 'commissionPct', width: 150, render: commissionBadgeRender() },
  { title: t('table.scheme'), key: 'product_info.scheme', width: 100 },
  { title: t('table.totalRevenue'), key: 'totalRevenue', width: 120, render: numRenderFor('totalRevenue') },
  { title: t('table.commission'), key: 'costs.commission', width: 110, render: costRenderFor('costs.commission') },
  { title: t('table.acquiring'), key: 'costs.acquiring', width: 100, render: costRenderFor('costs.acquiring') },
  { title: t('table.orderProcessing'), key: 'costs.order_processing', width: 130, render: costRenderFor('costs.order_processing') },
  { title: t('table.logistics'), key: 'costs.logistics', width: 100, render: costRenderFor('costs.logistics') },
  { title: t('table.deliveryToPickup'), key: 'costs.delivery_to_pickup', width: 140, render: costRenderFor('costs.delivery_to_pickup') },
  { title: t('table.placement'), key: 'costs.placement', width: 120, render: costRenderFor('costs.placement') },
  { title: t('table.returnProcessing'), key: 'costs.return_processing', width: 130, render: costRenderFor('costs.return_processing') },
  { title: t('table.returnLogistics'), key: 'costs.return_logistics', width: 120, render: costRenderFor('costs.return_logistics') },
  { title: t('table.disposal'), key: 'costs.disposal', width: 90, render: costRenderFor('costs.disposal') },
  { title: t('table.ovhProcessing'), key: 'costs.ovh_processing', width: 130, render: costRenderFor('costs.ovh_processing') },
  { title: t('table.operationalErrors'), key: 'costs.operational_errors', width: 130, render: costRenderFor('costs.operational_errors') },
  { title: t('table.payPerClick'), key: 'costs.pay_per_click', width: 110, render: costRenderFor('costs.pay_per_click') },
  { title: t('table.payPerOrder'), key: 'costs.pay_per_order', width: 110, render: costRenderFor('costs.pay_per_order') },
  { title: t('table.starProducts'), key: 'costs.star_products', width: 100, render: costRenderFor('costs.star_products') },
  { title: t('table.paidBrand'), key: 'costs.paid_brand', width: 100, render: costRenderFor('costs.paid_brand') },
  { title: t('table.reviewsCost'), key: 'costs.reviews_cost', width: 100, render: costRenderFor('costs.reviews_cost') },
  { title: t('table.discountPoints'), key: 'costs.discount_points', width: 110, render: costRenderFor('costs.discount_points') },
  { title: t('table.partnerPrograms'), key: 'costs.partner_programs', width: 140, render: costRenderFor('costs.partner_programs') },
  { title: t('table.compensation'), key: 'costs.compensation', width: 120, render: costRenderFor('costs.compensation') },
  { title: t('table.otherServices'), key: 'costs.other_services', width: 110, render: costRenderFor('costs.other_services') },
  { title: t('table.totalCosts'), key: 'totalCosts', width: 120, render: costRenderFor('totalCosts'), cellProps: () => ({ style: 'font-weight: 700' }) },
  { title: t('table.netProfit'), key: 'netProfit', width: 120, render: profitRenderFor('netProfit') },
  { title: t('table.profitPerUnit'), key: 'profitPerUnit', width: 110, render: profitRenderFor('profitPerUnit') },
  { title: t('table.profitShare'), key: 'profitShare', width: 100, render: profitShareRender() },
  { title: t('table.totalQuantity'), key: 'totalQuantity', width: 100 },
])

const EXPAND_COL_WIDTH = 48

const scrollX = computed(() =>
  columns.value.reduce((sum, col) => sum + (col.width || 0), 0) + EXPAND_COL_WIDTH + 40
)

// Nested-only columns (breakdown details not shown in main table)
const nestedOnlyColumns = computed(() => [
  { title: t('table.commissionRatio'), key: 'commission_ratio', width: 120 },
  { title: t('table.deliveryBreakdown'), key: 'delivery_breakdown', width: 280, render: deliveryBreakdownRender() },
  { title: t('table.returnBreakdown'), key: 'return_breakdown', width: 280, render: returnBreakdownRender() },
  { title: t('table.services'), key: 'services_list', width: 280, render: servicesRender() },
])

const nestedColumns = computed(() => {
  const ghost = { key: '__ghost', width: EXPAND_COL_WIDTH, render: () => '' }
  return [
    ghost,
    { title: t('table.name'), key: 'name', width: 160, fixed: 'left', render(row) { return h('span', { style: 'font-weight: 600' }, row.name) } },
    { title: t('table.postingDate'), key: 'sku', width: 120 },
    { title: t('table.offerId'), key: 'offer_id', width: 120, ellipsis: { tooltip: true } },
    { title: t('table.totalRevenue'), key: 'totalRevenue', width: 120, render: numRenderFor('totalRevenue') },
    { title: t('table.commission'), key: 'costs.commission', width: 110, render: costRenderFor('costs.commission') },
    { title: t('table.deliveryToPickup'), key: 'costs.delivery_to_pickup', width: 130, render: costRenderFor('costs.delivery_to_pickup') },
    { title: t('table.returnLogistics'), key: 'costs.return_logistics', width: 120, render: costRenderFor('costs.return_logistics') },
    { title: t('table.netProfit'), key: 'netProfit', width: 120, render: profitRenderFor('netProfit') },
    { title: t('table.commissionRatio'), key: 'commission_ratio', width: 100 },
    { title: t('table.deliveryBreakdown'), key: 'delivery_breakdown', width: 280, render: deliveryBreakdownRender() },
    { title: t('table.returnBreakdown'), key: 'return_breakdown', width: 280, render: returnBreakdownRender() },
    { title: t('table.services'), key: 'services_list', width: 280, render: servicesRender() },
  ]
})

function mapPostingToRow(p) {
  return {
    name: p.posting_number,
    offer_id: p.offer_id,
    sku: p.date,
    product_info: null,
    totalRevenue: p.seller_price_per_instance,
    costs: {
      commission: p.commission_amount,
      acquiring: 0,
      order_processing: 0,
      logistics: 0,
      delivery_to_pickup: p.delivery_charge,
      placement: 0,
      return_processing: 0,
      return_logistics: p.return_charge,
      disposal: 0,
      ovh_processing: 0,
      operational_errors: 0,
      pay_per_click: 0,
      pay_per_order: 0,
      star_products: 0,
      paid_brand: 0,
      reviews_cost: 0,
      discount_points: 0,
      partner_programs: 0,
      compensation: 0,
      other_services: 0,
    },
    totalCosts: p.commission_amount + p.delivery_charge + p.return_charge,
    netProfit: p.net,
    profitPerUnit: p.net,
    totalQuantity: 1,
    commission_ratio: p.commission_ratio,
    delivery_details: p.delivery_details,
    return_details: p.return_details,
    services: p.services,
  }
}

function expandedRowRender(row) {
  const postings = (row.postings || []).map(mapPostingToRow)
  return h(NDataTable, {
    columns: nestedColumns.value,
    data: postings,
    bordered: false,
    singleLine: false,
    size: 'small',
    scrollX: scrollX.value + nestedOnlyColumns.value.reduce((sum, col) => sum + (col.width || 0), 0),
    themeOverrides: {
      tdPaddingLarge: 'unset !important',
      tdPaddingMedium: 'unset !important',
      tdPaddingSmall: 'unset !important',
    },
    rowKey: (p) => p.name,
  })
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
  <n-data-table
    :columns="columns"
    :data="products"
    :bordered="false"
    :single-line="false"
    :max-height="tableMaxHeight"
    :scroll-x="scrollX"
    :row-key="rowKey"
    :expanded-row-keys="expandedRowKeys"
    @update:expanded-row-keys="onExpandedRowKeysChange"
  />
</template>
<style>
.n-data-table-tr.n-data-table-tr--expanded > .n-data-table-td.n-data-table-td--last-col{
  padding: 0 !important;
}
</style>
