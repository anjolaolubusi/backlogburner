<template>
    <header>
        <h1>{{ title }}</h1>
    </header>
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
