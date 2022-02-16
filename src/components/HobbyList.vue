<template>
    <div :key="hobby.id" v-for="hobby in sourceData">
        <Hobby @edit-hobby="$emit('edit-hobby', hobby.id)" @delete-hobby="$emit('delete-hobby', hobby.id)" @call-api="$emit('call-api', hobby.id)" :item="hobby" />
    </div>
</template>

<script>
import Hobby from './Hobby.vue'
export default {
    name: 'HobbyList',
    props: {
        sourceData: Array
    },
    components: {
        Hobby
    },
    emits: ['delete-hobby', 'edit-hobby', 'call-api']
}
</script>
