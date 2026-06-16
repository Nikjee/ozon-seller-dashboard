// @vitest-environment jsdom
import { mount } from '@vue/test-utils'
import { ref, nextTick } from 'vue'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import TransactionTotals from '../TransactionTotals.vue'
import { useI18n } from '../../composables/useI18n.js'
import { useTransactionTotals } from '../../composables/useTransactionTotals.js'

vi.mock('../../composables/useI18n.js', () => ({ useI18n: vi.fn() }))
vi.mock('../../composables/useTransactionTotals.js', () => ({ useTransactionTotals: vi.fn() }))
vi.mock('naive-ui', () => ({
  NSpin: { name: 'NSpin', props: ['show'], template: '<div class="n-spin" :data-show="String(show)"><slot /></div>' },
  NAlert: { name: 'NAlert', props: ['type', 'closable'], template: '<div class="n-alert" :data-type="type"><header><slot name="header" /></header><div><slot /></div></div>' },
  NButton: { name: 'NButton', emits: ['click'], template: '<button class="n-button" @click="$emit(\'click\')"><slot /></button>' },
  NCard: { name: 'NCard', template: '<div class="n-card"><slot /></div>' },
  NStatistic: { name: 'NStatistic', props: ['label', 'value', 'type'], template: '<div class="n-statistic" :data-type="type"><span class="label">{{ label }}</span><span class="value">{{ value }}</span></div>' },
  NEmpty: { name: 'NEmpty', props: ['description'], template: '<div class="n-empty">{{ description }}</div>' }
}))

beforeEach(() => {
  vi.clearAllMocks()
})

const createStubs = () => ({
  NSpin: { name: 'NSpin', props: ['show'], template: '<div class="n-spin" :data-show="String(show)"><slot /></div>' },
  NAlert: { name: 'NAlert', props: ['type', 'closable'], template: '<div class="n-alert" :data-type="type"><header><slot name="header" /></header><div><slot /></div></div>' },
  NButton: { name: 'NButton', emits: ['click'], template: '<button class="n-button" @click="$emit(\'click\')"><slot /></button>' },
  NCard: { name: 'NCard', template: '<div class="n-card"><slot /></div>' },
  NStatistic: { name: 'NStatistic', props: ['label', 'value', 'type'], template: '<div class="n-statistic" :data-type="type"><span class="label">{{ label }}</span><span class="value">{{ value }}</span></div>' },
  NEmpty: { name: 'NEmpty', props: ['description'], template: '<div class="n-empty">{{ description }}</div>' }
})

describe('TransactionTotals.vue', () => {
  it('renders financial totals and refreshes on mount', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const refresh = vi.fn()
    vi.mocked(useTransactionTotals).mockReturnValue({
      month: ref(1),
      year: ref(2026),
      data: ref({ accrualsForSale: 100, saleCommission: -20, processingAndDelivery: 10, servicesAmount: 5, othersAmount: 3, refundsAndCancellations: -7, totalCompensation: 15, moneyTransfer: 106 }),
      loading: ref(false),
      error: ref(null),
      accrualsForSale: ref(100),
      totalCompensation: ref(15),
      saleCommission: ref(-20),
      servicesAmount: ref(5),
      processingAndDelivery: ref(10),
      refundsAndCancellations: ref(-7),
      moneyTransfer: ref(106),
      othersAmount: ref(3),
      refresh,
    })

    const wrapper = mount(TransactionTotals)
    await nextTick()

    expect(refresh).toHaveBeenCalledTimes(1)
    expect(wrapper.text()).toContain('totals.title')
    expect(wrapper.text()).toContain('100')
    expect(wrapper.text()).toContain('106')
    const stats = wrapper.findAll('.n-statistic')
    expect(stats.find((node) => node.text().includes('totals.saleCommission') && node.text().includes('-20'))?.attributes('data-type')).toBe('error')
    expect(stats.find((node) => node.text().includes('totals.moneyTransfer') && node.text().includes('106'))?.attributes('data-type')).toBeUndefined()
  })

  it('renders loading, error, and empty states', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const refresh = vi.fn()

    vi.mocked(useTransactionTotals).mockReturnValue({
      month: ref(1),
      year: ref(2026),
      data: ref(null),
      loading: ref(true),
      error: ref(null),
      accrualsForSale: ref(0),
      totalCompensation: ref(0),
      saleCommission: ref(0),
      servicesAmount: ref(0),
      processingAndDelivery: ref(0),
      refundsAndCancellations: ref(0),
      moneyTransfer: ref(0),
      othersAmount: ref(0),
      refresh,
    })

    const loadingWrapper = mount(TransactionTotals)
    await nextTick()
    expect(loadingWrapper.get('.n-spin').attributes('data-show')).toBe('true')

    vi.mocked(useTransactionTotals).mockReturnValue({
      month: ref(1),
      year: ref(2026),
      data: ref(null),
      loading: ref(false),
      error: ref('Finance totals failed'),
      accrualsForSale: ref(0),
      totalCompensation: ref(0),
      saleCommission: ref(0),
      servicesAmount: ref(0),
      processingAndDelivery: ref(0),
      refundsAndCancellations: ref(0),
      moneyTransfer: ref(0),
      othersAmount: ref(0),
      refresh,
    })

    const errorWrapper = mount(TransactionTotals)
    await nextTick()
    expect(errorWrapper.text()).toContain('Finance totals failed')
    await errorWrapper.get('button').trigger('click')
    expect(refresh).toHaveBeenCalledTimes(3)

    vi.mocked(useTransactionTotals).mockReturnValue({
      month: ref(1),
      year: ref(2026),
      data: ref(null),
      loading: ref(false),
      error: ref(null),
      accrualsForSale: ref(0),
      totalCompensation: ref(0),
      saleCommission: ref(0),
      servicesAmount: ref(0),
      processingAndDelivery: ref(0),
      refundsAndCancellations: ref(0),
      moneyTransfer: ref(0),
      othersAmount: ref(0),
      refresh,
    })

    const emptyWrapper = mount(TransactionTotals)
    await nextTick()
    expect(emptyWrapper.text()).toContain('noData')
  })
})
