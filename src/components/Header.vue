<template>
    <header>
        <h1>{{ title }}</h1>
    </header>
    <button @click="LogoutGoogle">Logout</button>
</template>

<script>
export default {
    name: 'Header',
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
