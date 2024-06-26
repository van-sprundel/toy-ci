import { createRouter, createWebHistory } from 'vue-router'
import DashboardView from '@/views/DashboardView.vue'
import BuildOverview from '@/views/BuildOverview.vue'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'dashboard',
            component: DashboardView
        },
        {
            path: '/builds/:buildId',
            name: 'dashboard',
            component: BuildOverview
        },
        //    {
        //      path: '/about',
        //      name: 'about',
        //      component: () => import('../views/AboutView.vue')
        //    }
    ]
})

export default router
