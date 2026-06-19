// @vitest-environment jsdom
import { mount } from '@vue/test-utils'
import { ref, nextTick } from 'vue'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import ProductRow from '../ProductRow.vue'
import { useI18n } from '../../composables/useI18n.js'

vi.mock('../../composables/useI18n.js', () => ({ useI18n: vi.fn() }))
vi.mock('../PostingRow.vue', () => ({
  default: {
    name: 'PostingRow',
    props: ['posting'],
    template: '<div class="posting-row-stub">{{ posting.posting_number }}</div>'
  }
}))

beforeEach(() => {
  vi.clearAllMocks()
})

function makeProduct(overrides = {}) {
  return {
    name: 'Test Widget',
    sku: 'SKU-001',
    has_fbo_stocks: true,
    has_fbs_stocks: false,
    summary: {
      total_quantity: 100,
      total_revenue: 50000,
      total_commission: -10000,
      total_delivery: -5000,
      total_returns: -2000,
      service_expenses: 3000,
      expenses_cats: { ad: 500, storage: 1000, logistics: 800, compensation: 200, other: 500 },
      net_profit: 35000
    },
    costs: {
      commission: -10000,
      acquiring: -500,
      order_processing: -300,
      logistics: -4000,
      delivery_to_pickup: -1000,
      placement: -200,
      return_processing: -300,
      return_logistics: -200,
      disposal: 0,
      ovh_processing: 0,
      operational_errors: 0,
      pay_per_click: 0,
      pay_per_order: 0,
      star_products: 0,
      paid_brand: 0,
      reviews_cost: 0,
      discount_points: 0,
      partner_programs: 0,
      compensation: 0,
      other_services: 0
    },
    postings: [
      { posting_number: 'ABC-1', date: '2025-01-15', seller_price_per_instance: 500, commission_amount: -100, delivery_charge: -50, return_charge: 0, net: 350 },
      { posting_number: 'ABC-2', date: '2025-01-16', seller_price_per_instance: 700, commission_amount: -120, delivery_charge: -60, return_charge: 0, net: 520 }
    ],
    ...overrides
  }
}

describe('ProductRow.vue', () => {
  it('renders compact mode with product basic info', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const product = makeProduct()
    const wrapper = mount(ProductRow, {
      props: { product, enriched: false }
    })
    await nextTick()

    expect(wrapper.text()).toContain('Test Widget')
    expect(wrapper.text()).toContain('FBO')
    expect(wrapper.text()).toContain('100')
  })

  it('renders positive profit with positive class in compact mode', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const product = makeProduct({ summary: { ...makeProduct().summary, net_profit: 35000 } })
    const wrapper = mount(ProductRow, {
      props: { product, enriched: false }
    })
    await nextTick()

    const profitEl = wrapper.find('.product-row__profit')
    expect(profitEl.classes()).toContain('amount-positive')
    expect(profitEl.classes()).not.toContain('amount-negative')
  })

  it('renders negative profit with negative class in compact mode', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const product = makeProduct({ summary: { ...makeProduct().summary, net_profit: -5000 } })
    const wrapper = mount(ProductRow, {
      props: { product, enriched: false }
    })
    await nextTick()

    const profitEl = wrapper.find('.product-row__profit')
    expect(profitEl.classes()).toContain('amount-negative')
    expect(profitEl.classes()).not.toContain('amount-positive')
  })

  it('toggles expense breakdown on info icon click', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const product = makeProduct()
    const wrapper = mount(ProductRow, {
      props: { product, enriched: false }
    })
    await nextTick()

    // Expense breakdown should not be visible initially
    expect(wrapper.find('.expense-bar').exists()).toBe(false)

    // Click the info icon to show expenses
    const infoIcon = wrapper.find('.product-row__info-icon')
    expect(infoIcon.exists()).toBe(true)
    await infoIcon.trigger('click')
    await nextTick()

    // Expense breakdown should now be visible
    expect(wrapper.find('.expense-bar').exists()).toBe(true)
    expect(wrapper.text()).toContain('adExpense')
    expect(wrapper.text()).toContain('storageFees')

    // Click again to hide
    await infoIcon.trigger('click')
    await nextTick()
    expect(wrapper.find('.expense-bar').exists()).toBe(false)
  })

  it('does not show expense icon when service_expenses is 0', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const product = makeProduct({ summary: { ...makeProduct().summary, service_expenses: 0 } })
    const wrapper = mount(ProductRow, {
      props: { product, enriched: false }
    })
    await nextTick()

    expect(wrapper.find('.product-row__info-icon').exists()).toBe(false)
  })

  it('expands postings on row click in compact mode', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const product = makeProduct()
    const wrapper = mount(ProductRow, {
      props: { product, enriched: false }
    })
    await nextTick()

    // Postings should not be visible initially
    expect(wrapper.find('.product-row__children').exists()).toBe(false)

    // Click the main row to expand
    await wrapper.find('.product-row__main').trigger('click')
    await nextTick()

    // Postings section should now be visible with posting numbers
    const children = wrapper.find('.product-row__children')
    expect(children.exists()).toBe(true)
    expect(children.text()).toContain('ABC-1')
    expect(children.text()).toContain('ABC-2')

    // Click again to collapse
    await wrapper.find('.product-row__main').trigger('click')
    await nextTick()
    expect(wrapper.find('.product-row__children').exists()).toBe(false)
  })

  it('expands postings on row click and shows header labels', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const product = makeProduct()
    const wrapper = mount(ProductRow, {
      props: { product, enriched: false }
    })
    await nextTick()

    await wrapper.find('.product-row__main').trigger('click')
    await nextTick()

    const header = wrapper.find('.product-row__children-header')
    expect(header.text()).toContain('posting')
    expect(header.text()).toContain('date')
    expect(header.text()).toContain('price')
    expect(header.text()).toContain('commission')
    expect(header.text()).toContain('delivery')
    expect(header.text()).toContain('returns')
    expect(header.text()).toContain('net')
  })

  it('renders enriched mode with cost breakdown grid', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const product = makeProduct()
    const wrapper = mount(ProductRow, {
      props: { product, enriched: true }
    })
    await nextTick()

    // Should show the enriched cost breakdown
    const costsGrid = wrapper.find('.enriched-costs__grid')
    expect(costsGrid.exists()).toBe(true)
    // Check a few cost items appear
    expect(wrapper.text()).toContain('table.commission')
    expect(wrapper.text()).toContain('table.acquiring')
    expect(wrapper.text()).toContain('table.logistics')
  })

  it('renders enriched mode with postings section', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const product = makeProduct()
    const wrapper = mount(ProductRow, {
      props: { product, enriched: true }
    })
    await nextTick()

    const postingsSection = wrapper.find('.product-row__children')
    expect(postingsSection.exists()).toBe(true)
    expect(postingsSection.text()).toContain('ABC-1')
    expect(postingsSection.text()).toContain('ABC-2')
    expect(wrapper.findAll('.posting-row-stub').length).toBe(2)
  })

  it('renders enriched mode without postings section when no postings', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const product = makeProduct({ postings: [] })
    const wrapper = mount(ProductRow, {
      props: { product, enriched: true }
    })
    await nextTick()

    expect(wrapper.find('.product-row__children').exists()).toBe(false)
  })

  it('renders enriched mode cost items with zero values as dash', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const product = makeProduct({
      costs: {
        commission: 0,
        acquiring: 0,
        order_processing: 0,
        logistics: 0,
        delivery_to_pickup: 0,
        placement: 0,
        return_processing: 0,
        return_logistics: 0,
        disposal: 0,
        ovh_processing: 0,
        operational_errors: 0,
        pay_per_click: 0,
        pay_per_order: 0,
        star_products: 0,
        paid_brand: 0,
        reviews_cost: 0,
        discount_points: 0,
        partner_programs: 0,
        compensation: 0,
        other_services: 0
      }
    })
    const wrapper = mount(ProductRow, {
      props: { product, enriched: true }
    })
    await nextTick()

    // All zero values should render as em-dash '—'
    const valueEls = wrapper.findAll('.enriched-costs__value')
    valueEls.forEach(el => {
      expect(el.text()).toBe('—')
    })
  })
})
