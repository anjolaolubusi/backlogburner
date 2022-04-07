<!-- 
This file represents the Header shown in all pages of Backlog Burner.
Creator: Anjolaoluwa Olubusi
-->
<template>
    <header>
        <h1 style="font-family: 'Roboto', sans-serif;">{{ title }}</h1>
    </header>
</template>

<script>

export default {

    name: 'Header',
    props: {
        //Title of website
        title: {
            type: String,
            default: 'Backlog Burner'
        }
    },
    methods: {
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
