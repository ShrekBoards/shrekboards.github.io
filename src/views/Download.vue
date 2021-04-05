<template>
    <div class="download">
        <Navbar/>
        <div class="row">
          <div class="col s6 offset-s3">
            <h2>Download</h2>
            <p>For more information on how to use these files, refer to the <router-link to="/help">help</router-link> documentation.</p>
            <div class="card darken-1">
              <ul>
                <li><div class="valign-wrapper"><DownloadButton :filename="'MASTER.DAT'" :data="masterDat"/>{{ Math.round(masterDat.length / Math.pow(2, 20)) }} MB</div></li>
                <li><div class="valign-wrapper"><DownloadButton :filename="'MASTER.DIR'" :data="masterDir"/>{{ Math.round(masterDir.length / Math.pow(2, 10)) }} KB</div></li>
              </ul>
            </div>
          </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject, readonly } from 'vue';
import { useRouter } from 'vue-router';
import DownloadButton from '@/components/DownloadButton.vue';
import Navbar from '@/components/Navbar.vue';

export default defineComponent({
  name: 'Download',
  components: {
      DownloadButton,
      Navbar,
  },
  setup() {
    const masterDatGlobal = inject("masterDat");
    const masterDirGlobal = inject("masterDir");
    const masterDat = readonly((masterDatGlobal as any).value);
    const masterDir = readonly((masterDirGlobal as any).value);

    // If the MASTER.DAT or MASTER.DIR are not generated, redirect to the upload page
    if (masterDat.length == 0 || masterDat.length == 0) {
      const router = useRouter();
      router.replace({ name: "Upload" });
    }

    return {
        masterDat,
        masterDir,
    }
  }
});
</script>

<style scoped>
li {
  padding: 10px;
}
</style>