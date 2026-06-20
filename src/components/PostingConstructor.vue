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
  warehouses, selectedWarehouseId, loadWarehouses,
  clusters, selectedClusterIds, loadClusters,
  allProducts, productSearchQuery, products, productSearchResults,
  loadProducts, addProduct, removeProduct,
  draftId, submitDraft,
  timeslots, selectedTimeslot, loadTimeslots,
  supplyResult, submitSupply,
  loading, error, reset
} = usePostingConstructor()

const timeslotDateRange = ref(null)

const selectedWarehouseName = computed(() => {
  const w = warehouses.value.find(w => w.warehouse_id === selectedWarehouseId.value)
  return w ? (w.name || `ID: ${w.warehouse_id}`) : ''
})

const selectedClusterName = computed(() => {
  if (!selectedClusterIds.value.length) return ''
  const c = clusters.value.find(c => c.cluster_id === selectedClusterIds.value[0])
  return c ? (c.name || `ID: ${c.cluster_id}`) : ''
})

const totalProductQuantity = computed(() =>
  products.value.reduce((sum, p) => sum + (p.quantity || 0), 0)
)

onMounted(() => {
  loadWarehouses()
})

watch(currentStep, (step) => {
  if (step === 2 && !clusters.value.length) loadClusters()
  if (step === 3 && !allProducts.value.length) loadProducts()
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
  loadWarehouses()
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

      <!-- Step 1: Warehouse Selection -->
      <div v-if="currentStep === 1" class="step-content">
        <h3>{{ t('postingConstructor.selectWarehouse') }}</h3>
        <n-spin :show="loading">
          <template v-if="error">
            <n-alert type="error" closable>
              <template #header>{{ t('postingConstructor.error') }}</template>
              {{ error }}
            </n-alert>
            <n-button @click="loadWarehouses" style="margin-top: 12px;">
              {{ t('postingConstructor.retry') }}
            </n-button>
          </template>
          <template v-else-if="warehouses.length">
            <n-radio-group v-model:value="selectedWarehouseId" vertical>
              <n-radio
                v-for="w in warehouses"
                :key="w.warehouse_id"
                :value="w.warehouse_id"
                style="margin-bottom: 8px;"
              >
                {{ w.name }} <span v-if="w.address" class="step-hint">— {{ w.address }}</span>
              </n-radio>
            </n-radio-group>
          </template>
          <n-empty v-else :description="t('postingConstructor.noWarehouses')" />
        </n-spin>
      </div>

      <!-- Step 2: Cluster Selection -->
      <div v-if="currentStep === 2" class="step-content">
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
            <n-radio-group v-model:value="selectedClusterIds" vertical>
              <n-radio
                v-for="c in clusters"
                :key="c.cluster_id"
                :value="c.cluster_id"
                style="margin-bottom: 8px;"
              >
                {{ c.name }}
              </n-radio>
            </n-radio-group>
          </template>
          <n-empty v-else :description="t('postingConstructor.noClusters')" />
        </n-spin>
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
            <div v-if="productSearchResults.length" class="product-search-results">
              <div
                v-for="p in productSearchResults"
                :key="p.sku"
                class="product-search-item"
              >
                <span>{{ p.name }} ({{ p.offer_id }})</span>
                <n-button size="small" @click="addProduct(p.sku, p.offer_id, p.name, 1)">
                  {{ t('postingConstructor.addProduct') }}
                </n-button>
              </div>
            </div>
            <n-empty v-else :description="t('postingConstructor.noProducts')" />

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
            <n-button type="primary" @click="submitSupply">
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
          :disabled="(currentStep === 1 && !selectedWarehouseId) ||
                     (currentStep === 2 && !selectedClusterIds.length) ||
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
  color: var(--ctp-text);
}

.step-content h4 {
  margin: 16px 0 8px;
  color: var(--ctp-subtext1);
}

.step-hint {
  color: var(--ctp-subtext0);
  font-size: 0.9em;
}

.product-search-results {
  max-height: 240px;
  overflow-y: auto;
  border: 1px solid var(--ctp-surface1);
  border-radius: 6px;
  margin-bottom: 12px;
}

.product-search-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  border-bottom: 1px solid var(--ctp-surface1);
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
  border-bottom: 1px solid var(--ctp-surface1);
}

.added-product-name {
  flex: 1;
  color: var(--ctp-text);
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
  border-top: 1px solid var(--ctp-surface1);
  margin-top: 16px;
}
</style>
