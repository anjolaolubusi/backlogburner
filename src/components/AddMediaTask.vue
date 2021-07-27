<template>
<transition name="modal">
        <div v-if="isOpen">
            <div class="overlay" @click.self="isOpen = false;">
                <div class="modal">
                    <form @submit="onSubmit" class="add-form">
                        <div class="form-control">
                            <label>Name: </label>
                            <input type="text" v-model="nameOfMedia" name="name" placeholder="Enter name" />
                        </div>
                        <div class="form-control">
                            <label>Type of Media: </label>
                            <select class="form-control" @change="chooseMediaType($event)" v-model="selectedType">
                                <option :value="null" selected disabled>Choose Media Type</option>
                                <option v-for="mediaType in mediaTypes" :value="mediaType" :key="mediaType.ID">{{ mediaType.Type }}</option>
                            </select>
                        </div>


<vue-cal
    class="vuecal--date-picker"
    xsmall
    hide-view-selector
    :time="false"
    :transitions="false"
    active-view="month"
    :disable-views="['week']"
    @cell-focus="selectedDate = $event"
    >
</vue-cal>
                        <input type="submit" value="Sumbit Task" />
                    </form>
                        <button @click="PrintSelectedDate()">PrintDate</button>

                </div>
            </div>
        </div>
    </transition>
    <button @click="isOpen=!isOpen" :style="{background: color}">{{text}}</button>
</template>

<script>
import VueCal from 'vue-cal'
import 'vue-cal/dist/vuecal.css'

export default({
    name: 'AddMediaTask',
    components:{
        VueCal
    },
    data(){
        return{
            selectedType: null,
            nameOfMedia: '',
            reminder: false,
            isOpen: false,
            selectedDate: null
        }
    },
    props: {
        mediaTypes: Array,
        text: String,
        color: String
    },
    methods: {
        chooseMediaType(event){
            this.selectedType = event.target.options[event.target.options.selectedIndex]._value
        },
        onSubmit(e){
            e.preventDefault()

            if(!this.nameOfMedia){
                alert("Please add in a name")
                return
            }

            if(this.selectedType == null){
                alert("Please select a media type")
                return
            }

            if(this.selectedDate == null){
                alert("Please select a start date")
                return
            }

            const newMediaTask = {name: this.nameOfMedia, 
            type_id: this.selectedType.ID, 
            reminder: this.reminder,
            startDate: this.selectedDate}

            this.$emit('add-media', newMediaTask)

            this.nameOfMedia = '';
            this.selectedType = null;
            this.reminder = false;
            this.selectedDate = null;
        },
        PrintSelectedDate(){
            console.log(this.selectedDate);
        }
    },
    emits: ['add-media']
})
</script>

<style scoped>
.modal {
  width: 500px;
  margin: 0px auto;
  padding: 20px;
  background-color: #fff;
  border-radius: 2px;
  box-shadow: 0 2px 8px 3px;
  transition: all 0.2s ease-in;
  font-family: Helvetica, Arial, sans-serif;
}
.fadeIn-enter {
  opacity: 0;
}

.fadeIn-leave-active {
  opacity: 0;
  transition: all 0.2s step-end;
}

.fadeIn-enter .modal,
.fadeIn-leave-active.modal {
  transform: scale(1.1);
}


.overlay {
  position: fixed;
  top: 0;
  left: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
  background: #00000094;
  z-index: 999;
  transition: opacity 0.2s ease;
}
</style>