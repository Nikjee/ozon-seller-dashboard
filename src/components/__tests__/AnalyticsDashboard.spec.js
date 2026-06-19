// @vitest-environment jsdom
import { mount } from '@vue/test-utils'
import { ref, nextTick } from 'vue'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import AnalyticsDashboard from '../AnalyticsDashboard.vue'
import { useI18n } from '../../composables/useI18n.js'
import { useAnalyticsDashboard } from '../../composables/useAnalyticsDashboard.js'

vi.mock('../../composables/useI18n.js', () => ({ useI18n: vi.fn() }))
vi.mock('../../composables/useAnalyticsDashboard.js', () => ({ useAnalyticsDashboard: vi.fn() }))
vi.mock('naive-ui', () => ({
  NSpin: { name: 'NSpin', props: ['show'], template: '<div class="n-spin" :data-show="String(show)"><slot /></div>' },
  NAlert: { name: 'NAlert', props: ['type', 'closable'], template: '<div class="n-alert" :data-type="type"><header><slot name="header" /></header><div><slot /></div></div>' },
  NButton: { name: 'NButton', emits: ['click'], template: '<button class="n-button" @click="$emit(\'click\')"><slot /></button>' },
  NCard: { name: 'NCard', template: '<div class="n-card"><slot /></div>' },
  NStatistic: { name: 'NStatistic', props: ['label', 'value', 'type'], template: '<div class="n-statistic" :data-type="type"><span class="label">{{ label }}</span><span class="value">{{ value }}</span></div>' },
  NTag: { name: 'NTag', props: ['type'], template: '<span class="n-tag" :data-type="type"><slot /></span>' },
  NDataTable: { name: 'NDataTable', props: ['columns', 'data'], template: '<div class="n-data-table"><div v-for="row in data" :key="row.sku || row.name" class="n-data-row">{{ row.name }} {{ row.sku }} {{ row.current_stock }} {{ row.turnover_grade }} {{ row.days_without_sales }}</div></div>' },
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
  NTag: { name: 'NTag', props: ['type'], template: '<span class="n-tag" :data-type="type"><slot /></span>' },
  NDataTable: { name: 'NDataTable', props: ['columns', 'data'], template: '<div class="n-data-table"><div v-for="row in data" :key="row.sku || row.name" class="n-data-row">{{ row.name }} {{ row.sku }} {{ row.current_stock }} {{ row.turnover_grade }} {{ row.days_without_sales }}</div></div>' },
  NEmpty: { name: 'NEmpty', props: ['description'], template: '<div class="n-empty">{{ description }}</div>' }
})

describe('AnalyticsDashboard.vue', () => {
  it('renders analytics data and refreshes on mount', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const refresh = vi.fn()
    vi.mocked(useAnalyticsDashboard).mockReturnValue({
      month: ref(1),
      year: ref(2026),
      data: ref({ totalRevenue: 5000, totalExpenses: 1200, totalProfit: 3800, margin: 0.76, products: [{ name: 'Widget', sku: 'SKU-1', current_stock: 42, ads: 3, idc: 11, turnover_grade: 'popular', days_without_sales: 2 }, { name: 'Gadget', sku: 'SKU-2', current_stock: 15, ads: 1, idc: 5, turnover_grade: 'deficit', days_without_sales: 8 }] }),
      loading: ref(false),
      error: ref(null),
      products: ref([{ name: 'Widget', sku: 'SKU-1', current_stock: 42, ads: 3, idc: 11, turnover_grade: 'popular', days_without_sales: 2 }, { name: 'Gadget', sku: 'SKU-2', current_stock: 15, ads: 1, idc: 5, turnover_grade: 'deficit', days_without_sales: 8 }]),
      overallAds: ref(2),
      overallIdc: ref(8),
      turnoverGrades: ref({ popular: 1, deficit: 1 }),
      stockBalanceTotal: ref(57),
      refresh,
    })

    const wrapper = mount(AnalyticsDashboard)
    await nextTick()

    expect(refresh).toHaveBeenCalledTimes(1)
    expect(wrapper.text()).toContain('analytics.title')
    expect(wrapper.text()).toContain('57')
    expect(wrapper.text()).toContain('Widget')
    expect(wrapper.text()).toContain('SKU-2')
    expect(wrapper.text()).toContain('popular: 1')
    expect(wrapper.text()).toContain('deficit: 1')
  })

  it('renders loading, error, and empty states', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const refresh = vi.fn()

    vi.mocked(useAnalyticsDashboard).mockReturnValue({
      month: ref(1),
      year: ref(2026),
      data: ref(null),
      loading: ref(true),
      error: ref(null),
      products: ref([]),
      overallAds: ref(0),
      overallIdc: ref(0),
      turnoverGrades: ref({}),
      stockBalanceTotal: ref(0),
      refresh,
    })

    const loadingWrapper = mount(AnalyticsDashboard)
    await nextTick()
    expect(loadingWrapper.get('.n-spin').attributes('data-show')).toBe('true')

    vi.mocked(useAnalyticsDashboard).mockReturnValue({
      month: ref(1),
      year: ref(2026),
      data: ref(null),
      loading: ref(false),
      error: ref('Analytics failed'),
      products: ref([]),
      overallAds: ref(0),
      overallIdc: ref(0),
      turnoverGrades: ref({}),
      stockBalanceTotal: ref(0),
      refresh,
    })

    const errorWrapper = mount(AnalyticsDashboard)
    await nextTick()
    expect(errorWrapper.text()).toContain('Analytics failed')
    await errorWrapper.get('button').trigger('click')
    expect(refresh).toHaveBeenCalledTimes(3)

    vi.mocked(useAnalyticsDashboard).mockReturnValue({
      month: ref(1),
      year: ref(2026),
      data: ref(null),
      loading: ref(false),
      error: ref(null),
      products: ref([]),
      overallAds: ref(0),
      overallIdc: ref(0),
      turnoverGrades: ref({}),
      stockBalanceTotal: ref(0),
      refresh,
    })

    const emptyWrapper = mount(AnalyticsDashboard)
    await nextTick()
    expect(emptyWrapper.text()).toContain('noData')
  })
})
