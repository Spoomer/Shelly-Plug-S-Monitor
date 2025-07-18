<script setup lang="ts">
import { reactive } from 'vue'
import Shelly from './components/Shelly.vue'
import Archive from './components/Archive.vue'
const state = reactive({
  archive: false,
})

</script>

<template>
  <header>
    <div>
      <h2>Shelly Plug S</h2>
    </div>
  </header>

  <main>
    <button class="btn btn-primary mb-3" @click="()=>state.archive=!state.archive">Switch to {{state.archive ? "Dashboard" : "Archive" }}</button>
    <Shelly v-if="!state.archive"/>
    <Archive v-if="state.archive"/>
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
