<!-- 
This file represents the Login Page of Backlog Burner.
Creator: Anjolaoluwa Olubusi
-->
<template>
    <div style="text-align: center;">
    <h3>Login</h3> <br />
    <button @click="LoginMicrosoft">Login With Your Work/School Account</button>
    <button @click="LoginGoogle" :disabled="!Vue3GoogleOauth.isInit || this.$cookies.isKey('accessToken')">Login With Your Gmail</button>
    <button @click="Login">Login As Guest</button>
    </div>
</template>

<script>
import * as  msal from '@azure/msal-browser'
//import { useCookies } from "vue3-cookies";
import { inject} from "vue";

export default ({
    name: 'LoginView',
    data(){
        return{
            //Stores the user's username
            username: null, 
            //Stores the user's id from authenticating
            userID: null,
            //Stores the response of authenticating with Google or Outlook
            loginResponse: null,
            //Represents msal object used to talk with Microsoft
            msalClient: null,
            //Login Request For Microsoft
            loginRequest: {
                client_id: "0b1cbc4a-fe05-456f-ae2e-2e38cc6d741c",
                scopes: ["openid", "profile", "User.Read"]
            },
            //Token Request object 
            tokenRequest: {
                scopes: ["User.Read", "Calendars.Read"]
            }
        }
    },
    methods: {
        /**
         * Logs the user in with Microsoft
         * Pre-conditions: None
         * Post-conditions: User is logged in with Mircosoft and transitions to the Schedule Page
         */
        async LoginMicrosoft(){
            console.log(`LOGIN SORUCE: ${this.$loginSource}`);
            this.msalClient = new msal.PublicClientApplication(this.$msalConfig)
            this.loginResponse = await this.$msalClient.loginPopup(this.loginRequest).catch(
                error => { alert(error)}
            );
            console.log(this.loginResponse);
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
            console.log(response)
            this.$cookies.set("loginSource",'O');
            this.$cookies.set("accessToken", response.accessToken);
            this.$cookies.set("expDate", response.expiresOn.getTime())
            if(response != null){
                this.$router.push({ name: 'Schedule'});
            }
        },
        /**
         * Logs the user in with Google
         * Pre-conditions: None
         * Post-conditions: User is logged in with Google and transitions to the Schedule Page
         */
        async LoginGoogle(){
            try{
                await this.$gAuth.signIn();
                //await this.$gAuth.getAuthCode();
                var authResponse = this.$gAuth.instance.currentUser.get().getAuthResponse();
                let exDate = new Date();
                exDate.setTime(exDate.getTime() + authResponse.expires_in * 1000)
                this.$cookies.set("expDate", exDate.getTime())
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
        /**
         * Logs the user in
         * Pre-conditions: None
         * Post-conditions: User is logged in and transitions to the Schedule Page
         */
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
        /**
         * Pre-conditions: None
         * Post-conditions: MsalClient object is initalized and cookies are initalized
         */
//       this.$msalClient = new msal.PublicClientApplication(this.$msalConfig); 
        console.log(`${this.$cookies.get("loginSource")} - ${this.$cookies.isKey("accessToken")}`)
        if(this.$cookies.get("loginSource") != null && this.$cookies.isKey("accessToken")){
            this.$router.push({ name: 'Schedule'});
        }else{
            this.$cookies.set("loginSource",'');
        }
    },
    setup() {
        //Imports Google Auth
        const Vue3GoogleOauth = inject("Vue3GoogleOauth");
        return { Vue3GoogleOauth };
  },
})
</script>