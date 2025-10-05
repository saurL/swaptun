<template>
  <div class="flex flex-col min-h-screen w-full bg-background">
    <!-- Header -->
    <header
      class="flex-shrink-0 px-6 pb-4 pt-safe-plus border-b border-secondary shadow-sm sticky top-0 z-20"
    >
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <img
            src="/src/assets/images/icon.svg"
            alt="Swaply"
            class="h-10 w-10"
          />
          <h1
            class="text-2xl font-bold bg-gradient-to-r from-primary via-primary-light to-primary-lighter bg-clip-text text-transparent"
          >
            Swaply
          </h1>
        </div>

        <!-- Actions header (optionnel pour plus tard) -->
        <div class="flex items-center gap-4">
          <slot name="header-actions" />
        </div>
      </div>
    </header>

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
