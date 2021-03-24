<template>
    <div class="download">
        <button v-on:click="buttonclick">{{ filename }}</button>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: "DownloadButton",
    props: {
        filename: {
            required: true,
            type: String,
        },
        data: {
            required: true,
            type: Uint8Array,
        }
    },
    setup(props) {
        /**
         * Button click handler.
         *
         * Serves the file data out as a file download.
         */
        function buttonclick() {
            const blob = new Blob([props.data], {type: "application/octet-stream"});
            const link = document.createElement("a");
            link.href = window.URL.createObjectURL(blob);
            link.download = props.filename;
            link.click();
        }

        return {
            buttonclick,
        }
    }
})
</script>

<style scoped>
</style>