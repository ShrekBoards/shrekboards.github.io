<template>
    <div class="attack">
        <button type="button" class="collapsible" v-on:click="expand">{{ a.name }}</button>
        <div class="content">
            <ul>
                <!-- Create a form entry for every field in the attack. -->
                <li v-for="field in Object.keys(a)" :key="field">
                    <div v-if="field !== 'name' && field !== 'hitboxes'">
                        <div v-if="a[field] === true || a[field] === false">{{ field }}: <input v-model="a[field]" type="checkbox"/></div>
                        <div v-else>{{ field }}: <input v-model.number="a[field]"/></div>
                    </div>
                </li>
            </ul>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, reactive } from 'vue';

export default defineComponent({
  name: 'Attack',
  props: {
    attack: { type: Object, required: true }
  },
  setup(props) {
      const a = reactive(props.attack);
      return { a };
  },
  methods: {
    expand: function(event: Event) {
      /**
       * Onclick handler for expanding and closing an attack entry.
       *
       * Params:
       *   event: The onclick event passed from Vue.
       */
      const button = event.target as HTMLElement;
        if (button !== null) {
          button.classList.toggle("active");
          const content = button.nextElementSibling as HTMLElement;
          if (content !== null) {
            if (content.style.display === "block") {
              content.style.display = "none";
            } else {
              content.style.display = "block";
            }
          }
        }
      }
    },
});
</script>

<style scoped>
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
</style>