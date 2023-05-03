<template>
  <div class="help">
    <Navbar />
    <div class="row">
      <div class="col s6 offset-s3">
        <h2>Help</h2>
        <p>For help with getting the files, using the tool and replacing the files, see specific console documentation:</p>
        <ul class="list">
          <li><router-link to="/help/gamecube">Gamecube</router-link></li>
          <li><router-link to="/help/pc">PC</router-link></li>
        </ul>
        <p>For help on the attack fields, see the <router-link to="/help/fields">Fields</router-link> documentation.</p>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Navbar from '@/components/Navbar.vue';

export default defineComponent({
  name: 'help-page',
  components: {
      Navbar,
  },
});
</script>

<style scoped>
ul.list {
  list-style: inside;
}

.list > li {
  list-style: inside;
  padding: 5px;
}
</style>