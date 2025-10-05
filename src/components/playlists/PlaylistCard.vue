<template>
  <Card hoverable padding="md" variant="white">
    <div class="flex items-start justify-between gap-4">
      <div class="flex-1 min-w-0">
        <h3 class="text-lg font-semibold text-text-primary truncate mb-2">
          {{ playlist.name }}
        </h3>
        <p
          v-if="playlist.description"
          class="text-sm text-text-secondary line-clamp-2 mb-3"
        >
          {{ playlist.description }}
        </p>
      </div>
      <button
        @click.stop="$emit('share', playlist)"
        class="flex-shrink-0 p-2 rounded-full bg-gradient-to-r from-primary-light to-primary-lighter text-white hover:shadow-lg transition-all active:scale-95"
        title="Share this playlist"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="h-5 w-5"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"
          />
        </svg>
      </button>
    </div>
  </Card>
</template>

<script setup lang="ts">
import Card from "@/components/common/Card.vue";
import type Playlist from "@/models/playlist";

interface Props {
  playlist: Playlist;
}

defineProps<Props>();

defineEmits<{
  share: [playlist: Playlist];
}>();
</script>
