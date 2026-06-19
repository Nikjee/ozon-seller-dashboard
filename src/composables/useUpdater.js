import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export function useUpdater() {
  const checking = ref(false)
  const error = ref(null)

  async function authHeaders() {
    try {
      const token = await invoke('get_updater_token')
      if (token) {
        return { Authorization: `Bearer ${token}` }
      }
    } catch {
      // public repo or dev build
    }
    return {}
  }

  async function checkForUpdates() {
    if (typeof window !== 'object' || !window.__TAURI_INTERNALS__) {
      return
    }
    try {
      checking.value = true
      error.value = null
      const { check } = await import('@tauri-apps/plugin-updater')
      const { ask } = await import('@tauri-apps/plugin-dialog')
      const { relaunch } = await import('@tauri-apps/plugin-process')
      const token = await authHeaders()
      const update = await check({
        headers: { ...token, Accept: 'application/vnd.github.raw' },
      })
      if (update) {
        const notes = update.body
          ? `Update ${update.version} available!\n\nRelease notes:\n${update.body}`
          : `Update ${update.version} available!`
        const yes = await ask(notes, {
          title: 'Update Available',
          kind: 'info',
          okLabel: 'Update',
          cancelLabel: 'Later',
        })
        if (yes) {
          await update.downloadAndInstall(undefined, {
            headers: { ...token, Accept: 'application/octet-stream' },
          })
          await relaunch()
        }
      }
    } catch (e) {
      console.error('[updater] check failed:', e)
      error.value = typeof e === 'string' ? e : (e.message || 'Update check failed')
    } finally {
      checking.value = false
    }
  }

  return { checking, error, checkForUpdates }
}
