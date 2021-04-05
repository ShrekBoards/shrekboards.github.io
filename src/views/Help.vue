<template>
  <div class="help">
    <Navbar />
    <h1>This is a help page</h1>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Navbar from '@/components/Navbar.vue';

export default defineComponent({
  name: 'Help',
  components: {
      Navbar,
  },
});
</script>