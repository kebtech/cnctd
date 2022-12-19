<template>
<div class="download">
    <div class="heading">Downloads</div>
    <button @click="listBuilds">List Files</button>
    <div class="mac_builds">
        <div v-for="build in builds.mac"
            :key="build.filename">
            <button class="download" @click="download(build)">{{ build.filename }}</button>
        </div>
    </div>
</div>
</template>

<script lang="ts">
import { server } from '@/server/server';
import { defineComponent, reactive } from 'vue';

class S3File {
    last_modified: string;
    path: string;
    size: number;
    filename: string;
}

export default defineComponent({
    setup() {

        const download = (app: S3File) => {
            // const url = `https://cnctd.world/file/${app.path}/${app.filename}`
            const url = `http://192.168.1.7:5051/api/file?path=${app.path}/${app.filename}&bucket=cnctd-builds`
            const link = document.createElement("a");
            link.href = url;
            link.download = app.filename;
            link.click();
        };

        const builds = reactive({
            mac: [] as S3File[],
            ios: [] as S3File[],
        });

        const listBuilds = () => {
            server.post('builds', 'list')
                .then(r => {
                    console.log(r)
                    let files = r.data;
                    builds.mac.length = 0;
                    builds.ios.length = 0;
                    files.mac.forEach((f: S3File) => builds.mac.push(f))
                    files.ios.forEach((f: S3File) => builds.ios.push(f))
                })
        }

        return {
            download,
            listBuilds,
            builds
        }
    },

})
</script>

<style scoped>
.download {
    color: white;
}
button {
    background: #14141499;
    border-radius: 5px;
    color: white;
}
</style>