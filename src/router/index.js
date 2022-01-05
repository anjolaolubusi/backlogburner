import {createRouter, createWebHistory} from 'vue-router'
import LoginView from '../views/LoginView'
import ScheduleView from '../views/ScheduleView'

const routes =[
    {
        path: '/login',
        name: 'Login',
        component: LoginView,
    },
    {
        path: '/schedule',
        name: 'Schedule',
        component: ScheduleView
    },
    {
        path: "/",
        redirect: {
            name: 'Login'
        }
    }
]

const router = createRouter({
    history: createWebHistory(process.env.BASE_URL),
    routes
})

export default router