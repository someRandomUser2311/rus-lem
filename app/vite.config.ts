import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// Tauri expects a fixed port and doesn't want the screen cleared.
export default defineConfig({
  plugins: [svelte()],
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    watch: { ignored: ['**/src-tauri/**'] },
    // Browser preview: commands go to rusticoder-web.
    proxy: { '/invoke': 'http://127.0.0.1:1430' },
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: { target: 'safari15', sourcemap: false },
})
