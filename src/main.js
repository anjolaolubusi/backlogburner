import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import GAuth from 'vue3-google-oauth2'


const app = createApp(App);

app.config.globalProperties.$msalConfig  = {
    auth: {
    clientId: "0b1cbc4a-fe05-456f-ae2e-2e38cc6d741c",
    authority: "https://login.microsoftonline.com/common",
    redirectUri: "http://localhost:8080/login",
    postLogoutRedirectUr: "http://localhost:8080/login"
    }
}

const gAuthOptions = { clientId: '367446401447-su3f7kil6mt816kltl0ia2r2k0idplfl.apps.googleusercontent.com', scope: 'profile https://www.googleapis.com/auth/calendar https://www.googleapis.com/auth/calendar.events', prompt: 'consent', fetch_basic_profile: false }


app.config.globalProperties.$msalClient = null

app.config.globalProperties.$loginResponse = null

app.config.globalProperties.$tokenResponse = null

app
.use(router)
.use(GAuth, gAuthOptions)
.mount('#app')
