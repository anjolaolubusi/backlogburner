<template>
    <!-- <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: space-between;"> -->
    <button style="margin-top: 0px" @click="openEventModal()">{{text}}</button> <br />
    <button style="margin-top: 0px; background-color: rgba(253, 156, 66, 0.9)" @click="errors=[];isOpenSC = !isOpenSC">Add Hobby</button>
    <!-- </div> -->

  <transition name="modal" v-on:after-enter="addTempEvent">
    <div v-if="isOpen">
        <div class="overlay">
            <div class="modal" style="width: 55%">
                <form @submit="onSubmit">
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: flex-end;">
                    <button style="background-color: rgba(25, 25, 25, 1);font-size: 16px;" @click="isOpen = false;">Close</button>
                </div>

                <h2 style="text-align: center;">Add Event</h2>
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: space-between;">
                    <div>
                            <p v-if="errors.length > 0">
                                <b>Please correct the following error(s):</b>
                                <ul>
                                  <li v-for="error in errors" :key="error">{{ error }}</li>
                                </ul>
                            </p>

                            <label>What is the name of the event? </label>
                            <input v-model="eventName" type="text" placeholder="Enter Name Of Event" size="15" style="font-size:11pt"/> <br/>
                            <label v-tooltip="{text: 'If you have a class every Wednesday that starts at 8:00 am and that the next Wednesday is on the 25th April 2022, then type 04/25/2022 8:00 am', theme: {'background-color': '#fffff0', color: '#000000', placement: 'top', width: '16em', padding: '0.4rem', 'font-size': '0.8em'}}">When does an indiviual session of the event start? </label>
                            <input type="date" v-model="eventStartDate" @input="updateStartDate"/> <input type="time" v-model="eventStartTime" @input="updateStartTime" /> <br/>
                            <label v-tooltip="{text: 'If you have a class every Wednesday that ends at 8:50 am and that the next Wednesday is on the 25th April 2022, then type 04/25/2022 8:50 am', theme: {'background-color': '#fffff0', color: '#000000', placement: 'top', width: '16em', padding: '0.4rem', 'font-size': '0.8em'}}">When does an indiviual session of the event end? </label>
                            <input type="date" v-model="eventEndDate" :min="eventStartDate" />  <input type="time" v-model="eventEndTime" /><br />
                            <label v-tooltip="{text: 'If the event happens every day, then select Daily. If the event happens every week, then select Weekly.', theme: {'background-color': '#fffff0', color: '#000000', placement: 'top', width: '18em', padding: '0.4rem', 'font-size': '0.8em'}}">How often does this event repeat? </label>
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
                                            <li style="margin-bottom: 8px;"><span>Ends</span></li>
                                        </div>
                                        <div v-on:click="recurType='Never'">
                                            <input type="radio" value="Never" v-model="recurType"/> This event will never stop recurring
                                        </div>

                                        <div v-on:click="recurType='endDate'">
                                            <input type="radio" value="endDate" v-model="recurType" /> This event will stop recurring on: <input type="date" v-model="recurEndDate" />
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
                                            <label>This event repeats every: </label>
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
                                            <input type="radio" value="Never" v-model="recurType" /> This event never stops recurring
                                        </div>

                                        <div v-on:click="recurType='OnDate'">
                                            <input type="radio" value="OnDate" v-model="recurType" /> This event stops recurring on <input type="date" v-model="recurEndDate" />
                                        </div>

                                        <div v-on:click="recurType='OnOcuurance'">
                                            <label><input type="radio" value="OnOcuurance" v-model="recurType" /> This event stops recurring after <input type="number" v-model="numOfOccurance" size="5"/> occurrence(s)</label>
                                        </div>
                                </div>
                                </ol>
                                
                            </div> 
                            <br />
                            <div>
                            </div>
                    </div>

                    <vue-cal @event-drop="onEventDrag" timeFormat="h:mm am" twelveHour :time-step="30"  resize-x small ref="addEventModal" @event-drag-create="tempFunc($event)" @event-resizing="EventChange($event)" :on-event-create="onEventCliclEventModal" :selected-date="selectedDate" :editable-events="{ title: false, drag: true, resize: true, delete: true, create: true}" :snap-to-time="5" :drag-to-create-threshold="15" :events="listOfEvents" active-view="day" :disable-views="['years', 'year',]"  style="max-width: 460px;height: 500px;" class="vuecal--full-height-delete"></vue-cal>
                </div>
                <input type="submit" value="Add Event To Calendar" style="background-color: green;font-size: 16px;color: white;" />  
                </form>
            </div>
        </div>
    </div>
  </transition> 

  <transition name="modal2">
    <div v-if="isOpenSC">
        <div class="overlay">
            <div class="modal" style="width: 70%">
                <form @submit="pushHobby">
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: flex-end;">
                    <button style="background-color: black;font-size: 16px;" @click="isOpenSC = false;">Close</button>
                </div>
                <h2 style="text-align: center;">Add Hobby</h2>
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: center;">
                    <div>
                            <p v-if="errors.length > 0">
                                <b>Please correct the following error(s):</b>
                                <ul>
                                  <li v-for="error in errors" :key="error">{{ error }}</li>
                                </ul>
                            </p>
                            <label>What is the name of the hobby? <input type="text" placeholder="Enter Name of Hobby" v-model="hobbyName" size="15" style="font-size:11pt"/>  </label>
                            <br>
                            <label>How is long does the hobby go for? <input type="number" style="font-size:11pt" min="0" v-model="hobbyHours" onfocus="if (this.value=='') this.value='0';" oninput="this.value = this.value.replace(/[^0-9.]/g, '').replace(/(\..*?)\..*/g, '$1');" size="5"/> hr <input type="number" min="0" v-model="hobbyMinutes" onfocus="if (this.value=='') this.value='0';" oninput="this.value = this.value.replace(/[^0-9.]/g, '').replace(/(\..*?)\..*/g, '$1');" size="5"/> min </label>
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
                                                <label>Repeat every <input name="occurDay" type="number" v-model="dailyOccurNum" min="0" size="5" /> day(s) </label>
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
                                            <label>When should this hobby start?<input type="date" v-model="hobbyRecurStartDate" /> </label>
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
                            <label v-if="eventRecurrance == 'Just Once'">Within which days do you want this hobby to happen: 
                            <input  type="date" v-model="hobbyRanges.start" /> - <input type="date" v-model="hobbyRanges.end"/>
                            </label>
                            <br />
                            <!-- <DatePicker v-model="hobbyRanges" is-range /> -->

                            <br />
                            <br />
                    </div>

                    <vue-cal :min-date="minDate" :max-date="maxDate" small hide-view-selector :selected-date="selectedDate" :events="listOfEvents" active-view="week" :disable-views="['years']"  style="max-width: 460px;height: 500px;"></vue-cal>
                </div>
                <input style="padding: 7px; margin-top: 10px;background-color: green;color: white;font-size: 1.1rem;" type="submit" value="Save Hobby" />  
                </form>
            </div>
        </div>
    </div>
  </transition>


</template>

<script>
import VueCal from 'vue-cal'
import 'vue-cal/dist/vuecal.css'
import 'vue-cal/dist/drag-and-drop.js'
import { inject} from "vue";

//import Datepicker from 'vue3-date-time-picker';
//import * as  msal from '@azure/msal-browser'

export default({
    name: 'AddMediaTask',
    components:{
        VueCal,
    },
    data(){
        return{
            //isOpen: boolean to check if the Add Event Popup is open
            isOpen: false, 
            //graphConfig: object to store the Microsoft configuration data
            graphConfig: {
                graphMeEndpoint: "https://graph.microsoft.com/v1.0/me",
                graphMailEndpoint: "https://graph.microsoft.com/v1.0/me/messages",
                graphCalendarEndpoint: "https://graph.microsoft.com/v1.0/me/events?$select=subject,body,bodyPreview,organizer,attendees,start,end,location,recurrence"
            },
            //acessToken: stores the access token for Google or Microsoft
            accessToken: null,
            //eventName: Name of the Event
            eventName: '',
            //isOpenSC: boolean to check if the Add Hobby Popup is open
            isOpenSC: false,
            //selectedEvent: Object that represents the selected event
            selectedEvent: null,
            //eventStartDate: Object that represents an event's start date
            eventStartDate: '',
            //eventStartTime: Object that represents an event's start time
            eventStartTime: '',
            //eventEndDate: Object that represents an event's end date
            eventEndDate: '',
            //eventEndTime: Object that represents an event's end time
            eventEndTime: '',
            //eventTimings: Object that represent an event's timings
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
            //dailyOccurNum: Scalar variable that represents how often an event occurs (Every x days)
            dailyOccurNum: null,
            //errors: List of errors
            errors: [],
            //recurEndDate: When the event stops recurring
            recurEndDate: null,
            //numOfOccurance: Number of Occurances for an event
            numOfOccurance: null,
            //hobbyName: Name of hobby
            hobbyName: null,
            //hobbyRanges: Date Ranges for hobbies
            hobbyRanges: {
                start: '',
                end: ''
            },
            //hobbyHours: Number of hours for a hobby
            hobbyHours: '0',
            //hobbyMinutes: Number of minutes for a hobby
            hobbyMinutes: '0',
            //hobbyRecurStartDate: When the hobby starts recurring
            hobbyRecurStartDate: null,
            modelIsOpen: null
        }
    },
    props: {
        text: String,
        color: String,
        listOfEvents: Array,
        selectedDate: Date,
    },
    methods: {
        /*
         * Adds day to selectedDayOfTheWeek list
         * Pre-condition: None
         * Post-condition: Adds day key to the selectedDayOfTheWeek variable if the day key is not added to the selectedDayOfTheWeek. 
            If the day key is in the selectedDayOfTheWeek variable, then the day key is removed from selectedDayOfTheWeek.
        */
        addToDaysOfWeek(day){
            if(!this.selectedDayOfTheWeek.includes(day)){
                this.selectedDayOfTheWeek.push(day)
            }else{
                this.selectedDayOfTheWeek = this.selectedDayOfTheWeek.filter(element => element != day);
            }
        },
        /*
         * Prints the currently selected event
         * Pre-condition: None
         * Post-condition: None
         */
        printCurrentEvent(){
            console.log(this.eventStartDate);
            this.textToTime(this.eventStartDate);
        },
        /**
         * Open the Add Event Modal
         * Pre-condition: None
         * Post-condition: The Add Event Modal is opened and the form is cleared
         */
        async openEventModal(){
            this.selectedDayOfTheWeek = [];
            this.recurType = null,
            this.dailyOccurNum = null,
            this.errors = [];
            this.recurEndDate = null,
            this.numOfOccurance = null,
            this.eventStartDate = '',
            this.eventTimings = {
                start: new Date(),
                end: new Date()
            },
            this.eventRecurrance = 'Just Once',
            this.isOpen = !this.isOpen;
            return this.isOpen;
        },
        /**
         * Convert Form datestring to Date object
         * Pre-condition: None
         * Post-condition: None
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
         * Updates Add Event form when event is created in the Add Event calendar
         * Pre-condition: Event is created in Add Event Calendar
         * Post-condition: eventTimings, eventStartDate, eventStartTime, eventEndDate and eventEndTime are updated
         */
        async tempFunc(data){
            this.eventTimings.start = data.start;
            this.eventTimings.end = data.end;
            let startArr = this.getDateInFormat(data.start).split('T');
            this.eventStartDate = startArr[0];
            this.eventStartTime = startArr[1];
            let endArr = this.getDateInFormat(data.end).split('T');
            this.eventEndDate = endArr[0];
            this.eventEndTime = endArr[1];
        },
        /**
         * Updates the end times on the UI From when the event is streched in the Add Event Calendar
         * Pre-condition: Event is changed in Add Event Calendar
         * Post-condition eventTimings, eventEndDate and eventEndTime are updated
         */
        async EventChange(data){
            this.eventTimings.end = data.end;
            let endArr = this.getDateInFormat(data.end).split('T');
            this.eventEndDate = endArr[0];
            this.eventEndTime = endArr[1];
        },
        /**
         * Adds Event when the user clicks-and-drags the cursor in the Add Event Calendar
         * Pre-condition: User clicks and drags on the Add Event calendar
         * Post-condition: Event is created and UI is updated
         */
        async onEventCliclEventModal(event, deleteEventFunction){
            let startArr = this.getDateInFormat(event.start).split('T');
            this.eventStartDate = startArr[0];
            this.eventStartTime = startArr[1];
            if(!this.selectedEvent){
                this.selectedEvent = event;
                this.eventTimings.start = event.start;
                this.eventTimings.end = event.end;
                let endArr = this.getDateInFormat(event.end).split('T');
                this.eventEndDate = endArr[0];
                this.eventEndTime = endArr[1];
                this.deleteEventFunction = deleteEventFunction;
                return event;
            }else{
                this.deleteEventFunction();
                this.selectedEvent = event;
                this.eventTimings.start = event.start;
                this.eventTimings.end = event.end;
                let endArr = this.getDateInFormat(event.end).split('T');
                this.eventEndDate = endArr[0];
                this.eventEndTime = endArr[1];
                this.deleteEventFunction = deleteEventFunction;
                return event;
            }
        },
        /**
         * Adds the Event to the main calendar
         * Pre-condition: The form in the Add Event modal is completed
         * Post-condition: The event is added to the main calendar
         */
        async onSubmit(e){
            e.preventDefault();
            this.errors = [];
            if(!this.eventName || this.eventName == ""){
                this.errors.push("Name is requried.");
            }

            if(this.selectedEvent){
                if(this.selectedEvent.start == null){
                    this.errors.push("Start date is required.");
                }
                if(this.selectedEvent.end == null){
                    this.errors.push("End date is required.")
                }
            }else{
                this.errors.push("Event must be drawn required.");
            }

            if(this.eventRecurrance == 'Daily'){
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
            }

            if(this.eventRecurrance == 'Weekly'){
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
            }

            if(this.errors.length > 0){
                return true;
            }
    
            var newEvent = {title: this.eventName, 
                start: this.textToTime(this.eventStartDate + 'T' + this.eventStartTime),
                end: this.textToTime(this.eventEndDate + 'T' + this.eventEndTime),
                source: "M",
                class: 'hc',
            }


            if(this.recurEndDate){
            var endDate = new Date();
            var split = this.recurEndDate.split('-');
            endDate.setYear(parseInt(split[0]));
            endDate.setMonth(parseInt(split[1])-1);
            endDate.setDate(parseInt(split[2]));
            endDate.setHours(this.selectedEvent.end.getHours());
            endDate.setMinutes(this.selectedEvent.end.getMinutes());
            endDate.setSeconds(this.selectedEvent.end.getSeconds());
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
            console.log(newEvent);
            this.$emit('add-cal-event', newEvent);
            this.deleteEventFunction();
            this.eventName = "";
        },
        /**
         * Adds the Hobby to the main screen
         * Pre-condition: The form in the Add Hobby Modal is completed
         * Post-condition: The hobby is added to the main screen
         */
        async pushHobby(e){
            e.preventDefault()
            this.errors = [];
            if(!this.hobbyName){
                this.errors.push("Name is required")
            }

            if(this.hobbyHours == '0' && this.hobbyMinutes == '0'){
                this.errors.push("Hobby length is required")
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

                // if(this.hobbyRanges.start === '' || this.hobbyRanges.end === ''){
                //     this.errors.push("Fill in the dates which the hobby's timing should based on")
                // }
                // let numOfSecondsInAWeek = 2 * 604800 * 1000
                // if(this.textToTime(this.hobbyRanges.end + 'T23:59') - this.textToTime(this.hobbyRanges.start + 'T00:00') > numOfSecondsInAWeek){
                //     this.errors.push("Choose a smaller sample window for the last step in the Recurrance Details section")
                // }
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

                // if(this.hobbyRanges.start === '' || this.hobbyRanges.end === ''){
                //     this.errors.push("Fill in the dates which the hobby's timing should based on")
                // }

                // let numOfSecondsInAWeek = 2 * 604800 * 1000
                // if(this.textToTime(this.hobbyRanges.end + 'T23:59') - this.textToTime(this.hobbyRanges.start + 'T00:00') > numOfSecondsInAWeek){
                //     this.errors.push("Choose a smaller sample window for the last step in the Recurrance Details section")
                // }                
            }

            if(this.eventRecurrance == 'Just Once'){
                if(this.hobbyRanges.start === '' || this.hobbyRanges.end === ''){
                    this.errors.push("Fill in the dates which the hobby should happen")
                }
                
                let numOfSecondsInAWeek = 2 * 604800 * 1000
                if(this.textToTime(this.hobbyRanges.end + 'T23:59') - this.textToTime(this.hobbyRanges.start + 'T00:00') > numOfSecondsInAWeek){
                    this.errors.push("Choose a smaller window to place a hobby in")
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
            console.log(this.hobbyRanges);
            let recurStartDate = null
            if(this.hobbyRecurStartDate){
                let recurStartArr = this.hobbyRecurStartDate.split('-');
                recurStartDate = new Date(recurStartArr[0], recurStartArr[1]-1, recurStartArr[2]);
            }
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
            startDate = new Date(startArr[0], startArr[1]-1, startArr[2]);
            endDate = new Date(endArr[0], endArr[1]-1, endArr[2]);
            }
            startDate.setHours(0, 0, 0, 0);
            endDate.setHours(23, 59, 59, 59);
            let newEvent = {
                title: this.hobbyName,
                selectedDate: {
                    start: startDate,
                    end: endDate
                },
                length: 60 * parseInt(this.hobbyHours) + parseInt(this.hobbyMinutes),
                class: 'sc',
                source: 'M',
                recurStartDate: recurStartDate
            }
            if(this.recurEndDate){
              endDate = new Date();
              let split = this.recurEndDate.split('-')
              //let timeSplit = this.eventEndTime.split(':')
              endDate.setYear(parseInt(split[0]));
              endDate.setMonth(parseInt(split[1])-1);
              endDate.setDate(parseInt(split[2]));
            //   endDate.setHours(timeSplit[0]);
            //   endDate.setMinutes(timeSplit[1]);
            //   endDate.setSeconds(0);
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
            
            this.$emit('add-sc', newEvent)
            this.hobbyName = '';
            this.hobbyRanges = {
                start: '',
                end: ''
            };
            this.hobbyHours = '0';
            this.hobbyMinutes = '0';
        },
        /**
         * Calls the Microsoft Gaph API to bring in data
         * Pre-condition: None
         * Post-condition: Data is retrived from Micrsoft
         */
        async callMSGraph(endpoint, token) {
            const headers = new Headers();
            const bearer = `Bearer ${token}`;
            headers.append("Authorization", bearer);
            
            const options = {
                method: "GET",
                headers: headers
            };

            console.log('request made to Graph API at: ' + new Date().toString());

            return fetch(endpoint, options)
                .then((response) => { 
                    return response.json().then((data) => {
                        console.dir(data)
                        return data.value;
                    }).catch((err) => {
                        alert(err);
                    }) 
                });
        },
        /**
         * Pulls data from Outlook
         * Pre-condition: None
         * Post-condition: Data is retrived from Outlook
         */
        async PullFromOutlook(){
            if(new Date() > new Date(this.$cookies.get("exDate"))){
                let loginRequest = {
                    client_id: "0b1cbc4a-fe05-456f-ae2e-2e38cc6d741c",
                    scopes: ["openid", "profile", "User.Read"]
                }
                let loginResponse =  await this.$msalClient.loginPopup(loginRequest).catch(
                    error => { alert(error)}
                );
                this.$cookies.set("accessToken", loginResponse.accessToken);
                this.$cookies.set("expDate", loginResponse.expiresOn.getTime())
            }
            var cal_data = await this.callMSGraph(this.graphConfig.graphCalendarEndpoint, this.$cookies.get("accessToken"))
            this.$emit('pull-outlook-event', cal_data)
            },
        /**
         * Pulls data from Google
         * Pre-condition: None
         * Post-condition: Data is retrived from Google
         */
        async CallGoogleApi(endpoint){
            //console.log(`AccessToken: ${this.$cookies.get("accessToken")}`)
            /*var curDate = new Date().setTime(Date.now());                
            if(curDate >= this.$cookies.get("expirationDate")){
                var authResponse = this.$gAuth.instance.currentUser.get().getAuthResponse();
                this.$route.params.accessToken = authResponse.accessToken;
                this.$route.params.expirationDate = new Date().setTime(authResponse.expirationDate);
            }*/
            const headers = new Headers();
            const bearer = `Bearer ${this.$cookies.get("accessToken")}`;
            headers.append("Authorization", bearer);
            const options = {
                method: "GET",
                headers: headers
            }
            console.log('request made to Google at: ' + new Date().toString())
            return fetch(endpoint, options)
                .then((response) => { 
                    return response.json().then((data) => {
                        console.dir(data)
                        return data;
                    }).catch((err) => {
                        alert(err);
                    }) 
                });
        },
        /**
         * Pulls data from Google Calendar
         * Pre-condition: None
         * Post-condition: Data is retrived from Google Calendar
         */
        async PullFromGoogle(){
            if(new Date() > new Date(this.$cookies.get("exDate"))){
                await this.$gAuth.signIn();
                let authResponse = this.$gAuth.instance.currentUser.get().getAuthResponse();
                let exDate = new Date();
                exDate.setTime(exDate.getTime() + authResponse.expires_in * 1000)
                this.$cookies.set("expDate", exDate.getTime())
                this.$cookies.set("accessToken", authResponse.access_token);
                this.$cookies.set("loginSource",'G');
            }
            let calendar_list = await this.CallGoogleApi("https://www.googleapis.com/calendar/v3/users/me/calendarList");
            console.log(calendar_list);
            let chosen_id = calendar_list.items[0].id;
            let event_endpoint = `https://www.googleapis.com/calendar/v3/calendars/${chosen_id}/events`;
            let events = (await this.CallGoogleApi(event_endpoint)).items;
            this.$emit('pull-google-event', events)
        },
        /**
         * Gets Calendar Data based on how the user logged in
         * Pre-condition: The user logs in with gmail or outlook
         * Post-condition: Data is retrived from the user's account
         */
        async getCalendarData(){
            if(this.$cookies.get("loginSource") == 'O'){
                this.PullFromOutlook()
            }

            if(this.$cookies.get("loginSource") == 'G'){
                this.PullFromGoogle()
            }
        },
        /**
         * Javascript Date is converted to a specific date string format
         * Pre-condition: None
         * Post-condition: Returns date string of parameter
         */
        getDateInFormat(date){
            let d = new Date(date),
            month = '' + (d.getMonth() + 1),
            day = '' + d.getDate(),
            year = d.getFullYear(),
            hour = d.getHours(),
            minute = d.getMinutes();
            

            if (month.length < 2) 
                month = '0' + month;
            if (day.length < 2) 
                day = '0' + day;
            if (hour < 10){
                hour = '0' + hour
            }
            if (minute < 10){
                minute = '0' + minute
            }
            let dateString = [year, month, day].join('-');
            let timeString = [hour, minute].join(':');
            return [dateString, timeString].join('T');
        },
        /**
         * Add event to Add Event modal calendar
         * Pre-condition: The Add Event modal has just been opened
         * Post-condition: An event is added to the Add Event modal calendar
         */
        addTempEvent(){
            let currDate = new Date();
            let numOfSeconds = currDate.getMinutes()/(30.0)
            numOfSeconds = Math.floor(numOfSeconds) - 1;
            currDate.setMinutes(numOfSeconds)
            this.selectedEvent = this.$refs.addEventModal.createEvent(
                currDate,
                30
            )
        },
        /**
         * Updates the event shown in the Add Event Modal calendar
         * Pre-condition: Start Date of the event changes 
         * Post-condition: The Add Event Modal calendar is updated to reflect this change
         */
        updateStartDate(newString){
            let dateArr = newString.target.value.split('-');
            this.selectedEvent.start.setFullYear(dateArr[0], dateArr[1]-1, dateArr[2]);
            this.selectedEvent.startTimeMinutes = this.selectedEvent.start.getHours() * 60 + this.selectedEvent.start.getMinutes();
        },
        /**
         * Updates the event shown in the Add Event Modal calendar
         * Pre-condition: End Date of the event changes 
         * Post-condition: The Add Event Modal calendar is updated to reflect this change
         */
        updateEndDate(newString){
            let dateArr = newString.target.value.split('-');
            this.selectedEvent.end.setFullYear(dateArr[0], dateArr[1]-1, dateArr[2]);
            this.selectedEvent.endTimeMinutes = this.selectedEvent.end.getHours() * 60 + this.selectedEvent.end.getMinutes();
        },
        /**
         * Updates the event shown in the Add Event Modal calendar
         * Pre-condition: Start Time of the event changes 
         * Post-condition: The Add Event Modal calendar is updated to reflect this change
         */
        updateStartTime(newString){
            let timeArr = newString.target.value.split(':');            
            this.selectedEvent.start.setHours(timeArr[0]) 
            this.selectedEvent.start.setMinutes(timeArr[1])
            this.selectedEvent.startTimeMinutes = this.selectedEvent.start.getHours() * 60 + this.selectedEvent.start.getMinutes();
        },
        /**
         * Updates the event shown in the Add Event Modal calendar
         * Pre-condition: End Time of the event changes 
         * Post-condition: The Add Event Modal calendar is updated to reflect this change
         */        
        updateEndTime(newString){
            let timeArr = newString.target.value.split(':');            
            this.selectedEvent.end.setHours(timeArr[0]) 
            this.selectedEvent.end.setMinutes(timeArr[1])
            this.selectedEvent.endTimeMinutes = this.selectedEvent.end.getHours() * 60 + this.selectedEvent.end.getMinutes();
        },   
        /**
         * Updates the Add Event Modal form when the new event is dragged
         * Pre-condition: The new event is dragged
         * Post-condition: The Add Event Modal form is updated to reflect this change
         */                                   
        onEventDrag(data){
            let newStartDateArr = this.getDateInFormat(data.event.start).split('T');
            this.eventStartTime = newStartDateArr[1];
            this.eventStartDate = newStartDateArr[0];
            let newEndDateArr = this.getDateInFormat(data.event.end).split('T');
            this.eventEndDate = newEndDateArr[0];
            this.eventEndTime = newEndDateArr[1];

        }
    },
    //Pushes the following events to next view in the hireachy
    emits: ['add-cal-event', 'pull-outlook-event', 'add-sc', 'pull-google-event'],
    mounted(){
        //Is ran when AddCalendarEvent is mounted
        this.getCalendarData();
    },
    watch: {
        /** 
         * Updates a set of variables when eventTimings changes
         * Pre-conditions: eventTimings changes
         * Post-conditions: Updates selectedEvent, eventStartDate, eventStartTime, eventEndDate and eventEndTime 
         */ 
        eventTimings: function(val){
            if(this.selectedEvent){
            this.selectedEvent.start = val.start;
            this.selectedEvent.startTimeMinutes = val.start.getHours() * 60 + val.start.getMinutes();
            this.selectedEvent.end = val.end;
            this.selectedEvent.endTimeMinutes = val.end.getHours() * 60 + val.end.getMinutes();
            let startArr = this.getDateInFormat(val.start).split('T');
            this.eventStartDate = startArr[0];
            this.eventStartTime = startArr[1];
            let endArr = this.getDateInFormat(val.end).split('T');
            this.eventEndDate = endArr[0];
            this.eventEndTime = endArr[1];            
            }
        }
    },
    computed: {
        /**
         * Returns the start date of the hobby, or the current date subtracted by 10 days, in a datestring
         * Pre-conditions: None
         * Post-conditions: Returns the start date of the hobby, or the current date subtracted by 10 days, in a datestring
         */
        minDate (){
            console.log(this.hobbyRanges);
            if(this.hobbyRanges.start == ""){
                return new Date().subtractDays(10)
            }else{
                return this.textToTime(this.hobbyRanges.start)
            }
        },
        /**
         * Returns the start date of the hobby, or the current date added by 10 days, in a datestring
         * Pre-conditions: None
         * Post-conditions: Returns the start date of the hobby, or the current date added by 10 days, in a datestring
         */
        maxDate (){
            if(this.hobbyRanges.end == ''){
                return new Date().addDays(10)
            }else{
                return this.textToTime(this.hobbyRanges.end)
            }
        }
    },
    setup() {
        //Imports Google Auth
        const Vue3GoogleOauth = inject("Vue3GoogleOauth");
        return { Vue3GoogleOauth };
    },   
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
button {
  padding: 7px;
  margin-top: 10px;
  background-color: green;
  color: white;
  font-size: 1.1rem;
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

.vuecal__cell--disabled {text-decoration: line-through;}
.vuecal__cell--before-min {color: #b6d6c7;}
.vuecal__cell--after-max {color: #008b8b;}
.vuecal__time-cell-line.hours:before {border-color: #42b983;}

</style>