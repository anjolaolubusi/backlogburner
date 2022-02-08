<template>
    <button @click="openEventModal()">{{text}}</button>
    <button @click="isOpenSC = !isOpenSC">Add Hobby</button>


  <transition name="modal">
    <div v-if="isOpen">
        <div class="overlay" @click.self="isOpen = false;">
            <div class="modal" style="width: 50%">
                <h2 style="text-align: center;">Add Class</h2>
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: center;">
                    <div>
                        <form @submit="onSubmit">
                            <p v-if="errors.length > 0">
                                <b>Please correct the following error(s):</b>
                                <ul>
                                  <li v-for="error in errors" :key="error">{{ error }}</li>
                                </ul>
                            </p>

                            <label>Name: </label>
                            <input v-model="eventName" type="text" placeholder="Enter Title" /> <br/>
                            <label>Start Date: </label>
                            <DatePicker ref="eventModalPicker" v-model="eventTimings" mode="time" :input-debounce="100" :update-on-input="true" is-range :minute-increment="5" is24hr >
                            </DatePicker>
                            <br/> 
                            <label>Repeat: </label>
                            <select v-model="eventRecurrance">
                                <option v-for="listValue in recurranceTypes" :value="listValue" :key="listValue">
                                    {{listValue}}
                                </option>
                            </select>       
                            <div v-if="eventRecurrance != 'Never'">
                                <h3>Recurrance</h3>
                                <div v-if="eventRecurrance == 'Daily' "> 
                                    <label>Repeat every <input name="occurDay" type="number" v-model="dailyOccurNum" min="0"/> day(s) </label>
                                    <br>
                                    Ends:
                                    <br>
                                    <input type="radio" value="Never" v-model="recurType" /> Never
                                    <br>
                                    <input type="radio" value="endDate" v-model="recurType" /> On <input type="date" v-model="recurEndDate" />
                                    <br>
                                    <label><input type="radio" value="OnOcuurance" v-model="recurType" /> After <input type="number" min="0" v-model="numOfOccurance" /> occurance(s)</label>
                                </div>
                                <div v-if="eventRecurrance == 'Weekly'">
                                    <label>Repeat every <input type="number" v-model="dailyOccurNum" min="0" /> week(s)</label>
                                    <br>
                                    <label>Repeat on: </label>
                                    <br>
                                        <label for="monday"> <input type="checkbox" value="monday" v-model="selectedDayOfTheWeek" />M</label>
                                        <label for="tuesday"> <input type="checkbox" value="tuesday" v-model="selectedDayOfTheWeek" />T</label>
                                        <label for="wednesday"> <input type="checkbox" value="wednesday" v-model="selectedDayOfTheWeek" />W</label>
                                        <label for="thursday"> <input type="checkbox" value="thursday" v-model="selectedDayOfTheWeek" />T</label>
                                        <label for="friday"> <input type="checkbox" value="friday" v-model="selectedDayOfTheWeek" />F</label>
                                        <label for="saturday"> <input type="checkbox" value="saturday" v-model="selectedDayOfTheWeek" />S</label>
                                        <label for="sunday"> <input type="checkbox" value="sunday" v-model="selectedDayOfTheWeek" />S</label>
                                    <br>
                                        Ends:
                                        <br>
                                        <input type="radio" value="Never" v-model="recurType" /> Never
                                        <br>
                                        <input type="radio" value="OnDate" v-model="recurType" /> On <input type="date" v-model="recurEndDate" />
                                        <br>
                                        <label><input type="radio" value="OnOcuurance" v-model="recurType" /> After <input type="number" v-model="numOfOccurance"/> occurances</label>
                                </div>
                                
                            </div> 
                            <input type="submit" value="Add Calendar To Event" />  
                        </form>
                    </div>

                    <vue-cal ref="addEventModal" @event-drag-create="tempFunc($event)" @event-resizing="EventChange($event)" :on-event-create="onEventCliclEventModal" hide-view-selector :selected-date="selectedDate" :editable-events="{ title: false, drag: false, resize: true, delete: true, create: true}" :snap-to-time="5" :drag-to-create-threshold="5" :events="listOfEvents" active-view="day" :disable-views="['years', 'year', 'month', 'week']"  style="max-width: 460px;height: 500px;" class="vuecal--full-height-delete"></vue-cal>
                </div>
                <button @click="printCurrentEvent()">Print Curr Event</button>

            </div>
        </div>
    </div>
  </transition> 

  <transition name="modal">
    <div v-if="isOpenSC">
        <div class="overlay" @click.self="isOpenSC = false;">
            <div class="modal" style="width: 50%">
                
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: center;">
                    <div>
                        <input type="text" placeholder="Enter Title" /> <br/>
                        <input type="time" step="3600000" />
                    </div>
                    <vue-cal small hide-view-selector :selected-date="selectedDate" editable-events :events="listOfEvents" active-view="day" :disable-views="['years', 'year', 'month', 'week']"  style="max-width: 460px;height: 640px;" />
                </div>
                <button>Save Changes </button>
            </div>
        </div>
    </div>
  </transition>


</template>



<script>
import VueCal from 'vue-cal'
import 'vue-cal/dist/vuecal.css'
import {DatePicker} from 'v-calendar'
import 'v-calendar/dist/style.css';
//import Datepicker from 'vue3-date-time-picker';
//import * as  msal from '@azure/msal-browser'

export default({
    name: 'AddMediaTask',
    components:{
        VueCal,
        DatePicker,
    },
    data(){
        return{
            nameOfMedia: '',
            startDate: null,
            isOpen: false,
            endDate: null,
            username: null,
            graphConfig: {
                graphMeEndpoint: "https://graph.microsoft.com/v1.0/me",
                graphMailEndpoint: "https://graph.microsoft.com/v1.0/me/messages",
                graphCalendarEndpoint: "https://graph.microsoft.com/v1.0/me/events?$select=subject,body,bodyPreview,organizer,attendees,start,end,location,recurrence"
            },
            accessToken: null,
            selectedSCDate: null,
            eventName: '',
            isOpenSC: false,
            lengthOfSC: null,
            selectedEvent: null,
            eventStartDate: this.selectedDate,
            eventTimings: {
                start: new Date(),
                end: new Date()
            },
            eventRecurrance: 'Never',
            recurranceTypes: ['Never', 'Daily', 'Weekly'],
            daysOfTheWeek: ['M', 'T', 'W', 'T', 'F', 'S', 'S'],
            selectedDayOfTheWeek: [],
            recurType: null,
            dailyOccurNum: null,
            errors: [],
            recurEndDate: null,
            numOfOccurance: null
        }
    },
    props: {
        text: String,
        color: String,
        listOfEvents: Array,
        selectedDate: Date,
    },
    methods: {
        printCurrentEvent(){
            if(this.eventRecurrance == 'Daily'){
                console.log(`Occur Num: ${this.dailyOccurNum}`);
                console.log(`RecurType: ${this.recurType}`);
            }

            if(this.eventRecurrance == 'Weekly'){
                console.log(`selectedDays: ${this.selectedDayOfTheWeek}`);
            }
        },
        async openEventModal(){
            this.isOpen = !this.isOpen;
            return this.isOpen;
        },
        async tempFunc(data){
            this.eventTimings.start = data.start;
            this.eventTimings.end = data.end;
        },
        async EventChange(data){
            this.eventTimings.end = data.end;
            this.$refs.eventModalPicker.value = {start: this.eventTimings.start, end: data.end};
        },
        async onEventCliclEventModal(event, deleteEventFunction){
            if(!this.selectedEvent){
                this.selectedEvent = event;
                this.eventTimings.start = event.start;
                this.eventTimings.end = event.end;
                this.deleteEventFunction = deleteEventFunction;
                return event;
            }else{
                this.deleteEventFunction();
                this.selectedEvent = event;
                this.eventTimings.start = event.start;
                this.eventTimings.end = event.end;
                this.deleteEventFunction = deleteEventFunction;
                return event;
            }
        },
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
                this.errors.push("The start and end timings are required.");
            }

            // if(this.eventRecurrance == 'Daily'){

            // }
            console.log(`ERRORS: ${this.errors}`);
            if(this.errors.length > 0){
                return true;
            }
    
            var newEvent = {title: this.eventName, 
                start: this.selectedEvent.start,
                end: this.selectedEvent.end,
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

            this.$emit('add-cal-event', newEvent);
            this.deleteEventFunction();
            this.eventName = "";
        },
        async InsertEvent(item){
            console.dir(item);
        },
        async pushSC(e){
            e.preventDefault()
            if(!this.eventName){
                alert("Please add in a name")
                return
            }
            if(this.selectedSCDate == null){
                alert("Please select a start date")
                return
            }
            const newEvent = {
                title: this.eventName,
                selectedDate: this.selectedSCDate,
                length: parseInt(this.lengthOfSC),
                class: 'sc',
                source: 'M'
            }
            this.$emit('add-sc', newEvent)
            this.eventName = '';
            this.selectedSCDate = null;
        },
        PrintSelectedDate(){
            console.log(`Start Date: ${this.startDate} End Date: ${this.endDate}`);
        },
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
        async PullFromOutlook(){
            var cal_data = await this.callMSGraph(this.graphConfig.graphCalendarEndpoint, this.$cookies.get("accessToken"))
            this.$emit('pull-outlook-event', cal_data)
            },
        async CallGoogleApi(endpoint){
            console.log(`AccessToken: ${this.$cookies.get("accessToken")}`)
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
        async PullFromGoogle(){
            var calendar_list = await this.CallGoogleApi("https://www.googleapis.com/calendar/v3/users/me/calendarList");
            //TODO: put code to ask user which calendar
            var chosen_id = calendar_list.items[0].id;
            var event_endpoint = `https://www.googleapis.com/calendar/v3/calendars/${chosen_id}/events`;
            var events = (await this.CallGoogleApi(event_endpoint)).items;
            this.$emit('pull-google-event', events)
        },
        async getCalendarData(){
            if(this.$cookies.get("loginSource") == 'O'){
                this.PullFromOutlook()
            }

            if(this.$cookies.get("loginSource") == 'G'){
                this.PullFromGoogle()
            }
        }
    },
    emits: ['add-cal-event', 'pull-outlook-event', 'add-sc', 'pull-google-event'],
    mounted(){
        this.getCalendarData();
    },
    watch: {
        eventTimings: function(val){
            if(this.selectedEvent){
            this.selectedEvent.start = val.start;
            this.selectedEvent.startTimeMinutes = val.start.getHours() * 60 + val.start.getMinutes();
            this.selectedEvent.end = val.end;
            this.selectedEvent.endTimeMinutes = val.end.getHours() * 60 + val.end.getMinutes();
            }
        }
    }
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
</style>