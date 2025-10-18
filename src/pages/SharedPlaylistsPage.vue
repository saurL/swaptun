<template>
  <!-- Loading Overlay -->
  <LoadingOverlay :show="sending" message="Sending playlist..." />

  <!-- Router View for Modal -->
  <router-view />

  <!-- Loading State -->
  <div v-if="loading" class="flex justify-center items-center py-20">
    <LoadingSpinner />
  </div>

  <!-- Empty State -->
  <div
    v-else-if="sharedPlaylists.length === 0"
    class="flex flex-col items-center justify-center py-20 px-4"
  >
    <div class="text-6xl mb-4">🎵</div>
    <h2 class="text-xl font-semibold text-[#2E2E2E] mb-2">
      No shared playlists yet
    </h2>
    <p class="text-[#7D7D7D] text-center">
      When friends share playlists with you, they'll appear here
    </p>
  </div>

  <!-- Shared Playlists List -->
  <div v-else class="flex-1 px-4 py-4 space-y-3">
    <div
      v-for="shared in sharedPlaylists"
      :key="shared.id"
      @click="togglePlaylist(shared)"
      class="bg-[#FFF8F3] rounded-lg p-4 shadow-sm hover:shadow-md transition-all cursor-pointer relative"
      :class="{ 'ring-2 ring-[#CB5520]': expandedPlaylistId === shared.id }"
    >
      <!-- New Badge -->
      <div
        v-if="!shared.viewed"
        class="absolute top-2 right-2 bg-[#CB5520] text-white text-xs font-semibold px-2 py-1 rounded-full"
      >
        NEW
      </div>

      <div class="flex items-start gap-3">
        <div
          class="flex-shrink-0 w-12 h-12 bg-[#E87A3A] rounded-lg flex items-center justify-center text-white text-xl"
        >
          🎵
        </div>
        <div class="flex-1 min-w-0">
          <h3 class="text-lg font-semibold text-[#2E2E2E] truncate">
            {{ shared.playlist.playlist.name }}
          </h3>
          <p class="text-sm text-[#7D7D7D] mt-1">
            Shared by
            <span class="font-medium text-[#CB5520]">{{
              shared.shared_by.username
            }}</span>
          </p>
          <p class="text-xs text-[#7D7D7D] mt-1">
            {{ formatDate(shared.shared_at.toString()) }}
          </p>
          <p class="text-xs text-[#7D7D7D] mt-1">
            {{
              shared.playlist.musics.length > 0
                ? `${shared.playlist.musics.length} tracks`
                : "No tracks"
            }}x
          </p>
        </div>
        ²
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
          v-if="
            expandedPlaylistId === shared.id &&
            shared.playlist.musics &&
            shared.playlist.musics.length > 0
          "
          class="mt-4 pt-4 border-t border-[#F4C9A6]"
        >
          <div class="space-y-2 max-h-96 overflow-y-auto">
            <div
              v-for="(music, index) in shared.playlist.musics"
              :key="index"
              class="p-3 bg-[#FFFFFF] rounded-lg hover:bg-[#F4C9A6] transition-colors"
            >
              <p class="text-sm font-medium text-[#2E2E2E]">
                {{ music.title }}
              </p>
              <p class="text-xs text-[#7D7D7D]">
                {{ music.artist }} • {{ music.album }}
              </p>
              <p v-if="music.genre" class="text-xs text-[#7D7D7D] mt-1">
                {{ music.genre }}
              </p>
            </div>
          </div>
        </div>
      </Transition>

      <!-- Actions -->
      <div class="mt-3 flex gap-2">
        <button
          @click.stop="viewPlaylist(shared)"
          class="flex-1 bg-[#E87A3A] hover:bg-[#FF985C] text-white font-semibold py-2 px-4 rounded-lg transition-colors"
        >
          View Playlist
        </button>
        <button
          @click.stop="sendPlaylist(shared)"
          :disabled="!hasConnectedPlatforms"
          class="flex-1 bg-[#F4C9A6] hover:bg-[#E87A3A] hover:text-white text-[#CB5520] font-semibold py-2 px-4 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          :title="
            hasConnectedPlatforms
              ? 'Send to platform'
              : 'Connect a platform first'
          "
        >
          Send
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { useRouter } from "vue-router";
import { storeToRefs } from "pinia";
import { useSharedPlaylistsStore } from "@/store/sharedPlaylists";
import type { SharedPlaylist } from "@/models/playlist";
import { useSendPlaylist } from "@/composables/useSendPlaylist";
import { formatRelativeTime, utcToLocal } from "@/utils/helpers";
import LoadingSpinner from "@/components/common/LoadingSpinner.vue";
import LoadingOverlay from "@/components/common/LoadingOverlay.vue";

const router = useRouter();
const sharedPlaylistsStore = useSharedPlaylistsStore();
const { loading } = storeToRefs(sharedPlaylistsStore);

const sendPlaylistComposable = useSendPlaylist();
const {
  sending,
  hasConnectedPlatforms,
  hasSinglePlatform,
  sendToDefaultPlatform,
} = sendPlaylistComposable;

const expandedPlaylistId = ref<number | null>(null);

// Use the sorted playlists from the store (unviewed first, then by date)
const sharedPlaylists = computed(() => sharedPlaylistsStore.sortedPlaylists);

onMounted(async () => {
  // Fetch only if data is stale (respects cache timeout)
  sharedPlaylistsStore
    .fetchSharedPlaylists(false)
    .then()
    .catch((error) => {
      console.error("Failed to fetch shared playlists:", error);
    });
});

// Mark all as viewed when leaving the page
onUnmounted(() => {
  sharedPlaylistsStore.markAllAsViewed();
});

const togglePlaylist = async (shared: SharedPlaylist) => {
  // Mark as viewed if not already
  if (!shared.viewed) {
    try {
      await sharedPlaylistsStore.markAsViewed(shared.id);
    } catch (error) {
      console.error("Failed to mark playlist as viewed:", error);
    }
  }

  const isCurrentlyExpanded = expandedPlaylistId.value === shared.id;

  if (isCurrentlyExpanded) {
    // Collapse
    expandedPlaylistId.value = null;
  } else {
    // Expand (musics are already loaded in shared.playlist.musics)
    expandedPlaylistId.value = shared.id;
  }
};

const openPlaylist = async (shared: SharedPlaylist) => {
  // Mark as viewed if not already
  if (!shared.viewed) {
    try {
      await sharedPlaylistsStore.markAsViewed(shared.id);
    } catch (error) {
      console.error("Failed to mark playlist as viewed:", error);
    }
  }
};

const viewPlaylist = async (shared: SharedPlaylist) => {
  await openPlaylist(shared);
  // TODO: Navigate to playlist details page or open playlist modal
  console.log("View playlist:", shared.playlist.playlist.id);
};

const sendPlaylist = async (shared: SharedPlaylist) => {
  await openPlaylist(shared);

  // If only one platform is connected, send directly
  if (hasSinglePlatform.value) {
    const success = await sendToDefaultPlatform(
      Number(shared.playlist.playlist.id)
    );
    if (success) {
      console.log("Playlist sent successfully");
      // TODO: Show success toast
    } else {
      console.error("Failed to send playlist");
      // TODO: Show error toast
    }
  } else {
    // Multiple platforms - show modal to choose
    router.push({
      name: "send-playlist",
      params: { playlistId: shared.playlist.playlist.id },
    });
  }
};

const formatDate = (utcDateString: string): string => {
  const localDate = utcToLocal(utcDateString);
  return formatRelativeTime(localDate);
};
</script>
