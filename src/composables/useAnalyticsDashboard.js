import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export function useAnalyticsDashboard() {
  const month = ref(new Date().getMonth() + 1)
  const year = ref(new Date().getFullYear())

  const data = ref(null)
  const loading = ref(false)
  const error = ref(null)

  async function load() {
    loading.value = true
    error.value = null
    try {
      data.value = await invoke('get_stock_analytics', {
        skus: []
      })
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

  const products = computed(() => data.value?.products || data.value?.items || [])

  const overallAds = computed(() => {
    const list = products.value || []
    if (!list.length) return 0
    const total = list.reduce((sum, item) => sum + Number(item?.ads ?? item?.ads_count ?? 0), 0)
    return total / list.length
  })

  const overallIdc = computed(() => {
    const list = products.value || []
    if (!list.length) return 0
    const total = list.reduce((sum, item) => sum + Number(item?.idc ?? item?.idc_days ?? 0), 0)
    return total / list.length
  })

  const turnoverGrades = computed(() => {
    const list = products.value || []
    return list.reduce((acc, item) => {
      const grade = item?.turnover_grade || item?.turnoverGrade || item?.idc_grade || item?.idcGrade
      if (!grade) return acc
      acc[grade] = (acc[grade] || 0) + 1
      return acc
    }, {})
  })

  const stockBalanceTotal = computed(() => {
    const list = products.value || []
    return list.reduce((sum, item) => {
      return sum + Number(item?.current_stock ?? item?.stock_balance ?? item?.stockBalance ?? 0)
    }, 0)
  })

  watch([month, year], () => {
    load()
  })

  return {
    month,
    year,
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
