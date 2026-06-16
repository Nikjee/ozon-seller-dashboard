import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export function useTransactionTotals() {
  const month = ref(new Date().getMonth() + 1)
  const year = ref(new Date().getFullYear())

  const data = ref(null)
  const loading = ref(false)
  const error = ref(null)

  function getDateRange() {
    const firstDay = new Date(year.value, month.value - 1, 1)
    const lastDay = new Date(year.value, month.value, 0)

    const formatDate = (date) => {
      const y = date.getFullYear()
      const m = String(date.getMonth() + 1).padStart(2, '0')
      const d = String(date.getDate()).padStart(2, '0')
      return `${y}-${m}-${d}`
    }

    return {
      dateFrom: formatDate(firstDay),
      dateTo: formatDate(lastDay)
    }
  }

  async function load() {
    loading.value = true
    error.value = null
    try {
      const { dateFrom, dateTo } = getDateRange()
      data.value = await invoke('get_finance_totals', {
        dateFrom,
        dateTo
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

  const accrualsForSale = computed(() => data.value?.accrualsForSale ?? data.value?.accruals_for_sale ?? 0)
  const totalCompensation = computed(() => data.value?.totalCompensation ?? data.value?.total_compensation ?? 0)
  const saleCommission = computed(() => data.value?.saleCommission ?? data.value?.sale_commission ?? 0)
  const servicesAmount = computed(() => data.value?.servicesAmount ?? data.value?.services_amount ?? 0)
  const processingAndDelivery = computed(() => data.value?.processingAndDelivery ?? data.value?.processing_and_delivery ?? 0)
  const refundsAndCancellations = computed(() => data.value?.refundsAndCancellations ?? data.value?.refunds_and_cancellations ?? 0)
  const moneyTransfer = computed(() => data.value?.moneyTransfer ?? data.value?.money_transfer ?? 0)
  const othersAmount = computed(() => data.value?.othersAmount ?? data.value?.others_amount ?? 0)

  watch([month, year], () => {
    load()
  })

  return {
    month,
    year,
    data,
    loading,
    error,
    accrualsForSale,
    totalCompensation,
    saleCommission,
    servicesAmount,
    processingAndDelivery,
    refundsAndCancellations,
    moneyTransfer,
    othersAmount,
    refresh
  }
}
