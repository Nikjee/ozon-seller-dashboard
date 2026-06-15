<script setup>
import { useI18n } from '../composables/useI18n.js'
import ProductRow from './ProductRow.vue'

defineProps({ products: Array })
const { t } = useI18n()
</script>

<template>
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
