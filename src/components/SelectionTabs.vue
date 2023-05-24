<template>
    <div class="selection-tabs">
        <div class="row">
          <div>
            <ul v-if="gametype == 'characters'" class="tabs">
              <li class="tab"><router-link :to="`/characters/blackknight`" class="active">Characters</router-link></li>
              <li class="tab"><router-link :to="`/stages/aiplayground`">Stages</router-link></li>
            </ul>
            <ul v-else-if="gametype == 'stages'" class="tabs">
              <li class="tab"><router-link :to="`/characters/blackknight`">Characters</router-link></li>
              <li class="tab"><router-link :to="`/stages/aiplayground`" class="active">Stages</router-link></li>
            </ul>
          </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'selection-tabs',
  props: {
    gametype: {
        required: true,
        type: String,
    }
  },
  mounted() {
    M.Tabs.init(document.querySelectorAll(".tabs"));
  }
});
</script>

<style scoped>
</style>