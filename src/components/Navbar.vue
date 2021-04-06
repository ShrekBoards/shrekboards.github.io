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
                <router-link v-if="characterJsonDefined" :to="`/characters/${firstCharacter}`" class="brand-logo">Shab'aint</router-link>
                <router-link v-else to="/upload" class="brand-logo">Shab'aint</router-link>
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
import { defineComponent, inject } from 'vue';
import ResetButton from '@/components/ResetButton.vue';
import SaveButton from '@/components/SaveButton.vue';
import M from 'materialize-css';

export default defineComponent({
  name: 'Navbar',
  components: {
      ResetButton,
      SaveButton,
  },
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
  },
  mounted() {
    const elems = document.querySelectorAll(".dropdown-trigger");
    M.Dropdown.init(elems, {
        constrainWidth: true,
        coverTrigger: false,
    });
  }
});
</script>