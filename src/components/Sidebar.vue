<template>
    <div class="sidebar sidenav-fixed">
        <ul>
            <router-link v-for="character in characters" :key="character" :to="`/characters/${character}`">
                <div v-if="selected == character" class="active">
                    <li>
                        <div class="valign-wrapper">
                            <img :src="`/icons/${character}.png`"/>
                            {{ character }}
                        </div>
                    </li>
                </div>
                <div v-else class="inactive">
                    <li>
                        <div class="valign-wrapper">
                            <img :src="`/icons/${character}.png`"/>
                            {{ character }}
                        </div>
                    </li>
                </div>
            </router-link>
        </ul>
    </div>
</template>

<script lang="ts">
import { defineComponent, PropType, ref } from 'vue';
import { ShrekSuperSlamCharacterAttackCollection } from '@/types';

export default defineComponent({
  name: 'Sidebar',
  props: {
    x: {
        required: true,
        type: Object as PropType<ShrekSuperSlamCharacterAttackCollection>,
    },
    selected: {
        required: true,
        type: String,
    }
  },
  setup(props) {
      const characters = ref(Object.keys(props.x).sort());
      return { characters };
  }
});
</script>

<style scoped>
.sidebar {
    position: fixed;
    height: 94vh;
    width: 23vw;
    overflow: hidden;
    overflow-y: none;
    border-right: 1px solid rgba(0,0,0,0.14);
}

.sidebar:hover {
    overflow-y: auto;
}

.active {
    background-color: #ee6e73;
    color: white;
}

a {
    color: black;
    text-decoration: none;
}

img {
    width: 50px;
    height: 50px;
    padding-right: 10px;
}

ul {
    padding: 0px;
    list-style-type: none;
    text-align: left;
}

li {
    padding: 5px;
}

.inactive:hover {
    background-color: #CCC;
}
</style>