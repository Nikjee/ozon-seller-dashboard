<script setup>
import { useI18n } from '../composables/useI18n.js'

defineProps({ message: String })
const emit = defineEmits(['retry'])
const { t } = useI18n()
</script>

<template>
<div class="error-banner container">
    <div class="error-banner__content">
      <div>
        <div class="error-banner__title">{{ t('loadError') }}</div>
        <div class="error-banner__message">{{ message }}</div>
      </div>
      <button class="btn btn-primary" @click="emit('retry')">{{ t('retry') }}</button>
    </div>
  </div>
</template>
