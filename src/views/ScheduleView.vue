<template>
    <router-link id="Logout" to="/logout">Logout</router-link>

    <div class="fullHeight" style="display: flex;justify-content: center;gap: 1%;">
      <div class="smallCalendar">
        <vue-cal ref="smallCalendar" events-count-on-year-view @view-change="updateCalenderViews('1', $event)" xsmall @cell-focus="selectedDate = $event" :selected-date="selectedDate" hide-view-selector :events="drawingList" active-view="month" :disable-views="['years', 'week', 'day', 'year']" />
      </div>
      <div class="bigCalendar fullHeight">
        <vue-cal small ref="bigCalendar" @view-change="updateCalenderViews('0', $event)" hide-view-selector @cell-focus="selectedDate = $event" :selected-date="selectedDate"  events-on-month-view="true" :time="true" active-view="week" :disable-views="['years', 'day', 'year', 'month']" today-button :events="drawingList"/>
      </div>
    </div>
      
    <AddCalendarEvent  @add-cal-event="addMediaTask" text="Add Task" color="green" v-bind:listOfEvents="drawingList" v-bind:selectedDate="selectedDate"  @pull-outlook-event="addOutlookTask" @add-sc="addSC" @pull-google-event="addGoogleTask"/>
    <button @click="apiTest">Send Data</button>
    <button @click="printCurrentTask">Print List</button>
    <button @click="printListOfItems">Print List of Items</button> 
</template>

<script>
import VueCal from 'vue-cal'
import 'vue-cal/dist/vuecal.css'
import AddCalendarEvent from '../components/AddCalendarEvent.vue'
import {MODEL_API} from '../api-common'

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
      SC: [],
      selectedDate: null,
    }
  },
  methods: {
    updateCalenderViews(call_name, event){
      console.dir(event);
      console.dir(this.$refs.smallCalendar);
      if(call_name == '1'){
        let tempDate = null;
        if(this.selectedDate){
          tempDate = new Date(this.selectedDate);
        }else{
          tempDate = new Date();
        }
        tempDate.setMonth(event.startDate.getMonth());
        this.$refs.bigCalendar.switchView('week', tempDate);
      }
      if(call_name == '0'){
        let tempDate = null;
        if(this.selectedDate){
          tempDate = new Date(this.selectedDate);
        }else{
          tempDate = new Date();
        }

        if(tempDate <= event.startDate){
          console.log("BOOM");
          tempDate.setDate(tempDate.getDate() + 7);
          this.selectedDate = tempDate;
          //this.$refs.smallCalendar.switchView(this.$refs.smallCalendar.view.id, tempDate);
        }
        if(tempDate >= event.startDate){
          tempDate.setDate(tempDate.getDate() - 7);
          this.selectedDate = tempDate;
          //this.$refs.smallCalendar.switchView(this.$refs.smallCalendar.view.id, tempDate);
        }
      }
    },
    addMediaTask(mediaTask){
      this.listOfEvents.push(mediaTask);
      this.addToDrawingList(mediaTask);
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
        recurrence: null}
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
        // if(newEvent.recurrence.pattern.type == "daily"){
        //   if(newEvent.recurrence.range.type == "endDate")
        //   var recurEndDate = new Date(newEvent.recurrence.range.endDate);
        //   var timeDiff = (newEvent.end - newEvent.start)/1000;     
        //   for(var curr_start = new Date(newEvent.start.getTime()); curr_start <= recurEndDate; curr_start.setDate(curr_start.getDate() + 1)){
        //       var newEnd = new Date(curr_start);
        //       newEnd.setSeconds(newEnd.getSeconds() + timeDiff);
        //       var drawEvent = {
        //         title: newEvent.title,
        //         start: new Date(curr_start),
        //         end: new Date(newEnd),
        //         source: "O",
        //         class: 'hc',
        //         recurrence: newEvent.recurrence,
        //             deletable: false,
        //             resizable: false,
        //             draggable: false
        //       };
        //       this.drawingList.push(drawEvent);
        //   }
        //   return true;
        //   //For each between start day and end day add event
        // }

        if(newEvent.recurrence.pattern == 'Daily'){
          if(newEvent.recurrence.recurranceType == "endDate"){
            let timeDiff = (newEvent.end - newEvent.start)/1000;
            for(let curr_start=new Date(newEvent.start.getTime()); curr_start <= newEvent.recurrence.endDate; curr_start.setDate(curr_start.getDate() + newEvent.recurrence.frequency)){
              let newEnd = new Date(curr_start);
              newEnd.setSeconds(newEnd.getSeconds() + timeDiff);
              let drawEvent = {
                title: newEvent.title,
                start: new Date(curr_start),
                end: new Date(newEnd),
                source: "M",
                class: 'hc',
                recurrence: newEvent.recurrence,
                    deletable: false,
                    resizable: false,
                    draggable: false
              };
              this.drawingList.push(drawEvent);
            }
          }

          if(newEvent.recurrence.recurranceType == "OnOcuurance"){
            let timeDiff = (newEvent.end - newEvent.start)/1000;
            let curr_start=new Date(newEvent.start.getTime());
            for(let i = 0; i < newEvent.recurrence.numOfOccurance; i++){
              let newEnd = new Date(curr_start);
              newEnd.setSeconds(newEnd.getSeconds() + timeDiff);
              let drawEvent = {
                title: newEvent.title,
                start: new Date(curr_start),
                end: new Date(newEnd),
                source: "M",
                class: 'hc',
                recurrence: newEvent.recurrence,
                    deletable: false,
                    resizable: false,
                    draggable: false
              };
              this.drawingList.push(drawEvent);
              curr_start.setDate(curr_start.getDate() + newEvent.recurrence.frequency);         
            }
          }

          if(newEvent.recurrence.recurranceType == "Never"){
            let timeDiff = (newEvent.end - newEvent.start)/1000;
            let endDate = new Date(newEvent.start.getTime());
            endDate.setDate(newEvent.start.getDate() + 365);
            for(let curr_start=new Date(newEvent.start.getTime()); curr_start <= endDate; curr_start.setDate(curr_start.getDate() + newEvent.recurrence.frequency)){
              let newEnd = new Date(curr_start);
              newEnd.setSeconds(newEnd.getSeconds() + timeDiff);
              let drawEvent = {
                title: newEvent.title,
                start: new Date(curr_start),
                end: new Date(newEnd),
                source: "M",
                class: 'hc',
                recurrence: newEvent.recurrence,
                    deletable: false,
                    resizable: false,
                    draggable: false
              };
              this.drawingList.push(drawEvent);
            }
          }
        }

        if(newEvent.recurrence.pattern == 'Weekly'){
          if(newEvent.recurrence.recurranceType == 'OnDate'){
            let timeDiff = (newEvent.end - newEvent.start)/1000;
            for(let curr_start=new Date(newEvent.start.getTime()); curr_start <= newEvent.recurrence.endDate; curr_start.setDate(curr_start.getDate() + newEvent.recurrence.frequency * 7)){
              let days = ['sunday', 'monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday'];
              for(let i = 0; i < newEvent.recurrence.selectedDayOfTheWeek.length; i++){
                let temp_event = curr_start;
                let diff = days.indexOf(newEvent.recurrence.selectedDayOfTheWeek[i]) - temp_event.getDay();
                temp_event.setDate(curr_start.getDate() + diff);
                let newEnd = new Date(temp_event);
                newEnd.setSeconds(newEnd.getSeconds() + timeDiff);
                let drawEvent = {
                  title: newEvent.title,
                  start: new Date(curr_start),
                  end: new Date(newEnd),
                  source: "M",
                  class: 'hc',
                  recurrence: newEvent.recurrence,
                      deletable: false,
                      resizable: false,
                      draggable: false
                  };
                  this.drawingList.push(drawEvent);
                }
            }
          }

          if(newEvent.recurrence.recurranceType == 'Never'){
            let timeDiff = (newEvent.end - newEvent.start)/1000;
            let endDate = new Date(newEvent.start.getTime());
            endDate.setDate(newEvent.start.getDate() + 365);
            for(let curr_start=new Date(newEvent.start.getTime()); curr_start <= endDate; curr_start.setDate(curr_start.getDate() + newEvent.recurrence.frequency * 7)){
              let days = ['sunday', 'monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday'];
              for(let i = 0; i < newEvent.recurrence.selectedDayOfTheWeek.length; i++){
                let temp_event = curr_start;
                let diff = days.indexOf(newEvent.recurrence.selectedDayOfTheWeek[i]) - temp_event.getDay();
                temp_event.setDate(curr_start.getDate() + diff);
                let newEnd = new Date(temp_event);
                newEnd.setSeconds(newEnd.getSeconds() + timeDiff);
                let drawEvent = {
                  title: newEvent.title,
                  start: new Date(curr_start),
                  end: new Date(newEnd),
                  source: "M",
                  class: 'hc',
                  recurrence: newEvent.recurrence,
                      deletable: false,
                      resizable: false,
                      draggable: false
                  };
                  this.drawingList.push(drawEvent);
                }
            }
          }          

          if(newEvent.recurrence.recurranceType == 'OnOcuurance'){
            let timeDiff = (newEvent.end - newEvent.start)/1000;
            let curr_start=new Date(newEvent.start.getTime());
            let count = 0;
            while(count < newEvent.recurrence.numOfOccurance){
              let days = ['sunday', 'monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday'];
              for(let i = 0; i < newEvent.recurrence.selectedDayOfTheWeek.length; i++){
                if(count < newEvent.recurrence.numOfOccurance){
                  let temp_event = curr_start;
                  let diff = days.indexOf(newEvent.recurrence.selectedDayOfTheWeek[i]) - temp_event.getDay();
                  temp_event.setDate(curr_start.getDate() + diff);
                  let newEnd = new Date(temp_event);
                  newEnd.setSeconds(newEnd.getSeconds() + timeDiff);
                  let drawEvent = {
                    title: newEvent.title,
                    start: new Date(curr_start),
                    end: new Date(newEnd),
                    source: "M",
                    class: 'hc',
                    recurrence: newEvent.recurrence,
                        deletable: false,
                        resizable: false,
                        draggable: false
                    };
                    this.drawingList.push(drawEvent);
                    count++;
                  }
                }
                console.log(`COUNT: ${count}`);
                curr_start.setDate(curr_start.getDate() + newEvent.recurrence.frequency * 7);
            }
          }
        }
      }else{
      this.drawingList.push(newEvent);
      }
    },
    addSC(newSC){
      this.SC.push(newSC)
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
      var EndOfCycle = new Date(monday);
      EndOfCycle.setDate(EndOfCycle.getDate() - (EndOfCycle.getDay() - 7));
      var data = {
        //monday: this.getMonday(new Date()),
        monday: monday,
        EndOfCycle: EndOfCycle,
        listOfEvents: this.drawingList.filter(event => ( 0 < (event.start.getTime() - monday.getTime()) && (event.start.getTime() - monday.getTime()) < 1000 * 60 * 60 * 24 * 7 )),
        newEvent: [this.SC]
      }
      await MODEL_API.post("model", data)
        .then((res) => {
          //var newSchedule = JSON.parse(res.config.data);
          console.log(`Response from schedule api: `);
          console.dir(res.data);
          for (var i = 0; i < res.data.length; i++){
            var newItem = res.data[0];
            newItem.start = new Date(newItem.start);
            newItem.end = new Date(newItem.end);
            this.listOfEvents.push(newItem);
            this.addToDrawingList(newItem);
          }
        })
    }
  },
  mounted() {
    if(!this.$cookies.isKey("accessToken")){
      this.$router.push({ name: 'Login'});
    }
  }
}
</script>

<style>
.vuecal__event.sc {background-color: rgba(253, 156, 66, 0.9);border: 1px solid rgb(233, 136, 46);color: #fff;}
.vuecal__event.hc {background-color: rgba(255, 102, 102, 0.9);border: 1px solid rgb(235, 82, 82);color: #fff;}

.vuecal__menu, .vuecal__cell-events-count {background-color: #42b983;}
.vuecal__title-bar {background-color: #e4f5ef;}
.vuecal__cell--today, .vuecal__cell--current {background-color: rgba(240, 240, 255, 0.4);}
.vuecal:not(.vuecal--day-view) .vuecal__cell--selected {background-color: rgba(235, 255, 245, 0.4);}
.vuecal__cell--selected:before {border-color: rgba(66, 185, 131, 0.5);}
/* Cells and buttons get highlighted when an event is dragged over it. */
.vuecal__cell--highlighted:not(.vuecal__cell--has-splits),
.vuecal__cell-split--highlighted {background-color: rgba(195, 255, 225, 0.5);}
.vuecal__arrow.vuecal__arrow--highlighted,
.vuecal__view-btn.vuecal__view-btn--highlighted {background-color: rgba(136, 236, 191, 0.25);}

.smallCalendar .vuecal__cell-events-count {
  width: 4px;
  min-width: 0;
  height: 4px;
  padding: 0;
  color: transparent;
}

.bigCalendar .vuecal--month-view .vuecal__cell {height: 80px;}

.bigCalendar .vuecal--month-view .vuecal__cell-content {
  justify-content: flex-start;
  height: 100%;
  align-items: flex-end;
}

.bigCalendar .vuecal--month-view .vuecal__cell-date {padding: 4px;}
.bigCalendar .vuecal--month-view .vuecal__no-event {display: none;}

.CalendarCon{
  display: flex;
  
  gap: 1%;
}

.bigCalendar{
  width:100%;
  height: 100%;
}

.smallCalendar{
  width: 30%;
  height: 30%;
  min-width: 250px;
  min-height: 250px;
}

#Logout{
  text-align: center;
  display: block;
  margin: 0 auto;
}


</style>
