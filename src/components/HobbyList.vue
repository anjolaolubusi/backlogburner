<template>
    <div v-for="hobby in sourceData"  :key="hobby.id">
        <Hobby @edit-hobby="$emit('edit-hobby', hobby.id)" @delete-hobby="$emit('delete-hobby', hobby.id)" @call-api="$emit('call-api', hobby.id);" :item="hobby" />
    </div>
</template>

<script>
import Hobby from './Hobby.vue'
export default {
    name: 'HobbyList',
    props: {
        sourceData: Array //List of hobbies
    },
    components: {
        Hobby
    },
    methods: {
    },
    emits: ['delete-hobby', 'edit-hobby', 'call-api'],
    watch: {
    }
}
</script>
