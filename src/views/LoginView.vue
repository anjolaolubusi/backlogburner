<template>
    <h3>Login</h3>
    <form>
        <label>Username</label>
        <input type="text" name="username" v-model="username" placeholder="Enter username" />
        <br>
        <input type="submit" @click.stop.prevent="onSubmit()" value="Sumbit" />
    </form>
</template>

<script>
import {DatabaseAPI}  from '../api-common'

export default ({
    name: 'LoginView',
    data(){
        return{
            username: null,
            userID: null
        }
    },
    methods: {
        async onSubmit(){
            var res = await DatabaseAPI.get('users', {params: {name: this.username} })
            .catch((err) => alert(err));
            if(res.data[0] === undefined && typeof res.data[0] == 'undefined'){
                return
            }else{
                this.$router.push({ name: 'Schedule', params: { userID: res.data[0].ID } })
            }
        }
    }
})
</script>