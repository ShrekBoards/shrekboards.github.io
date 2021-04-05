<template>
    <div class="download">
        <Navbar/>
        <div class="row">
          <div class="col s6 offset-s3">
            <h2>Download</h2>
            <div class="card darken-1">
              <DownloadButton :filename="'MASTER.DAT'" :data="masterDat"/>
              <DownloadButton :filename="'MASTER.DIR'" :data="masterDir"/>
            </div>
          </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject, readonly } from 'vue';
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
    return {
        masterDat,
        masterDir,
    }
  }
});
</script>