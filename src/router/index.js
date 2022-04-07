/**
 * This file represents link routing of Backlog Burner.
 * Creator: Anjolaoluwa Olubusi
 */

import {createRouter, createWebHistory} from 'vue-router' //Imports routing object
import LoginView from '../views/LoginView' //Imports Login Page
import ScheduleView from '../views/ScheduleView' //Imports Schedule Page
import LogoutView from '../views/LogoutView' //Imports Logout Page
import PrivacyPolicy from '../views/PrivacyPolicy' //Imports Privacy Policy Page

//List of pages and their corresponding paths
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
        path: '/Logout',
        name: 'Logout',
        component: LogoutView
    },
    {
        path: "/",
        redirect: {
            name: 'Login'
        }
    },
    {
        path: '/Privacy',
        name: 'Privacy',
        component: PrivacyPolicy
    }
]

//Creates router
const router = createRouter({
    history: createWebHistory(process.env.BASE_URL),
    routes
})

//Exports router
export default router