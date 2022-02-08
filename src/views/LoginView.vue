<template>
    <h3>Login</h3>
    <!-- <button @click="LoginMicrosoft">Login With Your Work/School Account</button>
    <button @click="LoginGoogle" :disabled="!Vue3GoogleOauth.isInit || this.$cookies.isKey('accessToken')">Login With Your Gmail</button> -->
    <button @click="Login">Login</button>
</template>

<script>
import * as  msal from '@azure/msal-browser'
//import { useCookies } from "vue3-cookies";
import { inject} from "vue";

export default ({
    name: 'LoginView',
    data(){
        return{
            username: null,
            userID: null,
            loginResponse: null,
            msalClient: null,
            loginRequest: {
                client_id: "0b1cbc4a-fe05-456f-ae2e-2e38cc6d741c",
                scopes: ["openid", "profile", "User.Read"]
            },
            tokenRequest: {
                scopes: ["User.Read", "Calendars.Read"]
            }
        }
    },
    methods: {
        async LoginMicrosoft(){
            console.log(`LOGIN SORUCE: ${this.$loginSource}`);
            this.msalClient = new msal.PublicClientApplication(this.$msalConfig)
            this.loginResponse = await this.$msalClient.loginPopup(this.loginRequest).catch(
                error => { alert(error)}
            );
            this.tokenRequest.account = this.loginResponse.account
            let response = await this.$msalClient.acquireTokenSilent(this.tokenRequest).catch(
                error => {
                    console.warn("silent token acquisition fails. acquiring token using redirect");
                    if (error instanceof msal.InteractionRequiredAuthError) {
            // fallback to interaction when silent call fails
                    return this.$msalClient.acquireTokenPopup(this.tokenRequest).then(tokenResponse => {
                        return tokenResponse;
                    }).catch(error => {
                    alert(error);
                    });
                } else {
                    alert(error);
                }
                });
            //console.log(response); -- Store infomation in variable in cookies
            this.$cookies.set("loginSource",'O');
            this.$cookies.set("accessToken", response.accessToken);
            if(response != null){
                this.$router.push({ name: 'Schedule'});
            }
        },
        async LoginGoogle(){
            try{
                await this.$gAuth.signIn();
                //await this.$gAuth.getAuthCode();
                var authResponse = this.$gAuth.instance.currentUser.get().getAuthResponse();
                /*var curDate = new Date()
                curDate.setTime(Date.now());
                var exDate = new Date();
                exDate.setTime(authResponse.expires_at);
                if(curDate >= exDate){
                    authResponse = this.$gAuth.instance.currentUser.get().getAuthResponse();
                    exDate = new Date();
                    exDate.setTime(authResponse.expires_at);
                }*/
                console.log(authResponse);
                this.$cookies.set("accessToken", authResponse.access_token);
                this.$cookies.set("loginSource",'G');
                //this.$cookies.set("expirationDate", exDate);
                if(authResponse){
                    this.$router.push({ name: 'Schedule'});
                }
            }catch(error){
                console.log(error)
            }
        },
        async Login(){
            try{
                this.$cookies.set("accessToken", 'NONE');
                this.$cookies.set("loginSource", 'M');
                this.$router.push({ name: 'Schedule'});
            }catch(error){
                console.log(error);
            }
        }
    },
    mounted(){
//       this.$msalClient = new msal.PublicClientApplication(this.$msalConfig); 
        console.log(`${this.$cookies.get("loginSource")} - ${this.$cookies.isKey("accessToken")}`)
        if(this.$cookies.get("loginSource") != null && this.$cookies.isKey("accessToken")){
            this.$router.push({ name: 'Schedule'});
        }else{
            this.$cookies.set("loginSource",'');
        }
    },
    setup() {
        const Vue3GoogleOauth = inject("Vue3GoogleOauth");
        return { Vue3GoogleOauth };
  },
})
</script>