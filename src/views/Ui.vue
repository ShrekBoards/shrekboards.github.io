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
              <Attack v-for="attack_name in Object.keys(characters[selected]).sort()" :key="loopKeyName(attack_name)" :name="attack_name" :attack="characters[selected][attack_name]"/>
            </div>
            <div v-else-if="gametype == 'stages'">
              <Stage v-for="stage_property in Object.keys(stages[selected]).sort()" :key="loopKeyName(stage_property)" :name="stage_property" :stage="stages[selected][stage_property]"/>
            </div>
          </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject, Ref } from 'vue';
import { useRouter } from 'vue-router';
import Attack from '@/components/Attack.vue';
import Navbar from '@/components/Navbar.vue';
import Sidebar from '@/components/Sidebar.vue';
import Stage from '@/components/Stage.vue';

export default defineComponent({
  name: 'edit-ui',
  components: {
    Attack,
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
      const attacksGlobalRef = inject("attacks") as Ref<object>;
      const stagesGlobalRef = inject("stages") as Ref<object>;

      // If no JSON has been generated, redirect to the upload form
      if (Object.keys(attacksGlobalRef.value).length == 0 ||
          Object.keys(stagesGlobalRef.value).length == 0) {
        const router = useRouter();
        router.replace({ name: "Upload" });
      }

      return { characters: attacksGlobalRef.value, stages: stagesGlobalRef.value };
  },
  methods: {
    loopKeyName(x: string) {
      return this.$props.selected + "/" + x;
    }
  }
});
</script>

<style scoped>
.ui {
  height: 100%;
}
</style>