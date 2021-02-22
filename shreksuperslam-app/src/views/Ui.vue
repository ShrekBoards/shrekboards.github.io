<template>
    <div class="ui">
        <Sidebar :x="x"/>
        <Attack v-for="a in x[char]" :key="a" :attack="a"/>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import Sidebar from '@/components/Sidebar.vue';
import Attack from '@/components/Attack.vue';
import { data } from '@/data'

export default defineComponent({
  name: 'UI',
  components: {
    Sidebar,
    Attack,
  },
  props: {
    char: {
      required: true,
      type: String
    },
  },
  setup(props) {
      const x = ref(data);
      return { x };
  }
});
</script>