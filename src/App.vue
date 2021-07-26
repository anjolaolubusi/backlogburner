<template>
  <div class="container">
    <Header />
    <router-view></router-view>
  </div>
</template>

<script>
import Header from './components/Header.vue'

import './assets/css/main.css'
export default {
  name: 'App',
  components: {
    Header
  },
  data() {
    return{
     
    }
  },
  methods: {
   
  },
  created() {

  }
}
</script>