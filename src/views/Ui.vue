<template>
    <div class="ui">
        <Navbar/>
        <div class="row">
          <div class="col s3">
            <Sidebar :x="attacks" :selected="selectedCharacter"/>
          </div>
          <div class="col s9">
            <h4>{{ selectedCharacter }}</h4>
            <Attack v-for="a in attacks[selectedCharacter]" :key="a" :attack="a"/>
          </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject, Ref } from 'vue';
import { useRouter } from 'vue-router';
import { ShrekSuperSlamCharacterAttackCollection } from '@/types';
import Sidebar from '@/components/Sidebar.vue';
import Attack from '@/components/Attack.vue';
import Navbar from '@/components/Navbar.vue';

export default defineComponent({
  name: 'edit-ui',
  components: {
    Sidebar,
    Attack,
    Navbar,
  },
  props: {
    // This comes from the URL
    selectedCharacter: {
      required: true,
      type: String
    }
  },
  setup() {
      // If no character attacks JSON has been generated, redirect to the upload form
      const attacksGlobal = inject("attacks") as Ref<ShrekSuperSlamCharacterAttackCollection>;
      if (Object.keys(attacksGlobal.value).length == 0) {
          const router = useRouter();
          router.replace({ name: "Upload" });
      }

      // Get the character attacks
      const attacks = inject("attacks") as ShrekSuperSlamCharacterAttackCollection;
      return { attacks };
  }
});
</script>

<style scoped>
.ui {
  height: 100%;
}
</style>