import { createApp, defineComponent, h, computed } from 'vue'
import './styles/base.css'
import './styles/components.css'
import { NConfigProvider, darkTheme } from 'naive-ui'
import App from './App.vue'
import { useTheme } from './composables/useTheme'

// On-screen error log (appended to body, updated via mutation observer)
const debugEl = document.createElement('div')
debugEl.id = 'debug-overlay'
debugEl.style.cssText = 'position:fixed;bottom:0;left:0;right:0;z-index:99999;background:#1e1e2e;color:#f38ba8;font:12px monospace;padding:8px 12px;max-height:200px;overflow-y:auto;border-top:2px solid #f38ba8;display:none'
document.body.appendChild(debugEl)

function pushError(msg) {
  const text = typeof msg === 'string' ? msg : (msg?.message || JSON.stringify(msg))
  const line = document.createElement('div')
  line.textContent = `[${new Date().toLocaleTimeString()}] ${text}`
  debugEl.insertBefore(line, debugEl.firstChild)
  while (debugEl.children.length > 20) debugEl.removeChild(debugEl.lastChild)
  debugEl.style.display = 'block'
  console.error('DEBUG:', msg)
}
window.onerror = (msg, source, line, col, err) => pushError(msg)
window.addEventListener('unhandledrejection', (e) => pushError(e.reason))

/**
 * Theme overrides for naive-ui. Must use concrete hex values instead of
 * CSS variable references (e.g. var(--bg)) because naive-ui internally
 * calls seemly/rgba() on these values, which cannot parse CSS custom
 * properties. Seemly failures silently break naive-ui's tab pane
 * switching (the TransitionGroup animation crashes).
 */
const lightThemeOverrides = {
  common: {
    primaryColor: '#179299',
    successColor: '#40a02b',
    errorColor: '#d20f39',
    warningColor: '#df8e1d',
    bodyColor: '#eff1f5',
    cardColor: '#e6e9ef',
    textColor1: '#4c4f69',
    textColor2: '#6c6f85',
    borderColor: '#ccd0da',
    borderRadius: '10px'
  }
}

const darkThemeOverrides = {
  common: {
    primaryColor: '#94e2d5',
    successColor: '#a6e3a1',
    errorColor: '#f38ba8',
    warningColor: '#f9e2af',
    bodyColor: '#1e1e2e',
    cardColor: '#313244',
    textColor1: '#cdd6f4',
    textColor2: '#a6adc8',
    borderColor: '#45475a',
    borderRadius: '10px'
  }
}

const AppRoot = defineComponent({
  setup() {
    const { theme } = useTheme()

    const themeOverrides = computed(() =>
      theme.value === 'dark' ? darkThemeOverrides : lightThemeOverrides
    )

    return () =>
      h(
        NConfigProvider,
        {
          themeOverrides: themeOverrides.value,
          theme: theme.value === 'dark' ? darkTheme : null
        },
        {
          default: () => h(App)
        }
      )
  }
})

const app = createApp(AppRoot)
app.config.errorHandler = (err, _instance, info) => pushError(`${err.message || err} [${info}]`)
app.mount('#app')
