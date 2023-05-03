<template>
    <div class="sidebar sidenav-fixed">
        <SelectionTabs :gametype="gametype"/>
        <ul>
            <router-link v-for="item in items" :key="item" :to="`/${gametype}/${item}`">
                <div v-if="selected == item" class="active">
                    <li>
                        <div class="valign-wrapper">
                            <img :src="`/images/icons/${item}.png`"/>
                            {{ item }}
                        </div>
                    </li>
                </div>
                <div v-else class="inactive">
                    <li>
                        <div class="valign-wrapper">
                            <img :src="`/images/icons/${item}.png`"/>
                            {{ item }}
                        </div>
                    </li>
                </div>
            </router-link>
        </ul>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import SelectionTabs from '@/components/SelectionTabs.vue';

export default defineComponent({
  name: 'sidebar-component',
  components: {
    SelectionTabs,
  },
  props: {
    x: {
        required: true,
        type: Object,
    },
    gametype: {
        required: true,
        type: String,
    },
    selected: {
        required: true,
        type: String,
    }
  },
  setup(props) {
      const items = ref(Object.keys(props.x).sort());
      return { items };
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