<template>
  <div class="options">
    <Navbar />
    <div class="row">
      <div class="col s6 offset-s3">
        <h2>Options</h2>
        <p>
          There is a dropdown on the upload form that allows you to specify
          additional loading options. For the simplest experience, leave these
          as the default values, however these options can help with specific
          advanced features.
        </p>
        <h3>Loading previous changes</h3>
        <p>
          If you would like to work on a previous set of changes, rather than
          using the values provided by the MASTER.DAT / DIR files, then you can
          load in a generated JSON file with the attack data.
        </p>
        <p>
          You can get this file as a download when you generate the MASTER.DAT
          and MASTER.DIR files, click the "ATTACKS.JSON" button below those two
          to download it. The filename is the timestamp it was generated. Feel
          free to rename it, but do not modify the contents.
        </p>
        <p>
          When it comes to restarting the process, you can expand the option
          dropdown and upload your downloaded JSON file by clicking the
          ATTACK.JSON upload button.
        </p>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Navbar from '@/components/Navbar.vue';

export default defineComponent({
  name: 'Options',
  components: {
      Navbar,
  },
});
</script>