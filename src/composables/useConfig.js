import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export function useConfig() {
  const configValid = ref(false)
  const configMessage = ref('')
  const saving = ref(false)

  async function check() {
    const result = await invoke('check_config')
    configValid.value = result.valid
    configMessage.value = result.message || ''
  }

  async function save(clientId, apiKey) {
    saving.value = true
    configMessage.value = ''
    try {
      const result = await invoke('save_config', {
        clientId,
        apiKey
      })
      configValid.value = result.valid
    } catch (e) {
      configMessage.value = typeof e === 'string' ? e : (e.message || 'Save failed')
    } finally {
      saving.value = false
    }
  }

  return { configValid, configMessage, saving, check, save }
}
