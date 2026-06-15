<script setup>
import { useI18n } from '../composables/useI18n.js'
const { t } = useI18n()
</script>

<template>
<div class="loading-overlay">
    <div class="loading-overlay__inner">
      <div class="loading-overlay__spinner"></div>
      <div class="loading-overlay__text">{{ t('loading') }}</div>
    </div>
  </div>
</template>
