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

<vue-cal
    class="vuecal--date-picker"
    xsmall
    hide-view-selector
    :time="false"
    :transitions="false"
    active-view="month"
    :disable-views="['week']"
    @cell-focus="startDate = $event"
    >
</vue-cal>

<vue-cal
    class="vuecal--date-picker"
    xsmall
    hide-view-selector
    :time="false"
    :transitions="false"
    active-view="month"
    :disable-views="['week']"
    @cell-focus="endDate = $event"
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
            nameOfMedia: '',
            startDate: null,
            isOpen: false,
            endDate: null
        }
    },
    props: {
        text: String,
        color: String
    },
    methods: {
        onSubmit(e){
            e.preventDefault()
            if(!this.nameOfMedia){
                alert("Please add in a name")
                return
            }
            if(this.startDate == null){
                alert("Please select a start date")
                return
            }
            if(this.endDate == null){
                alert("Please select an end date")
                return
            }

            const newEvent = {title: this.nameOfMedia, 
            start: this.startDate,
            end: this.endDate}
            this.$emit('add-cal-event', newEvent)
            this.nameOfMedia = '';
            this.startDate  = null;
            this.endDate = null;
        },
        PrintSelectedDate(){
            console.log(`Start Date: ${this.startDate} End Date: ${this.endDate}`);
        }
    },
    emits: ['add-cal-event']
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