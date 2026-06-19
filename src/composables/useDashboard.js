import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export function useDashboard() {
  const month = ref(new Date().getMonth() + 1)
  const year = ref(new Date().getFullYear())

  const data = ref(null)
  const loading = ref(false)
  const error = ref(null)
  const fboTotals = ref(null)
  const fboLoading = ref(false)

  async function load() {
    loading.value = true
    error.value = null
    fboTotals.value = null
    try {
      data.value = await invoke('get_dashboard_summary', {
        month: month.value,
        year: year.value
      })
      console.log("PRODUCTS", data.value);
      loadFboTotals()
    } catch (e) {
      error.value = typeof e === 'string' ? e : (e.message || 'Unknown error')
      data.value = null
    } finally {
      loading.value = false
    }
  }

  async function loadFboTotals() {
    fboLoading.value = true
    try {
      fboTotals.value = await invoke('get_fbo_totals', {
        month: month.value,
        year: year.value
      })
    } catch (e) {
      console.error('FBO totals unavailable:', e)
      fboTotals.value = null
    } finally {
      fboLoading.value = false
    }
  }

  function refresh() {
    load()
  }

  const totals = computed(() => data.value?.totals || null)
  const accountExpenses = computed(() => data.value?.account_expenses || null)
  const products = computed(() => data.value?.products || [])
  const notDeliveredProducts = computed(() => data.value?.not_delivered || [])
  const totalProducts = computed(() => data.value?.total_products || 0)

  const months = [
    'Январь', 'Февраль', 'Март', 'Апрель', 'Май', 'Июнь',
    'Июль', 'Август', 'Сентябрь', 'Октябрь', 'Ноябрь', 'Декабрь'
  ]

  const monthLabel = computed(() => `${months[month.value - 1]} ${year.value}`)

  const years = computed(() => {
    const current = new Date().getFullYear()
    const list = []
    for (let y = current - 2; y <= current; y++) {
      list.push(y)
    }
    return list
  })

  watch([month, year], () => {
    load()
  })

  return {
    month,
    year,
    months,
    years,
    monthLabel,
    data,
    loading,
    error,
    totals,
    accountExpenses,
    products,
    notDeliveredProducts,
    totalProducts,
    fboTotals,
    fboLoading,
    refresh
  }
}
