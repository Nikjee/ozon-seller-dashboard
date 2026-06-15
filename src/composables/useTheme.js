import { ref, watch } from 'vue'

const THEME_KEY = 'ozon-dashboard-theme'

const theme = ref(localStorage.getItem(THEME_KEY) || 'light')

function applyTheme() {
  document.documentElement.setAttribute('data-theme', theme.value)
  localStorage.setItem(THEME_KEY, theme.value)
}

function toggle() {
  theme.value = theme.value === 'light' ? 'dark' : 'light'
}

applyTheme()

export function useTheme() {
  watch(theme, applyTheme)
  return { theme, toggle }
}
