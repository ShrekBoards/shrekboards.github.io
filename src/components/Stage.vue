<template>
    <div class="stage">
        <ul class="collapsible">
          <li>
            <div class="collapsible-header">
              <i class="material-icons iconadd">keyboard_arrow_right</i>
              <i class="material-icons iconremove">keyboard_arrow_down</i>
              {{ name }}
            </div>
            <div class="collapsible-body">
              <ul>
                <!-- Create a form entry for every field in the stage. We start with the text box entries. -->
                <li v-for="field in Object.keys(a).filter(k => typeof a[k] != 'boolean')" :key="field">
                  <div v-if="field !== 'name' && field !== 'keyframes' &&
                      (!field.startsWith('unknown') || (field.startsWith('unknown') && advancedModeEnabled))">
                    {{ field }}
                    <div class="input-field inline">
                      <input type="number" step="any" class="validate" v-model.number="a[field]"/>
                    </div>
                  </div>
                </li>
                <li v-for="field in Object.keys(a).filter(k => typeof a[k] == 'boolean')" :key="field">
                  <div v-if="!field.startsWith('unknown') || (field.startsWith('unknown') && advancedModeEnabled)">
                    <label>
                      <input v-model="a[field]" class="filled-in" type="checkbox"/>
                      <span>{{ field }}</span>
                    </label>
                  </div>
                </li>
              </ul>
            </div>
          </li>
          <!-- Keyframes -->
          <li v-if="a['keyframes'] != null && a['keyframes'].length > 0">
            <div class="collapsible-header" v-on:click="expand">
              <i class="material-icons iconadd">keyboard_arrow_right</i>
              <i class="material-icons iconremove">keyboard_arrow_down</i>
              Keyframes
            </div>
            <div class="collapsible-body">
              <div v-for="hitbox in a['keyframes']" :key="hitbox" class="hitbox card darken-1">
                <ul>
                  <li v-for="field in Object.keys(hitbox)" :key="field">
                    <div v-if="!field.startsWith('unknown') || (field.startsWith('unknown') && advancedModeEnabled)">
                      {{ field }}
                      <div class="input-field inline">
                        <input v-model.number="hitbox[field]"/>
                      </div>
                    </div>
                  </li>
                </ul>
              </div>
            </div>
          </li>
        </ul>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject, Ref, reactive } from 'vue';
import M from 'materialize-css';

export default defineComponent({
  name: 'stage-form',
  props: {
    name: { type: String, required: true },
    stage: { type: Object, required: true }
  },
  setup(props) {
      const a = reactive(props.stage);
      const advancedModeEnabled = (inject("advancedModeEnabled") as Ref<boolean>).value;
      return { a, advancedModeEnabled };
  },
  mounted() {
    const elems = document.querySelectorAll(".collapsible");
    M.Collapsible.init(elems, {});
  },
});
</script>

<style scoped>
.card {
  padding: 5px;
}

.iconadd {
  display:inline-block ;
}

.iconremove {
  display:none !important;
}

li.active .collapsible-header .material-icons.iconadd{
  display: none;
}

li.active .collapsible-header .material-icons.iconremove{
  display: inline-block !important;
}

input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

/* Firefox */
input[type=number] {
  -moz-appearance: textfield;
}

</style>