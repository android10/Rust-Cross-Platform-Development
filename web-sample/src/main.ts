import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

const app = createApp(App)

app.use(createPinia())
app.use(router)

// https://vuejs.org/guide/essentials/application.html#mounting-the-app
app.mount('#app')
