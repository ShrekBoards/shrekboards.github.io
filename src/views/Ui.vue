<template>
    <div class="ui">
        <Navbar/>
        <Sidebar :x="x"/>
        <Attack v-for="a in x[char]" :key="a" :attack="a"/>
    </div>
</template>

<script lang="ts">
import { defineComponent, reactive } from 'vue';
import Sidebar from '@/components/Sidebar.vue';
import Attack from '@/components/Attack.vue';
import Navbar from '@/components/Navbar.vue'
import { data } from '@/data'

export default defineComponent({
  name: 'UI',
  components: {
    Sidebar,
    Attack,
    Navbar,
  },
  props: {
    char: {
      required: true,
      type: String
    },
  },
  setup(props) {
      const x = reactive(data);
      return { x };
  }
});
</script>