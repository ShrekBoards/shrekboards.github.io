<template>
    <div class="nav navbar-fixed">
        <nav>
            <div class="nav-wrapper">
                <a href="#" class="brand-logo">Shab'aint</a>
                <ul id="nav-mobile" class="right hide-on-med-and-down">
                    <li><router-link to="/about">About</router-link></li>
                    <li><router-link to="/upload">Upload</router-link></li>
                    <li><router-link to="/ui/red">UI</router-link></li>
                    <li><router-view/></li>
                </ul>
            </div>
        </nav>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'Navbar',
});
</script>