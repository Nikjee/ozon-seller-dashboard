<script setup>
import { onMounted, ref } from "vue"
import { useTheme } from "./composables/useTheme.js"
import { useDashboard } from "./composables/useDashboard.js"
import { useI18n } from "./composables/useI18n.js"
import { useConfig } from "./composables/useConfig.js"
import { useUpdater } from "./composables/useUpdater.js"
import DashboardHeader from "./components/DashboardHeader.vue"
import StatsBar from "./components/StatsBar.vue"
import ProductTreeTable from "./components/ProductTreeTable.vue"
import EmptyState from "./components/EmptyState.vue"
import LoadingOverlay from "./components/LoadingOverlay.vue"
import ErrorBanner from "./components/ErrorBanner.vue"
import AccountExpensesPanel from "./components/AccountExpensesPanel.vue"
import "./App.css"

const { theme, toggle: toggleTheme } = useTheme()
const { month, year, months, years, monthLabel, loading, error, totals, accountExpenses, products, refresh } = useDashboard()
const { locale, t, toggle: toggleLang } = useI18n()
const { configValid, configMessage, saving, check, save } = useConfig()
const { checking: updateChecking, updateVersion, downloading, error: updateError, checkForUpdates, installUpdate } = useUpdater()

const cfgClientId = ref("")
const cfgApiKey = ref("")
const configChecked = ref(false)

onMounted(async () => {
  await check()
  configChecked.value = true
  if (configValid.value) {
    refresh()
    // Check for updates after initial data loads
    setTimeout(() => checkForUpdates(), 3000)
  }
})

async function handleSaveConfig() {
  await save(cfgClientId.value, cfgApiKey.value)
  if (configValid.value) {
    refresh()
  }
}
</script>

<template>
  <div v-if="!configChecked" class="config-screen">
    <div class="config-card">
      <h1>Loading...</h1>
    </div>
  </div>
  <div v-else-if="!configValid" class="config-screen">
    <div class="config-card">
      <h1>Ozon Seller Dashboard</h1>
      <p>Please enter your Ozon API credentials. You can find them in your Ozon seller account under Settings → API keys.</p>
      <div class="config-field">
        <label for="client-id">Client ID</label>
        <input id="client-id" v-model="cfgClientId" type="text" placeholder="123456" />
      </div>
      <div class="config-field">
        <label for="api-key">API Key</label>
        <input id="api-key" v-model="cfgApiKey" type="password" placeholder="xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx" />
      </div>
      <div class="config-actions">
        <button :disabled="saving" @click="handleSaveConfig">
          {{ saving ? "Saving..." : "Save & Continue" }}
        </button>
      </div>
      <p v-if="configMessage" class="config-error">{{ configMessage }}</p>
    </div>
  </div>
  <div v-else class="app" :class="theme">
    <div v-if="updateVersion" class="update-banner">
      <span>Update {{ updateVersion }} available</span>
      <button :disabled="downloading" @click="installUpdate">
        {{ downloading ? 'Downloading...' : 'Install' }}
      </button>
      <button class="update-dismiss" @click="updateVersion = null">×</button>
    </div>
    <div v-else-if="updateError" class="update-banner update-banner--error">
      <span>Update check failed: {{ updateError }}</span>
      <button class="update-dismiss" @click="updateError = null">×</button>
    </div>
    <DashboardHeader
      :month="month" :year="year" :months="months" :years="years"
      :month-label="monthLabel" :theme="theme"
      @update:month="month = $event"
      @update:year="year = $event"
      @toggle-theme="toggleTheme"
      @refresh="refresh"
    />
    <main class="main container">
      <LoadingOverlay v-if="loading" />
      <template v-else-if="error">
        <ErrorBanner :message="error" @retry="refresh" />
      </template>
      <template v-else-if="totals">
        <StatsBar :totals="totals" :account-expenses="accountExpenses" />
        <AccountExpensesPanel :account-expenses="accountExpenses" />
        <ProductTreeTable :products="products" />
      </template>
      <EmptyState v-else />
    </main>
  </div>
</template>