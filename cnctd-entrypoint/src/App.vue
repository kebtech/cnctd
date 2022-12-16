<template>
  <Dashboard />
</template>

<script lang="ts">
import { defineComponent, inject } from 'vue';
import Dashboard from './components/Dashboard/Dashboard.vue';


export default defineComponent({
  name: 'App',
  setup() {
    const store: any = inject('store');

    return {
      store
    }
  },
  components: {
    Dashboard
  }
});
</script>

<style>
@import url('https://fonts.googleapis.com/css2?family=Actor&family=Heebo:wght@400;500&family=Roboto:wght@300&family=Poppins:wght@300&family=Truculenta:opsz@12..72&family=Zen+Maru+Gothic:wght@400;700&family=Righteous&display=swap&family=Noto+Sans+Symbols');
body, html {
	height: 100%;
	width: 100%;
	margin: 0;
	overflow: hidden;
	padding: 0;
	overscroll-behavior: none;
	position: fixed;
	/* touch-action: none; */
	left: 0;
	right: 0;
	top: 0;
	bottom: 0;
}
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  /* margin-top: 60px; */
  /* background: v-bind('store.colors.background'); */
}
input {
    color: white;
    background: #333333;
    border: none;
    border-radius: 5px;
    padding: 5px;
}
input:focus-visible {
    outline: none;
}
</style>
