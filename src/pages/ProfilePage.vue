<template>
  <MainLayout>
    <div class="min-h-screen bg-[#121212] text-white py-8">
    <!-- Header -->
    <div class="max-w-4xl mx-auto px-6 text-center">
      <h1 class="text-3xl font-semibold mb-4">Mon Profil</h1>
    </div>

    <!-- Profil Card -->
    <div class="max-w-4xl mx-auto px-6 bg-[#1E1E1E] rounded-2xl shadow-lg p-8 space-y-6">
      <!-- Photo de profil et nom -->
      <div class="flex items-center space-x-6">
        <img src="" alt="Photo de profil" class="w-24 h-24 rounded-full border-4 border-[#00CFE8]" />
        <div>
          <h2 class="text-2xl font-semibold">{{ userName }}</h2>
          <p class="text-sm text-gray-400">{{ userAge }} ans, {{ userCity }}</p>
        </div>
      </div>

      <!-- Bio -->
      <div>
        <h3 class="text-lg font-semibold mb-2">Bio</h3>
        <p class="text-sm text-gray-300">{{ userBio }}</p>
      </div>

      <!-- Paramètres -->
      <div class="flex justify-between items-center text-sm">
        <span class="text-gray-300">Paramètres</span>
        <button class="bg-[#00CFE8] text-[#121212] py-2 px-4 rounded-lg hover:bg-[#FFC436] transition">
          Modifier
        </button>
      </div>
    </div>
  </div>
  </MainLayout>
</template>

<script setup>
import MainLayout from '../layouts/MainLayout.vue'
const props = defineProps({
  userName: String,
  userAge: Number,
  userCity: String,
  userBio: String
})
</script>

<style scoped>
</style>
