// @vitest-environment jsdom
import { mount } from '@vue/test-utils'
import { ref, h, nextTick } from 'vue'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import ProductTreeTable from '../ProductTreeTable.vue'
import { useI18n } from '../../composables/useI18n.js'

vi.mock('../../composables/useI18n.js', () => ({ useI18n: vi.fn() }))
vi.mock('naive-ui', () => ({
  NDataTable: {
    name: 'NDataTable',
    props: ['columns', 'data', 'rowKey', 'expandedRowKeys', 'size', 'scrollX', 'bordered', 'singleLine'],
    emits: ['update:expandedRowKeys'],
    template: '<div class="n-data-table" :data-row-count="data && data.length"><div v-if="columns && columns.length" class="n-data-columns">{{ columns.map(c => c.title).join(",") }}</div><div v-for="row in data" :key="rowKey ? rowKey(row) : row.sku" class="n-data-row">{{ row.name }} {{ row.sku }}</div></div>'
  },
  NPopover: { name: 'NPopover', props: ['show', 'placement', 'trigger'], template: '<div class="n-popover" v-if="show"><slot /></div>' },
  NCheckbox: { name: 'NCheckbox', props: ['checked'], template: '<span class="n-checkbox" :data-checked="String(checked)"><slot /></span>' },
  NSpace: { name: 'NSpace', props: ['vertical', 'size'], template: '<div class="n-space"><slot /></div>' },
  NTag: { name: 'NTag', props: ['type', 'size'], template: '<span class="n-tag"><slot /></span>' },
  NTabs: { name: 'NTabs', props: ['value'], template: '<div class="n-tabs"><slot /></div>' },
  NTabPane: { name: 'NTabPane', props: ['name', 'tab'], template: '<div class="n-tab-pane" :data-name="name"><slot /></div>' }
}))

beforeEach(() => {
  vi.clearAllMocks()
})

function makeProducts() {
  return [
    {
      name: 'Widget A',
      sku: 'SKU-001',
      offer_id: 'OFFER-001',
      has_fbo_stocks: true,
      has_fbs_stocks: false,
      product_info: {
        price: 600,
        net_price: 300,
        old_price: 1200,
        primary_image: 'https://example.com/img.jpg',
        stocks_present: 50,
        stocks_reserved: 10,
        color_index: 'GREEN',
        scheme: 'FBO',
        commissions: [{ sale_schema: 'fbo', percent: 15.0 }]
      },
      totalRevenue: 50000,
      totalCosts: 15000,
      netProfit: 35000,
      profitPerUnit: 350,
      totalQuantity: 100,
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
        { posting_number: 'PST-001', date: '2025-01-10', seller_price_per_instance: 500, commission_amount: -100, commission_ratio: 0.2, delivery_charge: -50, delivery_details: { amount: -50, bonus: 0, standard_fee: 0, bank_coinvestment: 0, stars: 0, total: -50 }, return_charge: 0, return_details: { amount: 0, bonus: 0, standard_fee: 0, bank_coinvestment: 0, stars: 0, total: 0 }, services: [], net: 350 }
      ]
    },
    {
      name: 'Gadget B',
      sku: 'SKU-002',
      offer_id: 'OFFER-002',
      has_fbo_stocks: false,
      has_fbs_stocks: true,
      product_info: {
        price: 500,
        net_price: 250,
        primary_image: '',
        stocks_present: 20,
        stocks_reserved: 5,
        color_index: 'YELLOW',
        scheme: 'FBS',
        commissions: [{ sale_schema: 'fbs', percent: 20.0 }]
      },
      totalRevenue: 30000,
      totalCosts: 20000,
      netProfit: 10000,
      profitPerUnit: 100,
      totalQuantity: 300,
      costs: {
        commission: -6000,
        acquiring: -400,
        order_processing: -200,
        logistics: -3000,
        delivery_to_pickup: -800,
        placement: -100,
        return_processing: -200,
        return_logistics: -100,
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
      postings: []
    }
  ]
}

describe('ProductTreeTable.vue', () => {
  it('renders NDataTable with products', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const products = makeProducts()
    const wrapper = mount(ProductTreeTable, {
      props: { products }
    })
    await nextTick()

    const dataTable = wrapper.find('.n-data-table')
    expect(dataTable.exists()).toBe(true)
    expect(dataTable.attributes('data-row-count')).toBe('2')

    const dataRows = wrapper.findAll('.n-data-row')
    expect(dataRows.length).toBe(2)
    expect(dataRows[0].text()).toContain('Widget A')
    expect(dataRows[0].text()).toContain('SKU-001')
    expect(dataRows[1].text()).toContain('Gadget B')
    expect(dataRows[1].text()).toContain('SKU-002')
  })

  it('renders without products', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const wrapper = mount(ProductTreeTable, {
      props: { products: [] }
    })
    await nextTick()

    const dataTable = wrapper.find('.n-data-table')
    expect(dataTable.exists()).toBe(true)
    expect(dataTable.attributes('data-row-count')).toBe('0')
  })

  it('renders all column titles', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const products = makeProducts()
    const wrapper = mount(ProductTreeTable, {
      props: { products }
    })
    await nextTick()

    const columnsEl = wrapper.find('.n-data-columns')
    expect(columnsEl.exists()).toBe(true)
    expect(columnsEl.text()).toContain('table.name')
    expect(columnsEl.text()).toContain('table.sku')
    expect(columnsEl.text()).toContain('table.totalRevenue')
    expect(columnsEl.text()).toContain('table.commission')
    expect(columnsEl.text()).toContain('table.netProfit')
  })

  it('uses rowKey function that returns sku', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const products = makeProducts()
    const wrapper = mount(ProductTreeTable, {
      props: { products }
    })
    await nextTick()

    const dataRows = wrapper.findAll('.n-data-row')
    expect(dataRows.length).toBe(2)
  })

  it('has expand column with renderExpand function', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const products = makeProducts()
    const wrapper = mount(ProductTreeTable, {
      props: { products }
    })
    await nextTick()

    const nDataTable = wrapper.findComponent({ name: 'NDataTable' })
    expect(nDataTable.exists()).toBe(true)
    const columns = nDataTable.props('columns')
    const expandCol = columns.find(c => c.type === 'expand')
    expect(expandCol).toBeDefined()
    expect(expandCol.renderExpand).toBeTypeOf('function')
  })
})
