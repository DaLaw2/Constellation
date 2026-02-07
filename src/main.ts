import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ContextMenu from '@imengyu/vue3-context-menu'
import App from './App.vue'
import { useSettingsStore } from './stores/settings'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
import '@imengyu/vue3-context-menu/lib/vue3-context-menu.css'
import './assets/styles/main.css'

const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.use(ContextMenu)
app.mount('#app')

// Load settings on startup
const settingsStore = useSettingsStore()
settingsStore.loadSettings()
