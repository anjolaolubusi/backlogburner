<!-- 
This file represents the root VueJS component.
Creator: Anjolaoluwa Olubusi
-->

<template>
  <div class="container fullHeight">
<!-- Places Header Component -->
    <Header />
<!-- Adds View to show current page -->
    <router-view></router-view>
  </div>
</template>


<script>
//Imports Header components
import Header from './components/Header.vue'

//Imports Backlog Burner's css files
import './assets/css/main.css'


export default {
  //Name of Component
  name: 'App',
  //List imported component
  components: {
    Header
  },
  //List of component variables
  data() {
    return{
     
    }
  },
  //List of component methods
  methods: {
   
  }
}
</script>
