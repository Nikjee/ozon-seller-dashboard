<script setup>
import { useI18n } from '../composables/useI18n.js'

defineProps({ theme: String })
const emit = defineEmits(['toggle'])
const { t } = useI18n()
</script>

<template>
<button class="theme-toggle" :title="theme === 'dark' ? t('lightMode') : t('darkMode')" @click="emit('toggle')">
    <span v-if="theme === 'dark'">&#9728;</span>
    <span v-else>&#9790;</span>
  </button>
</template>
