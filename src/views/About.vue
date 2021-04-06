<template>
  <div class="about">
    <Navbar />
    <div class="row">
      <div class="col s6 offset-s3">
        <h2>About</h2>
        <p>
          SHAB'AINT is a tool for modifying character attack properties in the Shrek SuperSlam game files. It allows
          for modifying and regenerating the files provided by the game, allowing for persistent modifications that
          can be loaded on consoles or in emulators.
        </p>
        <p>
          The name is a portmanteau of Shaba Games - the company that developed Shrek SuperSlam - and "'aint",
          in the style of the Wii Homebrew Nintendont. The name is credited to ShrekBoards community member scrub.
        </p>
        <p>
          It is developed by <a href="https://github.com/kirbyUK">kirby</a> using Vue 3, TypeScript, WebAssembly and
          Materialize. The WASM portion is written in Rust. Thanks to this combination, everything runs client-side -
          no data is sent to any servers.
        </p>
        <p>
          View the source code on <a href="https://github.com/ShrekBoards/shrekboards.github.io">GitHub</a>.
        </p>
        <p>
          This web interface is effectively a GUI around the
          shrek-superslam Rust <a href="https://github.com/ShrekBoards/shrek-superslam">crate</a>, which allows for
          programmatic access to the Shrek SuperSlam game files, and is built from the reverse-engineering effort for
          the game.
        </p>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Navbar from '@/components/Navbar.vue';

export default defineComponent({
  name: 'About',
  components: {
      Navbar,
  },
});
</script>