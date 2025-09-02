import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import App from './App.vue'
import { DEFAULT_THEME, isValidTheme } from './constants/themes'

// Ensure theme is set before app mounts
const saved = localStorage.getItem('theme')
const initialTheme = isValidTheme(saved) ? saved : DEFAULT_THEME
document.documentElement.setAttribute('data-theme', initialTheme)

const app = createApp(App)
const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)

app.use(pinia)
app.mount('#app')
