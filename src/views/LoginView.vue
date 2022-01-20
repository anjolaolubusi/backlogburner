<template>
    <h3>Login</h3>
    <button @click="LoginMicrosoft">Login With Your Work/School Account</button>
    <button @click="LoginGoogle">Login With Your Gmail</button>
</template>

<script>
import * as  msal from '@azure/msal-browser'
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
            this.loginResponse = await this.msalClient.loginPopup(this.loginRequest).catch(
                error => { alert(error)}
            );
            this.tokenRequest.account = this.loginResponse.account
            let response = await this.$msalClient.acquireTokenSilent(this.tokenRequest).catch(
                error => {
                    console.warn("silent token acquisition fails. acquiring token using redirect");
                    if (error instanceof msal.InteractionRequiredAuthError) {
            // fallback to interaction when silent call fails
                    return this.msalClient.acquireTokenPopup(this.tokenRequest).then(tokenResponse => {
                        return tokenResponse;
                    }).catch(error => {
                    alert(error);
                    });
                } else {
                    alert(error);
                }
                });
            this.$loginSource = 'O';
            if(response != null && this.$loginSource == 'O'){
                this.$router.push({ name: 'Schedule', params: { accessToken: response.accessToken, source: "O"}});
            }
        },
        async LoginGoogle(){
            try{
                await this.$gAuth.getAuthCode();
                var authResponse = this.$gAuth.instance.currentUser.get().getAuthResponse();
                var curDate = new Date()
                curDate.setTime(Date.now());
                var exDate = new Date();
                exDate.setTime(authResponse.expires_at);
                
                if(curDate >= exDate){
                    authResponse = this.$gAuth.instance.currentUser.get().getAuthResponse();
                    exDate = new Date();
                    exDate.setTime(authResponse.expires_at);
                }
                this.$loginSource = 'G';
                if(authResponse && this.$loginSource == 'G'){
                    this.$router.push({ name: 'Schedule', params: { accessToken: authResponse.access_token, source: "G", expirationDate: exDate}});
                }
            }catch(error){
                console.log(error)
            }
        }
    },
    mounted(){
       this.$msalClient = new msal.PublicClientApplication(this.$msalConfig); 
       console.log(`LOGIN SORUCE: ${this.$loginSource}`);
       this.$loginSource = 'NEW TEMP';
       console.log(`LOGIN SORUCE: ${this.$loginSource}`);
    },
})
</script>