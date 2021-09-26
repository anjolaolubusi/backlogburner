<template>
    <div class="mid">
      <div class="calendar-area">
          <div class="calendar">
            <vue-cal :time="true" active-view="day" :disable-views="['years']" today-button :events="listOfEvents" />
          </div>
      </div>
      <div>
        <AddCalendarEvent  @add-cal-event="addMediaTask" text="Add Task" color="green"/>
      </div>
    </div>
</template>

<script>
import VueCal from 'vue-cal'
import 'vue-cal/dist/vuecal.css'
import AddCalendarEvent from '../components/AddCalendarEvent.vue'
export default {
  name: 'Schedule',
  components: {
    VueCal,
    AddCalendarEvent
  },
  props: {
      username: String
  },
  data() {
    return{
      listOfEvents: []
    }
  },
  methods: {
    addMediaTask(mediaTask){
      this.listOfEvents.push(mediaTask)
    }
  },
  created() {

  }
}
</script>