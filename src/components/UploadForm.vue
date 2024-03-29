<template>
  <div class="upload">
    <form id="theform" v-on:submit.prevent="formsubmit">
      <!-- Card content, the upload form -->
      <div class="card darken-1">
        <div class="file-field input-field">
          <div class="btn">
            <span>MASTER.DAT</span>
            <input type="file" id="masterdat" name="files[]" required />
          </div>
          <div class="file-path-wrapper">
            <input class="file-path validate" type="text">
          </div>
        </div>
        <div class="file-field input-field">
          <div class="btn">
            <span>MASTER.DIR</span>
            <input type="file" id="masterdir" name="files[]" required />
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

        <!-- Advanced mode toggle -->
        <label>
          <input id="advanced" class="filled-in" type="checkbox"/>
          <span>Show unknown fields</span>
        </label>

        <!-- Rainbow preload spinner -->
        <div id="upload-preloader" class="preloader-wrapper big loader">
          <div class="spinner-layer spinner-blue">
            <div class="circle-clipper left">
              <div class="circle"></div>
            </div><div class="gap-patch">
              <div class="circle"></div>
            </div><div class="circle-clipper right">
              <div class="circle"></div>
            </div>
          </div>
          <div class="spinner-layer spinner-red">
            <div class="circle-clipper left">
              <div class="circle"></div>
            </div><div class="gap-patch">
              <div class="circle"></div>
            </div><div class="circle-clipper right">
              <div class="circle"></div>
            </div>
          </div>
          <div class="spinner-layer spinner-yellow">
            <div class="circle-clipper left">
              <div class="circle"></div>
            </div><div class="gap-patch">
              <div class="circle"></div>
            </div><div class="circle-clipper right">
              <div class="circle"></div>
            </div>
          </div>
          <div class="spinner-layer spinner-green">
            <div class="circle-clipper left">
              <div class="circle"></div>
            </div><div class="gap-patch">
              <div class="circle"></div>
            </div><div class="circle-clipper right">
              <div class="circle"></div>
            </div>
          </div>
        </div>

        <!-- Submit button, in the card footer -->
        <div class="card-action">
          <button class="waves-effect waves-light btn" type="submit" name="action">
            <i class="material-icons left">file_upload</i>Submit
          </button>
        </div>
      </div>
    </form>
    <div id="error">
      <i class="material-icons left">error</i>
      <i>Error uploading files. Check you uploaded the correct files, and selected the correct console.</i>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, inject, Ref } from 'vue';
import { useRouter } from 'vue-router';
import { ShrekSuperSlamCharacterCollection, ShrekSuperSlamStageCollection } from "../types"
import M from 'materialize-css';

export default defineComponent({
    name: 'upload-form',
    setup() {
        type WasmExtractCharactersFunction = (
            masterDat: Uint8Array,
            masterDir: Uint8Array,
            console: number) => ShrekSuperSlamCharacterCollection;

        type WasmExtractStagesFunction = (
          masterDat: Uint8Array,
          masterDir: Uint8Array,
          console: number) => ShrekSuperSlamStageCollection;
          
        const wasmExtractCharacters = inject("wasmExtractCharacters") as WasmExtractCharactersFunction;
        const wasmExtractStages = inject("wasmExtractStages") as WasmExtractStagesFunction;
        const masterDatGlobal = inject("masterDat") as Ref<Uint8Array>;
        const masterDirGlobal = inject("masterDir") as Ref<Uint8Array>;
        const consoleGlobal = inject("console") as Ref<number>;
        const charactersGlobal = inject("characters") as Ref<ShrekSuperSlamCharacterCollection>;
        const stagesGlobal = inject("stages") as Ref<ShrekSuperSlamStageCollection>;
        const advancedModeEnabledGlobal = inject("advancedModeEnabled") as Ref<boolean>;
        const router = useRouter();

        /**
         * Enables the preloader element.
         */
        function enablePreloader() {
          const preloader = document.getElementById("upload-preloader");
          if (preloader !== null) {
            preloader.classList.add("active");
          }
        }

        /**
         * Disables the preloader element.
         */
        function disablePreloader() {
          const preloader = document.getElementById("upload-preloader");
          if (preloader !== null) {
            preloader.classList.remove("active");
          }
        }

        /**
         * Resets the program state.
         */
        function resetState() {
          masterDatGlobal.value = new Uint8Array();
          masterDirGlobal.value = new Uint8Array();
          consoleGlobal.value = 0;
          charactersGlobal.value = {};
          stagesGlobal.value = {};
        }

        /**
         * Enables the error message.
         */
        function displayErrorMessage() {
          const error = document.getElementById("error");
          if (error !== null) {
            error.style.visibility = "visible";
          }
        }

        /**
         * Actions to run in the event of an error.
         */
        function error() {
            disablePreloader();
            resetState();
            displayErrorMessage();
        }

        /**
         * Function to call once the two files from the form have completed loading.
         *
         * Transforms the loaded bytes to the character and stages JSON, and stores the
         * global state for use in the rest of the application.
         *
         * Params:
         *   masterDat: The loaded bytes of the MASTER.DAT file.
         *   masterDir: The loaded bytes of the MASTER.DIR file.
         *   gameconsole: An integer between 1 and 4 representing the selected console version.
         *   advancedModeEnabled: True to enable advanced mode (shows unknown fields), false to not.
         */
        function fileLoadCompleteCall(
          masterDat: Uint8Array,
          masterDir: Uint8Array,
          gameconsole: number,
          advancedModeEnabled: boolean,
        ) {
          try {
            // Save off the form inputs
            masterDatGlobal.value = masterDat;
            masterDirGlobal.value = masterDir;
            consoleGlobal.value = gameconsole;
            advancedModeEnabledGlobal.value = advancedModeEnabled;

            // Read the characters and stages from the MASTER.DAT and DIR pair
            // using the wasm functions.
            charactersGlobal.value = wasmExtractCharacters(
                masterDat,
                masterDir,
                gameconsole
            );

            stagesGlobal.value = wasmExtractStages(
                masterDat,
                masterDir,
                gameconsole
            );
            
            // Get the first character alphabetically to navigate to
            const characterNames = Object.keys(charactersGlobal.value);
            characterNames.sort();

            if (characterNames.length > 0) {
              // Navigate to the main UI
              router.push({
                  name: "CharacterUi",
                  params: { selected: characterNames[0] }
              });
            } else {
              error();
            }
          } catch (e) {
            error();
          }
        }

        /**
         * Form submit handler.
         *
         * Extracts files and fields from the form and submits them to the
         * WebAssembly function, that extracts the characters from the files
         * and returns them as a JSON object.
         *
         * Params:
         *   event: The formsubmit event passed from Vue.
         */
        function formsubmit() {
            // Enable the preloader
            enablePreloader();

            // Get the console version from the form
            let gameconsole = 0;
            document.getElementsByName("gameconsole").forEach((item, ) => {
                if ((item as HTMLInputElement).checked) {
                    gameconsole = parseInt((item as HTMLInputElement).value);
                }
            });

            // Check if advanced mode is enabled
            const advancedModeEnabled = (document.getElementById("advanced") as HTMLInputElement)?.checked ?? false;

            // Load the files out of the form
            const masterDirFilereader = document.getElementById("masterdir") as HTMLInputElement;
            const masterDatFilereader = document.getElementById("masterdat") as HTMLInputElement;
            if (masterDirFilereader !== null && masterDatFilereader !== null &&
                  masterDirFilereader.files !== null && masterDatFilereader.files !== null) {

                // The MASTER.DAT and MASTER.DIR are mandatory, read them out.
                const masterDir = masterDirFilereader.files[0].arrayBuffer().then(buffer => new Uint8Array(buffer));
                const masterDat = masterDatFilereader.files[0].arrayBuffer().then(buffer => new Uint8Array(buffer));

                // With all the files read (or they will be once the associated
                // promise resolves), do the final bit of processing.
                Promise.all([masterDir, masterDat])
                  .then((values) => fileLoadCompleteCall(values[1], values[0], gameconsole, advancedModeEnabled));
            }
        }

        return {
            formsubmit,
        };
    },
    mounted() {
      const elems = document.querySelectorAll(".collapsible");
      M.Collapsible.init(elems, {});
    },
});
</script>

<style scoped>
.card {
  padding-left: 5px;
  padding-right: 5px;
  padding-top: 1px;
}

.loader {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  margin: auto;
}

#error {
  color: red;
  visibility: hidden;
}

.iconadd {
  display:inline-block ;
}

.iconremove {
  display:none !important;
}

li.active .collapsible-header .material-icons.iconadd{
  display: none;
}

li.active .collapsible-header .material-icons.iconremove{
  display: inline-block !important;
}

input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

/* Firefox */
input[type=number] {
  -moz-appearance: textfield;
}
</style>