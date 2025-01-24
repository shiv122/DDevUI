<script setup>
import Sidenavbar from "~/components/layout/Sidenavbar.vue";
import Header from "~/components/layout/Header.vue";
definePageMeta({
  colorMode: "dark",
});
</script>

<template>
  <div
    class="grid min-h-screen w-full md:grid-cols-[220px_1fr] lg:grid-cols-[280px_1fr]"
  >
    <Sidenavbar />
    <div class="flex flex-col">
      <Header />
      <slot />
    </div>
  </div>
</template>
