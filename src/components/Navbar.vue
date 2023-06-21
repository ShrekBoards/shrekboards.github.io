<template>
    <ul id="helpdropdown" class="dropdown-content">
        <li><router-link to="/help">Help</router-link></li>
        <li class="divider"></li>
        <li><router-link to="/help/gamecube">Gamecube</router-link></li>
        <li><router-link to="/help/pc">PC</router-link></li>
        <li class="divider"></li>
        <li><router-link to="/help/fields">Fields</router-link></li>
    </ul>
    <div class="nav navbar-fixed">
        <nav>
            <div class="nav-wrapper">
                <router-link v-if="characterJsonDefined" :to="`/characters/${firstCharacter}`">
                    <img src="/images/logo.png" class="logo" />
                    <div class="brand-logo">Shab'aint</div>
                </router-link>
                <router-link v-else to="/upload">
                    <img src="/images/logo.png" class="logo" />
                    <div class="brand-logo">Shab'aint</div>
                </router-link>
                <ul id="nav-mobile" class="right">
                    <li><ResetButton/></li>
                    <li><SaveButton/></li>
                    <li>
                        <router-link v-if="characterJsonDefined" :to="`/characters/${firstCharacter}`"><i class="material-icons left">home</i>Home</router-link>
                        <router-link v-else to="/upload"><i class="material-icons left">home</i>Home</router-link>
                    </li>
                    <li><a class="dropdown-trigger" data-target="helpdropdown"><i class="material-icons left">help_outline</i>Help<i class="material-icons right">arrow_drop_down</i></a></li>
                    <li><router-link to="/about"><i class="material-icons left">info_outline</i>About</router-link></li>
                    <li><router-view/></li>
                </ul>
            </div>
        </nav>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject, Ref } from 'vue';
import ResetButton from '@/components/ResetButton.vue';
import SaveButton from '@/components/SaveButton.vue';
import M from 'materialize-css';
import { ShrekSuperSlamCharacterCollection } from "../types"

export default defineComponent({
  name: 'navbar-component',
  components: {
      ResetButton,
      SaveButton,
  },
  setup() {
    const charactersGlobal = inject("characters") as Ref<ShrekSuperSlamCharacterCollection>;
    const characterJsonDefined = (Object.keys(charactersGlobal.value).length > 0);
    let firstCharacter = "";
    if (characterJsonDefined) {
        firstCharacter = Object.keys(charactersGlobal.value).sort()[0]
    }
    return {
        characterJsonDefined,
        firstCharacter
    };
  },
  mounted() {
    const elems = document.querySelectorAll(".dropdown-trigger");
    M.Dropdown.init(elems, {
        closeOnClick: true,
        constrainWidth: true,
        coverTrigger: false,
    });
  }
});
</script>

<style scoped>
img.logo {
    width: 54px;
    margin: 5px;
}
</style>