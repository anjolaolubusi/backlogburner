import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

const app = createApp(App);

app.config.globalProperties.$msalConfig  = {
    auth: {
    clientId: "0b1cbc4a-fe05-456f-ae2e-2e38cc6d741c",
    authority: "https://login.microsoftonline.com/common",
    redirectUri: "http://localhost:8080/login",
    postLogoutRedirectUr: "http://localhost:8080/login"
    }
}

app.config.globalProperties.$msalClient = null

app.config.globalProperties.$loginResponse = null

app.config.globalProperties.$tokenResponse = null

app
.use(router)
.mount('#app')
