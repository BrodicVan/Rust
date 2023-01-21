import { createRouter, createWebHistory } from 'vue-router'

import login from '../components/Login.vue'
import reg from '../components/Register.vue'
import manager from '../components/Manager.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'main',
      component: login
    },
    {
      path: '/login',
      name: 'login',
      component: login
    },
    {
      path: '/reg',
      name: 'reg',
      component: reg
    },
    {
      path: '/manager',
      name: 'manager',
      component: manager
    }
  ]
})

export default router

router.beforeEach((to, from, next) => {

  if(sessionStorage.getItem("user")!==null || (to.name=='main'||to.name=='login'||to.name=='reg'))
  {
    next();
  }
  else
  {
    next({
      name: 'main',
    })
  }
  
})