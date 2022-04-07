/**
 * This file initalizes Backlog Burner with the necessary authentication objects, VueJS objects and VueJS components.
 * Creator: Anjolaoluwa Olubusi
 */ 

import { createApp } from 'vue' //Imports createApp function
import App from './App.vue' //Imports App Component
import router from './router' //Imports router object
import GAuth from 'vue3-google-oauth2' //Import Google Authentication object
import VueCookies from 'vue3-cookies' //Import Vue3 Cookies object
import * as  msal from '@azure/msal-browser' //Imports Microsoft Authentication object
import tooltip from "./directives/tooltip.js"; //Import Vue 3 Tooltips package
import "./assets/tooltip.css"; //Imports Vue 3 Tooltips CSS files

const app = createApp(App); //Creates VueJS application object

//Configuration for Microsoft Authentication
var msalConfigTemp = {
    auth: {
        clientId: "0b1cbc4a-fe05-456f-ae2e-2e38cc6d741c",
        authority: "https://login.microsoftonline.com/common",
        redirectUri: process.env.VUE_APP_REDIRECT_URL,
        postLogoutRedirectUr: process.env.VUE_APP_REDIRECT_URL,
        mainWindowRedirectUri: "localhost:8080/login"
    }
}

//Creates global copy of the Microsoft Authentication configuration
app.config.globalProperties.$msalConfig  = {
    auth: {
    clientId: "0b1cbc4a-fe05-456f-ae2e-2e38cc6d741c",
    authority: "https://login.microsoftonline.com/common",
    redirectUri: process.env.VUE_APP_REDIRECT_URL,
    postLogoutRedirectUr: process.env.VUE_APP_REDIRECT_URL
    }
}

//Configuration for Google Authentication
const gAuthOptions = { clientId: '367446401447-su3f7kil6mt816kltl0ia2r2k0idplfl.apps.googleusercontent.com', scope: 'profile https://www.googleapis.com/auth/calendar https://www.googleapis.com/auth/calendar.events', prompt: 'consent', fetch_basic_profile: false }

//app.config.globalProperties.$loginSource = 'TEMP'

//Creates gloabl Microsoft Authentication client variable 
app.config.globalProperties.$msalClient = new msal.PublicClientApplication(msalConfigTemp); 

//Creates gloabl Microsoft Authentication client variable 
app.config.globalProperties.$loginResponse = null

app.config.globalProperties.$tokenResponse = null


app
//Imports Router
.use(router)
//Import Google Authentication
.use(GAuth, gAuthOptions)
//Import Cookie Usage
.use(VueCookies)
//Loads tooltips
.directive("tooltip", tooltip)
//Mounts the App Component as the root component
.mount('#app')