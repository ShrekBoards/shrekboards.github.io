<template>
    <div class="character">
        <h5>Attacks</h5>
        <Attack v-for="attack_name in Object.keys(character['attacks']).sort()" :key="loopKeyName(attack_name)" :name="attack_name" :attack="character['attacks'][attack_name]"/>

        <h5>Physics</h5>
        <Physics :key="'physics/' + name" :physics="character['physics']['player_physics']"/>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Attack from '@/components/Attack.vue';
import Physics from '@/components/Physics.vue';

export default defineComponent({
  name: 'character-form',
  components: {
    Attack,
    Physics,
  },
  props: {
    name: { type: String, required: true },
    character: { type: Object, required: true }
  },
  methods: {
    loopKeyName(x: string) {
      return this.$props.name + "/" + x;
    }
  }
});
</script>
