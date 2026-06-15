<script setup>
import { ref, computed } from 'vue'
import { useI18n } from '../composables/useI18n.js'
import { formatRub } from '../utils.js'

const props = defineProps({ accountExpenses: Object })
const { t } = useI18n()
const open = ref(false)

const cats = computed(() => [
  { key: 'ad', label: t('accountAd'), color: 'var(--ctp-mauve)', total: props.accountExpenses?.cats?.ad || 0, items: props.accountExpenses?.details?.ad || [] },
  { key: 'logistics', label: t('accountLogistics'), color: 'var(--ctp-blue)', total: props.accountExpenses?.cats?.logistics || 0, items: props.accountExpenses?.details?.logistics || [] },
  { key: 'storage', label: t('accountStorage'), color: 'var(--ctp-peach)', total: props.accountExpenses?.cats?.storage || 0, items: props.accountExpenses?.details?.storage || [] },
])
</script>

<template>
  <div v-if="accountExpenses" class="account-expenses container">
    <button class="account-expenses__toggle" @click="open = !open">
      <span class="account-expenses__chevron" :class="{ 'account-expenses__chevron--open': open }">&#9658;</span>
      {{ t('accountExpenses') }} ({{ formatRub(accountExpenses.cats.ad + accountExpenses.cats.logistics + accountExpenses.cats.storage) }})
    </button>

    <div v-if="open" class="account-expenses__body">
      <div v-for="cat in cats" :key="cat.key" class="account-expenses__cat" v-show="cat.items.length > 0">
        <div class="account-expenses__cat-header">
          <span class="account-expenses__cat-dot" :style="{ background: cat.color }"></span>
          <span class="account-expenses__cat-label">{{ cat.label }}</span>
          <span class="account-expenses__cat-total">{{ formatRub(cat.total) }}</span>
          <span class="account-expenses__cat-count">{{ cat.items.length }} ops</span>
        </div>
        <div class="account-expenses__items">
          <div v-for="item in cat.items" :key="item.operation_id" class="account-expenses__item">
            <span class="account-expenses__item-date">{{ item.date?.slice(0, 10) }}</span>
            <span class="account-expenses__item-name">{{ item.name }}</span>
            <span class="account-expenses__item-amount">{{ formatRub(item.amount) }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
