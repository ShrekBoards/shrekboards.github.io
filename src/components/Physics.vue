<template>
    <div class="physics">
        <ul class="collapsible">
          <li>
            <div class="collapsible-header">
              <i class="material-icons iconadd">keyboard_arrow_right</i>
              <i class="material-icons iconremove">keyboard_arrow_down</i>
              physics
            </div>
            <div class="collapsible-body">
              <ul>
                <!-- Create a form entry for every field in the physics object. We start with the text box entries. -->
                <li v-for="field in Object.keys(a).filter(k => typeof a[k] != 'boolean')" :key="field">
                  <div v-if="!field.startsWith('unknown') || (field.startsWith('unknown') && advancedModeEnabled)">
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
        </ul>
    </div>
</template>

<script lang="ts">
import { defineComponent, inject, Ref, reactive } from 'vue';
import M from 'materialize-css';

export default defineComponent({
  name: 'physics-form',
  props: {
    physics: { type: Object, required: true },
  },
  setup(props) {
      const a = reactive(props.physics);
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