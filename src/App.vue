<template>
  <NuxtLayout class="h-full flex flex-col">
    <NuxtPage />
  </NuxtLayout>
</template>

<script setup>
useHead({
  bodyAttrs: {
    class: "main",
  },
});
</script>

<style>
.main {
  width: 100vw;
  height: 100vh;
}
#__nuxt {
  @apply h-full;
}
</style>
