<template>
    <div class="attack">
        <ul class="collapsible">
          <li>
            <div class="collapsible-header" v-on:click="expand"><i class="material-icons">keyboard_arrow_right</i>{{ a.name }}</div>
            <div class="collapsible-body">
              <ul>
                <!-- Create a form entry for every field in the attack. -->
                <li v-for="field in Object.keys(a)" :key="field">
                    <div v-if="field !== 'name' && field !== 'hitboxes' && field !== 'projectile'">
                      <div v-if="a[field] === true || a[field] === false">
                        <p>
                          <input class="no-materialize" v-model="a[field]" type="checkbox"/>
                          <span>{{ field }}:</span>
                        </p>
                      </div>
                      <div v-else>
                        {{ field }}:
                        <div class="input-field inline">
                          <input v-model.number="a[field]"/>
                        </div>
                      </div>
                    </div>
                </li>
              </ul>
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
.no-materialize input {

}

/*
.attack {
    padding-left: 280px;
    padding-top: 2px;
    padding-bottom: 2px;
}

.collapsible {
    background-color: #eee;
    color: #444;
    cursor: pointer;
    padding: 18px;
    width: 100%;
    border: none;
    text-align: left;
    outline: none;
    font-size: 15px;
}

.active, .collapsible:hover {
  background-color: #ccc;
}

.content {
  padding: 0 18px;
  display: none;
  overflow: hidden;
  background-color: #f1f1f1;
}

ul {
    padding: 0px;
    list-style-type: none;
    text-align: left;
}

li {
    padding: 5px;
}
*/
</style>