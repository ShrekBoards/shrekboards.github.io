<template>
  <div class="upload">
    <form id="theform" v-on:submit.prevent="formsubmit">
      <p>MASTER.DAT: <input class="file-selector" type="file" id="masterdat" name="files[]" /></p>
      <p>MASTER.DIR: <input class="file-selector" type="file" id="masterdir" name="files[]" /></p>
      <input type="radio" id="gamecube" name="gameconsole" value=0>
      <label for="console">Gamecube</label><br>
      <input type="radio" id="pc" name="gameconsole" value=1>
      <label for="console">PC</label><br>
      <input type="radio" id="ps2" name="gameconsole" value=2>
      <label for="console">PS2</label><br>
      <input type="radio" id="xbox" name="gameconsole" value=3>
      <label for="console">Xbox</label><br>
      <input type="submit">
    </form>
  </div>
</template>

<script lang="ts">
import { ShrekSuperSlamCharacterAttackCollection } from "../types"
import { defineComponent, inject } from 'vue';
import { useRouter } from 'vue-router';

export default defineComponent({
    name: 'UploadForm',
    setup() {
        const wasmExtractCharacterAttacks = inject("wasmExtractCharacterAttacks") as Function;
        const attacksGlobal = inject("attacks");
        const router = useRouter();
        /**
         * Form submit handler.
         *
         * Extracts files and fields from the form and submits them to the
         * WebAssembly function, that extracts the character attacks from the files
         * and returns them as a JSON object.
         *
         * Params:
         *   event: The formsubmit event passed from Vue.
         */
        function formsubmit(event: Event) {
            // Get crap out of form
            const masterDirFilereader = document.getElementById("masterdir") as HTMLInputElement;
            const masterDatFilereader = document.getElementById("masterdat") as HTMLInputElement;
            if (masterDirFilereader != null && masterDatFilereader != null &&
                masterDirFilereader.files != null && masterDatFilereader.files != null) {
                const masterDir = masterDirFilereader.files[0].arrayBuffer().then(buffer => new Uint8Array(buffer));
                const masterDat = masterDatFilereader.files[0].arrayBuffer().then(buffer => new Uint8Array(buffer));
                Promise.all([masterDir, masterDat]).then((values) => {
                    let gameconsole = 0;
                    document.getElementsByName("gameconsole").forEach((item, _) => {
                        if ((item as HTMLInputElement).checked) {
                            gameconsole = parseInt((item as HTMLInputElement).value);
                        }
                    })

                    // Submit to wasm function
                    const attacks = wasmExtractCharacterAttacks(
                        values[1],
                        values[0],
                        gameconsole
                    );
                    (attacksGlobal as any).value = attacks;

                    // Disable the event resubmission
                    if (event.preventDefault)
                        event.preventDefault();

                    router.push({
                        name: "UI",
                        params: { selectedCharacter: "red" }
                    });
                });
            }
        }

        return {
            formsubmit,
        };
    },
});
</script>