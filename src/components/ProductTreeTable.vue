<script setup>
import { h, ref } from 'vue'
import { useI18n } from '../composables/useI18n.js'
import { formatRubCompact } from '../utils.js'
import ProductRow from './ProductRow.vue'

const props = defineProps({ 
  products: Array,
  enriched: {
    type: Boolean,
    default: false
  }
})

const { t } = useI18n()

const expandedRowKeys = ref([])

function onExpandedRowKeysChange(keys) {
  expandedRowKeys.value = keys
}

function numRender(row, column) {
  const keys = column.key.split('.')
  let val = row
  for (const k of keys) {
    if (val === undefined || val === null) break
    val = val[k]
  }
  if (!val) return '—'
  return formatRubCompact(val)
}

function profitRender(row, column) {
  const keys = column.key.split('.')
  let val = row
  for (const k of keys) {
    if (val === undefined || val === null) break
    val = val[k]
  }
  if (!val) return '—'
  const isPositive = val >= 0
  return h('span', {
    class: isPositive ? 'amount-positive' : 'amount-negative',
    style: { fontWeight: '600' }
  }, formatRubCompact(val))
}

const columns = [
  {
    title: t('table.name'),
    key: 'name',
    fixed: 'left',
    width: 200,
    ellipsis: { tooltip: true },
    render(row) { return h('span', { style: 'font-weight: 600' }, row.name) }
  },
  { title: t('table.sku'), key: 'sku', width: 100 },
  { title: t('table.totalRevenue'), key: 'totalRevenue', width: 120, render: numRender },
  { title: t('table.commission'), key: 'costs.commission', width: 110, render: numRender },
  { title: t('table.acquiring'), key: 'costs.acquiring', width: 100, render: numRender },
  { title: t('table.orderProcessing'), key: 'costs.order_processing', width: 130, render: numRender },
  { title: t('table.logistics'), key: 'costs.logistics', width: 100, render: numRender },
  { title: t('table.deliveryToPickup'), key: 'costs.delivery_to_pickup', width: 140, render: numRender },
  { title: t('table.placement'), key: 'costs.placement', width: 120, render: numRender },
  { title: t('table.returnProcessing'), key: 'costs.return_processing', width: 130, render: numRender },
  { title: t('table.returnLogistics'), key: 'costs.return_logistics', width: 120, render: numRender },
  { title: t('table.disposal'), key: 'costs.disposal', width: 90, render: numRender },
  { title: t('table.ovhProcessing'), key: 'costs.ovh_processing', width: 130, render: numRender },
  { title: t('table.operationalErrors'), key: 'costs.operational_errors', width: 130, render: numRender },
  { title: t('table.payPerClick'), key: 'costs.pay_per_click', width: 110, render: numRender },
  { title: t('table.payPerOrder'), key: 'costs.pay_per_order', width: 110, render: numRender },
  { title: t('table.starProducts'), key: 'costs.star_products', width: 100, render: numRender },
  { title: t('table.paidBrand'), key: 'costs.paid_brand', width: 100, render: numRender },
  { title: t('table.reviewsCost'), key: 'costs.reviews_cost', width: 100, render: numRender },
  { title: t('table.discountPoints'), key: 'costs.discount_points', width: 110, render: numRender },
  { title: t('table.partnerPrograms'), key: 'costs.partner_programs', width: 140, render: numRender },
  { title: t('table.compensation'), key: 'costs.compensation', width: 120, render: numRender },
  { title: t('table.otherServices'), key: 'costs.other_services', width: 110, render: numRender },
  { title: t('table.totalCosts'), key: 'totalCosts', width: 120, render: numRender, cellProps: () => ({ style: 'font-weight: 700' }) },
  { title: t('table.netProfit'), key: 'netProfit', width: 120, render: profitRender },
  { title: t('table.profitPerUnit'), key: 'profitPerUnit', width: 110, render: numRender },
  { title: t('table.totalQuantity'), key: 'totalQuantity', width: 100 },
]

function expandedRowRender(row) {
  return h(ProductRow, { product: row, enriched: true })
}

const rowKey = row => row.sku
</script>

<template>
  <template v-if="!enriched">
    <div v-if="products.length" class="tree-table container">
      <div class="tree-table__header">
        <span class="tree-table__th tree-table__th--expand"></span>
        <span class="tree-table__th tree-table__th--name">{{ t('product') }}</span>
        <span class="tree-table__th tree-table__th--num">{{ t('sold') }}</span>
        <span class="tree-table__th tree-table__th--amount">{{ t('revenue') }}</span>
        <span class="tree-table__th tree-table__th--amount">{{ t('commission') }}</span>
        <span class="tree-table__th tree-table__th--amount">{{ t('delivery') }}</span>
        <span class="tree-table__th tree-table__th--amount">{{ t('returns') }}</span>
        <span class="tree-table__th tree-table__th--amount">{{ t('services') }}</span>
        <span class="tree-table__th tree-table__th--net">{{ t('profit') }}</span>
      </div>
      <ProductRow
        v-for="product in products"
        :key="product.sku"
        :product="product"
      />
    </div>
  </template>
  <template v-else>
    <n-data-table
      :columns="columns"
      :data="products"
      :bordered="false"
      :single-line="false"
      :max-height="540"
      scroll-x="2000"
      :row-key="rowKey"
      :expanded-row-keys="expandedRowKeys"
      :expanded-row-render="expandedRowRender"
      @update:expanded-row-keys="onExpandedRowKeysChange"
    />
  </template>
</template>
