<template>
    <div :key="media.id" v-for="media in sourceData">
        <Media @toggle-remainder="$emit('toggle-remainder', media.id)" @delete-task="$emit('delete-task', media.id)" :item="media" />
    </div>
</template>

<script>
import Media from './Media'
export default {
    name: 'MediaList',
    props: {
        sourceData: Array
    },
    components: {
        Media
    },
    emits: ['delete-task', 'toggle-remainder']
}
</script>
