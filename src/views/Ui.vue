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
            <SaveButton/>
          </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject, } from 'vue';
import { ShrekSuperSlamCharacterAttackCollection } from '@/types';
import Sidebar from '@/components/Sidebar.vue';
import Attack from '@/components/Attack.vue';
import Navbar from '@/components/Navbar.vue';
import SaveButton from '@/components/SaveButton.vue';

export default defineComponent({
  name: 'UI',
  components: {
    Sidebar,
    Attack,
    Navbar,
    SaveButton,
  },
  props: {
    // This comes from the URL
    selectedCharacter: {
      required: true,
      type: String
    }
  },
  setup(props) {
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