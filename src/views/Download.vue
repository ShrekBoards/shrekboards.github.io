<template>
    <div class="download">
        <Navbar/>
        <DownloadButton :filename="'MASTER.DAT'" :data="masterDat"/>
        <DownloadButton :filename="'MASTER.DIR'" :data="masterDir"/>
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