<!-- 
This file represents the root VueJS component.
Creator: Anjolaoluwa Olubusi
-->

<template>
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@mdi/font@5.8.55/css/materialdesignicons.min.css">
<link rel="stylesheet" href="https://use.fontawesome.com/releases/v5.2.0/css/all.css">

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
