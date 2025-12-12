import { defineConfig } from 'vite'

export default defineConfig({
  server: {
    port: 4173,
    strictPort: true,
    host: true
  },
  optimizeDeps: {
    include: ['vue']
  }
})