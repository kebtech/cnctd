import { createApp } from 'vue'
import App from './App.vue'
import store from './store'

createApp(App).provide('store', store).mount('#app')
