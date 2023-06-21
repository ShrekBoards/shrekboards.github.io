<template>
    <div class="save" v-if="isVisible()">
        <div id="savemodal" class="modal">
            <div class="modal-content">
                <h3><i class="material-icons left medium">error</i>Error</h3>
                <p>
                    There was an error generating the new files. All fields are numeric, please ensure
                    all fields are only decimal numbers and try again. If the problem persists, try resetting
                    and starting again.
                </p>
            </div>
            <div class="modal-footer">
                <a class="modal-close waves-effect waves btn-flat"><i class="material-icons left">check</i>Accept</a>
            </div>
        </div>
        <a v-if="isEnabled()" class="waves-effect waves-light btn" v-on:click="buttonclick"><i class="material-icons left">build</i>Generate</a>
        <a v-else class="waves-effect waves-light btn disabled" v-on:click="buttonclick"><i class="material-icons left">build</i>Generate</a>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject, Ref } from 'vue';
import { useRouter } from 'vue-router';
import M from 'materialize-css';
import { ShrekSuperSlamCharacterCollection, ShrekSuperSlamStageCollection } from '@/types';

export default defineComponent({
    name: "save-button",
    setup() {
        type WasmRecreateGameFilesFunction = (
            masterDat: Uint8Array,
            masterDir: Uint8Array,
            console: number,
            characters: ShrekSuperSlamCharacterCollection,
            stages: ShrekSuperSlamStageCollection) => Uint8Array[];
        const wasmRecreateGameFiles = inject("wasmRecreateGameFiles") as WasmRecreateGameFilesFunction;
        const masterDatGlobal = inject("masterDat") as Ref<Uint8Array>;
        const masterDirGlobal = inject("masterDir") as Ref<Uint8Array>;
        const consoleGlobal = inject("console") as Ref<number>;
        const charactersGlobal = inject("characters") as Ref<ShrekSuperSlamCharacterCollection>;
        const stagesGlobal = inject("stages") as Ref<ShrekSuperSlamStageCollection>;
        const router = useRouter();
        /**
         * Function to determine if the button should render at all
         */
        function isVisible() {
            return Object.keys(charactersGlobal.value).length > 0;
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
            try {
                const newFiles = wasmRecreateGameFiles(
                    masterDatGlobal.value,
                    masterDirGlobal.value,
                    consoleGlobal.value,
                    charactersGlobal.value,
                    stagesGlobal.value,
                );

                masterDatGlobal.value = newFiles[0];
                masterDirGlobal.value = newFiles[1];

                router.push("/download");
            } catch (e) {
                // Pop up the error modal
                const saveModalElement = document.getElementById("savemodal");
                if (saveModalElement !== null) {
                    const modal = M.Modal.getInstance(saveModalElement);
                    modal.open();
                }
            }
        }

        return {
            isVisible,
            isEnabled,
            buttonclick,
        }
    },
    mounted() {
        const elems = document.querySelectorAll(".modal");
        M.Modal.init(elems, {});
    }
})
</script>

<style scoped>
.save {
    float: right;
}

.modal {
    color: black;
    overflow-y: unset;
}

.modal-content > p {
    line-height: 1.5;
}

.material-icons {
    font-size: unset;
}
</style>