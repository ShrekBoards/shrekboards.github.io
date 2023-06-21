<template>
    <div class="ui">
        <Navbar/>
        <div class="row">
          <div class="col s3">
            <Sidebar v-if="gametype == 'characters'" :x="characters" :gametype="gametype" :selected="selected"/>
            <Sidebar v-else-if="gametype == 'stages'" :x="stages" :gametype="gametype" :selected="selected"/>
          </div>
          <div class="col s9">
            <h4>{{ selected }}</h4>
            <div v-if="gametype == 'characters'">
              <Character :name="selected" :character="characters[selected]"/>
            </div>
            <div v-else-if="gametype == 'stages'">
              <Stage :name="selected" :stage="stages[selected]"/>
            </div>
          </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject, Ref } from 'vue';
import { useRouter } from 'vue-router';
import Character from '@/components/Character.vue';
import Navbar from '@/components/Navbar.vue';
import Sidebar from '@/components/Sidebar.vue';
import Stage from '@/components/Stage.vue';

export default defineComponent({
  name: 'edit-ui',
  components: {
    Character,
    Navbar,
    Sidebar,
    Stage,
  },
  props: {
    // These comes from the URL:
    // /<characters|stages>/:selected
    gametype: {
      required: true,
      type: String
    },
    selected: {
      required: true,
      type: String
    }
  },
  setup() {
      const charactersGlobalRef = inject("characters") as Ref<object>;
      const stagesGlobalRef = inject("stages") as Ref<object>;

      // If no JSON has been generated, redirect to the upload form
      if (Object.keys(charactersGlobalRef.value).length == 0 ||
          Object.keys(stagesGlobalRef.value).length == 0) {
        const router = useRouter();
        router.replace({ name: "Upload" });
      }

      return { characters: charactersGlobalRef.value, stages: stagesGlobalRef.value };
  },
});
</script>

<style scoped>
.ui {
  height: 100%;
}
</style>