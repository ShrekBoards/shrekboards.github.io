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
import { defineComponent, inject, Ref } from 'vue';
import { useRouter } from 'vue-router';
import M from 'materialize-css';
import { ShrekSuperSlamCharacterAttackCollection } from "../types"

export default defineComponent({
    name: "ResetButton",
    setup() {
        const masterDatGlobal = inject("masterDat") as Ref<Uint8Array>;
        const masterDirGlobal = inject("masterDir") as Ref<Uint8Array>;
        const consoleGlobal = inject("console") as Ref<number>;
        const attacksGlobal = inject("attacks") as Ref<ShrekSuperSlamCharacterAttackCollection>;
        const router = useRouter();
        /**
         * Function to determine if the state has been cleared or not.
         */
        function stateCleared() {
            return (
                (masterDatGlobal.value.length == 0)
                && (masterDirGlobal.value.length == 0)
                && (consoleGlobal.value == 0)
                && (Object.keys(attacksGlobal.value).length == 0)
            );
        }

        /**
         * Button click handler.
         *
         * Deletes the current state and redirects to the upload form.
         */
        function buttonclick() {
            if (!stateCleared()) {
                masterDatGlobal.value = new Uint8Array();
                masterDirGlobal.value = new Uint8Array();
                consoleGlobal.value = 0;
                attacksGlobal.value = {};
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