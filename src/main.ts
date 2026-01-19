import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
import './assets/styles/main.css'

const pinia = createPinia()
const app = createApp(App)

app.use(pinia)
app.mount('#app')
