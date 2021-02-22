<template>
    <div class="nav">
        <router-link to="/about">About</router-link>
        <router-link to="/upload">Upload</router-link>
        <router-link to="/ui/red">UI</router-link>
        <router-view/>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'Navbar',
});
</script>

<style scoped>
.nav {
    width: 100%;
    position: fixed;
    background-color: deepskyblue;
    top: 0;
    overflow: hidden;
}

.nav a {
    float: left;
    display: block;
    color: #f2f2f2;
    text-align: center;
    padding: 18px 16px;
    text-decoration: none;
}
</style>