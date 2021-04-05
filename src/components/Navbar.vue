<template>
    <div class="nav navbar-fixed">
        <nav>
            <div class="nav-wrapper">
                <router-link v-if="characterJsonDefined" :to="`/characters/${firstCharacter}`" class="brand-logo">Shab'aint</router-link>
                <router-link v-else to="/upload" class="brand-logo">Shab'aint</router-link>
                <ul id="nav-mobile" class="right hide-on-med-and-down">
                    <li><router-link to="/help">Help</router-link></li>
                    <li><router-link to="/about">About</router-link></li>
                    <li><router-view/></li>
                </ul>
            </div>
        </nav>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject } from 'vue';

export default defineComponent({
  name: 'Navbar',
  setup() {
    const attacksGlobal = inject("attacks");
    const characterJsonDefined = (Object.keys((attacksGlobal as any).value).length > 0);
    let firstCharacter = "";
    if (characterJsonDefined) {
        firstCharacter = Object.keys((attacksGlobal as any).value).sort()[0]
    }
    return {
        characterJsonDefined,
        firstCharacter
    };
  }
});
</script>