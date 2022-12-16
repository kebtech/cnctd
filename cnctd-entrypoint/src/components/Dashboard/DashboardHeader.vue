<template>
<div class="dashboard_header">
    <div class="logo"
        @click="dashboard.currentView = 'Home'">
        <img class="logo_img" src="../../assets/logo.png">
    </div>
    <div class="headings">
        <div v-for="heading in ['Download', 'Sign Up', 'WebApp']"
            :key="heading"
            class="heading"
            @click="heading === 'WebApp'? window.open('https://app.cnctd.world'): dashboard.currentView = heading"
            :class="{ 
                selected: dashboard.currentView === heading,
                web_app_button: heading === 'WebApp'
            }">
            {{heading}}
        </div>
    </div>
</div>
</template>

<script lang="ts">
import { defineComponent, inject } from 'vue';
import { dashboard } from './dashboard';

export default defineComponent({
    setup() {
        const store: any = inject('store');

        return {
            dashboard,
            store,
            window
        }
    }
})
</script>

<style scoped>
.dashboard_header {
    position: fixed;
    height: 80px;
    width: 100%;
    background: v-bind("store.colors.background");
    filter: drop-shadow(2px 4px 6px black);
    display: flex;
    flex-direction: row;
    z-index: 1;
    /* border-bottom: 1px solid #ffffff15; */
}
.logo {
    margin: auto;
    margin-top: 15px;
    margin-left: 10px;
    cursor: pointer;
}
.logo_img {
    border-radius: 50%;
    height: 50px;
    width: 50px;
}
.headings {
    display: flex;
    flex-direction: row;
    justify-content: right;
    color: white;
    padding: 10px;
}
.heading {
    opacity: .9;
    font-size: 1.2rem;
    margin: auto;
    font-family: 'Heebo';
    font-weight: 400;
    cursor: pointer;
    margin-left: 20px;
}
.heading:hover {
    opacity: 1;
    text-decoration: underline;
}

.selected {
    opacity: 1;
    text-decoration: underline;
}
.web_app_button {
    background: v-bind("store.colors.collection");
    padding: 10px;
    border-radius: 5px;
    opacity: 1;
}
.web_app_button:hover {
    /* background: none; */
    /* outline: 2px solid v-bind("store.colors.collection"); */
    outline: 2px solid #ffffff;
    /* color: v-bind("store.colors.collection"); */
    text-decoration: none;
}
</style>