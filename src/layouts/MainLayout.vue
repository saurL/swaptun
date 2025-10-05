<template>
  <div class="flex flex-col min-h-screen w-full bg-background pt-safe">
    <!-- Header -->
    

    <!-- Main content area with bottom padding to prevent content from being hidden under nav -->
    <main class="flex-1 overflow-y-auto px-6 py-6 pb-safe-plus-nav">
      <div class="max-w-7xl mx-auto">
        <slot />
      </div>
    </main>

    <!-- Bottom Navigation -->
    <BottomNav class="flex-shrink-0" />
  </div>
</template>

<script setup lang="ts">
import BottomNav from "@/components/nav/BottomNav.vue";
</script>
