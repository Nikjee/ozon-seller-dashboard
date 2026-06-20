import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

function pollStatus(invokeFn, checkSuccess, checkFailed, maxRetries = 30) {
  return new Promise((resolve, reject) => {
    let attempts = 0

    async function tick() {
      attempts++
      try {
        const result = await invokeFn()
        if (checkSuccess(result)) return resolve(result)
        if (checkFailed(result)) return reject(new Error('Operation failed'))
      } catch (e) {
        return reject(new Error(typeof e === 'string' ? e : (e.message || 'Unknown error')))
      }
      if (attempts >= maxRetries) return reject(new Error('Operation timed out'))
      setTimeout(tick, 1000)
    }

    tick()
  })
}

const INITIAL_STATE = {
  currentStep: 1,
  warehouses: [],
  selectedWarehouseId: null,
  clusters: [],
  selectedClusterId: null,
  warehouseSearchQuery: '',
  clusterSearchQuery: '',
  allProducts: [],
  productSearchQuery: '',
  products: [],
  draftOperationId: null,
  draftInfo: null,
  draftId: null,
  timeslots: [],
  selectedTimeslot: null,
  supplyOperationId: null,
  supplyResult: null,
  loading: false,
  error: null,
}

export function usePostingConstructor() {
  // ── Step state ──
  const currentStep = ref(INITIAL_STATE.currentStep)

  // ── Step 1: Warehouse ──
  const warehouses = ref(INITIAL_STATE.warehouses)
  const selectedWarehouseId = ref(INITIAL_STATE.selectedWarehouseId)

  // ── Step 2: Clusters ──
  const clusters = ref(INITIAL_STATE.clusters)
  const selectedClusterId = ref(INITIAL_STATE.selectedClusterId)

  // ── Search queries ──
  const warehouseSearchQuery = ref(INITIAL_STATE.warehouseSearchQuery)
  const clusterSearchQuery = ref(INITIAL_STATE.clusterSearchQuery)

  // ── Step 3: Products ──
  const allProducts = ref(INITIAL_STATE.allProducts)
  const productSearchQuery = ref(INITIAL_STATE.productSearchQuery)
  const products = ref(INITIAL_STATE.products)

  // ── Steps 4-5: Draft ──
  const draftOperationId = ref(INITIAL_STATE.draftOperationId)
  const draftInfo = ref(INITIAL_STATE.draftInfo)
  const draftId = ref(INITIAL_STATE.draftId)

  // ── Step 6: Timeslot ──
  const timeslots = ref(INITIAL_STATE.timeslots)
  const selectedTimeslot = ref(INITIAL_STATE.selectedTimeslot)

  // ── Step 7: Supply creation ──
  const supplyOperationId = ref(INITIAL_STATE.supplyOperationId)
  const supplyResult = ref(INITIAL_STATE.supplyResult)

  // ── Shared ──
  const loading = ref(INITIAL_STATE.loading)
  const error = ref(INITIAL_STATE.error)

  // ── Computed ──
  const productSearchResults = computed(() => {
    const q = productSearchQuery.value.toLowerCase().trim()
    if (!q) return allProducts.value
    return allProducts.value.filter(
      (p) =>
        (p.name && p.name.toLowerCase().includes(q)) ||
        (p.offer_id && p.offer_id.toLowerCase().includes(q))
    )
  })

  // ── Navigation ──
  function goToStep(n) {
    currentStep.value = n
  }

  // ── Step 1: Load warehouses ──
  async function loadWarehouses() {
    loading.value = true
    error.value = null
    try {
      warehouses.value = await invoke('get_available_warehouses')
    } catch (e) {
      error.value = typeof e === 'string' ? e : (e.message || 'Unknown error')
      warehouses.value = []
    } finally {
      loading.value = false
    }
  }

  // ── Step 2: Load clusters ──
  async function loadClusters() {
    loading.value = true
    error.value = null
    try {
      clusters.value = await invoke('get_cluster_list')
    } catch (e) {
      error.value = typeof e === 'string' ? e : (e.message || 'Unknown error')
      clusters.value = []
    } finally {
      loading.value = false
    }
  }

  // ── Step 3: Load products ──
  async function loadProducts() {
    loading.value = true
    error.value = null
    try {
      const result = await invoke('list_products')
      allProducts.value = result?.products ?? []
    } catch (e) {
      error.value = typeof e === 'string' ? e : (e.message || 'Unknown error')
      allProducts.value = []
    } finally {
      loading.value = false
    }
  }

  function addProduct(sku, offerId, name, quantity) {
    products.value.push({ sku, offer_id: offerId, name, quantity })
  }

  function removeProduct(index) {
    products.value.splice(index, 1)
  }

  // ── Step 4: Submit draft ──
  async function submitDraft() {
    loading.value = true
    error.value = null
    try {
      const items = JSON.stringify(
        products.value.map((p) => ({ sku: p.sku, quantity: p.quantity }))
      )
      const result = await invoke('create_supply_draft', {
        clusterIds: [selectedClusterId.value],
        items,
      })
      draftOperationId.value = result?.operation_id ?? null
      await pollDraftInfo()
    } catch (e) {
      error.value = typeof e === 'string' ? e : (e.message || 'Unknown error')
    } finally {
      loading.value = false
    }
  }

  // ── Step 5: Poll draft info ──
  async function pollDraftInfo() {
    const opId = draftOperationId.value
    if (!opId) {
      error.value = 'No draft operation ID'
      return
    }
    try {
      const result = await pollStatus(
        () => invoke('get_draft_info', { operationId: opId }),
        (r) => r?.status === 'CALCULATION_STATUS_SUCCESS',
        (r) => r?.status === 'CALCULATION_STATUS_FAILED'
      )
      draftInfo.value = result
      draftId.value = result?.draft_id ?? null
    } catch (e) {
      error.value = typeof e === 'string' ? e : (e.message || 'Unknown error')
    }
  }

  // ── Step 6: Load timeslots ──
  async function loadTimeslots(dateFrom, dateTo) {
    loading.value = true
    error.value = null
    try {
      const warehouseIds = selectedWarehouseId.value ? [selectedWarehouseId.value] : []
      timeslots.value = await invoke('get_timeslots', {
        draftId: draftId.value,
        warehouseIds,
        dateFrom,
        dateTo,
      })
    } catch (e) {
      error.value = typeof e === 'string' ? e : (e.message || 'Unknown error')
      timeslots.value = []
    } finally {
      loading.value = false
    }
  }

  // ── Step 7: Submit supply ──
  async function submitSupply() {
    loading.value = true
    error.value = null
    try {
      if (!selectedTimeslot.value) {
        error.value = 'No timeslot selected'
        return
      }
      const ts = selectedTimeslot.value
      const result = await invoke('create_supply_from_draft', {
        draftId: draftId.value,
        warehouseId: selectedWarehouseId.value,
        timeslotFrom: ts.from ?? ts.timeslot_from ?? '',
        timeslotTo: ts.to ?? ts.timeslot_to ?? '',
      })
      supplyOperationId.value = result?.operation_id ?? null
      await pollSupplyStatus()
    } catch (e) {
      error.value = typeof e === 'string' ? e : (e.message || 'Unknown error')
    } finally {
      loading.value = false
    }
  }

  // ── Poll supply status ──
  async function pollSupplyStatus() {
    const opId = supplyOperationId.value
    if (!opId) {
      error.value = 'No supply operation ID'
      return
    }
    try {
      const result = await pollStatus(
        () => invoke('get_supply_create_status', { operationId: opId }),
        (r) => r?.status === 'DraftSupplyCreateStatusSuccess',
        (r) => r?.status === 'DraftSupplyCreateStatusFailed'
      )
      supplyResult.value = result
    } catch (e) {
      error.value = typeof e === 'string' ? e : (e.message || 'Unknown error')
    }
  }

  // ── Reset ──
  function reset() {
    currentStep.value = INITIAL_STATE.currentStep
    warehouses.value = INITIAL_STATE.warehouses
    selectedWarehouseId.value = INITIAL_STATE.selectedWarehouseId
    clusters.value = INITIAL_STATE.clusters
    selectedClusterId.value = INITIAL_STATE.selectedClusterId
    warehouseSearchQuery.value = INITIAL_STATE.warehouseSearchQuery
    clusterSearchQuery.value = INITIAL_STATE.clusterSearchQuery
    allProducts.value = INITIAL_STATE.allProducts
    productSearchQuery.value = INITIAL_STATE.productSearchQuery
    products.value = INITIAL_STATE.products
    draftOperationId.value = INITIAL_STATE.draftOperationId
    draftInfo.value = INITIAL_STATE.draftInfo
    draftId.value = INITIAL_STATE.draftId
    timeslots.value = INITIAL_STATE.timeslots
    selectedTimeslot.value = INITIAL_STATE.selectedTimeslot
    supplyOperationId.value = INITIAL_STATE.supplyOperationId
    supplyResult.value = INITIAL_STATE.supplyResult
    loading.value = INITIAL_STATE.loading
    error.value = INITIAL_STATE.error
  }

  return {
    // step state
    currentStep,
    goToStep,
    // step 1
    warehouses,
    selectedWarehouseId,
    loadWarehouses,
    // step 2
    clusters,
    selectedClusterId,
    loadClusters,
    // search queries
    warehouseSearchQuery,
    clusterSearchQuery,
    // step 3
    allProducts,
    productSearchQuery,
    products,
    productSearchResults,
    loadProducts,
    addProduct,
    removeProduct,
    // step 4-5
    draftOperationId,
    draftInfo,
    draftId,
    submitDraft,
    pollDraftInfo,
    // step 6
    timeslots,
    selectedTimeslot,
    loadTimeslots,
    // step 7
    supplyOperationId,
    supplyResult,
    submitSupply,
    pollSupplyStatus,
    // shared
    loading,
    error,
    reset,
  }
}
