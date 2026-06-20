<script setup>
import { ref, watch, onMounted, computed } from 'vue'
import {
  NSteps, NStep, NButton, NInput, NInputNumber, NDatePicker,
  NSpin, NAlert, NEmpty, NRadioGroup, NRadio, NCard, NStatistic
} from 'naive-ui'
import { useI18n } from '../composables/useI18n.js'
import { usePostingConstructor } from '../composables/usePostingConstructor.js'

const { t } = useI18n()
const {
  currentStep, goToStep,
  clusters, selectedClusterId, loadClusters,
  clusterSearchQuery,
  allProducts, productSearchQuery, products, productSearchResults,
  loadProducts, addProduct, removeProduct,
  draftId, submitDraft,
  timeslots, selectedTimeslot, loadTimeslots,
  supplyResult, submitSupply,
  loading, error, reset
} = usePostingConstructor()

// ── Step 2: Warehouse (local, derived from selected cluster) ──
const selectedWarehouseId = ref(null)
const warehouseSearchQuery = ref('')

const clusterWarehouses = computed(() => {
  if (!selectedClusterId.value) return []
  const cluster = clusters.value.find(c => c.cluster_id === selectedClusterId.value)
  return cluster?.warehouses ?? []
})

const filteredClusterWarehouses = computed(() => {
  const q = warehouseSearchQuery.value.toLowerCase().trim()
  const whs = clusterWarehouses.value
  if (!q) return whs
  return whs.filter(w => w.name && w.name.toLowerCase().includes(q))
})

const selectedWarehouseName = computed(() => {
  if (!selectedWarehouseId.value) return ''
  const w = clusterWarehouses.value.find(w => w.warehouse_id === selectedWarehouseId.value)
  return w ? (w.name || `ID: ${w.warehouse_id}`) : ''
})

// ── Other computed ──
const timeslotDateRange = ref(null)

const selectedClusterName = computed(() => {
  if (!selectedClusterId.value) return ''
  const c = clusters.value.find(c => c.cluster_id === selectedClusterId.value)
  return c ? (c.name || `ID: ${c.cluster_id}`) : ''
})

const filteredClusters = computed(() => {
  const q = clusterSearchQuery.value.toLowerCase().trim()
  if (!q) return clusters.value
  return clusters.value.filter(c => c.name && c.name.toLowerCase().includes(q))
})

const totalProductQuantity = computed(() =>
  products.value.reduce((sum, p) => sum + (p.quantity || 0), 0)
)

const availableProducts = computed(() => {
  const selectedSkus = new Set(products.value.map(p => p.sku))
  return productSearchResults.value.filter(p => !selectedSkus.has(p.sku))
})

onMounted(() => {
  // Load clusters for step 1 on mount
  if (!clusters.value.length) loadClusters()
})

watch(currentStep, (step) => {
  if (step === 1 && !clusters.value.length) loadClusters()
  if (step === 3 && !allProducts.value.length) loadProducts()
  // Reset warehouse selection when going back to step 1 (cluster changes)
  if (step === 1) { selectedWarehouseId.value = null; warehouseSearchQuery.value = '' }
})

watch(draftId, (val) => {
  if (val && currentStep.value === 4) goToStep(5)
})

watch(supplyResult, (val) => {
  if (val && currentStep.value === 6) goToStep(7)
})

function handleLoadTimeslots() {
  if (!timeslotDateRange.value || timeslotDateRange.value.length < 2) return
  loadTimeslots(
    selectedWarehouseId.value,
    new Date(timeslotDateRange.value[0]).toISOString().split('T')[0],
    new Date(timeslotDateRange.value[1]).toISOString().split('T')[0]
  )
}

function formatTimeslot(ts) {
  const from = ts.from ?? ts.timeslot_from ?? ''
  const to = ts.to ?? ts.timeslot_to ?? ''
  return `${from} — ${to}`
}

function handleCreateAnother() {
  reset()
  loadClusters()
}
</script>

<template>
  <div class="view-container">
    <div class="view-header">
      <h2>{{ t('postingConstructor.title') }}</h2>
      <details class="view-info">
        <summary>{{ t('postingConstructor.info') }}</summary>
        <p>{{ t('postingConstructor.infoContent') }}</p>
      </details>
    </div>
    <div class="view-content">
      <n-steps :current="currentStep" size="small" style="margin-bottom: 24px;">
        <n-step :title="t('postingConstructor.step1')" />
        <n-step :title="t('postingConstructor.step2')" />
        <n-step :title="t('postingConstructor.step3')" />
        <n-step :title="t('postingConstructor.step4')" />
        <n-step :title="t('postingConstructor.step5')" />
        <n-step :title="t('postingConstructor.step6')" />
        <n-step :title="t('postingConstructor.step7')" />
      </n-steps>

      <!-- Step 1: Cluster Selection -->
      <div v-if="currentStep === 1" class="step-content">
        <h3>{{ t('postingConstructor.cluster') }}</h3>
        <n-spin :show="loading">
          <template v-if="error">
            <n-alert type="error" closable>
              <template #header>{{ t('postingConstructor.error') }}</template>
              {{ error }}
            </n-alert>
            <n-button @click="loadClusters" style="margin-top: 12px;">
              {{ t('postingConstructor.retry') }}
            </n-button>
          </template>
          <template v-else-if="clusters.length">
            <div class="selection-container">
              <n-input
                v-model:value="clusterSearchQuery"
                :placeholder="t('postingConstructor.searchClusters')"
                clearable
                style="margin-bottom: 12px;"
              />
              <div class="selection-grid">
                <div
                  v-for="c in filteredClusters"
                  :key="c.cluster_id"
                  class="selection-card"
                  :class="{ 'selection-card--selected': selectedClusterId === c.cluster_id }"
                  @click="selectedClusterId = c.cluster_id"
                  :title="c.name"
                >
                  <div class="selection-card__title">{{ c.name }}</div>
                </div>
              </div>
            </div>
          </template>
          <n-empty v-else :description="t('postingConstructor.noClusters')" />
        </n-spin>
      </div>

      <!-- Step 2: Warehouse Selection (from selected cluster) -->
      <div v-if="currentStep === 2" class="step-content">
        <h3>{{ t('postingConstructor.selectWarehouse') }}</h3>
        <template v-if="!selectedClusterId">
          <n-empty :description="t('postingConstructor.noWarehouses')" />
        </template>
        <template v-else-if="clusterWarehouses.length">
          <div class="selection-container">
            <n-input
              v-model:value="warehouseSearchQuery"
              :placeholder="t('postingConstructor.searchWarehouses')"
              clearable
              style="margin-bottom: 12px;"
            />
            <div class="selection-grid">
              <div
                v-for="w in filteredClusterWarehouses"
                :key="w.warehouse_id"
                class="selection-card"
                :class="{ 'selection-card--selected': selectedWarehouseId === w.warehouse_id }"
                @click="selectedWarehouseId = w.warehouse_id"
                :title="w.name"
              >
                <div class="selection-card__title">{{ w.name }}</div>
              </div>
            </div>
          </div>
        </template>
        <n-empty v-else :description="t('postingConstructor.noWarehouses')" />
      </div>

      <!-- Step 3: Products -->
      <div v-if="currentStep === 3" class="step-content">
        <h3>{{ t('postingConstructor.step3') }}</h3>
        <n-spin :show="loading">
          <template v-if="error">
            <n-alert type="error" closable>
              <template #header>{{ t('postingConstructor.error') }}</template>
              {{ error }}
            </n-alert>
            <n-button @click="loadProducts" style="margin-top: 12px;">
              {{ t('postingConstructor.retry') }}
            </n-button>
          </template>
          <template v-else>
            <n-input
              v-model:value="productSearchQuery"
              :placeholder="t('postingConstructor.productSearch')"
              clearable
              style="margin-bottom: 12px;"
            />
            <div v-if="availableProducts.length" class="product-search-results">
              <div
                v-for="p in availableProducts"
                :key="p.sku"
                class="product-search-item"
              >
                <span>{{ p.name }} ({{ p.offer_id }})</span>
                <n-button size="small" @click="addProduct(p.sku, p.offer_id, p.name, 1)">
                  {{ t('postingConstructor.addProduct') }}
                </n-button>
              </div>
            </div>
            <n-empty v-if="!availableProducts.length && !loading" :description="t('postingConstructor.noProducts')" />

            <div v-if="products.length" class="added-products">
              <h4>{{ t('postingConstructor.totalItems') }}: {{ totalProductQuantity }}</h4>
              <div
                v-for="(p, idx) in products"
                :key="p.sku"
                class="added-product-row"
              >
                <span class="added-product-name">{{ p.name }}</span>
                <n-input-number
                  v-model:value="p.quantity"
                  :min="1"
                  size="small"
                  style="width: 120px;"
                />
                <n-button size="small" type="error" @click="removeProduct(idx)">
                  {{ t('postingConstructor.removeProduct') }}
                </n-button>
              </div>
            </div>
          </template>
        </n-spin>
      </div>

      <!-- Step 4: Draft Creation -->
      <div v-if="currentStep === 4" class="step-content">
        <h3>{{ t('postingConstructor.summary') }}</h3>
        <n-card style="margin-bottom: 16px;">
          <p><strong>{{ t('postingConstructor.warehouse') }}:</strong> {{ selectedWarehouseName }}</p>
          <p><strong>{{ t('postingConstructor.cluster') }}:</strong> {{ selectedClusterName }}</p>
          <p><strong>{{ t('postingConstructor.step3') }}:</strong> {{ totalProductQuantity }}</p>
        </n-card>

        <n-spin :show="loading">
          <template v-if="error">
            <n-alert type="error" closable>
              <template #header>{{ t('postingConstructor.error') }}</template>
              {{ error }}
            </n-alert>
          </template>
          <template v-else-if="draftId">
            <n-alert type="success">
              <template #header>{{ t('postingConstructor.draftCreated') }}</template>
              ID: {{ draftId }}
            </n-alert>
          </template>
          <template v-else>
            <n-button type="primary" @click="submitDraft">
              {{ t('postingConstructor.createDraft') }}
            </n-button>
          </template>
        </n-spin>
      </div>

      <!-- Step 5: Timeslot Selection -->
      <div v-if="currentStep === 5" class="step-content">
        <h3>{{ t('postingConstructor.selectTimeslot') }}</h3>
        <div class="timeslot-controls">
          <n-date-picker
            v-model:value="timeslotDateRange"
            type="daterange"
            style="margin-bottom: 12px;"
          />
          <n-button @click="handleLoadTimeslots" :disabled="!timeslotDateRange">
            {{ t('postingConstructor.loadTimeslots') }}
          </n-button>
        </div>

        <n-spin :show="loading">
          <template v-if="error">
            <n-alert type="error" closable>
              <template #header>{{ t('postingConstructor.error') }}</template>
              {{ error }}
            </n-alert>
          </template>
          <template v-else-if="timeslots.length">
            <n-radio-group v-model:value="selectedTimeslot" vertical>
              <n-radio
                v-for="(ts, idx) in timeslots"
                :key="idx"
                :value="ts"
                style="margin-bottom: 8px;"
              >
                {{ formatTimeslot(ts) }}
              </n-radio>
            </n-radio-group>
          </template>
          <n-empty v-else-if="!loading" :description="t('postingConstructor.noTimeslots')" />
        </n-spin>
      </div>

      <!-- Step 6: Supply Creation -->
      <div v-if="currentStep === 6" class="step-content">
        <h3>{{ t('postingConstructor.summary') }}</h3>
        <n-card style="margin-bottom: 16px;">
          <p><strong>{{ t('postingConstructor.warehouse') }}:</strong> {{ selectedWarehouseName }}</p>
          <p><strong>{{ t('postingConstructor.cluster') }}:</strong> {{ selectedClusterName }}</p>
          <p><strong>{{ t('postingConstructor.step3') }}:</strong> {{ totalProductQuantity }}</p>
          <p v-if="selectedTimeslot">
            <strong>{{ t('postingConstructor.selectTimeslot') }}:</strong> {{ formatTimeslot(selectedTimeslot) }}
          </p>
        </n-card>

        <n-spin :show="loading">
          <template v-if="error">
            <n-alert type="error" closable>
              <template #header>{{ t('postingConstructor.error') }}</template>
              {{ error }}
            </n-alert>
          </template>
          <template v-else-if="supplyResult">
            <n-alert type="success">
              <template #header>{{ t('postingConstructor.supplyCreated') }}</template>
            </n-alert>
          </template>
          <template v-else>
            <n-button type="primary" @click="submitSupply(selectedWarehouseId)">
              {{ t('postingConstructor.createSupply') }}
            </n-button>
          </template>
        </n-spin>
      </div>

      <!-- Step 7: Result -->
      <div v-if="currentStep === 7" class="step-content">
        <n-alert type="success">
          <template #header>{{ t('postingConstructor.supplyCreated') }}</template>
          <p v-if="supplyResult && supplyResult.supply_order_ids">
            <strong>{{ t('postingConstructor.supplyOrderIds') }}:</strong>
            {{ supplyResult.supply_order_ids.join(', ') }}
          </p>
        </n-alert>
        <n-button type="primary" @click="handleCreateAnother" style="margin-top: 16px;">
          {{ t('postingConstructor.createAnother') }}
        </n-button>
      </div>

      <!-- Navigation -->
      <div class="step-navigation">
        <n-button v-if="currentStep > 1 && currentStep < 7" @click="goToStep(currentStep - 1)">
          {{ t('postingConstructor.back') }}
        </n-button>
        <span v-else></span>
        <n-button
          v-if="currentStep < 6"
          type="primary"
          :disabled="(currentStep === 1 && !selectedClusterId) ||
                     (currentStep === 2 && !selectedWarehouseId) ||
                     (currentStep === 3 && !products.length) ||
                     (currentStep === 4 && !draftId) ||
                     (currentStep === 5 && !selectedTimeslot)"
          @click="goToStep(currentStep + 1)"
        >
          {{ t('postingConstructor.next') }}
        </n-button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.step-content {
  min-height: 200px;
  margin-bottom: 16px;
}

.step-content h3 {
  margin-bottom: 12px;
  color: var(--text);
}

.step-content h4 {
  margin: 16px 0 8px;
  color: var(--text);
}

.step-hint {
  color: var(--text-subtle);
  font-size: 0.9em;
}

.product-search-results {
  max-height: 240px;
  overflow-y: auto;
  border: 1px solid var(--border);
  border-radius: 6px;
  margin-bottom: 12px;
}

.product-search-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  border-bottom: 1px solid var(--border);
}

.product-search-item:last-child {
  border-bottom: none;
}

.added-products {
  margin-top: 16px;
}

.added-product-row {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 0;
  border-bottom: 1px solid var(--border);
}

.added-product-name {
  flex: 1;
  color: var(--text);
}

.timeslot-controls {
  display: flex;
  gap: 12px;
  align-items: flex-start;
  margin-bottom: 16px;
  flex-wrap: wrap;
}

.step-navigation {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-top: 16px;
  border-top: 1px solid var(--border);
  margin-top: 16px;
}

.selection-container {
  margin-bottom: 16px;
}

.selection-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 8px;
}

.selection-card {
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 12px 16px;
  cursor: pointer;
  transition: border-color 0.15s, background-color 0.15s;
}

.selection-card:hover {
  border-color: var(--ctp-blue);
  background: var(--bg-surface);
}

.selection-card--selected {
  border-color: var(--ctp-blue);
  background: var(--bg-overlay);
}

.selection-card__title {
  font-weight: 600;
  color: var(--text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
