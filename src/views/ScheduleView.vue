<template>
    <div class="mid">
      <div class="calendar-area">
          <div class="calendar">
            <vue-cal :time="true" active-view="day" :disable-views="['years']" today-button :events="drawingList" />
          </div>
      </div>
      <div>
        <AddCalendarEvent  @add-cal-event="addMediaTask" text="Add Task" color="green" @pull-outlook-event="addOutlookTask"/>
      </div>
    </div>
    <button @click="apiTest">Send Data</button>
    <button @click="printCurrentTask">Print List</button>
</template>

<script>
import VueCal from 'vue-cal'
import 'vue-cal/dist/vuecal.css'
import AddCalendarEvent from '../components/AddCalendarEvent.vue'
import {ILP_API} from '../api-common'
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
      listOfEvents: [],
      drawingList: []
    }
  },
  methods: {
    addMediaTask(mediaTask){
      this.listOfEvents.push(mediaTask);
      this.drawingList.push(mediaTask);
    },
    addOutlookTask(cal_data){
      this.listOfEvents = this.listOfEvents.filter(event => event.source != 'O');
      for(let i = 0; i < cal_data.length; i++){
        const newEvent = {title: cal_data[i].subject, 
        start: new Date(cal_data[i].start.dateTime),
        end: new Date(cal_data[i].end.dateTime),
        source: "O",
        class: 'hc'}
        this.listOfEvents.push(newEvent);
        this.drawingList.push(newEvent)
      }
    },
    printCurrentTask(){
      console.log(this.listOfEvents);
    },
    getMonday(d) {
      d = new Date(d);
      var day = d.getDay(),
      diff = d.getDate() - day + (day == 0 ? -6:1); // adjust when day is sunday
      return new Date(d.setDate(diff));
    },
    async apiTest(){
      var data = {
        //monday: this.getMonday(new Date()),
        monday: new Date("10/4/2021"),
        listOfEvents: this.listOfEvents,
      }
      await ILP_API.post("model", data)
        .then((res) => {
          //var newSchedule = JSON.parse(res.config.data);
          console.log(`Response from schedule api: `);
          console.dir(res.data);
        })
    }
  },
  created() {

  }
}
</script>

<style>
.vuecal__event.sc {background-color: rgba(253, 156, 66, 0.9);border: 1px solid rgb(233, 136, 46);color: #fff;}
.vuecal__event.hc {background-color: rgba(255, 102, 102, 0.9);border: 1px solid rgb(235, 82, 82);color: #fff;}
</style>