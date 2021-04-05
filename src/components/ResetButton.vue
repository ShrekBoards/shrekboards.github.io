<template>
    <div v-if="!stateCleared()" class="reset">
        <div id="modal1" class="modal">
            <div class="modal-content">
                <h3><i class="material-icons left medium">warning</i>Reset</h3>
                <p>This will reset all changes made and ask you to reupload the original game files. Are you sure?</p>
            </div>
            <div class="modal-footer">
                <a class="modal-close waves-effect waves-red btn-flat"><i class="material-icons left">close</i>Close</a>
                <a class="modal-close waves-effect waves-green btn-flat" v-on:click="buttonclick"><i class="material-icons left">check</i>Confirm</a>
            </div>
        </div>
        <a class="waves-effect waves-light btn red darken-1 modal-trigger" data-target="modal1"><i class="material-icons left">clear</i>Reset</a>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject } from 'vue';
import { useRouter } from 'vue-router';
import M from 'materialize-css';

export default defineComponent({
    name: "ResetButton",
    setup() {
        const masterDatGlobal = inject("masterDat");
        const masterDirGlobal = inject("masterDir");
        const consoleGlobal = inject("console");
        const attacksGlobal = inject("attacks");
        const router = useRouter();
        /**
         * Function to determine if the state has been cleared or not.
         */
        function stateCleared() {
            return (
                ((masterDatGlobal as any).value.length == 0)
                && ((masterDirGlobal as any).value.length == 0)
                && (((consoleGlobal as any).value) == 0)
                && (Object.keys((attacksGlobal as any).value).length == 0)
            );
        }

        /**
         * Button click handler.
         *
         * Deletes the current state and redirects to the upload form.
         */
        function buttonclick() {
            if (!stateCleared()) {
                (masterDatGlobal as any).value = new Uint8Array();
                (masterDirGlobal as any).value = new Uint8Array();
                (consoleGlobal as any).value = 0;
                (attacksGlobal as any).value = {};
                router.push({name: "Upload"});
            }
        }

        return {
            stateCleared,
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
.modal {
    color: black;
    overflow-y: unset;
}

.material-icons {
    font-size: unset;
}
</style>