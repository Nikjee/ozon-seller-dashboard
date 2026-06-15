import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  server: {
    strictPort: true,
    host: '0.0.0.0'
  },
  clearScreen: false
})
