<template>
  <div class="bg-white p-4 rounded-xl border border-secondary shadow-sm">
    <div class="flex items-center justify-between gap-4">
      <!-- Left side: Playlist info -->
      <div class="flex-1 space-y-3">
        <SkeletonLoader height="24px" width="70%" rounded="md" />
        <SkeletonLoader height="16px" width="40%" rounded="md" />
      </div>

      <!-- Right side: Share button -->
      <SkeletonLoader height="40px" width="40px" rounded="full" />
    </div>
  </div>
</template>

<script setup lang="ts">
import SkeletonLoader from '@/components/common/SkeletonLoader.vue'
</script>
