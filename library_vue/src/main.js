import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import axios from 'axios'
import user_info from './components/AllUserInfo.vue'

const app = createApp(App)

app.use(router)
app.use(ElementPlus)
// axios.defaults.baseURL = 'http://localhost:3000' 

axios.defaults.baseURL = '/api'

app.config.globalProperties.$axios = axios
app.config.globalProperties.$rust = 'http://localhost:3333'



app.mount('#app')



