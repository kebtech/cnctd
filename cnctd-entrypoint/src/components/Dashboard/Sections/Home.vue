<template>
<div class="home">
    <div class="heading">{{ dashboard.descriptions.home }}</div>
</div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { dashboard } from '../dashboard';

export default defineComponent({
    setup() {

        return {
            dashboard
        }
    },

})
</script>

<style scoped>
.home {
    position: relative;
    color: white;
}
.heading {
    font-size: 1.5rem;
    text-align: left;
    margin-left: 80px;
}
</style>