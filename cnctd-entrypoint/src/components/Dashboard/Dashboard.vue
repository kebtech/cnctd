<template>
<div class="dashboard">
    <DashboardHeader/>
    <div class="padding"></div>
    <Home v-show="dashboard.currentView === 'Home'" />
    <Download v-if="dashboard.currentView === 'Download'" />
    <SignUp v-show="dashboard.currentView === 'Sign Up'" />
    <footer>
        <DashboardFooter />
    </footer>
</div>
</template>

<script lang="ts">
import { defineComponent, inject } from 'vue';
import { dashboard } from './dashboard';
import DashboardHeader from './DashboardHeader.vue';
import Home from './Sections/Home.vue';
import Download from './Sections/Download.vue';
import SignUp from './Sections/SignUp.vue';
import DashboardFooter from './DashboardFooter.vue';

export default defineComponent({
    setup() {
        const store: any = inject('store');

        return {
            store,
            dashboard
        }
    },
    components: {
        DashboardHeader,
        DashboardFooter,
        Home,
        Download,
        SignUp
    }
})

</script>

<style scoped>
.dashboard {
    background: v-bind('store.colors.background');
    position: absolute;
    left: 0;
    right: 0;
    top: 0;
    bottom: 0;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
}
.padding {
    position: relative;
    margin-top: 100px;
}
footer {
    margin-top: auto;
}
</style>