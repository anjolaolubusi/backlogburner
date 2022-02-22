
<template>
    <router-link id="Logout" to="/logout">Logout</router-link>

    <div class="fullHeight" style="display: flex;justify-content: center;gap: 1%;">
      <div class="smallCalendar">
        <AddCalendarEvent @add-cal-event="addMediaTask" text="Add Event" color="green" v-bind:listOfEvents="drawingList" v-bind:selectedDate="new Date()"  @pull-outlook-event="addOutlookTask" @add-sc="addSC" @pull-google-event="addGoogleTask"/>
        <vue-cal ref="smallCalendar" events-count-on-year-view @view-change="updateCalenderViews('1', $event)" today-button xsmall @cell-focus="selectedDate = $event" :selected-date="selectedDate" hide-view-selector :events="drawingList" active-view="month" :disable-views="['years', 'week', 'day', 'year']" class="vuecal--date-picker" />
        <br/>
        <h3>List of Submitted Hobbies</h3>
        <HobbyList v-bind:sourceData="SC" @edit-hobby="openHobbyEditModal" @delete-hobby="deleteHobby" @call-api="apiTest" />
      </div>
      <div class="bigCalendar fullHeight">
        <vue-cal small ref="bigCalendar" :on-event-dblclick="openEditEvent" hide-view-selector @cell-focus="selectedDate = $event" :selected-date="selectedDate"  events-on-month-view="true" :time="true" active-view="week" :disable-views="['years', 'day', 'year', 'month']" :events="drawingList"/>
      </div>
    </div>



  <transition name="modal">
    <div v-if="editHobbyModalBool">
        <div class="overlay">
            <div class="modal" style="width: 57%">
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: flex-end;">
                    <button style="background-color: black;font-size: 16px;color: white;padding: 7px;margin-top: 10px;" @click="editHobbyModalBool = false;errors=[];">Close</button>
                </div>
                <h2 style="text-align: center;">Edit Hobby</h2>
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: center;">
                    <div>
                        <form @submit="editHobby">
                            <p v-if="errors.length > 0">
                              <b>Please correct the following error(s):</b>
                              <ul>
                                <li v-for="error in errors" :key="error">{{ error }}</li>
                              </ul>
                            </p>
                            <label>Name: <input type="text" placeholder="Enter Name of Hobby" v-model="hobbyName" />  </label>
                            <br>
                            <label> Length: <input type="number" min="0" v-model="hobbyHours"  oninput="this.value = this.value.replace(/[^0-9.]/g, '').replace(/(\..*?)\..*/g, '$1');"/> hr <input type="number" min="0" v-model="hobbyMinutes" oninput="this.value = this.value.replace(/[^0-9.]/g, '').replace(/(\..*?)\..*/g, '$1');"/> min </label>
                            <br>
                            <label>Repeat: </label>
                            <select v-model="eventRecurrance">
                                <option v-for="listValue in recurranceTypes" :value="listValue" :key="listValue">
                                    {{listValue}}
                                </option>
                            </select>       
                            <br />
                            <div v-if="eventRecurrance != 'Just Once'">
                                <h3 style="margin-bottom: 0px">Recurrance Details</h3>
                                <ol v-if="eventRecurrance == 'Daily'">
                                    <div> 
                                        <li>
                                            <div style="margin-bottom: 8px">
                                                <label>Start Date: <input type="date" v-model="hobbyRecurStartDate" /> </label>
                                            </div>
                                        </li>
                                        <li>
                                            <div style="margin-bottom: 8px;">
                                                <label>Repeat every <input name="occurDay" type="number" v-model="dailyOccurNum" min="0" size="5" /> day(s) </label>
                                            </div>
                                        </li>
                                        <div>
                                            <li style="margin-bottom: 8px;"><span>Ends</span></li>
                                        </div>
                                        <div v-on:click="recurType='Never'">
                                            <input type="radio" value="Never" v-model="recurType"/> Never
                                        </div>

                                        <div v-on:click="recurType='endDate'">
                                            <input type="radio" value="endDate" v-model="recurType" /> Date: <input type="date" v-model="recurEndDate" />
                                        </div>
                                        <div  v-on:click="recurType='OnOcuurance'">
                                            <label><input type="radio" value="OnOcuurance" v-model="recurType" size="5" /> After <input type="number" min="0" v-model="numOfOccurance" size="5" /> occurrence(s)</label>
                                        </div>
                                        <li>
                                            <div style="margin-top: 8px;">
                                            <label>Within which days do you want use to derive the time for this new hobby? <br /> 
                                            <input  type="date" v-model="hobbyRanges.start" /> - <input type="date" v-model="hobbyRanges.end"/>
                                            </label>
                                            </div>
                                        </li>
                                    </div>
                                </ol>
                                <ol v-if="eventRecurrance == 'Weekly'">
                                <div>
                                    <li>
                                        <div style="margin-bottom: 8px">
                                            <label>Start Date: <input type="date" v-model="hobbyRecurStartDate" /> </label>
                                        </div>
                                    </li>
                                    <li>
                                        <div><label>Repeat every <input type="number" v-model="dailyOccurNum" min="0" size="5"/> week(s)</label></div>
                                    </li>
                                    <br />
                                    <div>
                                        <li>
                                            <label>Repeat on: </label>
                                        </li>
                                        <label for="monday" @click="addToDaysOfWeek('monday')"> <input type="checkbox" value="monday" v-model="selectedDayOfTheWeek" />Mon</label>
                                        <label for="tuesday" @click="addToDaysOfWeek('tuesday')"> <input type="checkbox" value="tuesday" v-model="selectedDayOfTheWeek" />Tues</label>
                                        <label for="wednesday" @click="addToDaysOfWeek('wednesday')"> <input type="checkbox" value="wednesday" v-model="selectedDayOfTheWeek" />Wed</label>
                                        <label for="thursday" @click="addToDaysOfWeek('thursday')"> <input type="checkbox" value="thursday" v-model="selectedDayOfTheWeek" />Thurs</label>
                                        <label for="friday" @click="addToDaysOfWeek('friday')"> <input type="checkbox" value="friday" v-model="selectedDayOfTheWeek" />Fri</label>
                                        <label for="saturday" @click="addToDaysOfWeek('saturday')"> <input type="checkbox" value="saturday" v-model="selectedDayOfTheWeek" />Sat</label>
                                        <label for="sunday" @click="addToDaysOfWeek('sunday')"> <input type="checkbox" value="sunday" v-model="selectedDayOfTheWeek" />Sun</label>
                                    </div>
                                    <br />
                                        <li>Ends</li>
                                        <div v-on:click="recurType='Never'">
                                            <input type="radio" value="Never" v-model="recurType" /> Never
                                        </div>

                                        <div v-on:click="recurType='OnDate'">
                                            <input type="radio" value="OnDate" v-model="recurType" /> On <input type="date" v-model="recurEndDate" />
                                        </div>

                                        <div v-on:click="recurType='OnOcuurance'">
                                            <label><input type="radio" value="OnOcuurance" v-model="recurType" /> After <input type="number" v-model="numOfOccurance" size="5"/> occurrence(s)</label>
                                        </div>
                                        <li>
                                            <div style="margin-top: 8px;">
                                            <label>Within which days do you want use to derive the time for this new hobby? <br /> 
                                            <input  type="date" v-model="hobbyRanges.start" /> - <input type="date" v-model="hobbyRanges.end"/>
                                            </label>
                                            </div>
                                        </li>
                                </div>
                                </ol>
                                
                            </div> 
                            <br />
                            <label v-if="eventRecurrance == 'Just Once'">Within which days do you want this hobby to happen? <br />
                            <input  type="date" v-model="hobbyRanges.start" /> - <input type="date" v-model="hobbyRanges.end"/>
                            </label>
                            <br />
                            <!-- <DatePicker v-model="hobbyRanges" is-range /> -->
                            <br>
                            <input style="padding: 7px; margin-top: 10px;background-color: green;color: white;font-size: 1.1rem;" type="submit" value="Save Hobby" />  
                            <br>
                        </form>
                    </div>
                    <vue-cal small hide-view-selector :selected-date="selectedDate" :events="drawingList" active-view="week" :disable-views="['years']"  style="max-width: 460px;height: 500px;" />
                </div>
            </div>
        </div>
    </div>
  </transition>

  <transition name="modal">
    <div v-if="removeEventModalBool">
        <div class="overlay">
            <div class="modal" style="width: 28%">
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: flex-end;">
                    <button style="background-color: black;font-size: 16px;color: white;padding: 7px;margin-top: 10px;" @click="removeEventModalBool = false;errors=[];">Close</button>
                </div>
                <h2 style="text-align: center;">Edit Event</h2>
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: center;">
                    <div>
                        <!-- <form @submit="editEvent"> -->
                            <label>What is the name of the event?: </label>
                            <input v-model="eventName" type="text" placeholder="Enter Title" size="15" /> <br/>
                            <label>When does the event start? </label>
                            <input type="date" v-model="eventStartDate" size="30" /> <input type="time" v-model="eventStartTime" /> <br />
                            <label>When does the event end? </label>
                            <input type="date" v-model="eventEndDate" size="30" /> <input type="time" v-model="eventEndTime" /> <br/>
                            <label>How often does this event repeat? </label>
                            <select v-model="eventRecurrance">
                                <option v-for="listValue in recurranceTypes" :value="listValue" :key="listValue">
                                    {{listValue}}
                                </option>
                            </select>       
                            <br />
                            <div v-if="eventRecurrance != 'Just Once'">
                                <h3 style="margin-bottom: 0px">Recurrance Details</h3>
                                <ol v-if="eventRecurrance == 'Daily'">
                                    <div> 
                                        <li>
                                            <div style="margin-bottom: 8px;">
                                                <label>Repeat every <input name="occurDay" type="number" v-model="dailyOccurNum" min="0" size="5" /> day(s) </label>
                                            </div>
                                        </li>
                                        <div>
                                            <li style="margin-bottom: 8px;"><span>When will the recurring event end?</span></li>
                                        </div>
                                        <div v-on:click="recurType='Never'">
                                            <input type="radio" value="Never" v-model="recurType"/> This event will never stop recurring 
                                        </div>

                                        <div v-on:click="recurType='endDate'">
                                            <input type="radio" value="endDate" v-model="recurType" /> This event will end on: <input type="date" v-model="recurEndDate" />
                                        </div>
                                        <div  v-on:click="recurType='OnOcuurance'">
                                            <label><input type="radio" value="OnOcuurance" v-model="recurType" size="5" /> This event will end after: <input type="number" min="0" v-model="numOfOccurance" size="5" /> occurrence(s)</label>
                                        </div>
                                    </div>
                                </ol>
                                <ol v-if="eventRecurrance == 'Weekly'">
                                <div>
                                    <li>
                                        <div><label>Repeat every <input type="number" v-model="dailyOccurNum" min="0" size="5"/> week(s)</label></div>
                                    </li>
                                    <br />
                                    <div>
                                        <li>
                                            <label>Repeat on: </label>
                                        </li>
                                        <label for="monday" @click="addToDaysOfWeek('monday')"> <input type="checkbox" value="monday" v-model="selectedDayOfTheWeek" />Mon</label>
                                        <label for="tuesday" @click="addToDaysOfWeek('tuesday')"> <input type="checkbox" value="tuesday" v-model="selectedDayOfTheWeek" />Tues</label>
                                        <label for="wednesday" @click="addToDaysOfWeek('wednesday')"> <input type="checkbox" value="wednesday" v-model="selectedDayOfTheWeek" />Wed</label>
                                        <label for="thursday" @click="addToDaysOfWeek('thursday')"> <input type="checkbox" value="thursday" v-model="selectedDayOfTheWeek" />Thurs</label>
                                        <label for="friday" @click="addToDaysOfWeek('friday')"> <input type="checkbox" value="friday" v-model="selectedDayOfTheWeek" />Fri</label>
                                        <label for="saturday" @click="addToDaysOfWeek('saturday')"> <input type="checkbox" value="saturday" v-model="selectedDayOfTheWeek" />Sat</label>
                                        <label for="sunday" @click="addToDaysOfWeek('sunday')"> <input type="checkbox" value="sunday" v-model="selectedDayOfTheWeek" />Sun</label>
                                    </div>
                                    <br />
                                        <li>Ends</li>
                                        <div v-on:click="recurType='Never'">
                                            <input type="radio" value="Never" v-model="recurType" /> Never
                                        </div>

                                        <div v-on:click="recurType='OnDate'">
                                            <input type="radio" value="OnDate" v-model="recurType" /> On <input type="date" v-model="recurEndDate" />
                                        </div>

                                        <div v-on:click="recurType='OnOcuurance'">
                                            <label><input type="radio" value="OnOcuurance" v-model="recurType" /> After <input type="number" v-model="numOfOccurance" size="5"/> occurrence(s)</label>
                                        </div>
                                </div>
                                </ol>
                                
                            </div> 
                            <br />
                            <button style="padding: 7px; margin-top: 10px;background-color: green;color: white;font-size: 1.1rem;" @click="editEvent">Update Event</button>
                            <button style="padding: 7px; margin-top: 10px;background-color: rgba(220, 25, 25, 1);color: white;font-size: 1.1rem;" @click="removeEvent">Remove Event</button>
                        <!-- </form> -->
                        <br />
                    </div>
                </div>
            </div>
        </div>
    </div>
  </transition> 

  <transition name="modal">
    <div v-if="openApiModal">
        <div class="overlay">
            <div class="modal" style="width: 50%">
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: flex-end;">
                    <button style="background-color: rgba(220, 25, 25, 1);font-size: 16px;color:white;padding: 7px;margin-top: 10px;" @click="openApiModal = false;apiResponse=[];isApiDone=false;">Close</button>
                </div>
                <h2 style="text-align: center;">Gaining New Schedule</h2>
                <br />
                  <div>
                    <p v-if="errors.length > 0">
                      <b>Please correct the following error(s):</b>
                      <ul>
                        <li v-for="error in errors" :key="error">{{ error }}</li>
                      </ul>
                    </p>
                  </div>
                <div v-if="isApiDone" style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: center;">
                      <form @submit="SaveHobbies">
                  <div v-for="hobby in SC" :key="hobby.id">
                    <div v-if="hobby.id == hobbyId">
                      <h2 style="margin-bottom: 0px"> {{hobby.title}} </h2>
                    <div v-for="timing in apiResponse" :value="timing" :key="(timing.title, timing.start, timing.end)">
                      <div v-if="timing.title == hobby.title">
                        <input type="checkbox" :value="timing" v-model="apiHobbyTime" /> 
                        <label v-if="hobby.recurrence == null"> From {{moment(timing.start).format('MMMM Do YYYY, h:mm:ss a')}} to {{moment(timing.end).format('MMMM Do YYYY, h:mm:ss a')}}</label>
                        <label v-if="hobby.recurrence != null"> From {{moment(timing.start).format('h:mm a')}} to {{moment(timing.end).format('h:mm a')}}</label>
                      </div>

                    </div>
                    </div>
                  </div>
                      </form>
                </div>
                <button style="padding: 7px; margin-top: 10px;background-color: green;color: white;font-size: 1.1rem;" :disabled="!isApiDone" @click.self="SaveHobbies">Save</button>
            </div>
        </div>
    </div>
  </transition>


</template>

<script>
import VueCal from 'vue-cal'
import 'vue-cal/dist/vuecal.css'
import AddCalendarEvent from '../components/AddCalendarEvent.vue'
import {MODEL_API} from '../api-common'
import HobbyList from '../components/HobbyList.vue'
import moment from 'moment'

export default {
  name: 'Schedule',
  components: {
    VueCal,
    AddCalendarEvent,
    HobbyList
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
      lastid: null,
      lastHobbyid: null,
      editHobbyModalBool: false,
      hobbyName: null,
      hobbyRanges: {
          start: '',
          end: ''
      },
      hobbyHours: null,
      hobbyMinutes: null,
      hobbyId: null,
      hobbyRecurStartDate: '',
      errors: [],
      removeEventModalBool: false,
      eventName: '',
      eventTimings: {
          start: new Date(),
          end: new Date()
      },
      eventRecurrance: 'Just Once',
      recurranceTypes: ['Just Once', 'Daily', 'Weekly'],
      daysOfTheWeek: ['M', 'T', 'W', 'T', 'F', 'S', 'S'],
      selectedDayOfTheWeek: [],
      recurType: null,
      dailyOccurNum: null,
      recurEndDate: null,
      numOfOccurance: null,
      eventStartDate: '',
      eventStartTime: '',
      eventEndDate: '',
      eventEndTime: '',
      eventId: null,
      isApiDone: false,
      openApiModal: false,
      apiHobbyTime: [],
      apiResponse: []
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
    },
    addMediaTask(mediaTask){
      console.log("ADDING TASK");
      if(this.listOfEvents.length > 0){
      mediaTask.m_id =  this.lastid + 1;
      this.lastid += 1;
      }else{
        mediaTask.m_id = 0;
        this.lastid = 0;
      }
      console.log(mediaTask);
      this.listOfEvents.push(mediaTask);
      this.addToDrawingList(mediaTask);
      return mediaTask.m_id;
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
                source: newEvent.source,
                class: newEvent.class,
                recurrence: newEvent.recurrence,
                    deletable: false,
                    resizable: false,
                    draggable: false,
                m_id: newEvent.m_id,
                content: '<i class="fa-solid fa-arrows-rotate"></i>'
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
                source: newEvent.source,
                class: newEvent.class,
                recurrence: newEvent.recurrence,
                    deletable: false,
                    resizable: false,
                    draggable: false,
                m_id: newEvent.m_id,
                content: '<i class="fa-solid fa-arrows-rotate"></i>'
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
                source: newEvent.source,
                class: newEvent.class,
                recurrence: newEvent.recurrence,
                    deletable: false,
                    resizable: false,
                    draggable: false,
                m_id: newEvent.m_id,
                content: '<i class="fa-solid fa-arrows-rotate"></i>'
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
                  source: newEvent.source,
                  class: 'hc',
                  recurrence: newEvent.recurrence,
                      deletable: false,
                      resizable: false,
                      draggable: false,
                  m_id: newEvent.m_id,
                  content: '<i class="fa-solid fa-arrows-rotate"></i>'
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
                  source: newEvent.source,
                  class: 'hc',
                  recurrence: newEvent.recurrence,
                      deletable: false,
                      resizable: false,
                      draggable: false,
                  m_id: newEvent.m_id,
                  content: '<i class="fa-solid fa-arrows-rotate"></i>'
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
                    source: newEvent.source,
                    class: 'hc',
                    recurrence: newEvent.recurrence,
                        deletable: false,
                        resizable: false,
                        draggable: false,
                    m_id: newEvent.m_id,
                    content: '<i class="fa-solid fa-arrows-rotate"></i>'
                    };
                    this.drawingList.push(drawEvent);
                    count++;
                  }
                }
                curr_start.setDate(curr_start.getDate() + newEvent.recurrence.frequency * 7);
            }
          }
        }
      }else{
      this.drawingList.push(newEvent);
      }
    },
    updateDrawingList(){

    },
    addSC(newSC){
      if(this.SC.length > 0){
      newSC.id = this.lastHobbyid+ 1;
      this.lastHobbyid = newSC.id
      }else{
        newSC.id = 0;
        this.lastHobbyid = 0;
      }
      this.SC.push(newSC)
      console.log(this.SC)
    },
    printCurrentTask(){
      console.log(this.eventStartTime);
    },
    printListOfItems(){
      console.log(this.listOfEvents);
      console.log(this.drawingList);
    },
    getMonday(d) {
      d = new Date(d);
      var day = d.getDay(),
      diff = d.getDate() - day + (day == 0 ? -6:1); // adjust when day is sunday
      return new Date(d.setDate(diff));
    },
    async apiTest(hobby_id){
      console.log(this.SC);
      this.hobbyId = hobby_id
      this.openApiModal = true;
      this.errors = [];
      if(this.SC.length < 1){
        this.errors.push("Hobbies are requried");
      }
      let data = null;
      let hobbyIndex = this.SC.findIndex(hobby => hobby.id == hobby_id);
        let monday = new Date(this.SC[hobbyIndex].selectedDate.start)
        monday.setHours(0, 0, 0);
        let EndOfCycle = new Date(this.SC[hobbyIndex].selectedDate.end);
        EndOfCycle.setHours(23, 59, 59);
        let temp = this.drawingList.filter(event => (monday <= event.start && event.end <= EndOfCycle))
        if(temp.length < 1){
          this.errors.push("Events are required.")
        }
        let hobby = {
          title: this.SC[hobbyIndex].title,
          selectedDate: this.SC[hobbyIndex].selectedDate.start,
          length: this.SC[hobbyIndex].length,
          class: this.SC[hobbyIndex].class,
          source: this.SC[hobbyIndex].source
        }
        data={
          //monday: this.getMonday(new Date()),
          monday: monday,
          EndOfCycle: EndOfCycle,
          listOfEvents: temp,
          newEvent: [hobby]
        };
        console.log(data);

      if(this.errors.length > 0){
        return true;
      }

      await MODEL_API.post("model", data)
        .then((res) => {
          //var newSchedule = JSON.parse(res.config.data);
          console.log(`Response from schedule api: `);
          console.dir(res.data);
          for (let i = 0; i < res.data.length; i++){
            res.data[i].start =  new Date(res.data[i].start);
            res.data[i].end =  new Date(res.data[i].end);
            res.data[i].start.setMonth(res.data[i].start.getMonth());
            res.data[i].end.setMonth(res.data[i].end.getMonth());
            res.data[i].recurrence = this.SC[hobbyIndex].recurrence;
            res.data[i].recurStartDate = this.SC[hobbyIndex].recurStartDate;
          }
          this.apiResponse = res.data;
          this.isApiDone = true;
        })
        .catch( (error) => {
          console.log(error)
          this.isApiDone = false;
          this.errors.push("Network Error: Contact The Website Administrator For More Information");
        })

      if(this.errors.length > 0){
        return true;
      }
    },
    openHobbyEditModal(hobbyId){
      let item = this.SC.find(hobby => hobby.id == hobbyId);
      console.log(item);
      this.hobbyId = hobbyId;
      this.hobbyName = item.title;
      this.hobbyRanges = {
        start: this.getDateInFormat(item.selectedDate.start),
        end: this.getDateInFormat(item.selectedDate.end),
      };
      this.hobbyHours = Math.floor(item.length / 60.0);
      this.hobbyMinutes = item.length % 60;
      this.hobbyRecurStartDate = this.getDateInFormat(item.recurStartDate);
      if(item.recurrence){
      this.eventRecurrance = item.recurrence.pattern;
      this.recurType = item.recurrence.recurranceTypes;
      this.dailyOccurNum = item.recurrence.frequency;
      this.recurEndDate = this.getDateInFormat(item.recurrence.endDate)
      this.numOfOccurance = item.recurrence.numOfOccurance;
      this.selectedDayOfTheWeek = item.recurrence.selectedDayOfTheWeek;
      }
      this.editHobbyModalBool = true;
    },
    deleteHobby(hobbyId){
      this.SC = this.SC.filter(event => event.id != hobbyId);
    },
    getDateInFormat(date){
      var d = new Date(date),
      month = '' + (d.getMonth() + 1),
      day = '' + d.getDate(),
      year = d.getFullYear()
      if (month.length < 2) 
          month = '0' + month;
      if (day.length < 2) 
          day = '0' + day
      return [year, month, day].join('-');
    },
    editHobby(e){
      e.preventDefault()
      this.errors = [];
      if(!this.hobbyName){
          this.errors.push("Name is required")
      }
      if(this.hobbyHours == null && this.hobbyMinutes == null){
          this.errors.push("Hobby length is required.")
      }

            if(this.eventRecurrance == 'Daily'){
                if(this.hobbyRecurStartDate == null || this.hobyyRecurStartDate == ''){
                    this.errors.push("Fill the Start Date")
                }
                if(this.dailyOccurNum == '' || this.dailyOccurNum == null){
                    this.errors.push("Fill the repeats every x day section")
                }

                if(this.recurType == null){
                    this.errors.push("Select when the event ends")  
                }else{
                    if(this.recurType == 'endDate' && this.recurEndDate == null){
                        this.errors.push("Ends on date is required")            
                    }
                    if(this.recurType == 'OnOcuurance' && this.numOfOccurance == null){
                        this.errors.push("The number of occurances is required")
                    }
                }

                if(this.hobbyRanges.start === '' || this.hobbyRanges.end === ''){
                    this.errors.push("Fill in the dates which the hobby's timing should based on")
                }
            }

            if(this.eventRecurrance == 'Weekly'){
                if(this.hobbyRecurStartDate == null || this.hobyyRecurStartDate == ''){
                    this.errors.push("Fill the Start Date")
                }
                if(this.dailyOccurNum == '' || this.dailyOccurNum == null){
                    this.errors.push("Fill the repeats every x week section")
                }
                if(this.selectedDayOfTheWeek.length == 0){
                    this.errors.push("Select the days of the week which the event will occur")
                }
                if(this.recurType == null){
                    this.errors.push("Select when the event ends");
                }else{
                if(this.dailyOccurNum == null){
                    this.errors.push("Occurance number is required")            
                }
                if(this.recurType == 'endDate' && this.recurEndDate == null){
                    this.errors.push("Ends on date is required")            
                }
                if(this.recurType == 'OnOcuurance' && this.numOfOccurance == null){
                    this.errors.push("The number of occurances is required")
                }
                }

                if(this.hobbyRanges.start === '' || this.hobbyRanges.end === ''){
                    this.errors.push("Fill in the dates which the hobby's timing should based on")
                }
            }

            if(this.eventRecurrance == 'Just Once'){
                if(this.hobbyRanges.start === '' || this.hobbyRanges.end === ''){
                    this.errors.push("Fill in the dates which the hobby should happen")
                }
            }

      if(this.errors.length > 0){
          return true;
      }
      
      if(this.hobbyHours == ''){
          this.hobbyHours = '0'
      }

      if(this.hobbyMinutes == ''){
          this.hobbyMinutes = '0'
      }
      let itemIndex = this.SC.findIndex(hobby => hobby.id == this.hobbyId);

      let startArr = this.hobbyRanges.start.split('-');
      let endArr = this.hobbyRanges.end.split('-');
      let startDate = new Date(startArr[0], startArr[1] - 1, startArr[2]);
      let endDate = new Date(endArr[0], endArr[1] - 1, endArr[2]);
      let recurStartDate = null;
      if(this.hobbyRecurStartDate){
        let recurStartArr = this.hobbyRecurStartDate.split('-');
        recurStartDate = new Date(recurStartArr[0], recurStartArr[1]-1, recurStartArr[2]);
      }
      console.log(recurStartDate);
      startDate.setHours(0, 0, 0, 0);
      endDate.setHours(23, 59, 59, 59);

      this.SC[itemIndex].title = this.hobbyName;
      this.SC[itemIndex].length = 60 * parseInt(this.hobbyHours) + parseInt(this.hobbyMinutes)
      this.SC[itemIndex].selectedDate = {
        start: startDate,
        end: endDate
      }
      this.SC[itemIndex].recurStartDate = recurStartDate;
      if(this.recurEndDate){
        endDate = new Date();
        let split = this.recurEndDate.split('-')
        endDate.setYear(parseInt(split[0]));
        endDate.setMonth(parseInt(split[1])-1);
        endDate.setDate(parseInt(split[2]));
      }
      if(this.eventRecurrance == 'Daily'){
        this.SC[itemIndex].recurrence = {
            pattern: this.eventRecurrance,
            recurranceType: this.recurType,
            frequency: parseInt(this.dailyOccurNum),
            endDate: endDate,
            numOfOccurance: parseInt(this.numOfOccurance)
        }
      }
      if(this.eventRecurrance == 'Weekly'){
            this.SC[itemIndex].recurrence = {
                pattern: this.eventRecurrance,
                selectedDayOfTheWeek: this.selectedDayOfTheWeek,
                recurranceType: this.recurType,
                frequency: parseInt(this.dailyOccurNum),
                endDate: endDate,
                numOfOccurance: parseInt(this.numOfOccurance)
            } 
      }
      this.editHobbyModalBool = false;
    },
    openEditEvent(event){
      this.errors = [];
      this.eventName = event.title;
      this.eventTimings.start = event.start;
      this.eventTimings.end = event.end;
      this.eventId = event.m_id;
      if(event.recurrence == null){
        this.eventRecurrance = 'Just Once'
      }else{
        this.eventRecurrance = event.recurrence.pattern;
        this.recurType = event.recurrence.recurranceType;
        this.dailyOccurNum = event.recurrence.frequency;
        this.endDate = event.recurrence.endDate;
        this.numOfOccurance = event.recurrence.numOfOccurance;
        if(event.recurrence.pattern == 'Weekly'){
          this.selectedDayOfTheWeek = event.recurrence.selectedDayOfTheWeek;
        }
      }
      this.eventStartDate = this.getDateInFormat(event.start);
      this.eventEndDate = this.getDateInFormat(event.end);
      let hours = '';
      let minute = '';
      if(event.start.getHours() < 10){
        hours = '0' + event.start.getHours();
      }else{
        hours = event.start.getHours();
      }
      if(event.start.getMinutes() < 10){
        minute = '0' + event.start.getMinutes()
      }else{
        minute = event.start.getMinutes()
      }
      this.eventStartTime = `${hours}:${minute}`;
      if(event.end.getHours() < 10){
        hours = '0' + event.end.getHours();
      }else{
        hours = event.end.getHours();
      }
      if(event.end.getMinutes() < 10){
        minute = '0' + event.end.getMinutes()
      }else{
        minute = event.end.getMinutes()
      }
      this.eventEndTime = `${hours}:${minute}`;
      this.removeEventModalBool = !this.removeEventModalBool;
    },
    removeEvent(){
      let itemIndex = this.listOfEvents.find(event => event.m_id == this.eventId).m_id;
      this.drawingList = this.drawingList.filter(item => item.m_id != itemIndex);
      this.listOfEvents = this.listOfEvents.filter(item => item.m_id != this.eventId);
      this.removeEventModalBool = false;
    },
    editEvent(){
      let newEvent = this.listOfEvents.find(event => event.m_id == this.eventId);
      newEvent.title = this.eventName;
      newEvent.start = this.textToTime(this.eventStartDate + 'T' + this.eventStartTime)
      newEvent.end = this.textToTime(this.eventStartDate + 'T' + this.eventEndTime)
      let endDate = null;
      if(this.recurEndDate){
        endDate = new Date();
        let split = this.recurEndDate.split('-')
        let timeSplit = this.eventEndTime.split(':')
        console.log(timeSplit)
        endDate.setYear(parseInt(split[0]));
        endDate.setMonth(parseInt(split[1])-1);
        endDate.setDate(parseInt(split[2]));
        endDate.setHours(timeSplit[0]);
        endDate.setMinutes(timeSplit[1]);
        endDate.setSeconds(0);
      }
      if(this.eventRecurrance == 'Daily'){
        newEvent.recurrence = {
            pattern: this.eventRecurrance,
            recurranceType: this.recurType,
            frequency: parseInt(this.dailyOccurNum),
            endDate: endDate,
            numOfOccurance: parseInt(this.numOfOccurance)
        }
      }
      if(this.eventRecurrance == 'Weekly'){
            newEvent.recurrence = {
                pattern: this.eventRecurrance,
                selectedDayOfTheWeek: this.selectedDayOfTheWeek,
                recurranceType: this.recurType,
                frequency: parseInt(this.dailyOccurNum),
                endDate: endDate,
                numOfOccurance: parseInt(this.numOfOccurance)
            } 
      }
      if(this.eventRecurrance == 'Just Once'){
        newEvent.recurrence = null
      }
      console.log(newEvent);
      this.drawingList = this.drawingList.filter(item => item.m_id != newEvent.m_id);
      this.listOfEvents = this.listOfEvents.filter(item => item.m_id != this.eventId);
      this.eventId = this.addMediaTask(newEvent);
    },
    textToTime(dateString){
        let regex = /(\d{4})[-](\d{2})[-](\d{2})[T](\d{2})[:](\d{2})/;
        let arr = dateString.match(new RegExp(regex));
        if(arr == null){
            return
        }
        let date = new Date();
        date.setFullYear(arr[1], arr[2]-1, arr[3]);
        date.setHours(arr[4], arr[5]);
        return date;
    },
    moment: function (date) {
      return moment(date);
    },
    SaveHobbies(e){
      e.preventDefault();
      console.log(this.apiHobbyTime);
      for(let i = 0; i < this.apiHobbyTime.length; i++){
        console.log(this.apiHobbyTime)
        if(this.apiHobbyTime[i].recurrence){
          if(this.apiHobbyTime[i].recurrence.pattern == 'Daily'){
            this.apiHobbyTime[i].start.setFullYear(this.apiHobbyTime[i].recurStartDate.getFullYear(), this.apiHobbyTime[i].recurStartDate.getMonth(), this.apiHobbyTime[i].recurStartDate.getDate())
            this.apiHobbyTime[i].end.setFullYear(this.apiHobbyTime[i].recurStartDate.getFullYear(), this.apiHobbyTime[i].recurStartDate.getMonth(), this.apiHobbyTime[i].recurStartDate.getDate())
            this.apiHobbyTime[i].recurrence.endDate.setHours(this.apiHobbyTime[i].end.getHours())
            this.apiHobbyTime[i].recurrence.endDate.setMinutes(this.apiHobbyTime[i].end.getSeconds())
            this.apiHobbyTime[i].recurrence.endDate.setSeconds(this.apiHobbyTime[i].end.getSeconds())
          }
        }
        this.addMediaTask(this.apiHobbyTime[i]);
      }
      this.apiHobbyTime = [];
      this.SC = this.SC.filter(hobby => hobby.id != this.hobbyId);
      this.openApiModal = false;
    },
    addToDaysOfWeek(day){
      if(!this.selectedDayOfTheWeek.includes(day)){
          this.selectedDayOfTheWeek.push(day)
      }else{
          this.selectedDayOfTheWeek = this.selectedDayOfTheWeek.filter(element => element != day);
      }
    },
    printList(){
      console.log(this.SC);
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

.vuecal__time-cell-line.hours:before {border-color: #42b983;}


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
</style>
