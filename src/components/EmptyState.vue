<script setup>
import { useI18n } from '../composables/useI18n.js'
const { t } = useI18n()
</script>

<template>
<div class="empty-state">
    <div class="empty-state__icon">&#9744;</div>
    <h2 class="empty-state__text">{{ t('noData') }}</h2>
    <p class="empty-state__text">{{ t('noDataHint') }}</p>
  </div>
</template>
