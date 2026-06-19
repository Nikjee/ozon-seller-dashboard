// @vitest-environment jsdom
import { mount } from '@vue/test-utils'
import { ref, nextTick } from 'vue'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import StockAnalytics from '../StockAnalytics.vue'
import { useI18n } from '../../composables/useI18n.js'
import { useStockAnalytics } from '../../composables/useStockAnalytics.js'

vi.mock('../../composables/useI18n.js', () => ({ useI18n: vi.fn() }))
vi.mock('../../composables/useStockAnalytics.js', () => ({ useStockAnalytics: vi.fn() }))
vi.mock('naive-ui', () => ({
  NSpin: { name: 'NSpin', props: ['show'], template: '<div class="n-spin" :data-show="String(show)"><slot /></div>' },
  NAlert: { name: 'NAlert', props: ['type', 'closable'], template: '<div class="n-alert" :data-type="type"><header><slot name="header" /></header><div><slot /></div></div>' },
  NButton: { name: 'NButton', emits: ['click'], template: '<button class="n-button" @click="$emit(\'click\')"><slot /></button>' },
  NCard: { name: 'NCard', template: '<div class="n-card"><slot /></div>' },
  NStatistic: { name: 'NStatistic', props: ['label', 'value', 'type'], template: '<div class="n-statistic" :data-type="type"><span class="label">{{ label }}</span><span class="value">{{ value }}</span></div>' },
  NDataTable: { name: 'NDataTable', props: ['columns', 'data'], template: '<div class="n-data-table"><div v-for="row in data" :key="row.warehouse_name || row.stock_type" class="n-data-row">{{ row.warehouse_name }} {{ row.stock_type }} {{ row.free_to_sell }} {{ row.reserved }} {{ row.total }}</div></div>' },
  NEmpty: { name: 'NEmpty', props: ['description'], template: '<div class="n-empty">{{ description }}</div>' },
  NPopover: { name: 'NPopover', props: ['show', 'placement', 'trigger'], template: '<div class="n-popover" v-if="show"><slot /></div>' },
  NCheckbox: { name: 'NCheckbox', props: ['checked'], template: '<span class="n-checkbox" :data-checked="String(checked)"><slot /></span>' },
  NSpace: { name: 'NSpace', props: ['vertical', 'size'], template: '<div class="n-space"><slot /></div>' }
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
  NDataTable: { name: 'NDataTable', props: ['columns', 'data'], template: '<div class="n-data-table"><div v-for="row in data" :key="row.warehouse_name || row.stock_type" class="n-data-row">{{ row.warehouse_name }} {{ row.stock_type }} {{ row.free_to_sell }} {{ row.reserved }} {{ row.total }}</div></div>' },
  NEmpty: { name: 'NEmpty', props: ['description'], template: '<div class="n-empty">{{ description }}</div>' },
  NPopover: { name: 'NPopover', props: ['show', 'placement', 'trigger'], template: '<div class="n-popover" v-if="show"><slot /></div>' },
  NCheckbox: { name: 'NCheckbox', props: ['checked'], template: '<span class="n-checkbox" :data-checked="String(checked)"><slot /></span>' },
  NSpace: { name: 'NSpace', props: ['vertical', 'size'], template: '<div class="n-space"><slot /></div>' }
})

describe('StockAnalytics.vue', () => {
  it('renders data state and calls refresh on mount', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const refresh = vi.fn()
    vi.mocked(useStockAnalytics).mockReturnValue({
      month: ref(1),
      year: ref(2026),
      data: ref({ stock_by_warehouse: [{ warehouse_name: 'Moscow', stock_type: 'FBO', free_to_sell: 12, reserved: 3, total: 15 }], stock_by_product: [] }),
      loading: ref(false),
      error: ref(null),
      stockByWarehouse: ref([{ warehouse_name: 'Moscow', stock_type: 'FBO', free_to_sell: 12, reserved: 3, total: 15 }]),
      stockByProduct: ref([]),
      totalFreeToSell: ref(12),
      totalReserved: ref(3),
      refresh,
    })

    const wrapper = mount(StockAnalytics)
    await nextTick()

    expect(refresh).toHaveBeenCalledTimes(1)
    expect(wrapper.text()).toContain('stocks.title')
    expect(wrapper.text()).toContain('stocks.freeStock')
    expect(wrapper.text()).toContain('12')
    expect(wrapper.text()).toContain('Moscow')
  })

  it('renders loading, error, and empty states', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const refresh = vi.fn()
    vi.mocked(useStockAnalytics).mockReturnValue({
      month: ref(1),
      year: ref(2026),
      data: ref(null),
      loading: ref(true),
      error: ref(null),
      stockByWarehouse: ref([]),
      stockByProduct: ref([]),
      totalFreeToSell: ref(0),
      totalReserved: ref(0),
      refresh,
    })

    const loadingWrapper = mount(StockAnalytics)
    await nextTick()
    expect(loadingWrapper.get('.n-spin').attributes('data-show')).toBe('true')

    vi.mocked(useStockAnalytics).mockReturnValue({
      month: ref(1),
      year: ref(2026),
      data: ref(null),
      loading: ref(false),
      error: ref('Failed to load stock report'),
      stockByWarehouse: ref([]),
      stockByProduct: ref([]),
      totalFreeToSell: ref(0),
      totalReserved: ref(0),
      refresh,
    })

    const errorWrapper = mount(StockAnalytics)
    await nextTick()
    expect(errorWrapper.text()).toContain('Failed to load stock report')
    await errorWrapper.get('button').trigger('click')
    expect(refresh).toHaveBeenCalledTimes(3)

    vi.mocked(useStockAnalytics).mockReturnValue({
      month: ref(1),
      year: ref(2026),
      data: ref(null),
      loading: ref(false),
      error: ref(null),
      stockByWarehouse: ref([]),
      stockByProduct: ref([]),
      totalFreeToSell: ref(0),
      totalReserved: ref(0),
      refresh,
    })

    const emptyWrapper = mount(StockAnalytics)
    await nextTick()
    expect(emptyWrapper.text()).toContain('noData')
  })
})
