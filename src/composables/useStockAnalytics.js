import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export function useStockAnalytics() {
  const data = ref(null)
  const loading = ref(false)
  const error = ref(null)

  async function load() {
    loading.value = true
    error.value = null
    try {
      data.value = {
        ...(await invoke('get_stock_report'))
      }
    } catch (e) {
      error.value = typeof e === 'string' ? e : (e.message || 'Unknown error')
      data.value = null
    } finally {
      loading.value = false
    }
  }

  function refresh() {
    load()
  }

  const stockByWarehouse = computed(() => data.value?.stock_by_warehouse || [])
  const stockByProduct = computed(() => data.value?.stock_by_product || [])
  const totalFreeToSell = computed(() => data.value?.total_free_to_sell || 0)
  const totalReserved = computed(() => data.value?.total_reserved || 0)

  return {
    data,
    loading,
    error,
    stockByWarehouse,
    stockByProduct,
    totalFreeToSell,
    totalReserved,
    refresh
  }
}
