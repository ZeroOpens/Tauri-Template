import { createApp } from "vue";
import App from "./App.vue";
import { createPinia } from 'pinia'

// 创建应用
const app = createApp(App)
// 创建pinia
const pinia = createPinia()
// 挂载pinia
app.use(pinia)
// 挂载应用
app.mount('#app')
