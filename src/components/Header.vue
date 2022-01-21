<template>
    <header>
        <h1>{{ title }}</h1>
    </header>
    <button @click="Logout">Logout</button>
</template>

<script>
export default {
    name: 'Header',
    inject:['loginSource'],
    props: {
        title: {
            type: String,
            default: 'Backlog Burner'
        }
    },
    methods: {
        async LogoutGoogle(){
            try {
                await this.$gAuth.signOut();
                console.log("isAuthorized", this.Vue3GoogleOauth.isAuthorized);
                this.user = "";
            } catch (error) {
                console.error(error);
            }
        },
        async LogoutMicrosoft(){
            try{                
                await this.$msalClient.logoutRedirect();
            } catch (error){
                console.error(error);
            }
        },
        async Logout(){
            if(this.$cookies.get("loginSource") == 'O'){
                this.LogoutMicrosoft();
            }

            if(this.$cookies.get("loginSource") == 'G'){
                this.LogoutGoogle();
            }
        }
    }
}
</script>

<style scoped>
    header {
    justify-content: center;
    display: flex;
    }

    header h1{
        color: black;
    }
</style>
