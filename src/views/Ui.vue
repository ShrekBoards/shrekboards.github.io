<template>
    <div class="ui">
        <Navbar/>
        <Sidebar :x="attacks"/>
        <Attack v-for="a in attacks[selectedCharacter]" :key="a" :attack="a"/>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject, } from 'vue';
import { ShrekSuperSlamCharacterAttackCollection } from '@/types';
import Sidebar from '@/components/Sidebar.vue';
import Attack from '@/components/Attack.vue';
import Navbar from '@/components/Navbar.vue'

export default defineComponent({
  name: 'UI',
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
  setup(props) {
      // Get the character attacks
      const attacks = inject("attacks") as ShrekSuperSlamCharacterAttackCollection;
      return { attacks };
  }
});
</script>