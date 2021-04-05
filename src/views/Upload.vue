<template>
    <div class="ui">
        <Navbar/>
        <div class="row">
          <div class="col s6 offset-s3">
            <h2>Upload</h2>
            <UploadForm/>
          </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import UploadForm from '@/components/UploadForm.vue'
import Navbar from '@/components/Navbar.vue'

export default defineComponent({
  name: 'Upload',
  components: {
    Navbar,
    UploadForm,
  },
});
</script>