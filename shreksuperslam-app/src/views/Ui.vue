<template>
    <div class="ui">
        <Sidebar :x="x"/>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import Sidebar from '@/components/Sidebar.vue';
import { data } from '@/data'

export default defineComponent({
  name: 'UI',
  components: {
    Sidebar,
  },
  setup() {
      const x = ref(data);
      return { x };
  }
});
</script>