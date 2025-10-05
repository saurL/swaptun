<template>
  <section class="space-y-4">
    <LoadingSpinner v-if="loading" size="lg" container-class="py-12" />

    <div v-else-if="error" class="p-6 rounded-xl bg-error/10 border border-error/20">
      <p class="text-error text-center">{{ error }}</p>
    </div>

    <div
      v-else-if="playlists.length === 0"
      class="p-12 rounded-xl bg-background-secondary border border-secondary text-center"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        class="h-16 w-16 mx-auto mb-4 text-text-secondary"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"
        />
      </svg>
      <p class="text-text-secondary text-lg">{{ emptyMessage }}</p>
    </div>

    <PlaylistGrid v-else>
      <PlaylistCard
        v-for="playlist in playlists"
        :key="playlist.id"
        :playlist="playlist"
        @share="$emit('share', playlist)"
      />
    </PlaylistGrid>
  </section>
</template>

<script setup lang="ts">
import LoadingSpinner from "@/components/common/LoadingSpinner.vue";
import PlaylistGrid from "./PlaylistGrid.vue";
import PlaylistCard from "./PlaylistCard.vue";
import type Playlist from "@/models/playlist";

interface Props {
  title: string;
  playlists: Playlist[];
  loading?: boolean;
  error?: string | null;
  icon?: string;
  count?: number;
  emptyMessage?: string;
}

withDefaults(defineProps<Props>(), {
  loading: false,
  error: null,
  icon: undefined,
  count: undefined,
  emptyMessage: "No playlists available",
});

defineEmits<{
  share: [playlist: Playlist];
}>();
</script>
