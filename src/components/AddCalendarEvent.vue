<template>
    <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: space-between;">
    <button style="margin-top: 0px" @click="openEventModal()">{{text}}</button> 
    <button style="margin-top: 0px" @click="isOpenSC = !isOpenSC">Add Hobby</button>
    </div>

  <transition name="modal">
    <div v-if="isOpen">
        <div class="overlay">
            <div class="modal" style="width: 50%">
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: flex-end;">
                    <button style="background-color: rgba(25, 25, 25, 1);font-size: 16px;" @click="isOpen = false;">Close</button>
                </div>

                <h2 style="text-align: center;">Add Event</h2>
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: space-between;">
                    <div>
                        <form @submit="onSubmit">
                            <p v-if="errors.length > 0">
                                <b>Please correct the following error(s):</b>
                                <ul>
                                  <li v-for="error in errors" :key="error">{{ error }}</li>
                                </ul>
                            </p>

                            <label>Name: </label>
                            <input v-model="eventName" type="text" placeholder="Enter Title" size="15" style="font-size:11pt"/> <br/>
                            <!-- <label>Start: </label> -->
                            <!-- <DatePicker ref="eventModalPicker" v-model="eventTimings" mode="time" :input-debounce="100" :update-on-input="true" is-range :minute-increment="5" is24hr >
                            </DatePicker> -->
                            <!-- <input type="datetime-local" v-model="eventStartDate" readonly /> <br/>
                            <label> End: </label>
                            <input type="datetime-local" v-model="eventEndDate" :min="eventStartDate" readonly /> <br /> -->
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
                            <input type="submit" value="Add Event To Calendar" style="background-color: green;font-size: 16px;color: white;" />  
                        </form>
                    </div>

                    <vue-cal timeFormat="h:mm am" twelveHour :time-step="30"  resize-x small ref="addEventModal" @event-drag-create="tempFunc($event)" @event-resizing="EventChange($event)" :on-event-create="onEventCliclEventModal" :selected-date="selectedDate" :editable-events="{ title: false, drag: true, resize: true, delete: true, create: true}" :snap-to-time="5" :drag-to-create-threshold="15" :events="listOfEvents" active-view="day" :disable-views="['years', 'year',]"  style="max-width: 460px;height: 500px;" class="vuecal--full-height-delete"></vue-cal>
                </div>
            </div>
        </div>
    </div>
  </transition> 

  <transition name="modal2">
    <div v-if="isOpenSC">
        <div class="overlay">
            <div class="modal" style="width: 70%">
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: flex-end;">
                    <button style="background-color: rgba(220, 25, 25, 1);font-size: 16px;" @click="isOpenSC = false;">Close</button>
                </div>
                <h2 style="text-align: center;">Add Hobby</h2>
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: center;">
                    <div>
                        <form @submit="pushHobby">
                            <p v-if="errors.length > 0">
                                <b>Please correct the following error(s):</b>
                                <ul>
                                  <li v-for="error in errors" :key="error">{{ error }}</li>
                                </ul>
                            </p>
                            <label>Name: <input type="text" placeholder="Enter Name of Hobby" v-model="hobbyName" size="15" style="font-size:11pt"/>  </label>
                            <br>
                            <label>Length: <input type="number" style="font-size:11pt" min="0" v-model="hobbyHours" onfocus="if (this.value=='') this.value='0';" oninput="this.value = this.value.replace(/[^0-9.]/g, '').replace(/(\..*?)\..*/g, '$1');" size="5"/> hr <input type="number" min="0" v-model="hobbyMinutes" onfocus="if (this.value=='') this.value='0';" oninput="this.value = this.value.replace(/[^0-9.]/g, '').replace(/(\..*?)\..*/g, '$1');" size="5"/> min </label>
                            <br>
                            <label>Within what days do you want this hobby to happen: 
                            <input  type="date" v-model="hobbyRanges.start" /> - <input type="date" v-model="hobbyRanges.end"/>
                            </label>
                            <!-- <DatePicker v-model="hobbyRanges" is-range /> -->
                            <br>
                            <input type="submit" value="Save Hobby" />  
                            <br>
                        </form>
                    </div>

                    <vue-cal :min-date="minDate" :max-date="maxDate" small hide-view-selector :selected-date="selectedDate" :events="listOfEvents" active-view="week" :disable-views="['years']"  style="max-width: 460px;height: 500px;"></vue-cal>
                </div>
            </div>
        </div>
    </div>
  </transition>


</template>



<script>
import VueCal from 'vue-cal'
import 'vue-cal/dist/vuecal.css'
import 'vue-cal/dist/drag-and-drop.js'


//import Datepicker from 'vue3-date-time-picker';
//import * as  msal from '@azure/msal-browser'

export default({
    name: 'AddMediaTask',
    components:{
        VueCal,
    },
    data(){
        return{
            isOpen: false,
            graphConfig: {
                graphMeEndpoint: "https://graph.microsoft.com/v1.0/me",
                graphMailEndpoint: "https://graph.microsoft.com/v1.0/me/messages",
                graphCalendarEndpoint: "https://graph.microsoft.com/v1.0/me/events?$select=subject,body,bodyPreview,organizer,attendees,start,end,location,recurrence"
            },
            accessToken: null,
            eventName: '',
            isOpenSC: false,
            selectedEvent: null,
            eventStartDate: '',
            eventEndDate: '',
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
            errors: [],
            recurEndDate: null,
            numOfOccurance: null,
            hobbyName: null,
            hobbyRanges: {
                start: '',
                end: ''
            },
            hobbyHours: '0',
            hobbyMinutes: '0'
        }
    },
    props: {
        text: String,
        color: String,
        listOfEvents: Array,
        selectedDate: Date,
    },
    methods: {
        addToDaysOfWeek(day){
            if(!this.selectedDayOfTheWeek.includes(day)){
                this.selectedDayOfTheWeek.push(day)
            }else{
                this.selectedDayOfTheWeek = this.selectedDayOfTheWeek.filter(element => element != day);
            }
        },
        printCurrentEvent(){
            console.log(this.eventStartDate);
            this.textToTime(this.eventStartDate);
        },
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
        async tempFunc(data){
            this.eventTimings.start = data.start;
            this.eventTimings.end = data.end;
            this.eventStartDate = this.getDateInFormat(data.start);
            this.eventEndDate = this.getDateInFormat(data.end);
        },
        async EventChange(data){
            this.eventTimings.end = data.end;
            this.eventEndDate = this.getDateInFormat(data.end);
        },
        async onEventCliclEventModal(event, deleteEventFunction){
            this.eventStartDate = this.getDateInFormat(event.start);
            if(!this.selectedEvent){
                this.selectedEvent = event;
                this.eventTimings.start = event.start;
                this.eventTimings.end = event.end;
                this.eventEndDate = this.getDateInFormat(event.end);
                this.deleteEventFunction = deleteEventFunction;
                return event;
            }else{
                this.deleteEventFunction();
                this.selectedEvent = event;
                this.eventTimings.start = event.start;
                this.eventTimings.end = event.end;
                this.eventEndDate = this.getDateInFormat(event.end);
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
        async pushHobby(e){
            e.preventDefault()
            this.errors = [];
            if(!this.hobbyName){
                this.errors.push("Name is required")
            }

            if(this.hobbyHours == null && this.hobbyMinutes == null){
                this.errors.push("Hobby length is required.")
            }
            
            console.log(this.hobbyRanges.start);
            if(this.hobbyRanges.start === '' || this.hobbyRanges.end === ''){
                this.errors.push("Fill in the dates which the hobby should happen")
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
            let startArr = this.hobbyRanges.start.split('-');
            let endArr = this.hobbyRanges.end.split('-');
            let startDate = new Date(startArr[0], startArr[1]-1, startArr[2]);
            let endDate = new Date(endArr[0], endArr[1]-1, endArr[2]);
            startDate.setHours(0, 0, 0, 0);
            endDate.setHours(23, 59, 59, 59);
            const newEvent = {
                title: this.hobbyName,
                selectedDate: {
                    start: startDate,
                    end: endDate
                },
                length: 60 * parseInt(this.hobbyHours) + parseInt(this.hobbyMinutes),
                class: 'sc',
                source: 'M'
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
        },
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
            this.eventStartDate = this.getDateInFormat(val.start);
            }
        }
    },
    computed: {
        minDate (){
            console.log(this.hobbyRanges);
            if(this.hobbyRanges.start == ""){
                return new Date().subtractDays(10)
            }else{
                return this.textToTime(this.hobbyRanges.start)
            }
        },
        maxDate (){
            if(this.hobbyRanges.end == ''){
                return new Date().addDays(10)
            }else{
                return this.textToTime(this.hobbyRanges.end)
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

.vuecal__cell--disabled {text-decoration: line-through;}
.vuecal__cell--before-min {color: #b6d6c7;}
.vuecal__cell--after-max {color: #008b8b;}
</style>