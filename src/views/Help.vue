<template>
  <div class="help">
    <Navbar />
    <div class="row">
      <div class="col s6 offset-s3">
        <h2>Help</h2>
      </div>
    </div>
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