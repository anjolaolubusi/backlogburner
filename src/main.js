import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import GAuth from 'vue3-google-oauth2'
import VueCookies from 'vue3-cookies'
import * as  msal from '@azure/msal-browser'


const app = createApp(App);

var msalConfigTemp = {
    auth: {
        clientId: "0b1cbc4a-fe05-456f-ae2e-2e38cc6d741c",
        authority: "https://login.microsoftonline.com/common",
        redirectUri: process.env.VUE_APP_REDIRECT_URL,
        postLogoutRedirectUr: process.env.VUE_APP_REDIRECT_URL,
        mainWindowRedirectUri: "localhost:8080/login"
    }
}

app.config.globalProperties.$msalConfig  = {
    auth: {
    clientId: "0b1cbc4a-fe05-456f-ae2e-2e38cc6d741c",
    authority: "https://login.microsoftonline.com/common",
    redirectUri: process.env.VUE_APP_REDIRECT_URL,
    postLogoutRedirectUr: process.env.VUE_APP_REDIRECT_URL
    }
}

const gAuthOptions = { clientId: '367446401447-su3f7kil6mt816kltl0ia2r2k0idplfl.apps.googleusercontent.com', scope: 'profile https://www.googleapis.com/auth/calendar https://www.googleapis.com/auth/calendar.events', prompt: 'consent', fetch_basic_profile: false }

//app.config.globalProperties.$loginSource = 'TEMP'

app.config.globalProperties.$msalClient = new msal.PublicClientApplication(msalConfigTemp); 

app.config.globalProperties.$loginResponse = null

app.config.globalProperties.$tokenResponse = null

app
.use(router)
.use(GAuth, gAuthOptions)
.use(VueCookies)
.mount('#app')