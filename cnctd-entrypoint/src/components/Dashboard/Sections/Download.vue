<template>
    <div class="download">
        <div class="heading">Downloads</div>
        <button @click="download('ios')">iOS</button>
        <button @click="download('mac')">Mac</button>
    </div>
    </template>
    
    <script lang="ts">
    import { defineComponent } from 'vue';
    
    export default defineComponent({
        setup() {
            const download = (app: string) => {
                const dataurl = app === 'ios'? '': '';
                const filename = app === 'mac'? '': '';

                const link = document.createElement("a");
                link.href = dataurl;
                link.download = filename;
                link.click();
            }

            return {
                download
            }
        },
    
    })
    </script>
    
    <style scoped>
    
    </style>