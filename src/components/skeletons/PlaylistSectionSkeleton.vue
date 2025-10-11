<template>
  <div class="space-y-4">
    <!-- Section header -->
    <div class="flex items-center gap-3">
      <SkeletonLoader height="32px" width="32px" rounded="full" />
      <SkeletonLoader height="28px" width="150px" rounded="md" />
    </div>

    <!-- Playlist cards -->
    <div class="space-y-3">
      <PlaylistCardSkeleton v-for="i in count" :key="i" />
    </div>
  </div>
</template>

<script setup lang="ts">
import SkeletonLoader from '@/components/common/SkeletonLoader.vue'
import PlaylistCardSkeleton from './PlaylistCardSkeleton.vue'

interface Props {
  count?: number
}

withDefaults(defineProps<Props>(), {
  count: 3
})
</script>
