<template>
    <div class="mid">
      <div class="calendar-area">
          <div class="calendar">
            <vue-cal :time="true" active-view="day" :disable-views="['years']" today-button :events="drawingList" />
          </div>
      </div>
      <div>
        <AddCalendarEvent  @add-cal-event="addMediaTask" text="Add Task" color="green" @pull-outlook-event="addOutlookTask" @add-sc="addSC" @pull-google-event="addGoogleTask"/>
      </div>
    </div>
    <button @click="apiTest">Send Data</button>
    <button @click="printCurrentTask">Print List</button>
    <button @click="printListOfItems">Print List of Items</button>
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
      drawingList: [],
      SC: null
    }
  },
  methods: {
    addMediaTask(mediaTask){
      this.listOfEvents.push(mediaTask);
      this.drawingList.push(mediaTask);
    },
    addOutlookTask(cal_data){
      this.listOfEvents = this.listOfEvents.filter(event => event.source != 'O');
      this.drawingList = this.drawingList.filter(event => event.source != 'O');      
      for(let i = 0; i < cal_data.length; i++){
        const newEvent = {title: cal_data[i].subject, 
        start: this.returnDateObject(cal_data[i].start.dateTime),
        end: this.returnDateObject(cal_data[i].end.dateTime),
        source: "O",
        class: 'hc',
        recurrence: cal_data[i].recurrence}
        this.listOfEvents.push(newEvent);
        this.addToDrawingList(newEvent);
      }
    },
    addGoogleTask(cal_data){
      console.log("GOOGLE TIME")
      this.listOfEvents = this.listOfEvents.filter(event => event.source != 'G');
      this.drawingList = this.drawingList.filter(event => event.source != 'G');
      for(let i = 0; i < cal_data.length; i++){
        const newEvent = {
        title: cal_data[i].summary,
        start: this.returnDateObject(cal_data[i].start.dateTime),
        end: this.returnDateObject(cal_data[i].end.dateTime),
        source: "G",
        class: 'hc',
        recurrence: null
        }
        this.listOfEvents.push(newEvent);
        this.addToDrawingList(newEvent);
      }

    },
    returnDateObject(dateString){
      var b = dateString.split(/\D+/);
      return new Date(Date.UTC(b[0], --b[1], b[2], b[3], b[4], b[5], b[6]));
    },
    addToDrawingList(newEvent){
      //TODO: Fic curr_start and newEvent.end
      if(newEvent.recurrence){
        if(newEvent.recurrence.pattern.type == "daily"){
          if(newEvent.recurrence.range.type == "endDate")
          var recurEndDate = new Date(newEvent.recurrence.range.endDate);
          var timeDiff = (newEvent.end - newEvent.start)/1000;     
          for(var curr_start = new Date(newEvent.start.getTime()); curr_start <= recurEndDate; curr_start.setDate(curr_start.getDate() + 1)){
              var newEnd = new Date(curr_start);
              newEnd.setSeconds(newEnd.getSeconds() + timeDiff);
              var drawEvent = {
                title: newEvent.title,
                start: new Date(curr_start),
                end: new Date(newEnd),
                source: "O",
                class: 'hc',
                recurrence: newEvent.recurrence
              };
              this.drawingList.push(drawEvent);
          }
          //For each between start day and end day add event
        }
      }else{
        this.drawingList.push(newEvent);
      }
    },
    addSC(newSC){
      this.SC = newSC
    },
    printCurrentTask(){
      console.log(this.drawingList);
    },
    printListOfItems(){
      console.log(this.listOfEvents);
    },
    getMonday(d) {
      d = new Date(d);
      var day = d.getDay(),
      diff = d.getDate() - day + (day == 0 ? -6:1); // adjust when day is sunday
      return new Date(d.setDate(diff));
    },
    async apiTest(){
      var monday = new Date(this.SC.selectedDate)
      if(monday.getDay() != 1){
        monday.setDate(monday.getDate() - (monday.getDay() - 1))
      }
      var data = {
        //monday: this.getMonday(new Date()),
        monday: monday,
        listOfEvents: this.listOfEvents.filter(event => ( 0 < (event.start.getTime() - monday.getTime()) && (event.start.getTime() - monday.getTime()) < 1000 * 60 * 60 * 24 * 7 )),
        newEvent: this.SC
      }
      await ILP_API.post("model", data)
        .then((res) => {
          //var newSchedule = JSON.parse(res.config.data);
          console.log(`Response from schedule api: `);
          console.dir(res.data);
          var newItem = res.data
          newItem.start = new Date(newItem.start)
          newItem.end = new Date(newItem.end)
          this.addMediaTask(newItem)
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