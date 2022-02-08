<template>
    <p>Logged Out</p>
</template>

<script>
import { inject} from "vue";

export default {
    name: 'LogoutVIew',
    methods: {},
    async mounted(){
        if(this.$cookies.get("loginSource") == "G"){
            await this.$gAuth.signOut().catch(
                error => { alert(error)}
            );
            this.$cookies.remove("loginSource");
            this.$cookies.remove("accessToken");
            window.location.href = `https://www.google.com/accounts/Logout?continue=https://appengine.google.com/_ah/logout?continue=${process.env.VUE_APP_REDIRECT_URL}`;

        }
        if(this.$cookies.get("loginSource") == "O"){
            this.$cookies.remove("loginSource");
            this.$cookies.remove("accessToken");
            this.$msalClient.logoutPopup({
                mainWindowRedirectUri: process.env.VUE_APP_REDIRECT_URL
            });
        }

        if(this.$cookies.get("loginSource") == "M"){
            this.$cookies.remove("loginSource");
            this.$cookies.remove("accessToken");
            this.$router.push({ name: 'Login'});
        }
    },
    setup() {
        const Vue3GoogleOauth = inject("Vue3GoogleOauth");
        return { Vue3GoogleOauth };
  },
}
</script>