// @vitest-environment jsdom
import { mount } from '@vue/test-utils'
import { ref, nextTick } from 'vue'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import PostingConstructor from '../PostingConstructor.vue'
import { useI18n } from '../../composables/useI18n.js'
import { usePostingConstructor } from '../../composables/usePostingConstructor.js'

vi.mock('../../composables/useI18n.js', () => ({ useI18n: vi.fn() }))
vi.mock('../../composables/usePostingConstructor.js', () => ({ usePostingConstructor: vi.fn() }))
vi.mock('naive-ui', () => ({
  NSteps: { name: 'NSteps', props: ['current', 'size'], template: '<div class="n-steps" :data-current="current"><slot /></div>' },
  NStep: { name: 'NStep', props: ['title'], template: '<div class="n-step">{{ title }}</div>' },
  NButton: { name: 'NButton', props: ['type', 'size', 'disabled'], emits: ['click'], template: '<button class="n-button" :data-type="type" :disabled="disabled" @click="$emit(\'click\')"><slot /></button>' },
  NInput: { name: 'NInput', props: ['value', 'placeholder', 'clearable'], emits: ['update:value'], template: '<input class="n-input" :value="value" :placeholder="placeholder" @input="$emit(\'update:value\', $event.target.value)" />' },
  NInputNumber: { name: 'NInputNumber', props: ['value', 'min', 'size'], emits: ['update:value'], template: '<input class="n-input-number" type="number" :value="value" @input="$emit(\'update:value\', Number($event.target.value))" />' },
  NDatePicker: { name: 'NDatePicker', props: ['value', 'type'], emits: ['update:value'], template: '<input class="n-date-picker" :value="value" />' },
  NSpin: { name: 'NSpin', props: ['show'], template: '<div class="n-spin" :data-show="String(show)"><slot /></div>' },
  NAlert: { name: 'NAlert', props: ['type', 'closable'], template: '<div class="n-alert" :data-type="type"><header><slot name="header" /></header><div><slot /></div></div>' },
  NEmpty: { name: 'NEmpty', props: ['description'], template: '<div class="n-empty">{{ description }}</div>' },
  NRadioGroup: { name: 'NRadioGroup', props: ['value', 'vertical'], emits: ['update:value'], template: '<div class="n-radio-group" :data-vertical="vertical"><slot /></div>' },
  NRadio: { name: 'NRadio', props: ['value'], template: '<label class="n-radio"><slot /></label>' },
  NCard: { name: 'NCard', template: '<div class="n-card"><slot /></div>' },
  NStatistic: { name: 'NStatistic', props: ['label', 'value'], template: '<div class="n-statistic"><span class="label">{{ label }}</span><span class="value">{{ value }}</span></div>' },
}))

beforeEach(() => {
  vi.clearAllMocks()
})

const createMockComposable = (overrides = {}) => ({
  currentStep: ref(1),
  goToStep: vi.fn(),
  warehouses: ref([]),
  selectedWarehouseId: ref(null),
  loadWarehouses: vi.fn(),
  clusters: ref([]),
  selectedClusterId: ref(null),
  warehouseSearchQuery: ref(''),
  clusterSearchQuery: ref(''),
  loadClusters: vi.fn(),
  allProducts: ref([]),
  productSearchQuery: ref(''),
  products: ref([]),
  productSearchResults: ref([]),
  loadProducts: vi.fn(),
  addProduct: vi.fn(),
  removeProduct: vi.fn(),
  draftId: ref(null),
  submitDraft: vi.fn(),
  timeslots: ref([]),
  selectedTimeslot: ref(null),
  loadTimeslots: vi.fn(),
  supplyResult: ref(null),
  submitSupply: vi.fn(),
  loading: ref(false),
  error: ref(null),
  reset: vi.fn(),
  ...overrides,
})

describe('PostingConstructor.vue', () => {
  it('renders initial state with step indicator and warehouse loading', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const mock = createMockComposable({ loading: ref(true) })
    vi.mocked(usePostingConstructor).mockReturnValue(mock)

    const wrapper = mount(PostingConstructor)
    await nextTick()

    expect(mock.loadWarehouses).toHaveBeenCalledTimes(1)
    expect(wrapper.find('.n-steps').attributes('data-current')).toBe('1')
    expect(wrapper.find('.n-spin').attributes('data-show')).toBe('true')
  })

  it('toggles loading spinner based on loading ref', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })

    vi.mocked(usePostingConstructor).mockReturnValue(
      createMockComposable({ loading: ref(true) })
    )
    const loadingWrapper = mount(PostingConstructor)
    await nextTick()
    expect(loadingWrapper.find('.n-spin').attributes('data-show')).toBe('true')

    vi.mocked(usePostingConstructor).mockReturnValue(
      createMockComposable({ loading: ref(false) })
    )
    const notLoadingWrapper = mount(PostingConstructor)
    await nextTick()
    expect(notLoadingWrapper.find('.n-spin').attributes('data-show')).toBe('false')
  })

  it('renders error alert with retry button', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const loadWarehouses = vi.fn()
    vi.mocked(usePostingConstructor).mockReturnValue(
      createMockComposable({ loading: ref(false), error: ref('Connection failed'), loadWarehouses })
    )

    const wrapper = mount(PostingConstructor)
    await nextTick()

    expect(wrapper.text()).toContain('Connection failed')
    expect(wrapper.text()).toContain('postingConstructor.error')

    const retryBtn = wrapper.find('.n-button')
    expect(retryBtn.exists()).toBe(true)
    await retryBtn.trigger('click')
    // called once on mount (onMounted) + once on retry click
    expect(loadWarehouses).toHaveBeenCalledTimes(2)
  })

  it('shows empty message when no warehouses', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    vi.mocked(usePostingConstructor).mockReturnValue(
      createMockComposable({ loading: ref(false), error: ref(null), warehouses: ref([]) })
    )

    const wrapper = mount(PostingConstructor)
    await nextTick()

    expect(wrapper.find('.n-empty').exists()).toBe(true)
    expect(wrapper.text()).toContain('postingConstructor.noWarehouses')
  })

  it('renders correct content for each wizard step', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })

    const steps = [
      { current: 2, check: 'postingConstructor.cluster' },
      { current: 3, check: 'postingConstructor.noProducts' },
      { current: 4, check: 'postingConstructor.createDraft' },
      { current: 5, check: 'postingConstructor.loadTimeslots' },
      { current: 6, check: 'postingConstructor.createSupply' },
      { current: 7, check: 'postingConstructor.createAnother' },
    ]

    for (const { current, check } of steps) {
      vi.mocked(usePostingConstructor).mockReturnValue(
        createMockComposable({ currentStep: ref(current) })
      )
      const wrapper = mount(PostingConstructor)
      await nextTick()
      expect(wrapper.text()).toContain(check)
    }
  })

  it('renders product search results in step 3', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    vi.mocked(usePostingConstructor).mockReturnValue(
      createMockComposable({
        currentStep: ref(3),
        productSearchResults: ref([
          { sku: 'SKU-001', name: 'Widget', offer_id: 'OFFER-001' },
          { sku: 'SKU-002', name: 'Gadget', offer_id: 'OFFER-002' },
        ]),
      })
    )

    const wrapper = mount(PostingConstructor)
    await nextTick()

    expect(wrapper.text()).toContain('Widget')
    expect(wrapper.text()).toContain('OFFER-001')
    expect(wrapper.text()).toContain('Gadget')
    expect(wrapper.text()).toContain('OFFER-002')
    expect(wrapper.text()).toContain('postingConstructor.addProduct')
  })

  it('handles draft creation with success state', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const submitDraft = vi.fn()
    const draftId = ref(null)
    vi.mocked(usePostingConstructor).mockReturnValue(
      createMockComposable({
        currentStep: ref(4),
        draftId,
        submitDraft,
      })
    )

    const wrapper = mount(PostingConstructor)
    await nextTick()

    // Create draft button visible
    expect(wrapper.text()).toContain('postingConstructor.createDraft')

    // Click create draft
    const createBtn = wrapper.find('.n-button')
    await createBtn.trigger('click')
    expect(submitDraft).toHaveBeenCalledTimes(1)

    // Set draft ID after submission
    draftId.value = 'DRAFT-ABC'
    await nextTick()

    expect(wrapper.text()).toContain('postingConstructor.draftCreated')
    expect(wrapper.text()).toContain('DRAFT-ABC')
  })

  it('handles supply creation with success alert', async () => {
    vi.mocked(useI18n).mockReturnValue({ t: (key) => key })
    const submitSupply = vi.fn()
    const supplyResult = ref(null)
    vi.mocked(usePostingConstructor).mockReturnValue(
      createMockComposable({
        currentStep: ref(6),
        supplyResult,
        submitSupply,
      })
    )

    const wrapper = mount(PostingConstructor)
    await nextTick()

    // Create supply button visible
    expect(wrapper.text()).toContain('postingConstructor.createSupply')

    // Click create supply
    const createBtn = wrapper.find('.n-button')
    await createBtn.trigger('click')
    expect(submitSupply).toHaveBeenCalledTimes(1)

    // Set supply result after submission
    supplyResult.value = { supply_order_ids: ['SP-100', 'SP-101'] }
    await nextTick()

    expect(wrapper.text()).toContain('postingConstructor.supplyCreated')
  })
})
