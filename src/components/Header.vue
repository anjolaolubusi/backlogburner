<template>
    <header>
        <h1>{{ title }}</h1>
    </header>
    <button>Logout</button>
</template>

<script>
export default {
    name: 'Header',
    props: {
        title: {
            type: String,
            default: 'Senior IS'
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
