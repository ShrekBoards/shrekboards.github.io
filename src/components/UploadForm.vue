<template>
  <div class="upload">
    <form id="theform">
      <div class="card darken-1">
        <div class="file-field input-field">
          <div class="btn">
            <span>MASTER.DAT</span>
            <input type="file" id="masterdat" name="files[]" />
          </div>
          <div class="file-path-wrapper">
            <input class="file-path validate" type="text">
          </div>
        </div>
        <div class="file-field input-field">
          <div class="btn">
            <span>MASTER.DIR</span>
            <input type="file" id="masterdir" name="files[]" />
          </div>
          <div class="file-path-wrapper">
            <input class="file-path validate" type="text">
          </div>
        </div>
        <p>
          <label>
            <input type="radio" id="gamecube" name="gameconsole" value=0 checked/>
            <span>Gamecube</span>
          </label>
        </p>
        <p>
          <label>
            <input type="radio" id="pc" name="gameconsole" value=1/>
            <span>PC</span>
          </label>
        </p>
        <p>
          <label>
            <input type="radio" id="ps2" name="gameconsole" value=2/>
            <span>PS2</span>
          </label>
        </p>
        <p>
          <label>
            <input type="radio" id="xbox" name="gameconsole" value=3/>
            <span>Xbox</span>
          </label>
        </p>
        <div class="card-action">
          <a class="waves-effect waves-light btn" v-on:click="formsubmit"><i class="material-icons left">file_upload</i>Submit</a>
        </div>
      </div>
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
        const masterDatGlobal = inject("masterDat");
        const masterDirGlobal = inject("masterDir");
        const consoleGlobal = inject("console");
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
                    });

                    // Save off the form inputs
                    (masterDatGlobal as any).value = values[1];
                    (masterDirGlobal as any).value = values[0];
                    (consoleGlobal as any).value = gameconsole;

                    // Submit to wasm function
                    const attacks = wasmExtractCharacterAttacks(
                        values[1],
                        values[0],
                        gameconsole
                    ) as ShrekSuperSlamCharacterAttackCollection;
                    (attacksGlobal as any).value = attacks;

                    // Get the first character alphabetically to navigate to
                    const characterNames = Object.keys(attacks);
                    characterNames.sort();

                    // Navigate to the main UI
                    router.push({
                        name: "UI",
                        params: { selectedCharacter: characterNames[0] }
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

<style scoped>
.card {
  padding-left: 5px;
  padding-right: 5px;
  padding-top: 1px;
}
</style>