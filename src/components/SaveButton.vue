<template>
    <div class="save" v-if="isVisible()">
        <a v-if="isEnabled()" class="waves-effect waves-light btn" v-on:click="buttonclick"><i class="material-icons left">build</i>Generate</a>
        <a v-else class="waves-effect waves-light btn disabled" v-on:click="buttonclick"><i class="material-icons left">build</i>Generate</a>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject, Ref } from 'vue';
import { useRouter } from 'vue-router';
import { ShrekSuperSlamCharacterAttackCollection } from '@/types';

export default defineComponent({
    name: "SaveButton",
    setup() {
        const wasmRecreateGameFiles = inject("wasmRecreateGameFiles") as Function;
        const masterDatGlobal = inject("masterDat") as Ref<Uint8Array>;
        const masterDirGlobal = inject("masterDir") as Ref<Uint8Array>;
        const consoleGlobal = inject("console") as Ref<number>;
        const attacksGlobal = inject("attacks") as Ref<ShrekSuperSlamCharacterAttackCollection>;
        const router = useRouter();
        /**
         * Function to determine if the button should render at all
         */
        function isVisible() {
            return Object.keys(attacksGlobal.value).length > 0;
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
        function buttonclick() {
            const newFiles = wasmRecreateGameFiles(
                masterDatGlobal.value,
                masterDirGlobal.value,
                consoleGlobal.value,
                attacksGlobal.value
            );

            masterDatGlobal.value = (newFiles[0] as Uint8Array);
            masterDirGlobal.value = (newFiles[1] as Uint8Array);

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