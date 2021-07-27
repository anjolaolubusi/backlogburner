<template>
    <div class="mid">
      <div class="calendar-area">
          <div class="calendar">
            <vue-cal :time="true" active-view="day" :disable-views="['years']" today-button :events="listOfEvents" />
          </div>
      </div>
      <div class="upcoming">
        <button @click="getScheduleEvents()">GetScheduleTest</button>
      <AddMediaTask :mediaTypes="mediaTypes" @add-media="addMediaTask" text="Add Task" color="green"/>
        <h2>To-Do</h2>
        <MediaList @toggle-remainder="toggleRemainder" @delete-task="deleteTask" :sourceData="mediaList" />
      </div>
    </div>
</template>

<script>
import MediaList from '../components/MediaList.vue'
import AddMediaTask from '../components/AddMediaTask.vue'
import {DatabaseAPI}  from '../api-common'
import VueCal from 'vue-cal'
import 'vue-cal/dist/vuecal.css'

export default {
  name: 'Schedule',
  components: {
    MediaList,
    AddMediaTask,
    VueCal
  },
  props: {
      username: String
  },
  data() {
    return{
      mediaList: [],
      mediaTypes: [],
      userID: 1,
      showDate: new Date(),
      listOfEvents: []
    }
  },
  methods: {
    async addMediaTask(newMedia){
      await DatabaseAPI.post("media", {
        userID: this.userID,
        mediaTypeId: newMedia.type_id,
        reminder: newMedia.reminder,
        mediaName: newMedia.name,
        startDate: `${newMedia.startDate.getFullYear()}-${newMedia.startDate.getMonth()+1}-${newMedia.startDate.getDate()}`,
        endDate: `${newMedia.startDate.getFullYear()}-${newMedia.startDate.getMonth()+1}-${newMedia.startDate.getDate()+1}`
        })
        .then((response) => {
          var newItem = JSON.parse(response.config.data);
          newItem["id"] = response.data.insertId;
          this.mediaList.push(newItem);
        })
        .then(this.getMediaTask())
        .catch((err) => alert(err))
    },
    async getMediaTask(){
      await DatabaseAPI.get("media", {params: {name: 'anjola'}})
        .then(response => this.mediaList = response.data)
        .then(() => {
            this.listOfEvents.length = 0
            for(var i = 0, size=this.mediaList.length; i < size; i++){
              this.listOfEvents.push({
                start: new Date(this.mediaList[i].StartDate),
                end: new Date (this.mediaList[i].EndDate),
                title: this.mediaList[i].mediaName,
                content: '<i class="v-icon material-icons">shopping_cart</i>',
                class: 'leisure',
                mediaId: this.mediaList[i].id
              })          
        }})
        .catch((err) => alert(err))
    },
    async deleteTask(id){
      if( confirm("Are you sure?")){
        await DatabaseAPI.delete("media", {params: {id: id}})
        .then(res => console.log(res))
        .then(this.mediaList = this.mediaList.filter((task) => task.id !== id))
        .then(this.listOfEvents = this.listOfEvents.filter((EV) => EV.id !== id))
        .then(this.getScheduleEvents())
        .catch( (err) => alert(err))
      }
    },
    async toggleRemainder(id){
      await DatabaseAPI.post("remind", {id: id})
      .then(
        this.mediaList = this.mediaList.map((media) => media.id === id ? {...media, remind: !media.remind}: media)
      )
      .catch((err) => alert(err))
    },
    async getMediaTypes(){
      await DatabaseAPI.get("mediatype")
        .then( (response) => this.mediaTypes = response.data)
    },
    setShowDate(d) {
				this.showDate = d;
		},
    async getScheduleEvents(){
        //loop over mediaList
        //Get schedule items from mediaList
        let promise = new Promise((resolve, reject) => {
          try{
            this.listOfEvents.length = 0
            for(var i = 0, size=this.mediaList.length; i < size; i++){
              this.listOfEvents.push({
                start: new Date(this.mediaList[i].StartDate),
                end: new Date (this.mediaList[i].EndDate),
                title: this.mediaList[i].mediaName,
                content: '<i class="v-icon material-icons">shopping_cart</i>',
                class: 'leisure',
                mediaId: this.mediaList[i].id
              });
            }
            resolve("done");
          }catch(error){
            reject(error);
          }
        });

        await promise;
    }
  },
  created() {
   /* if(this.$route.params.userID === undefined && typeof this.$route.params.userID == 'undefined'){
        this.$router.push({name: 'Login'})
        return
    }*/
    this.getMediaTask();
    this.getMediaTypes();
    //this.userID = this.$route.params.userID;
  }
}
</script>