import { createApp, ref } from 'vue'
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
            attacks: ref({}),
            wasmExtractCharacterAttacks: s3wasm.extract_character_attacks,
        },
    }).use(router).mount("#app");
});