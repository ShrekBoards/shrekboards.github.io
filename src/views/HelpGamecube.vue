<template>
  <div class="gamecube">
    <Navbar />
    <div class="row">
      <div class="col s6 offset-s3">
        <h2>Gamecube</h2>
        <p>To get the files from and repackage the Gamecube ISO, you will need the <a href="https://www.romhacking.net/utilities/619/">GCRebuilder</a> tool.</p>
        <h5>Extract the SuperSlam ISO</h5>
        <ol>
          <li>Open GCRebuilder.</li>
          <li>Click Image &rarr; Open, and select your Shrek SuperSlam Gamecube ISO.</li>
          <li>Right click on 'root' (with the picture of the disk) in the right-hand pane and click 'Export'.</li>
          <li>Export the ISO to a folder on your computer.</li>
          <li>Close GCRebuilder by going to Image &rarr; Close.</li>
        </ol>
        <img class="responsive-img" src="/images/help/gamecube1.png" alt="A screenshot of GCRebuilder showing the loaded Shrek SuperSlam ISO, with the 'root' element right-clicked."/>

        <h5>Upload the game files</h5>
        <ol>
            <li>Go to the folder you created with GCRebuilder.</li>
            <li>Go to the 'data' folder within.</li>
            <li>
                Make a copy of the MASTER.DAT and MASTER.DIR files and put them somewhere safe. These are the files we
                will be overwriting, so you need to do this to keep the originals!
            </li>
            <li>On the homepage of this site, upload the MASTER.DAT and MASTER.DIR files from the extracted folder.</li>
            <li>Select the console as 'Gamecube'.</li>
            <li>Click 'Submit'.</li>
        </ol>
        <img class="responsive-img" src="/images/help/gamecube2.png" alt="A screenshot of the extracted MASTER.DAT and MASTER.DIR files being uploaded to the tool."/>

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

        <h5>Replace the original game files, and repackage the ISO</h5>
        <ol>
            <li>Download the generated MASTER.DAT and MASTER.DIR files.</li>
            <li>
                Go to your extracted ISO folder, and replace the original MASTER.DAT and MASTER.DIR files in the
                'data' folder with the ones you just downloaded.
            </li>
            <li>
                Open GCRebuilder, and go to Root &rarr; Open, and select the folder named 'root' you extracted the
                ISO to.
            </li>
            <li>Click Root &rarr; Save, and give the ISO a new file name.</li>
            <li>Click Root &rarr; Rebuild to generate the ISO.</li>
            <li>You should be able to open your newly-built ISO in Dolphin and try out your changes!</li>
        </ol>
        <img class="responsive-img" src="/images/help/gamecube3.png" alt="A screenshot of GCRebuilder showing the loaded Shrek SuperSlam ISO, with the 'root' element right-clicked."/>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Navbar from '@/components/Navbar.vue';

export default defineComponent({
  name: 'Gamecube',
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