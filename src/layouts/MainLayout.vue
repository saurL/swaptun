<template>
  <header class="p-4 text-lg font-bold text-[#FFC436]">Swaply</header>

  <!-- Lien vers la page de login -->
  <a
    href="/login"
    class="absolute top-4 right-4 text-[#00CFE8] hover:text-[#FFC436] text-sm font-medium transition-all"
  >
    Login
  </a>

  <div class="px-4 pt-2 flex-1 flex flex-col min-h-0">
    <slot />
  </div>
  <BottomNav />
</template>

<script setup>
import BottomNav from "../components/nav/BottomNav.vue";
import { onMounted } from "vue";
const setMinHeight = () => {
  const header = document.querySelector("header");
  const bottomNav = document.querySelector(".bottom-nav");
  const main = document.querySelector("main");
  if (header && bottomNav && main) {
    const headerHeight = header.offsetHeight;
    const bottomNavHeight = bottomNav.offsetHeight;
    const minHeight = `calc(100vh - ${headerHeight + bottomNavHeight}px)`;
    main.style.minHeight = minHeight;
  }
};

onMounted(() => {
  setMinHeight();
});
</script>
