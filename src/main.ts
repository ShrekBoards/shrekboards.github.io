import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

import("@/wasm/pkg/index").then(s3wasm => {
    const app = createApp({
        template: `
        <div id="nav">
            <router-link to="/">Home</router-link> |
            <router-link to="/about">About</router-link> |
            <router-link to="/upload">Upload</router-link> |
            <router-link to="/ui/red">UI</router-link>
        </div>
        <router-view/>
        `,
        provide: {
            wasmExtractCharacterAttacks: s3wasm.extract_character_attacks,
        },
    }).use(router).mount("#app");
});
//createApp(App).use(router).mount("#app");
/*
import('../pkg/index').then(s3web => {

    console.log("All modules loaded");

    // Set the handler for the submit button on the input form
    document.getElementById("theform")!.onsubmit = function(event) {
        // Get crap out of form
        const master_dir_filereader = (<HTMLInputElement>document.getElementById("masterdir"));
        const master_dat_filereader = (<HTMLInputElement>document.getElementById("masterdat"));
        if (master_dir_filereader != null && master_dat_filereader != null) {
            const master_dir = master_dir_filereader.files![0].arrayBuffer().then(buffer => new Uint8Array(buffer));
            const master_dat = master_dat_filereader.files![0].arrayBuffer().then(buffer => new Uint8Array(buffer));
            Promise.all([master_dir, master_dat]).then((values) => {
                let gameconsole = 0;
                document.getElementsByName("gameconsole").forEach((item, _) => {
                    if ((<HTMLInputElement>item).checked) {
                        gameconsole = parseInt((<HTMLInputElement>item).value);
                    }
                })
        
                // Submit to wasm function
                var x: ShrekSuperSlamCharacterAttackCollection = s3web.extract_character_attacks(values[1], values[0], gameconsole);
            });
        }
    
        // Disable the event resubmission
        if (event.preventDefault)
            event.preventDefault();
        else
            return false;
    }
});
*/