<template>
    <div class="sidebar">
        <ul>
            <a v-for="character in characters" :key="character" href="#">
                <li>
                    <img :src="`/icons/${character}.png`"/>
                    {{ character }}
                </li>
            </a>
        </ul>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';

export default defineComponent({
  name: 'Sidebar',
  props: {
    x: { type: Object, required: true }
  },
  setup(props) {
      const characters = ref(Object.keys(props.x));
      return { characters };
  }
});
</script>

<style scoped>
.sidebar {
    position: fixed;
    width: 270px;
    background-color: #EEE;
    font-size: 18px;
    overflow-y: scroll;
    top: 0;
    bottom: 0;
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

li:hover {
    background-color: #CCC;
}
</style>