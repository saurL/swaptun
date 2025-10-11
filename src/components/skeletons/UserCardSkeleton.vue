<template>
  <div class="bg-white p-4 rounded-xl border border-secondary shadow-sm">
    <div class="flex items-center justify-between gap-4">
      <!-- User info -->
      <div class="flex-1">
        <SkeletonLoader height="20px" width="60%" rounded="md" />
      </div>

      <!-- Action button -->
      <SkeletonLoader height="36px" width="80px" rounded="xl" />
    </div>
  </div>
</template>

<script setup lang="ts">
import SkeletonLoader from '@/components/common/SkeletonLoader.vue'
</script>
