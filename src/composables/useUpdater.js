import { ref, onMounted } from 'vue'

export function useUpdater() {
  const checking = ref(false)
  const updateVersion = ref(null)
  const downloadProgress = ref(0)
  const downloading = ref(false)
  const error = ref(null)

  async function checkForUpdates() {
    // Only check on desktop Tauri runtime
    if (typeof window !== 'object' || !window.__TAURI_INTERNALS__) {
      return
    }
    try {
      checking.value = true
      error.value = null
      const { check } = await import('@tauri-apps/plugin-updater')
      const update = await check()
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
      const update = await check()
      if (update) {
        await update.downloadAndInstall()
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
