<template>
    <div class="ui">
        <Navbar/>
        <UploadForm/>
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