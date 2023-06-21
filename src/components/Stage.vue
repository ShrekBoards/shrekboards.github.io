<template>
  <div class="stage">
      <Spitter v-for="spitter_name in Object.keys(stage['spitters']).sort()" :key="loopKeyName(spitter_name)" :name="spitter_name" :spitter="stage['spitters'][spitter_name]"/>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Spitter from '@/components/Spitter.vue';

export default defineComponent({
name: 'character-form',
components: {
  Spitter,
},
props: {
  name: { type: String, required: true },
  stage: { type: Object, required: true }
},
methods: {
  loopKeyName(x: string) {
    return this.$props.name + "/" + x;
  }
}
});
</script>
