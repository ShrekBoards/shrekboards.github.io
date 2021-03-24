import { createApp, ref, resolveDirective } from 'vue'
import router from './router'

import("@/wasm/pkg/index").then(s3wasm => {
    createApp({
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
            masterDat: ref(new Uint8Array()),
            masterDir: ref(new Uint8Array()),
            console: ref(0),
            attacks: ref({}),
            wasmExtractCharacterAttacks: s3wasm.extract_character_attacks,
            wasmRecreateGameFiles: s3wasm.recreate_game_files,
        },
    }).use(router).mount("#app");
});