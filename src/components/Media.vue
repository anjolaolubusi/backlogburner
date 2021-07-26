<template>
<div
@dblclick="$emit('toggle-remainder', item.id)" 
:class="[item.remind ? 'remind': '', 'mediaTask']"
>
<p>{{ item.mediaName }} <i @click="$emit('delete-task', item.id)" class="fas fa-times"></i> </p>
</div>
</template>

<script>
export default {
    name: 'Media',
    props: {
        item: Object,
    },
    methods: {

    }
}
</script>

<style scoped>
p {
    text-align: left;
}

.fas {
color: red;
width: 10px;
height: 10px;
}

.mediaTask{
    background-color: grey;
    margin-bottom: 5px;
    padding-top: 5px;
    padding-left: 10px;

}

.mediaTask.remind{
    border-left: 5px solid green;
}

</style>