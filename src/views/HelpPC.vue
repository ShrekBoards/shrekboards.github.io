<template>
  <div class="pc">
    <Navbar />
    <div class="row">
      <div class="col s6 offset-s3">
        <h2>PC</h2>
        <p>
            Working with the PC version of the game is much easier than the <router-link to="/help/gamecube">Gamecube</router-link>
            version, which makes it ideal for quickly testing changes.
        </p>

        <h5>Upload the game files</h5>
        <ol>
            <li>
                Go to the game's install folder. If you installed to the default location, this will be
                <samp><nobr>C:\Program Files (x86)\Activision\Shrek SuperSlam</nobr></samp>.
            </li>
            <li>
                Make a copy of the MASTER.DAT and MASTER.DIR files and put them somewhere safe. These are the files we
                will be overwriting, so you need to do this to keep the originals!
            </li>
            <li>On the homepage of this site, upload the MASTER.DAT and MASTER.DIR files from the folder.</li>
            <li>Select the console as 'PC'.</li>
            <li>Click 'Submit'.</li>
        </ol>
        <img class="responsive-img" src="/images/help/pc1.png" alt="A screenshot of the MASTER.DAT and MASTER.DIR files being uploaded to the tool."/>

        <h5>Use the tool to make changes</h5>
        <ol>
            <li>
                Make the changes to the characters attacks as you desire. For information on what the properties
                all mean, see the <router-link to="/images/help/fields">Fields</router-link> documentation.
            </li>
            <li>Changes persist as you make them.</li>
            <li>When you are done making changes, click 'Generate'.</li>
        </ol>
        <img class="responsive-img" src="/images/help/generic1.png" alt="A screenshot of the extracted MASTER.DAT and MASTER.DIR files being uploaded to the tool."/>

        <h5>Replace the original game files</h5>
        <ol>
            <li>Download the generated MASTER.DAT and MASTER.DIR files.</li>
            <li>
                Go to the game's folder, and replace the original MASTER.DAT and MASTER.DIR files
                with the ones you just downloaded.
            </li>
            <li>You should be able to run the game with your new files try out your changes!</li>
        </ol>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Navbar from '@/components/Navbar.vue';

export default defineComponent({
  name: 'PC',
  components: {
      Navbar,
  }
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

ol > li {
  padding: 5px;
}
</style>