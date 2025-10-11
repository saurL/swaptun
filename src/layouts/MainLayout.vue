<template>
  <div class="flex flex-col min-h-screen w-full bg-background">
    <!-- Header - auto-hides on scroll down -->

    <!-- Main content area with top padding for header + bottom padding for nav -->
    <main
      class="flex-1 min-h-screen overflow-y-auto px-6 pb-safe-plus-nav overflow-x-auto"
    >
      <AppHeader />
      <slot />
    </main>

    <!-- Bottom Navigation -->
    <BottomNav class="flex-shrink-0" />
  </div>
</template>

<script setup lang="ts">
import BottomNav from "@/components/nav/BottomNav.vue";
import AppHeader from "@/components/common/AppHeader.vue";
</script>
