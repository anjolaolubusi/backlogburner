<template>
<div 
@mouseover="mouseHover = true" @mouseleave="mouseHover = false"
>
<p class="">{{ item.title }} 
    <i v-if="mouseHover" @click="$emit('edit-hobby', item.id)" class="fas fa-pen" style="font-size: 0.73em;"></i> 
    <i v-if="mouseHover" @click="$emit('delete-hobby', item.id)" class="fas fa-times" style="color: red;font-size: 0.73em;margin-left:7.5px"></i>
    <button  v-if="mouseHover" style="margin-left: 5px" @click="$emit('call-api', item.id)"> Get Placement </button>
</p>
</div>
</template>

<script>
export default {
    name: 'Media',
    props: {
        //Represents Hobby
        item: Object,
    },
    methods: {
    },
    data(){
        return{
            mouseHover: false //Represents if the mouse is over the hobby name
        }
    }
}
</script>

<style scoped>
p {
    text-align: left;
}
.fas {
width: 5px;
height: 5px;
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