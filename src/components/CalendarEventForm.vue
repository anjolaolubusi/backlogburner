<template>
                    <n-form ref="formRef" :model="model" :rules="rules">
                <div style="display: flex; flex-wrap: wrap; gap: 2%; justify-content: space-between;">
                    <div>
                            <n-form-item label="What is the name of the event?" path="eventName">
                            <n-input v-model:value="model.eventName" type="text" style="max-width: 70%" placeholder="Enter Name Of Event"/> <br/>
                            </n-form-item>
                            <n-form-item label="When does an indiviual session of the event start?" path="eventStartDate">
                            <n-date-picker v-model:value="model.eventStartDate" style="max-width: 70%" :update-value-on-close="true" type="datetime" format="yyyy-MM-dd HH:mm" clearable />
                            </n-form-item>
                            <n-form-item label="When does an indiviual session of the event end?" path="eventEndDate">
                            <n-date-picker v-model:value="model.eventEndDate" style="max-width: 70%" :update-value-on-close="true"  format="yyyy-MM-dd HH:mm" type="datetime" clearable />
                            </n-form-item>
                            <n-form-item label="How often does this event repeat?">
                            <n-select v-model:value="model.eventRecurrance" style="max-width: 70%" :options="recurOptions"/>
                            </n-form-item>
                            <div v-if="model.eventRecurrance != 'Just Once'">
                                <h3>Recurrance Details</h3>
                                <div v-if="model.eventRecurrance == 'Daily'">
                                    <p>{{model.eventRecurrance}}</p>
                                    <n-input-number v-model:value="model.recur.eventFrequency" clearable />
                                </div>
                            </div>
                    </div>

                    <!-- <vue-cal @event-drop="onEventDrag" timeFormat="h:mm am" twelveHour :time-step="30"  resize-x small ref="addEventModal" @event-drag-create="onEventDragCreate($event)" @event-resizing="EventChange($event)" :on-event-create="onEventClickAndDragAddEventModal" :selected-date="selectedDate" :editable-events="{ title: false, drag: true, resize: true, delete: true, create: true}" :snap-to-time="5" :drag-to-create-threshold="15" :events="listOfEvents" active-view="day" :disable-views="['years', 'year',]"  style="max-width: 460px;height: 500px;" class="vuecal--full-height-delete"></vue-cal> -->
                </div>
                <!-- <input type="submit" value="Add Event To Calendar" style="background-color: green;font-size: 16px;color: white;" />   -->

                <n-button color="#008000" @click="pushEvent">{{SubmitBtnName}}</n-button>

                </n-form>
</template>

<script>
// import VueCal from 'vue-cal'
// import 'vue-cal/dist/vuecal.css'
// import 'vue-cal/dist/drag-and-drop.js'
import { inject, ref} from "vue";
import {
NButton, NInput, NFormItem, 
NDatePicker, NForm, NSelect,
NInputNumber} from 'naive-ui'

export default ({
    name: 'CalendarEventForm',
    components:{
        // VueCal,
        NButton,
        NInput,
        NFormItem,
        NDatePicker,
        NForm,
        NSelect,
        NInputNumber
    },  
    props: {
        SubmitBtnName: String,
        //listOfEvents: List of user's event
        listOfEvents: Array,
    },
    setup(_, {emit}) {
        //Imports Google Auth
        const Vue3GoogleOauth = inject("Vue3GoogleOauth");
        const formRef = ref(null);
        var formValue = ref({
                    eventName: null,
                    eventStartDate: null,
                    eventEndDate: null,
                    eventRecurrance: 'Just Once',
                    recur: {
                        eventFrequency: 0
                    }
            });
        return {
            formRef,
            model: formValue,
            rules: {
                    eventName: {
                        required: true,
                        message: "Please input your name",
                        trigger: ["input", "blur"]
                    },
                    eventStartDate: {
                        required: true,
                        type: "number",
                        message: "Please input when the event starts",
                        trigger: ["change", "blur"]
                    },
                    eventEndDate: {
                        required: true,
                        type: "number",
                        message: "Please input when the event ends",
                        trigger: ["change", "blur"]
                    },
            },
            //eventRecurrance: ref(null),
            recurOptions: [
                {
                    label: 'Just Once',
                    value: 'Just Once'
                },
                {
                    label: 'Daily',
                    value: 'Daily'
                },
                {
                    label: 'Weekly',
                    value: 'Weekly'
                },
            ],
        /**
         * Adds the Event to the main calendar
         * Pre-condition: The form in the Add Event modal is completed
         * Post-condition: The event is added to the main calendar on the Schedule View.
         *  Returns true if there is are any errors
         *  Emits newEvent (the new event being created) to the Schedule view under the add-cal-event emit event.
         * Parameters:
         *  e - Javascript event that represents the form submission
         */
        async pushEvent(e){
            e.preventDefault();
            formRef.value?.validate((errors) => { if(!errors){
            let formResp = formValue.value;            
            var newEvent = {title: formResp.eventName, 
                start: new Date(formResp.eventStartDate),
                end: new Date(formResp.eventEndDate),
                source: "M",
                class: 'hc',
            }
            
            console.log(newEvent);
            emit('add-cal-event', newEvent);
        }else{
            console.log(errors);
            console.log(formValue.value);
        }})  
        },
            Vue3GoogleOauth };
    },   
})
</script>

<style>

</style>