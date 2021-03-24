<template>
    <div class="save">
        <button v-on:click="buttonclick">Generate</button>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject } from 'vue';

export default defineComponent({
    name: "SaveButton",
    setup() {
        const wasmRecreateGameFiles = inject("wasmRecreateGameFiles") as Function;
        const masterDatGlobal = inject("masterDat");
        const masterDirGlobal = inject("masterDir");
        const consoleGlobal = inject("console");
        const attacksGlobal = inject("attacks");
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
        }

        return {
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