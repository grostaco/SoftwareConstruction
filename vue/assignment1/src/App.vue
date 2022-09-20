<script setup lang="ts">
import ItemList from "./components/ItemList.vue"
</script>

<script lang="ts">
export default {
  components: {

  }
}
</script>

<template>
  <nav class="navbar dflex-justify-center">
    <div class="dflex dflex-col">
      <div class="bold font-lg">Software Construction</div>
      <div class="dflex dflex-gap-sm">
        <div>
          Made (not) with Rust!
        </div>
        ·
        <div>
          Assignment 1
        </div>
      </div>
    </div>
    <div class="dflex dflex-row dflex-gap-md">
      <a href="https://grostaco.github.io/SoftwareConstruction">Home</a>
      <a href="https://github.com/grostaco">GitHub</a>
      <a href="https://grostaco.herokuapp.com/">About Me</a>
      <a href="https://github.com/grostaco/SoftwareConstruction">This site's code</a>
    </div>
  </nav>
  <div class="divider"></div>
  <main>
    <ItemList/>
  </main>
</template>

<style scoped>
main {
  margin-top: 2.5rem;
}
</style>
