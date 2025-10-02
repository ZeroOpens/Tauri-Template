import { createRouter, createWebHistory } from 'vue-router'

// 一级路由
import Home from '@/pages/Home.vue'
import Pinia from '@/pages/Pinia.vue'
import More from '@/pages/More.vue'
import RequestIndex from '@/pages/request/index.vue'

// 二级路由
import FrontendRequest from '@/pages/request/Frontend-request.vue'
import BackendRequest from '@/pages/request/Backend-request.vue'


// 创建路由
const router = createRouter({
  history: createWebHistory(), //路由器工作模式
  routes: [  //路由规则
    { path: '/home', name: 'home', component: Home },
    { path: '/pinia', name: 'pinia', component: Pinia },
    { path: '/more', name: 'more', component: More },
    {
      path: '/request',
      name: 'request',
      component: RequestIndex,
      children: [
        { path: 'frontend', name: 'frontend-request', component: FrontendRequest },
        { path: 'backend', name: 'backend-request', component: BackendRequest },
      ],
    },
    // { path: '/', redirect: '/home' }  // 重定向
    { path: '/', redirect: '/request' }  // 重定向
  ]
})

// 导出路由
export default router
