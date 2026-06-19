import { ref, computed, watch, h } from 'vue'
import { NSpace, NCheckbox } from 'naive-ui'

function load(key) {
  try { return JSON.parse(localStorage.getItem(key) || '{}') }
  catch { return {} }
}

const gearIcon = () => h('span', {
  style: 'cursor:pointer;display:flex;align-items:center;color:var(--n-th-text-color)',
  innerHTML: '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>'
})

export function useColumnSettings(key, columnsRef, getTitle) {
  const storageKey = `ozon-cols-${key}`
  const columnVisibility = ref(load(storageKey))
  const showColumnSettings = ref(false)

  watch(columnVisibility, (v) => localStorage.setItem(storageKey, JSON.stringify(v)), { deep: true })

  const visibleColumns = computed(() =>
    columnsRef.value.filter(c => c.type === 'expand' || columnVisibility.value[c.key] !== false)
  )

  function toggleColumn(k) {
    const cur = columnVisibility.value[k]
    columnVisibility.value = { ...columnVisibility.value, [k]: cur === undefined ? false : !cur }
  }

  function resetColumns() {
    columnVisibility.value = {}
    localStorage.removeItem(storageKey)
  }

  const settingsColumn = computed(() => {
    const label = typeof getTitle === 'function' ? getTitle() : ''
    return {
      key: '__settings__',
      width: 48,
      minWidth: 48,
      fixed: 'right',
      title: '',
      filter: 'default',
      renderFilterIcon: gearIcon,
      renderFilterMenu: () => h('div', { style: 'min-width:200px;max-height:320px;overflow-y:auto;padding:4px 8px;' }, [
        label ? h('div', { style: 'font-weight:600;margin-bottom:6px;font-size:13px;' }, label) : null,
        h(NSpace, { vertical: true, size: 'small' }, () =>
          columnsRef.value
            .filter(c => c.type !== 'expand')
            .map(col => h('div', { key: col.key }, h(NCheckbox, {
              checked: columnVisibility.value[col.key] !== false,
              'onUpdate:checked': () => toggleColumn(col.key)
            }, { default: () => col.title || col.key })))
        )
      ])
    }
  })

  return { columnVisibility, showColumnSettings, visibleColumns, toggleColumn, resetColumns, settingsColumn }
}
