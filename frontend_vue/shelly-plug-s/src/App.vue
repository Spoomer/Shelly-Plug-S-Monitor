<script setup lang="ts">
import Shelly from './components/Shelly.vue'
</script>

<template>
  <header>
    <div>
      <h2>Shelly Plug S</h2>
    </div>
  </header>

  <main>
    <Shelly />
  </main>
</template>

<style scoped>
header {
  line-height: 1.5;
}
header div h2 {
  font-weight: bold;
}

@media (min-width: 1024px) {
  /* header {
    background-color: #222;
    padding: 20px;
    color: white;
  } */

  /* header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  } */
}
</style>
