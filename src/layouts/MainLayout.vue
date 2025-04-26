<template>
  <header class="p-4 text-lg font-bold text-[#FFC436]">Swaply</header>

  <!-- Lien vers la page de login -->
  <a
    href="/login"
    class="absolute top-4 right-4 text-[#00CFE8] hover:text-[#FFC436] text-sm font-medium transition-all"
  >
    Login
  </a>

  <main class="flex-1 px-4 pt-2">
    <slot />
  </main>

  <BottomNav />
</template>

<script setup>
import BottomNav from "../components/nav/BottomNav.vue";
</script>
