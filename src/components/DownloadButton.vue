<template>
    <div class="download">
        <a class="waves-effect waves-light btn-large" v-on:click="buttonclick"><i class="material-icons left">file_download</i>{{ name }}</a>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: "download-button-binary",
    props: {
        filename: {
            required: true,
            type: String,
        },
        displayname: {
            required: false,
            type: String,
        },
        data: {
            required: false,
            type: Uint8Array,
        },
    },
    setup(props) {
        /**
         * Button click handler.
         *
         * Serves the file data out as a file download.
         */
        function buttonclick() {
            const blob = props.data !== undefined ?
                new Blob([props.data], {type: "application/octet-stream"}) :
                null;
            if (blob !== null) {
                const link = document.createElement("a");
                link.href = window.URL.createObjectURL(blob);
                link.download = props.filename;
                link.click();
            }
        }

        const name = props.displayname !== undefined ? props.displayname : props.filename;

        return {
            buttonclick,
            name,
        }
    },
})
</script>

<style scoped>
.btn-large {
  margin-right: 15px;
}
</style>