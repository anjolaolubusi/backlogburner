<template>
    <div class="mid">
      <div class="calendar-area">
          <div class="calendar">
            <vue-cal :time="true" active-view="day" :disable-views="['years']" today-button :events="listOfEvents" />
          </div>
      </div>
      <div>
        <AddCalendarEvent  @add-cal-event="addMediaTask" text="Add Task" color="green" @pull-outlook-event="addOutlookTask"/>
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
    },
    addOutlookTask(cal_data){
      this.listOfEvents = this.listOfEvents.filter(event => event.source != 'O');
      for(let i = 0; i < cal_data.length; i++){
        const newEvent = {title: cal_data[i].subject, 
        start: new Date(cal_data[i].start.dateTime),
        end: new Date(cal_data[i].end.dateTime),
        source: "O"}
        this.listOfEvents.push(newEvent);
      }
    },
    printCurrentTask(){
      console.log(this.listOfEvents);
    }
  },
  created() {

  }
}
</script>