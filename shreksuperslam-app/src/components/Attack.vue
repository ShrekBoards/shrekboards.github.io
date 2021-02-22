<template>
    <div class="attack">
        <button type="button" class="collapsible">attack</button>
        <div class="content">
            <p>hello</p>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';

export default defineComponent({
  name: 'Attack',
  props: {
    attack: { type: Object, required: true }
  },
  setup(props) {
      const a = ref(Object.keys(props.attack));
      return { a };
  }
});

const coll = document.getElementsByClassName("collapsible") as HTMLCollectionOf<HTMLElement>;
let i;

for (i = 0; i < coll.length; i++) {
    coll[i].addEventListener("click", function(this: HTMLElement) {
        this.classList.toggle("active");
        const content = <HTMLElement>this.nextElementSibling;
        if (content !== null) {
            if (content.style.display === "block") {
                content.style.display = "none";
            } else {
                content.style.display = "block";
            }
        }
    });
} 
</script>

<style scoped>
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
</style>