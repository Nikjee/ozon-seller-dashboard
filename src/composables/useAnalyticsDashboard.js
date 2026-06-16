import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export function useAnalyticsDashboard() {
  const data = ref(null)
  const loading = ref(false)
  const error = ref(null)

  const products = computed(() => data.value?.products ?? [])
  const overallAds = computed(() => data.value?.overallAds ?? 0)
  const overallIdc = computed(() => data.value?.overallIdc ?? 0)
  const turnoverGrades = computed(() => data.value?.turnoverGrades ?? {})
  const stockBalanceTotal = computed(() => data.value?.stockBalanceTotal ?? 0)

  async function refresh() {
    loading.value = true
    error.value = null
    try {
      data.value = await invoke('get_analytics_dashboard_data')
    } catch (e) {
      error.value = e
      data.value = null
    } finally {
      loading.value = false
    }
  }

  return {
    data,
    loading,
    error,
    products,
    overallAds,
    overallIdc,
    turnoverGrades,
    stockBalanceTotal,
    refresh
  }
}
