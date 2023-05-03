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
        <h3>Advanced Mode</h3>
        <p>
          The attack objects are a collection of about 300 bytes, comprising of
          many fields, mostly in the form of decimal numbers. Which numbers
          correspond to what property of the attack is typically worked out
          through trial-and-error, by tweaking the values and seeing what
          happens. This is a long process for one person, especially when that
          one person doesn't actually play the game and doesn't realise some
          changes.
        </p>
        <p>
          Enter advanced mode. Enabling this gives access to some (not all!)
          'unknown' fields. These are fields that exist in the attack, but it
          is not known what they control. You can then modify these values,
          and work out what they do for yourself. If you manage to deduce what
          a field does, please open an
          <a href="https://github.com/ShrekBoards/shrek-superslam/issues">issue</a>
          on the main library explaining what the field does, or message kirby.
        </p>
        <p>
          Hopefully this will allow new fields to be discovered much sooner!
        </p>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Navbar from '@/components/Navbar.vue';

export default defineComponent({
  name: 'options-help-page',
  components: {
      Navbar,
  },
});
</script>