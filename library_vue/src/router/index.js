import { createRouter, createWebHistory } from 'vue-router'
import login from '../components/Login.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: login
    }
  ]
})

export default router
