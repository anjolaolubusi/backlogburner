<template>
<transition name="modal">
        <div v-if="isOpen">
            <div class="overlay" @click.self="isOpen = false;">
                <div class="modal">
                    <form @submit="onSubmit" class="add-form">
                        <div class="form-control">
                            <label>Name: </label>
                            <input type="text" v-model="nameOfMedia" name="name" placeholder="Enter name" />
                        </div>

<vue-cal
    class="vuecal--date-picker"
    xsmall
    hide-view-selector
    :time="false"
    :transitions="false"
    active-view="month"
    :disable-views="['week']"
    @cell-focus="startDate = $event"
    >
</vue-cal>

<vue-cal
    class="vuecal--date-picker"
    xsmall
    hide-view-selector
    :time="false"
    :transitions="false"
    active-view="month"
    :disable-views="['week']"
    @cell-focus="endDate = $event"
    >
</vue-cal>

                        <input type="submit" value="Sumbit Task" />
                    </form>
                        <button @click="PrintSelectedDate()">PrintDate</button>

                </div>
            </div>
        </div>
    </transition>

<transition name="modalSC">
        <div v-if="isOpenSC">
            <div class="overlay" @click.self="isOpenSC = false;">
                <div class="modal">
                    <form @submit="pushSC" class="add-form">
                        <div class="form-control">
                            <label>Name: </label>
                            <input type="text" v-model="eventName" name="name" placeholder="Enter name" />
                        </div>

                        <div>
                            <label>Length of Event: </label>
                            <input type="number" v-model="lengthOfSC" name="lengthOfSC" />
                        </div>

                        <vue-cal
                            class="vuecal--date-picker"
                            xsmall
                            hide-view-selector
                            :time="false"
                            :transitions="false"
                            active-view="month"
                            :disable-views="['week', 'year', 'day']"
                            @cell-focus="selectedSCDate = $event"
                            >
                        </vue-cal>

                        <input type="submit" value="Sumbit Task" />
                    </form>
                </div>
            </div>
        </div>
    </transition>

    <button @click="isOpenSC=!isOpenSC" :style="{background: color}">Add Soft Constraint</button>
    <button @click="isOpen=!isOpen" :style="{background: color}">{{text}}</button>
    <button @click="PullFromOutlook()">Pull from Outlook</button>
</template>



<script>
import VueCal from 'vue-cal'
import 'vue-cal/dist/vuecal.css'
//import * as  msal from '@azure/msal-browser'

export default({
    name: 'AddMediaTask',
    components:{
        VueCal
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
                graphCalendarEndpoint: "https://graph.microsoft.com/v1.0/me/events?$select=subject,body,bodyPreview,organizer,attendees,start,end,location"
            },
            accessToken: null,
            selectedSCDate: null,
            eventName: '',
            isOpenSC: false,
            lengthOfSC: null
        }
    },
    props: {
        text: String,
        color: String
    },
    methods: {
        async onSubmit(e){
            e.preventDefault()
            if(!this.nameOfMedia){
                alert("Please add in a name")
                return
            }
            if(this.startDate == null){
                alert("Please select a start date")
                return
            }
            if(this.endDate == null){
                alert("Please select an end date")
                return
            }

            const newEvent = {title: this.nameOfMedia, 
            start: this.startDate,
            end: this.endDate,
            source: "M",
            class: 'hc'}
            this.$emit('add-cal-event', newEvent)
            this.nameOfMedia = '';
            this.startDate  = null;
            this.endDate = null;
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
                length: this.lengthOfSC,
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
            var cal_data = await this.callMSGraph(this.graphConfig.graphCalendarEndpoint, this.$route.params.accessToken)
            this.$emit('pull-outlook-event', cal_data)
            }
    },
    emits: ['add-cal-event', 'pull-outlook-event', 'add-sc'],
    mounted(){
        if(this.$route.params.accessToken == null){
            this.$router.push({ name: 'Login'});
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