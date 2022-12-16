<template>
<div class="dashboard_footer">
    <div class="copyright"
        @click="window.open('https://kebtech.com')">
        {{'© ' + new Date().getFullYear() + ' kebTech, LLC'}}
    </div>
</div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    setup() {

        return {
            window
        }
    }
})

</script>

<style scoped>
.copyright {
    color: white;
    cursor: pointer;
    margin-bottom: 20px;
}
</style>