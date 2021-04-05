<template>
    <div class="save">
        <a class="waves-effect waves-light btn" v-on:click="buttonclick"><i class="material-icons left">build</i>Generate</a>
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