<template>
    <div class="attack">
        <ul class="collapsible">
          <li>
            <div class="collapsible-header" v-on:click="expand"><i class="material-icons">keyboard_arrow_right</i>{{ a.name }}</div>
            <div class="collapsible-body">
              <ul>
                <!-- Create a form entry for every field in the attack. We start with the text box entries. -->
                <li v-for="field in Object.keys(a).filter(k => typeof a[k] != 'boolean')" :key="field">
                  <div v-if="field !== 'name' && field !== 'hitboxes' && field !== 'projectile'">
                    {{ field }}
                    <div class="input-field inline">
                      <input v-model.number="a[field]"/>
                    </div>
                  </div>
                </li>
                <li v-for="field in Object.keys(a).filter(k => typeof a[k] == 'boolean')" :key="field">
                  <label>
                    <input v-model="a[field]" class="filled-in" type="checkbox"/>
                    <span>{{ field }}</span>
                  </label>
                </li>
              </ul>
            </div>
          </li>
          <!-- Hitboxes -->
          <li v-if="a['hitboxes'] != null && a['hitboxes'].length > 0">
            <div class="collapsible-header" v-on:click="expand"><i class="material-icons">keyboard_arrow_right</i>Hitboxes</div>
            <div class="collapsible-body">
              <div v-for="hitbox in a['hitboxes']" :key="hitbox" class="hitbox card darken-1">
                <ul>
                  <li v-for="field in Object.keys(hitbox)" :key="field">
                    {{ field }}
                    <div class="input-field inline">
                      <input v-model.number="hitbox[field]"/>
                    </div>
                  </li>
                </ul>
              </div>
            </div>
          </li>
          <!-- Projectile -->
          <li v-if="a['projectile'] != null">
            <div class="collapsible-header" v-on:click="expand"><i class="material-icons">keyboard_arrow_right</i>Projectile</div>
            <div class="collapsible-body">
              <div class="projectile">
                <ul>
                  <li v-for="field in Object.keys(a['projectile'])" :key="field">
                    {{ field }}
                    <div class="input-field inline">
                      <input v-model.number="a['projectile'][field]"/>
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
import { defineComponent, reactive } from 'vue';
import M from 'materialize-css';

export default defineComponent({
  name: 'Attack',
  props: {
    attack: { type: Object, required: true }
  },
  setup(props) {
      const a = reactive(props.attack);
      function expand(event: Event) {
        /**
         * Onclick handler for expanding and closing an attack entry.
         *
         * Modified the arrow icon in the attack box to indicate if the box
         * is expanded or not.
         *
         * Params:
         *   event: The onclick event passed from Vue.
         */
        const button = event.target as HTMLElement;
        if (button !== null && button.children.length > 0) {
          const currentIcon = button.children[0].innerHTML;
          if (currentIcon === "keyboard_arrow_right") {
            button.children[0].innerHTML = "keyboard_arrow_down";
          } else {
            button.children[0].innerHTML = "keyboard_arrow_right";
          }
        } else if (button.innerHTML === "keyboard_arrow_right") {
          button.innerHTML = "keyboard_arrow_down";
        } else if (button.innerHTML === "keyboard_arrow_down") {
          button.innerHTML = "keyboard_arrow_right";
        }
      }
      return { a, expand };
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
</style>