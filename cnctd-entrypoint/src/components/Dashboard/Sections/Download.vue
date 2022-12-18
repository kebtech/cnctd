<template>
    <div class="download">
        <div class="heading">Downloads</div>
        <button @click="listBuilds">List Files</button>
        <button @click="download('ios')">iOS</button>
        <button @click="download('mac')">Mac</button>
    </div>
    </template>
    
    <script lang="ts">
    import { server } from '@/server/server';
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
            };

            const listBuilds = () => {
                server.get('builds', 'list', {test: "test"});
            }

            return {
                download,
                listBuilds
            }
        },
    
    })
    </script>
    
    <style scoped>
    
    </style>