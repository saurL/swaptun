<template>
  <div class="text-white py-8 px-6 max-w-3xl mx-auto">
    <h2 class="text-4xl font-semibold text-left mb-2">Salut {{ userName }} !</h2>
    <p class="text-lg text-left text-gray-300">Partage tes playlists préférées ✨</p>
  </div>
</template>

<script setup>
const props = defineProps({
  userName: String
})
</script>
