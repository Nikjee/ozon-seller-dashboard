<script setup>
import { ref, onMounted } from 'vue'
import { useI18n } from '../composables/useI18n.js'
import MonthPicker from './MonthPicker.vue'
import DarkModeToggle from './DarkModeToggle.vue'

defineProps({ month: Number, year: Number, months: Array, years: Array, monthLabel: String, theme: String })
const emit = defineEmits(['update:month', 'update:year', 'toggle-theme', 'refresh'])
const { t, locale, toggle: toggleLang } = useI18n()

const appVersion = ref('')
onMounted(async () => {
  try {
    const { getVersion } = await import('@tauri-apps/api/app')
    appVersion.value = await getVersion()
  } catch {
    // not in Tauri or older build
  }
})
</script>

<template>
<div class="header">
    <div class="header__inner container">
      <div class="header__left">
        <h1 class="header__title">{{ t('title') }} <span class="header__version">v{{ appVersion }}</span></h1>
      </div>
      <div class="header__center">
        <MonthPicker
          :month="month" :year="year" :months="months" :years="years"
          @update:month="emit('update:month', $event)"
          @update:year="emit('update:year', $event)"
        />
      </div>
      <div class="header__right">
        <button class="btn lang-btn" @click="toggleLang">{{ locale.toUpperCase() }}</button>
        <DarkModeToggle :theme="theme" @toggle="emit('toggle-theme')" />
        <button class="btn btn-primary" @click="emit('refresh')">{{ t('update') }}</button>
      </div>
    </div>
  </div>
</template>
