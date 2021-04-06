<template>
    <div class="save" v-if="isVisible()">
        <a v-if="isEnabled()" class="waves-effect waves-light btn" v-on:click="buttonclick"><i class="material-icons left">build</i>Generate</a>
        <a v-else class="waves-effect waves-light btn disabled" v-on:click="buttonclick"><i class="material-icons left">build</i>Generate</a>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject } from 'vue';
import { useRouter } from 'vue-router';

export default defineComponent({
    name: "SaveButton",
    setup() {
        const wasmRecreateGameFiles = inject("wasmRecreateGameFiles") as Function;
        const masterDatGlobal = inject("masterDat");
        const masterDirGlobal = inject("masterDir");
        const consoleGlobal = inject("console");
        const attacksGlobal = inject("attacks");
        const router = useRouter();
        /**
         * Function to determine if the button should render at all
         */
        function isVisible() {
            return Object.keys((attacksGlobal as any).value).length > 0;
        }

        /**
         * Function to determine if the button should be enabled. 
         */
        function isEnabled() {
            return !router.currentRoute.value.path.startsWith("/download");
        }

        /**
         * Button click handler
         */
        function buttonclick(event: Event) {
            const newFiles = wasmRecreateGameFiles(
                (masterDatGlobal as any).value,
                (masterDirGlobal as any).value,
                (consoleGlobal as any).value,
                (attacksGlobal as any).value
            );

            (masterDatGlobal as any).value = (newFiles[0] as Uint8Array);
            (masterDirGlobal as any).value = (newFiles[1] as Uint8Array);

            router.push("/download");
        }

        return {
            isVisible,
            isEnabled,
            buttonclick,
        }
    }
})
</script>

<style scoped>
.save {
    float: right;
}
</style>