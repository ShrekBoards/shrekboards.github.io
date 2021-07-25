import { createApp, ref } from 'vue'
import router from './router'
import 'materialize-css/dist/css/materialize.min.css'
import 'material-design-icons/iconfont/material-icons.css'

import("@/wasm/pkg/index").then(s3wasm => {
    createApp({
        template: `
        <router-view/>
        `,
        provide: {
            masterDat: ref(new Uint8Array()),
            masterDir: ref(new Uint8Array()),
            console: ref(0),
            attacks: ref({}),
            advancedModeEnabled: ref(false),
            wasmExtractCharacterAttacks: s3wasm.extract_character_attacks,
            wasmRecreateGameFiles: s3wasm.recreate_game_files,
        },
    }).use(router).mount("#app");
});