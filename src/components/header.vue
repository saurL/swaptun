<template>
  <header class="bg-blue-600 text-white p-4">
    <div class="container mx-auto flex justify-between items-center">
      <h1 class="text-xl font-bold">Swaply</h1>
      <nav>
        <ul class="flex space-x-4">
          <li>
            <nuxt-link to="/" class="hover:underline">Home</nuxt-link>
          </li>
          <li>
            <nuxt-link to="/connectionForm" class="hover:underline"
              >connection</nuxt-link
            >
          </li>
        </ul>
      </nav>
    </div>
  </header>
</template>

<script setup>
// No additional logic needed for this basic header
</script>

<style scoped>
/* Add any custom styles here if needed */
</style>
