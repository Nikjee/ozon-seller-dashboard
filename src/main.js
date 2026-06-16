import { createApp, defineComponent, h } from 'vue'
import './styles/base.css'
import './styles/components.css'
import { NConfigProvider, darkTheme } from 'naive-ui'
import App from './App.vue'
import { useTheme } from './composables/useTheme'

const themeOverrides = {
  common: {
    primaryColor: 'var(--ctp-teal)',
    successColor: 'var(--ctp-green)',
    errorColor: 'var(--ctp-red)',
    warningColor: 'var(--ctp-yellow)',
    bodyColor: 'var(--bg)',
    cardColor: 'var(--bg-surface)',
    textColor1: 'var(--text)',
    textColor2: 'var(--text-subtle)',
    borderColor: 'var(--border)',
    borderRadius: 'var(--radius-md)'
  }
}

const AppRoot = defineComponent({
  setup() {
    const { theme } = useTheme()

    return () =>
      h(
        NConfigProvider,
        {
          themeOverrides,
          theme: theme.value === 'dark' ? darkTheme : null
        },
        {
          default: () => h(App)
        }
      )
  }
})

createApp(AppRoot).mount('#app')
