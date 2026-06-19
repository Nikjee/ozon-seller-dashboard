<script setup>
import { computed } from 'vue'
import { useI18n } from '../composables/useI18n.js'
import { formatRub, formatRubCompact } from '../utils.js'

const props = defineProps({ posting: Object })
const { t } = useI18n()
const isNetPositive = computed(() => (props.posting.net || 0) >= 0)
</script>

<template>
<div class="posting-row">
    <span class="posting-row__num">#{{ props.posting.posting_number }}</span>
    <span class="posting-row__date">{{ props.posting.date }}</span>
    <span class="posting-row__amount" :title="formatRub(props.posting.seller_price_per_instance)">{{ formatRubCompact(props.posting.seller_price_per_instance) }}</span>
    <span class="posting-row__amount amount-negative" :title="formatRub(props.posting.commission_amount)">{{ formatRubCompact(props.posting.commission_amount) }}</span>
    <span class="posting-row__amount amount-negative" :title="formatRub(props.posting.delivery_charge)">{{ formatRubCompact(props.posting.delivery_charge) }}</span>
    <span class="posting-row__amount amount-negative" :title="formatRub(props.posting.return_charge)">{{ formatRubCompact(props.posting.return_charge) }}</span>
    <span class="posting-row__amount amount-negative" :title="props.posting.ad_click_cost ? formatRub(props.posting.ad_click_cost) : ''">{{ props.posting.ad_click_cost ? formatRubCompact(props.posting.ad_click_cost) : '—' }}</span>
    <span class="posting-row__amount amount-negative" :title="props.posting.ad_order_cost ? formatRub(props.posting.ad_order_cost) : ''">{{ props.posting.ad_order_cost ? formatRubCompact(props.posting.ad_order_cost) : '—' }}</span>
    <span class="posting-row__amount posting-row__net" :class="isNetPositive ? 'amount-positive' : 'amount-negative'" :title="formatRub(props.posting.net)">
      {{ formatRubCompact(props.posting.net) }}
    </span>
  </div>
</template>
