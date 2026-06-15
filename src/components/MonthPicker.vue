<script setup>
import { useI18n } from '../composables/useI18n.js'

const props = defineProps({ month: Number, year: Number, months: Array, years: Array })
const emit = defineEmits(['update:month', 'update:year'])
const { t } = useI18n()

function setMonth(m) {
  let mVal = Number(m), yVal = props.year
  if (mVal < 1) { mVal = 12; yVal-- }
  if (mVal > 12) { mVal = 1; yVal++ }
  emit('update:month', mVal)
  emit('update:year', yVal)
}
</script>

<template>
<div class="month-picker">
    <button class="month-picker__btn" :title="t('previousMonth')" @click="setMonth(props.month - 1)">&larr;</button>
    <select class="month-picker__select" :value="props.month" @change="emit('update:month', Number($event.target.value))">
      <option v-for="(m, i) in props.months" :key="i" :value="i + 1">{{ m }}</option>
    </select>
    <select class="month-picker__select month-picker__select--year" :value="props.year" @change="emit('update:year', Number($event.target.value))">
      <option v-for="y in props.years" :key="y" :value="y">{{ y }}</option>
    </select>
    <button class="month-picker__btn" :title="t('nextMonth')" @click="setMonth(props.month + 1)">&rarr;</button>
  </div>
</template>
