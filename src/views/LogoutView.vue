<template>
    <p>Logged Out</p>
</template>

<script>
import { inject} from "vue";

export default {
    name: 'LogoutVIew',
    methods: {},
    async mounted(){
        //If the user logs in with Microsoft, then page logs them out of Microsoft and removes all stored data
        if(this.$cookies.get("loginSource") == "G"){
            await this.$gAuth.signOut().catch(
                error => { alert(error)}
            );
            this.$cookies.remove("loginSource");
            this.$cookies.remove("accessToken");
            this.$cookies.remove("expDate");
            window.location.href = `https://www.google.com/accounts/Logout?continue=https://appengine.google.com/_ah/logout?continue=${process.env.VUE_APP_REDIRECT_URL}`;

        }
        //If the user logs in with Google, then page logs them out of Microsoft and removes all stored data
        if(this.$cookies.get("loginSource") == "O"){
            this.$cookies.remove("loginSource");
            this.$cookies.remove("accessToken");
            this.$msalClient.logoutPopup({
                mainWindowRedirectUri: process.env.VUE_APP_REDIRECT_URL
            });
        }
        //If the user logs in without any account, then page removes all stored data
        if(this.$cookies.get("loginSource") == "M"){
            this.$cookies.remove("loginSource");
            this.$cookies.remove("accessToken");
            this.$router.push({ name: 'Login'});
        }
    },
    setup() {
        //Import Google Auth
        const Vue3GoogleOauth = inject("Vue3GoogleOauth");
        return { Vue3GoogleOauth };
  },
}
</script>