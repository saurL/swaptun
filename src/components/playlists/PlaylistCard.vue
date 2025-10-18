<template>
  <div class="relative" @click.self="toggleMusics">
    <Card
      hoverable
      padding="md"
      variant="white"
      :class="{ 'ring-2 ring-primary': isExpanded }"
      @click="toggleMusics"
    >
      <div class="flex items-start justify-between gap-4">
        <div class="flex-1 min-w-0">
          <h3 class="text-lg font-semibold text-text-primary truncate mb-2">
            {{ playlist.playlist.name }}
          </h3>
          <p
            v-if="playlist.playlist.description"
            class="text-sm text-text-secondary line-clamp-2 mb-3"
          >
            {{ playlist.playlist.description }}
          </p>
          <p class="text-xs text-text-secondary">
            {{ musics.length > 0 ? `${musics.length} tracks` : "No tracks" }}
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

      <!-- Musics list -->
      <Transition
        enter-active-class="transition-all duration-300 ease-out"
        enter-from-class="max-h-0 opacity-0"
        enter-to-class="max-h-[600px] opacity-100"
        leave-active-class="transition-all duration-300 ease-in"
        leave-from-class="max-h-[600px] opacity-100"
        leave-to-class="max-h-0 opacity-0"
      >
        <div
          v-if="isExpanded && musics.length > 0"
          class="mt-4 pt-4 border-t border-secondary"
        >
          <div class="space-y-2 max-h-96 overflow-y-auto">
            <div
              v-for="(music, index) in musics"
              :key="index"
              class="p-3 bg-background-secondary rounded-lg hover:bg-secondary transition-colors"
            >
              <p class="text-sm font-medium text-text-primary">
                {{ music.title }}
              </p>
              <p class="text-xs text-text-secondary">
                {{ music.artist }} • {{ music.album }}
              </p>
              <p v-if="music.genre" class="text-xs text-text-secondary mt-1">
                {{ music.genre }}
              </p>
            </div>
          </div>
        </div>
      </Transition>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
import Card from "@/components/common/Card.vue";
import type Playlist from "@/models/playlist";

interface Props {
  playlist: Playlist;
  expandedPlaylistId: string | null;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  share: [playlist: Playlist];
  expand: [playlistId: string | null];
}>();

const isExpanded = ref(false);

// Read musics directly from playlist (already loaded from backend)
const musics = computed(() => props.playlist.musics || []);

// Watch for external changes to expanded state
watch(
  () => props.expandedPlaylistId,
  (newVal) => {
    isExpanded.value = newVal === props.playlist.playlist.id;
  }
);

const toggleMusics = () => {
  const newExpandedState = !isExpanded.value;

  if (newExpandedState) {
    // Expanding: notify parent
    emit("expand", props.playlist.playlist.id);
  } else {
    // Collapsing: notify parent
    emit("expand", null);
  }
};
</script>
