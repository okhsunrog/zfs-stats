import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
// import { resolve } from 'node:path'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'
import vueDevTools from 'vite-plugin-vue-devtools'
import { THEMES, DEFAULT_THEME } from './src/constants/themes'
const isProduction = process.env.NODE_ENV === 'production'
// const __dirname = dirname(fileURLToPath(import.meta.url))

export default defineConfig({
  plugins: [
    {
      name: 'theme-inject',
      enforce: 'pre',
      transform(code, id) {
        if (id.endsWith('main.css')) {
          const daisyuiThemes = [DEFAULT_THEME, THEMES.DARK].join(', ')
          const injectedCode = `@plugin "daisyui" {\n  themes: ${daisyuiThemes};\n}`
          const modifiedCode = code + '\n' + injectedCode
          return { code: modifiedCode, map: null }
        }
        return undefined
      },
    },
    tailwindcss(),
    vue(),
    ...(!isProduction ? [vueDevTools()] : []),
  ],

  // 1. prevent vite from obscuring rust errors
  clearScreen: false,

  // Standard dev server config
  server: {
    port: 5173,
    strictPort: false,
    host: false,
  },

  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },

  build: {
    chunkSizeWarningLimit: 1000, // size in kB
  },

  // ensure proper resource paths in bundled app
  base: './',
})
