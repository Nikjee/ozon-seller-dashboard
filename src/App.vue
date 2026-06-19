<script setup>
import { onMounted, ref, watch } from "vue"
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
import StockAnalytics from './components/StockAnalytics.vue'
import TransactionTotals from './components/TransactionTotals.vue'
import AnalyticsDashboard from './components/AnalyticsDashboard.vue'
import "./App.css"

const { theme, toggle: toggleTheme } = useTheme()
const { month, year, months, years, monthLabel, loading, error, totals, accountExpenses, products, refresh } = useDashboard()
const { locale, t, toggle: toggleLang } = useI18n()
const { configValid, configMessage, saving, check, save } = useConfig()
const { checkForUpdates } = useUpdater()

const cfgClientId = ref("")
const cfgApiKey = ref("")
const configChecked = ref(false)
const activeView = ref('dashboard')

// Lazy-load tracking: each view mounts once on first activation and persists
const stocksActivated = ref(false)
const totalsActivated = ref(false)
const analyticsActivated = ref(false)

watch(activeView, (view) => {
  if (view === 'stocks') stocksActivated.value = true
  if (view === 'totals') totalsActivated.value = true
  if (view === 'analytics') analyticsActivated.value = true
})

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
    <DashboardHeader
      :month="month" :year="year" :months="months" :years="years"
      :month-label="monthLabel" :theme="theme"
      @update:month="month = $event"
      @update:year="year = $event"
      @toggle-theme="toggleTheme"
      @refresh="refresh"
    />
    <main class="main container">
      <n-tabs v-model:value="activeView" type="line" animated>
        <n-tab-pane name="dashboard" :tab="t('tabs.dashboard')">
          <div class="view-container">
            <div class="view-header">
              <h2>{{ t('tabs.dashboard') }}</h2>
              <details class="view-info">
                <summary>{{ t('info.dashboard') }}</summary>
                <p>{{ t('info.dashboardContent') }}</p>
              </details>
            </div>
            <div class="view-content">
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
            </div>
          </div>
        </n-tab-pane>
        <n-tab-pane name="stocks" :tab="t('tabs.stocks')">
          <StockAnalytics v-if="stocksActivated" />
        </n-tab-pane>
        <n-tab-pane name="totals" :tab="t('tabs.totals')">
          <TransactionTotals v-if="totalsActivated" />
        </n-tab-pane>
        <n-tab-pane name="analytics" :tab="t('tabs.analytics')">
          <AnalyticsDashboard v-if="analyticsActivated" />
        </n-tab-pane>
      </n-tabs>
    </main>
  </div>
</template>
