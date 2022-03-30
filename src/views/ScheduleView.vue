
<template>
    <div v-if="$cookies.get('loginSource') != null" style="justify-content: center;display: flex;">
        <button @click="$router.push('logout')">Logout</button>
    </div>
    <br />
    <div class="fullHeight" style="display: flex;justify-content: center;gap: 1%;">
      <div class="smallCalendar">
        <AddCalendarEvent @add-cal-event="addCalendarEvent" v-bind:listOfEvents="drawingList" v-bind:selectedDate="new Date()"  @pull-outlook-event="addOutlookTask" @add-hobby="addHobby" @pull-google-event="addGoogleTask"/> <br />
        <vue-cal ref="smallCalendar" events-count-on-year-view @view-change="updateCalenderViews('1', $event)" xsmall @cell-focus="selectedDate = $event" :selected-date="selectedDate" hide-view-selector :events="drawingList" active-view="month" :disable-views="['years', 'week', 'day', 'year']" class="vuecal--date-picker" />
        <br/>
        <h3>List of Submitted Hobbies</h3>
        <HobbyList v-bind:sourceData="hobbyList" @edit-hobby="openHobbyEditModal" @delete-hobby="deleteHobby" @call-api="apiTest" />
      </div>
      <div class="bigCalendar fullHeight">
        <button style="padding: 7px;margin-top: 0px;background-color: DarkSeaGreen;color: white;font-size: 1.1rem;" @click="selectedDate = new Date()">Return To Today</button>
        <vue-cal timeFormat="h:mm am" twelveHour small ref="bigCalendar" :on-event-dblclick="openEditEvent" hide-view-selector @cell-focus="selectedDate = $event" :selected-date="selectedDate"  events-on-month-view="true" :time="true" active-view="week" :disable-views="['years', 'day', 'year', 'month']" :events="drawingList"/>
      </div>
    </div>



  <transition name="modal">
    <div v-if="editHobbyModalBool">
        <div class="overlay">
            <div class="modal" style="width: 65%">
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
                            <label>What is the name of the hobby? <input type="text" placeholder="Enter Name of Hobby" v-model="hobbyName" />  </label>
                            <br>
                            <label>How is long does the hobby go for? <input type="number" min="0" v-model="hobbyHours"  oninput="this.value = this.value.replace(/[^0-9.]/g, '').replace(/(\..*?)\..*/g, '$1');"/> hr <input type="number" min="0" v-model="hobbyMinutes" oninput="this.value = this.value.replace(/[^0-9.]/g, '').replace(/(\..*?)\..*/g, '$1');"/> min </label>
                            <br>
                            <label>How often does this hobby repeat? </label>
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
                                                <label>When should this hobby start? <input type="date" v-model="hobbyRecurStartDate" /> </label>
                                            </div>
                                        </li>
                                        <li>
                                            <div style="margin-bottom: 8px;">
                                                <label>Repeat every <input name="occurDay" type="number" v-model="eventFrequency" min="0" size="5" /> day(s) </label>
                                            </div>
                                        </li>
                                        <div>
                                            <li style="margin-bottom: 8px;"><span>Ends</span></li>
                                        </div>
                                        <div v-on:click="recurType='Never'">
                                            <input type="radio" value="Never" v-model="recurType"/> This hobby never stops repeating
                                        </div>

                                        <div v-on:click="recurType='endDate'">
                                            <input type="radio" value="endDate" v-model="recurType" /> This hobby will stop repeating on <input type="date" v-model="recurEndDate" />
                                        </div>
                                        <div  v-on:click="recurType='OnOcuurance'">
                                            <label><input type="radio" value="OnOcuurance" v-model="recurType" size="5" /> This hobby will stop repeating after <input type="number" min="0" v-model="numOfOccurance" size="5" /> occurrence(s)</label>
                                        </div>
                                    </div>
                                </ol>
                                <ol v-if="eventRecurrance == 'Weekly'">
                                <div>
                                    <li>
                                        <div style="margin-bottom: 8px">
                                            <label>When should this hobby start? <input type="date" v-model="hobbyRecurStartDate" /> </label>
                                        </div>
                                    </li>
                                    <li>
                                        <div><label>Repeat every <input type="number" v-model="eventFrequency" min="0" size="5"/> week(s)</label></div>
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
                                            <input type="radio" value="Never" v-model="recurType" /> This hobby never stops repeating
                                        </div>

                                        <div v-on:click="recurType='OnDate'">
                                            <input type="radio" value="OnDate" v-model="recurType" /> This hobby will stop repeating on <input type="date" v-model="recurEndDate" />
                                        </div>

                                        <div v-on:click="recurType='OnOcuurance'">
                                            <label><input type="radio" value="OnOcuurance" v-model="recurType" /> This hobby will stop repeating after <input type="number" v-model="numOfOccurance" size="5"/> occurrence(s)</label>
                                        </div>
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
            <div class="modal" style="width: 35%">
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: flex-end;">
                    <button style="background-color: black;font-size: 16px;color: white;padding: 7px;margin-top: 10px;" @click="removeEventModalBool = false;errors=[];">Close</button>
                </div>
                <h2 style="text-align: center;">Edit Event</h2>
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: center;">
                    <div>
                        <!-- <form @submit="editEvent"> -->
                            <p v-if="errors.length > 0">
                                <b>Please correct the following error(s):</b>
                                <ul>
                                  <li v-for="error in errors" :key="error">{{ error }}</li>
                                </ul>
                            </p>

                            <label>What is the name of the event?: </label>
                            <input v-model="eventName" type="text" placeholder="Enter Title" size="15" /> <br/>
                            <label>When does an indiviual session of the event start? </label>
                            <input type="date" v-model="eventStartDate" size="30" /> <input type="time" v-model="eventStartTime" /> <br />
                            <label>When does an indiviual session of the event end? </label>
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
                                                <label>Repeat every <input name="occurDay" type="number" v-model="eventFrequency" min="0" size="5" /> day(s) </label>
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
                                        <div><label>Repeat every <input type="number" v-model="eventFrequency" min="0" size="5"/> week(s)</label></div>
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
                                            <input type="radio" value="Never" v-model="recurType" /> This event will never stop recurring
                                        </div>

                                        <div v-on:click="recurType='OnDate'">
                                            <input type="radio" value="OnDate" v-model="recurType" /> This event will stop recurring on: <input type="date" v-model="recurEndDate" />
                                        </div>

                                        <div v-on:click="recurType='OnOcuurance'">
                                            <label><input type="radio" value="OnOcuurance" v-model="recurType" /> This event will end after: <input type="number" v-model="numOfOccurance" size="5"/> occurrence(s)</label>
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
                  <div v-for="hobby in hobbyList" :key="hobby.id">
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
import { RRule, rrulestr } from 'rrule'

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
      //listOfEvents: List of calendar events
      listOfEvents: [],
      //drawingList: List of events that will be drawn to the screen
      drawingList: [],
      //hobbyList: List of hoobies
      hobbyList: [],
      //selectedDate: The selected date on the calendar
      selectedDate: null,
      //lastid: Id of the last created event
      lastid: null,
      //lastHobbyid: Id of the last crreated hobby
      lastHobbyid: null,
      //editHobbyModalBool: Boolean for if the edit Hobby Modal is open
      editHobbyModalBool: false,
      //hobbyName: Name of hobby
      hobbyName: null,
      //hobbyRanges: Date Ranges for hobbies
      hobbyRanges: {
          start: '',
          end: ''
      },
      //hobbyHours: Number of hours for a hobby
      hobbyHours: null,
      //hobbyMinutes: Number of minutes for a hobby
      hobbyMinutes: null,
      //hobbyId: Id of hobby
      hobbyId: null,
      //hobbyRecurStartDate: When the hobby starts recurring
      hobbyRecurStartDate: '',
      //errors: List of errors
      errors: [],
      //removeEventModalBool: Boolean for if remove Event Modal is open
      removeEventModalBool: false,
      //eventName: Name of event
      eventName: '',
      //eventTimings: Object of the start and end times of an event
      eventTimings: {
          start: new Date(),
          end: new Date()
      },
      //eventRecurrance: Object that represents how often an event recurrs
      eventRecurrance: 'Just Once',
      //recurranceTypes: List of event recurrances
      recurranceTypes: ['Just Once', 'Daily', 'Weekly'],
      //daysOfTheWeek: List of days of the week
      daysOfTheWeek: ['M', 'T', 'W', 'T', 'F', 'S', 'S'],
      //selectedDayOfTheWeek: List of selected week days
      selectedDayOfTheWeek: [],
      //recurType: Object that represents recurrance type
      recurType: null,
      //eventFrequency: Scalar variable that represents how often an event occurs (Every x days)
      eventFrequency: null,
      //recurEndDate: When the event stops recurring
      recurEndDate: null,
      //numOfOccurance: Number of Occurances for an event
      numOfOccurance: null,
      //eventStartDate: Object that represents an event's start date
      eventStartDate: '',
      //eventStartTime: Object that represents an event's start time
      eventStartTime: '',
      //eventEndDate: Object that represents an event's end date
      eventEndDate: '',
      //eventEndTime: Object that represents an event's end time
      eventEndTime: '',
      //eventId: Id of event
      eventId: null,
      //isApiDone: Boolean for if the api has responded with the schedule
      isApiDone: false,
      //openApiModal: Boolean for if the Get Placment Modal is open
      openApiModal: false,
      //apiHobbyTime: Stores the times of the new schedule
      apiHobbyTime: [],
      //apiResponse: Stores the response from the api
      apiResponse: []
    }
  },
  methods: {
    /**
     * Update large calendar if the small calendar changes
     * Pre-condition: Small calendar changes day
     * Post-condition: Big Calendar changes to the smaller calendar's date
     * Parameters:
     *  call_name: Integer that represents what calendar will be changed. A value of 1 indicates that the bigger Calendar on the right will be changed.
     *  event: Javascript event object that represents the event where the view changes
     */
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
    /**
     * Adds Event to both calendars
     * Pre-condition: None
     * Post-condition: The new event is added to both calendars. The function returns the id of the new event
     * Parameter:
     *  newEvent - Javascript object that represents the event that will be added
     */
    addCalendarEvent(newEvent){
      console.log("ADDING TASK");
      if(this.listOfEvents.length > 0){
      newEvent.m_id =  this.lastid + 1;
      this.lastid += 1;
      }else{
        newEvent.m_id = 0;
        this.lastid = 0;
      }
      console.log(newEvent);
      this.listOfEvents.push(newEvent);
      this.addToDrawingList(newEvent);
      return newEvent.m_id;
    },
    /**
     * Adds event from Outlook
     * Pre-condition: Data is retrived from outlook
     * Post-condition: The Outlook event is added to both calendars
     * Parameters:
     *  cal_data - Javascript array that represents the data recieved from Outlook 
     */
    addOutlookTask(cal_data){
      this.listOfEvents = this.listOfEvents.filter(event => event.source != 'O');
      this.drawingList = this.drawingList.filter(event => event.source != 'O');      
      for(let i = 0; i < cal_data.length; i++){
        let newEvent = {title: cal_data[i].subject, 
        start: this.returnDateObject(cal_data[i].start.dateTime),
        end: this.returnDateObject(cal_data[i].end.dateTime),
        source: "O",
        class: 'hc'}
        //Add code to create recurrence object based on api data
        if(cal_data[i].recurrence == null){
          newEvent.recurrence = null;
        }else{
          if(cal_data[i].recurrence.pattern.type == "daily"){
            newEvent.recurrence = {
              pattern: 'Daily',
              frequency: cal_data[i].recurrence.pattern.interval,
            }
            
            if(cal_data[i].recurrence.range.type == "endDate"){
              let regex = /(\d{4})[-](\d{2})[-](\d{2})/;
              let arr = cal_data[i].recurrence.range.endDate.match(new RegExp(regex));
              let endDateO = new Date();
              endDateO.setFullYear(arr[1], arr[2]-1, arr[3]);
              endDateO.setHours(0, 0);
              newEvent.recurrence.recurranceType = 'endDate'
              newEvent.recurrence.endDate = endDateO
            }

            if(cal_data[i].recurrence.range.type == "noEnd"){
              newEvent.recurrence.recurranceType = 'Never'
            }
          }

          if(cal_data[i].recurrence.pattern.type == "weekly"){
            newEvent.recurrence = {
              pattern: 'Weekly',
              frequency: cal_data[i].recurrence.pattern.interval,
              selectedDayOfTheWeek: cal_data[i].recurrence.pattern.daysOfWeek
            }

            if(cal_data[i].recurrence.range.type == "endDate"){
              let regex = /(\d{4})[-](\d{2})[-](\d{2})/;
              let arr = cal_data[i].recurrence.range.endDate.match(new RegExp(regex));
              let endDateO = new Date();
              endDateO.setFullYear(arr[1], arr[2]-1, arr[3]);
              endDateO.setHours(0, 0);
              newEvent.recurrence.recurranceType = 'OnDate';
              newEvent.recurrence.endDate = endDateO;
            }

            if(cal_data[i].recurrence.range.type == "noEnd"){
              newEvent.recurrence.recurranceType = 'Never'
            }
          }
        }
        this.listOfEvents.push(newEvent);
        this.addToDrawingList(newEvent);
      }
    },
    /**
     * Adds event from Google
     * Pre-condition: Data is retrived from Google
     * Post-condition: The Google event is added to both calendars
     * Parameters:
     *  cal_data - Javascript array that represents the data recieved from Google 
     */
    addGoogleTask(cal_data){
      console.log("GOOGLE TIME")
      this.listOfEvents = this.listOfEvents.filter(event => event.source != 'G');
      this.drawingList = this.drawingList.filter(event => event.source != 'G');
      for(let i = 0; i < cal_data.length; i++){
        let newEvent = {
        title: cal_data[i].summary,
        start: this.returnDateObject(cal_data[i].start.dateTime),
        end: this.returnDateObject(cal_data[i].end.dateTime),
        source: "G",
        class: 'hc',
        }
        if(cal_data[i].recurrence == null){
          newEvent.recurrence = null;
        }else{
          console.log(cal_data[i].recurrence[0])
          let recurOptions = rrulestr(cal_data[i].recurrence[0])
          console.log(recurOptions)
          if(recurOptions.options.freq == RRule.DAILY){
            newEvent.recurrence = {
              pattern: 'Daily',
              frequency: recurOptions.options.interval
            }
          }

          if(recurOptions.options.freq == RRule.WEEKLY){
            let selectedDays = recurOptions.options.byweekday;
            selectedDays.sort(function(a,b){
              return a - b;
            });

            for (let i =0; i < selectedDays.length; i++){
              selectedDays[i] = this.googleConvertNumToDay(selectedDays[i]);
            }

            newEvent.recurrence = {
              pattern: 'Weekly',
              frequency: recurOptions.options.interval,
              selectedDayOfTheWeek: selectedDays
            }
          }

            if(recurOptions.options.until != null){
              newEvent.recurrence.endDate = recurOptions.options.until
              newEvent.recurrence.recurranceType = 'endDate'
            }

            if(recurOptions.options.until == null){
              newEvent.recurrence.recurranceType = 'Never'
            }

            if(recurOptions.options.count != null){
              newEvent.recurrence.recurranceType = 'OnOcuurance'
              newEvent.recurrence.numOfOccurance = recurOptions.options.count
            }          
        }
        this.listOfEvents.push(newEvent);
        this.addToDrawingList(newEvent);
      }

    },
    /**
     * Converts Date object to date string
     * Pre-condition: None
     * Post-condition: Date object is returned
     * Parameter:
     *  dateString - String variables that represents a date as a string
     */
    returnDateObject(dateString){
      var b = dateString.split(/\D+/);
      return new Date(Date.UTC(b[0], --b[1], b[2], b[3], b[4], b[5], b[6]));
    },
    /**
     * Adds event to drawing list
     * Pre-condition: There is a new event
     * Post-condition: Adds event to drawing list (Thus both calendars are updated)
     * Parameter:
     *  newEvent - Javascript object that represents an event that will be added
     */
    addToDrawingList(newEvent){
      if(newEvent.recurrence){ 
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
                    class: newEvent.class,
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
    /**
     * Adds hobby to the hobbyList variable
     * Pre-condition: There is a new hobby
     * Post-condiition: Adds hobby to hobbyList
     * Parameters:
     *  newHobby - Javascript object that represents a hobby that will be added
     */
    addHobby(newHobby){
      if(this.hobbyList.length > 0){
      newHobby.id = this.lastHobbyid+ 1;
      this.lastHobbyid = newHobby.id
      }else{
        newHobby.id = 0;
        this.lastHobbyid = 0;
      }
      this.hobbyList.push(newHobby)
      console.log(this.hobbyList)
    },
    /**
     * Gets monday of the week in a given day
     * Pre-condition: None
     * Post-condition: Date object is returned
     * Parameter:
     *  d - Date object that represents the day to find the nearst monday 
     */
    getMonday(d) {
      d = new Date(d);
      var day = d.getDay(),
      diff = d.getDate() - day + (day == 0 ? -6:1); // adjust when day is sunday
      return new Date(d.setDate(diff));
    },
    /**
     * Sends data to the api backend and recieves the response
     * Pre-condition: There are events in the calendar and hobbies
     * Post-condition: Hobby timings are returned and stored in apiResponse
     * Parameter:
     *  hobby_id - Integer that represents the hobby's id
     */
    async apiTest(hobby_id){
      console.log(this.hobbyList);
      this.hobbyId = hobby_id
      this.openApiModal = true;
      this.errors = [];
      if(this.hobbyList.length < 1){
        this.errors.push("Hobbies are requried");
      }
      let data = null;
      let temp2 = null;
      let hobbyIndex = this.hobbyList.findIndex(hobby => hobby.id == hobby_id);
        let monday = new Date(this.hobbyList[hobbyIndex].selectedDate.start)
        monday.setHours(0, 0, 0);
        let EndOfCycle = new Date(this.hobbyList[hobbyIndex].selectedDate.end);
        EndOfCycle.setHours(23, 59, 59);
        let temp = this.drawingList.filter(event => (monday <= event.start && event.end <= EndOfCycle)).map((x) => x);
        if(temp.length < 1){
          this.errors.push("Events are required.")
        }
        if(this.hobbyList[hobbyIndex].recurrence){
          console.log("Bong")
          if(this.hobbyList[hobbyIndex].recurrence.pattern == "Weekly"){
            let tempDayOfWeek = this.hobbyList[hobbyIndex].recurrence.selectedDayOfTheWeek.map((x) => x);
            for (let i = 0; i < tempDayOfWeek.length; i++){
              tempDayOfWeek[i] = this.convertDayToNum(tempDayOfWeek[i]);
            }
            tempDayOfWeek.sort(function(a,b){
              return a - b;
            });
            temp2 = temp.map(a => Object.assign({}, a));
            console.log(temp2);
            temp2 = temp2.filter(event => tempDayOfWeek.indexOf(event.start.getDay()) != -1);
            for (let i = 0; i < temp2.length; i++){
              if(temp2[i].start.getDay() != tempDayOfWeek[0]){
                let diff = temp2[i].start.getDay() - tempDayOfWeek[0] - tempDayOfWeek.indexOf(temp2[i].start.getDay());
                if(diff > 0){
                  temp2[i].start.setDate(temp2[i].start.getDate() - diff)
                  temp2[i].end.setDate(temp2[i].end.getDate() - diff)
                }
              }
            }
            temp = temp2;
            console.log(tempDayOfWeek);
            monday.setDate(monday.getDate() + (tempDayOfWeek[0] - 1))
                        console.log(EndOfCycle);
            EndOfCycle.setFullYear(monday.getFullYear(), monday.getMonth(), monday.getDate())
            EndOfCycle.setDate(monday.getDate() + tempDayOfWeek.length - 1)
            console.log(EndOfCycle);
          }
        }
        let hobby = {
          title: this.hobbyList[hobbyIndex].title,
          selectedDate: this.hobbyList[hobbyIndex].selectedDate.start,
          length: this.hobbyList[hobbyIndex].length,
          class: 'sc',
          source: this.hobbyList[hobbyIndex].source
        }
        if(this.hobbyList[hobbyIndex].recurrence){
          hobby.recurType = 'R'
        }else{
          hobby.recurType = 'J'
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
            res.data[i].recurrence = this.hobbyList[hobbyIndex].recurrence;
            res.data[i].recurStartDate = this.hobbyList[hobbyIndex].recurStartDate;
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
    /**
     * Opens the Hobby Edit Modal
     * Pre-condition: There is a hobby to edit
     * Post-condition: The Edit Hobby Modal is opened with the hobby's data displayed
     * Parameter:
     *  hobbyId - Integer that represents the hobby's id
     */
    openHobbyEditModal(hobbyId){
      let item = this.hobbyList.find(hobby => hobby.id == hobbyId);
      console.log(item);
      this.hobbyId = hobbyId;
      this.hobbyName = item.title;
      this.hobbyRanges = {
        start: this.getDateInFormat(item.selectedDate.start),
        end: this.getDateInFormat(item.selectedDate.end),
      };
      this.hobbyHours = Math.floor(item.length / 60.0);
      this.hobbyMinutes = item.length % 60;
      if(item.recurStartDate){
      this.hobbyRecurStartDate = this.getDateInFormat(item.recurStartDate);
      }else{
      this.hobbyRecurStartDate = '';
      }
      if(item.recurrence){
      this.eventRecurrance = item.recurrence.pattern;
      this.recurType = item.recurrence.recurranceType;
      this.eventFrequency = item.recurrence.frequency;
      this.recurEndDate = this.getDateInFormat(item.recurrence.endDate)
      this.numOfOccurance = item.recurrence.numOfOccurance;
      if(item.recurrence.selectedDayOfTheWeek){
        this.selectedDayOfTheWeek = item.recurrence.selectedDayOfTheWeek;
      }else{
        this.selectedDayOfTheWeek = [];
      }
      }
      this.editHobbyModalBool = true;
    },
    /**
     * Deletes a selected hobby
     * Pre-condition: There is a hobby to delete
     * Post-condition: Selected hobby is deleted from hobbyList
     * Parameter:
     *  hobbyId - Integer that represents the hobby's id
     */
    deleteHobby(hobbyId){
      this.hobbyList = this.hobbyList.filter(event => event.id != hobbyId);
    },
    /**
     * Converts date into a date string created in a specific format (dd-mm-yyy)
     * Pre-condition: There is a date to convert
     * Post-condition: Date is converted to date string
     * Parameter:
     *  date - Date object used to create date string
     */
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
    /**
     * Changes hobby data
     * Pre-condition: There is new data to update in a selected hobby
     * Post-condition: The selected hobby now has the new data
     * Parameter:
     *  e - Javascript event that represents the form submission
     */
    editHobby(e){
      e.preventDefault()
      this.errors = [];
      if(!this.hobbyName){
          this.errors.push("Name is required")
      }
      if(this.hobbyHours == '0' && this.hobbyMinutes == '0'){
          this.errors.push("Hobby length is required.")
      }

      if(this.eventRecurrance == 'Just Once'){
          if(this.hobbyRanges.start === '' || this.hobbyRanges.end === ''){
              this.errors.push("Fill in the dates which the hobby should happen")
          }
          
          let numOfSecondsInAWeek = 2 * 604800 * 1000
          if(this.textToTime(this.hobbyRanges.end + 'T23:59') - this.textToTime(this.hobbyRanges.start + 'T00:00') > numOfSecondsInAWeek){
              this.errors.push("Choose a smaller window to place a hobby in")
          }
      }else{
          if(!this.hobbyRecurStartDate){
              this.errors.push("Fill the Start Date")
          }

          if(this.eventRecurrance == 'Daily'){
              if(this.eventFrequency == '' || this.eventFrequency == null || isNaN(this.eventFrequency) ){
                  this.errors.push('Fill the "Repeat every x day" section')
              }   
          }

          if(this.eventRecurrance == 'Weekly'){
              if(this.eventFrequency == '' || this.eventFrequency == null || isNaN(this.eventFrequency)){
                  this.errors.push('Fill the "Repeat every x week" section')
              }  

              if(this.selectedDayOfTheWeek.length == 0){
                  this.errors.push("Select the days of the week which the event will occur")
              }
          }

          if(this.recurType == null){
              this.errors.push("Select how the hobby recurrance ends")
          }else{                    
              if(this.recurType == 'endDate' && (this.recurEndDate == '' || this.recurEndDate == null)){
                  this.errors.push("Ends on date is required")            
              }
              console.log(this.numOfOccurance)
              if(this.recurType == 'OnOcuurance' && (this.numOfOccurance == null|| this.numOfOccurance == '' || isNaN(this.numOfOccurance))){
                  this.errors.push("The number of occurances is required")
              }
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
      let itemIndex = this.hobbyList.findIndex(hobby => hobby.id == this.hobbyId);
      let startDate = null;
      let endDate = null;
      if(this.eventRecurrance != 'Just Once'){
          startDate = new Date();
          endDate = new Date();
          let day = startDate.getDay();
          let diff = startDate.getDate() - day + (day == 0 ? -6:1); // adjust when day is sunday
          startDate = new Date(startDate.setDate(diff));
          diff = (endDate.getDay() == 0? 0: 7 - endDate.getDay());
          endDate.setDate(endDate.getDate() + diff);
          console.log(startDate);
          console.log(endDate);
      }else{      
      let startArr = this.hobbyRanges.start.split('-');
      let endArr = this.hobbyRanges.end.split('-');
      startDate = new Date(startArr[0], startArr[1] - 1, startArr[2]);
      endDate = new Date(endArr[0], endArr[1] - 1, endArr[2]);
      }
      let recurStartDate = null;
      if(this.hobbyRecurStartDate){
        let recurStartArr = this.hobbyRecurStartDate.split('-');
        recurStartDate = new Date(recurStartArr[0], recurStartArr[1]-1, recurStartArr[2]);
      }
      console.log(recurStartDate);
      startDate.setHours(0, 0, 0, 0);
      endDate.setHours(23, 59, 59, 59);

      this.hobbyList[itemIndex].title = this.hobbyName;
      this.hobbyList[itemIndex].length = 60 * parseInt(this.hobbyHours) + parseInt(this.hobbyMinutes)
      this.hobbyList[itemIndex].selectedDate = {
        start: startDate,
        end: endDate
      }
      this.hobbyList[itemIndex].recurStartDate = recurStartDate;
      if(this.recurEndDate){
        endDate = new Date();
        let split = this.recurEndDate.split('-')
        endDate.setYear(parseInt(split[0]));
        endDate.setMonth(parseInt(split[1])-1);
        endDate.setDate(parseInt(split[2]));
      }
      if(this.eventRecurrance != 'Just Once'){
        this.hobbyList[itemIndex].recurrence = {
            pattern: this.eventRecurrance,
            recurranceType: this.recurType,
            frequency: parseInt(this.eventFrequency),
            endDate: endDate,
            numOfOccurance: parseInt(this.numOfOccurance)
        }
        if(this.eventRecurrance == 'Weekly'){
            this.hobbyList[itemIndex].recurrence.selectedDayOfTheWeek = this.selectedDayOfTheWeek;
        }
      }
      this.editHobbyModalBool = false;
    },
    /**
     * Opens the Event Edit Modal
     * Pre-condition: There is a evnt to edit
     * Post-condition: The Edit Event Modal is opened with the event's data displayed
     */
    openEditEvent(event){
      this.errors = [];
      this.eventName = event.title;
      this.eventId = event.m_id;
      console.log(event);
      let curr_event = this.listOfEvents.find(ev => ev.m_id == event.m_id);
      this.eventTimings.start = curr_event.start;
      this.eventTimings.end = curr_event.end;
      if(curr_event.recurrence == null){
        this.eventRecurrance = 'Just Once'
      }else{
        this.eventRecurrance = curr_event.recurrence.pattern;
        this.recurType = curr_event.recurrence.recurranceType;
        this.eventFrequency = curr_event.recurrence.frequency;
        this.endDate = curr_event.recurrence.endDate;
        this.numOfOccurance = curr_event.recurrence.numOfOccurance;
        if(curr_event.recurrence.pattern == 'Weekly'){
          this.selectedDayOfTheWeek = curr_event.recurrence.selectedDayOfTheWeek;
        }
      }
      this.eventStartDate = this.getDateInFormat(curr_event.start);
      this.eventEndDate = this.getDateInFormat(curr_event.end);
      let hours = '';
      let minute = '';
      if(curr_event.start.getHours() < 10){
        hours = '0' + event.start.getHours();
      }else{
        hours = curr_event.start.getHours();
      }
      if(curr_event.start.getMinutes() < 10){
        minute = '0' + curr_event.start.getMinutes()
      }else{
        minute = curr_event.start.getMinutes()
      }
      this.eventStartTime = `${hours}:${minute}`;
      if(curr_event.end.getHours() < 10){
        hours = '0' + event.end.getHours();
      }else{
        hours = curr_event.end.getHours();
      }
      if(curr_event.end.getMinutes() < 10){
        minute = '0' + curr_event.end.getMinutes()
      }else{
        minute = curr_event.end.getMinutes()
      }
      this.eventEndTime = `${hours}:${minute}`;
      this.removeEventModalBool = !this.removeEventModalBool;
    },
    /**
     * Deletes a selected event
     * Pre-condition: There is a event to delete
     * Post-condition: Selected event is deleted from hobbyList
     */
    removeEvent(){
      let itemIndex = this.listOfEvents.find(event => event.m_id == this.eventId).m_id;
      this.drawingList = this.drawingList.filter(item => item.m_id != itemIndex);
      this.listOfEvents = this.listOfEvents.filter(item => item.m_id != this.eventId);
      this.removeEventModalBool = false;
    },
    /**
     * Changes event data
     * Pre-condition: There is new data to update in a selected event
     * Post-condition: The selected event now has the new data. Returns true if there are errors.
     */
    editEvent(){
      this.errors = [];
      if(!this.eventName || this.eventName == ""){
          this.errors.push("Name is requried.");
      }

      if(this.eventStartDate == '' && this.eventStartTime == '' && this.eventEndDate == '' && this.eventEndTime == ''){
          this.errors.push("The event timings are missing.");
      }else{
          if(this.eventStartDate == ''){
              this.errors.push("Start date is required.");
          }
          if(this.eventStartTime == ''){
              this.errors.push("The time when the event starts is required.");
          }
          if(this.eventEndDate == ''){
              this.errors.push("End date is required.")
          }
          if(this.eventEndTime == ''){
              this.errors.push("The time when the event ends is required.");
          }
      }

      if(this.eventRecurrance == 'Daily'){
          if(this.eventFrequency == '' || this.eventFrequency == null){
              this.errors.push('Fill the "Repeat every x day" section')
          }
      }

      if(this.eventRecurrance == 'Weekly'){
          if(this.eventFrequency == '' || this.eventFrequency == null){
              this.errors.push('Fill the "Repeat every x week" section')
          }
          if(this.selectedDayOfTheWeek.length == 0){
              this.errors.push("Select the days of the week which the event will occur")
          }
      }

      if(this.recurType == null && this.eventRecurrance != "Just Once"){
          this.errors.push("Select how the event recurrance ends");
      }else{
          if(this.recurType == 'endDate' && this.recurEndDate == null){
              this.errors.push("Ends on date is required")            
          }

          if(this.recurType == 'OnOcuurance' && this.numOfOccurance == null){
              this.errors.push("The number of occurances is required")
          }
      }

      if(this.errors.length > 0){
          return true;
      }
      let newEvent = this.listOfEvents.find(event => event.m_id == this.eventId);
      newEvent.title = this.eventName;
      newEvent.start = this.textToTime(this.eventStartDate + 'T' + this.eventStartTime)
      newEvent.end = this.textToTime(this.eventEndDate + 'T' + this.eventEndTime)
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
      if(this.eventRecurrance != 'Just Once'){
        newEvent.recurrence = {
            pattern: this.eventRecurrance,
            recurranceType: this.recurType,
            frequency: parseInt(this.eventFrequency),
            endDate: endDate,
            numOfOccurance: parseInt(this.numOfOccurance)
        }

        if(this.eventRecurrance == 'Weekly'){
            newEvent.recurrence.selectedDayOfTheWeek = this.selectedDayOfTheWeek;
        }
      }
      if(this.eventRecurrance == 'Just Once'){
        newEvent.recurrence = null
      }
      console.log(newEvent);
      this.drawingList = this.drawingList.filter(item => item.m_id != newEvent.m_id);
      this.listOfEvents = this.listOfEvents.filter(item => item.m_id != this.eventId);
      this.eventId = this.addCalendarEvent(newEvent);
    },
    /**
     * Converts dateString to Date object
     * Pre-condition: There is a date string in the format (dd-mm-yyyyTHH:MM)
     * Post-condition: A date object is returned
     *  dateString - String variables that represents a date as a string
     */
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
    /**
     * Converts date into moment object
     * Pre-condition: There is a date object to convert
     * Post-condition: A moment object is returned
     * Parameter:
     *  date - Date objec
     */
    moment: function (date) {
      return moment(date);
    },
    /**
     * Saves the selected hobby timings
     * Pre-condition: The API has responded with dara
     * Post-condition: The selected hobby shows the event in the calendar
     * Parameter:
     *  e - Javascript event that represents the form submission
     */
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

        if(this.apiHobbyTime[i].recurrence.pattern == 'Weekly'){
              this.apiHobbyTime[i].recurrence.selectedDayOfTheWeek = this.hobbyList.find(hobby => hobby.id == this.hobbyId).recurrence.selectedDayOfTheWeek
            this.apiHobbyTime[i].start.setFullYear(this.apiHobbyTime[i].recurStartDate.getFullYear(), this.apiHobbyTime[i].recurStartDate.getMonth(), this.apiHobbyTime[i].recurStartDate.getDate())
            this.apiHobbyTime[i].end.setFullYear(this.apiHobbyTime[i].recurStartDate.getFullYear(), this.apiHobbyTime[i].recurStartDate.getMonth(), this.apiHobbyTime[i].recurStartDate.getDate())
              this.drawingList = [];
              for(let j = 0; j < this.listOfEvents.length; j++){
                let event = this.listOfEvents[j];
                this.listOfEvents = this.listOfEvents.filter(item => item.m_id != event.m_id);
                event.m_id = -1;
                this.addCalendarEvent(event);
              }
            }
        }
        this.addCalendarEvent(this.apiHobbyTime[i]);
      }
      this.apiHobbyTime = [];
      this.hobbyList = this.hobbyList.filter(hobby => hobby.id != this.hobbyId);
      this.openApiModal = false;
    },
    /*
     * Adds day to selectedDayOfTheWeek list
     * Pre-condition: None
     * 
     * Post-condition: Adds day key to the selectedDayOfTheWeek variable if the day key is not added to the selectedDayOfTheWeek. 
        If the day key is in the selectedDayOfTheWeek variable, then the day key is removed from selectedDayOfTheWeek.
     * Parameters:
     *  day - String variable that represents a day when an event occurs
    */
    addToDaysOfWeek(day){
      if(!this.selectedDayOfTheWeek.includes(day)){
          this.selectedDayOfTheWeek.push(day)
      }else{
          this.selectedDayOfTheWeek = this.selectedDayOfTheWeek.filter(element => element != day);
      }
    },
    /**
     * Converts Day into a number (0-7)
     * Pre-conditions: There is a day to convert
     * Post-conditions: A number is returned that represents the day
     * Parameters:
     *  day - String variable that represents a day to convert into a number
     */
    convertDayToNum(day){
      if(day == "sunday"){
        return 0;
      }
      if(day == "monday"){
        return 1;
      }
      if(day == "tuesday"){
        return 2;
      }
      if(day == "wednesday"){
        return 3;
      }
      if(day == "thursday"){
        return 4;
      }
      if(day == "friday"){
        return 5;
      }
      if(day == "saturday"){
        return 6;
      }
    },
    /**
     * Converts the numbers from the google data into a day string
     * Pre-conditions: There is a day to convert
     * Post-conditions: A string is returned that represents the day
     * Parameters:
     *  day - Int variable that represents a day to convert into a string
     */    
    googleConvertNumToDay(day){
      if(day == 0){
        return "monday"
      }
      if(day == 1){
        return "tuesday"
      }
      if(day == 2){
        return "wednesday"
      }
      if(day == 3){
        return "thursday"
      }
      if(day == 4){
        return "friday"
      }
      if(day == 5){
        return "saturday"
      }
      if(day == 6){
        return "sunday"
      }        
    },
  },
  mounted() {
    //Checks if the user has an accessToken. If not, the website redirects to the login date
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
