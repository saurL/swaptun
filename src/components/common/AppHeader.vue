<template>
  <header
    :class="[
      'app-header',
      'bg-white ',
      'h-[var(--safe-area-inset-top)] mb-3',
      'transition-transform duration-300 ease-in-out',
      isVisible ? 'translate-y-0' : '-translate-y-full',
    ]"
  ></header>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";

const isVisible = ref(true);
let lastScrollY = 0;
let ticking = false;

const handleScroll = () => {
  if (!ticking) {
    window.requestAnimationFrame(() => {
      const currentScrollY = window.scrollY;

      // Show header when scrolling up or at top
      if (currentScrollY < lastScrollY || currentScrollY < 10) {
        isVisible.value = true;
      }
      // Hide header when scrolling down (and not at top)
      else if (currentScrollY > lastScrollY && currentScrollY > 80) {
        isVisible.value = false;
      }

      lastScrollY = currentScrollY;
      ticking = false;
    });
    ticking = true;
  }
};

onMounted(() => {
  window.addEventListener("scroll", handleScroll, { passive: true });
});

onUnmounted(() => {
  window.removeEventListener("scroll", handleScroll);
});
</script>

<style scoped>
.app-header {
  /* Will be hidden above the viewport when translate-y-full is applied */
  will-change: transform;
}
</style>
