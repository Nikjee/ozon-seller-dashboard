import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export function useUpdater() {
  const checking = ref(false)
  const updateVersion = ref(null)
  const downloadProgress = ref(0)
  const downloading = ref(false)
  const error = ref(null)

  /** Build authentication headers for private repo access */
  async function authHeaders() {
    try {
      const token = await invoke('get_updater_token')
      if (token) {
        return { Authorization: `Bearer ${token}` }
      }
    } catch {
      // Token not available — public repo or dev build
    }
    return {}
  }

  async function checkForUpdates() {
    // Only check on desktop Tauri runtime
    if (typeof window !== 'object' || !window.__TAURI_INTERNALS__) {
      return
    }
    try {
      checking.value = true
      error.value = null
      const { check } = await import('@tauri-apps/plugin-updater')
      const token = await authHeaders()
      // API contents endpoint needs application/vnd.github.raw Accept
      const update = await check({
        headers: { ...token, Accept: 'application/vnd.github.raw' },
      })
      if (update) {
        updateVersion.value = update.version
      } else {
        updateVersion.value = null
      }
    } catch (e) {
      console.error('[updater] check failed:', e)
      error.value = typeof e === 'string' ? e : (e.message || 'Update check failed')
      updateVersion.value = null
    } finally {
      checking.value = false
    }
  }

  async function installUpdate() {
    try {
      downloading.value = true
      error.value = null
      const { check } = await import('@tauri-apps/plugin-updater')
      const { relaunch } = await import('@tauri-apps/plugin-process')
      const token = await authHeaders()
      // Use same Accept header for check — endpoint is the API contents URL
      const update = await check({
        headers: { ...token, Accept: 'application/vnd.github.raw' },
      })
      if (update) {
        // API asset download endpoint needs application/octet-stream
        await update.downloadAndInstall((event) => {
          if (event.progress) {
            downloadProgress.value = event.progress
          }
        }, { headers: { ...token, Accept: 'application/octet-stream' } })
        await relaunch()
      }
    } catch (e) {
      error.value = typeof e === 'string' ? e : (e.message || 'Update failed')
    } finally {
      downloading.value = false
    }
  }

  return {
    checking,
    updateVersion,
    downloadProgress,
    downloading,
    error,
    checkForUpdates,
    installUpdate,
  }
}
