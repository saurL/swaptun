<template>
  <div class="flex flex-col h-full bg-[#FFFFFF] overflow-y-auto">
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
        @click="openPlaylist(shared)"
        class="bg-[#FFF8F3] rounded-lg p-4 shadow-sm hover:shadow-md transition-shadow cursor-pointer relative"
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
              {{ shared.playlist_name }}
            </h3>
            <p class="text-sm text-[#7D7D7D] mt-1">
              Shared by
              <span class="font-medium text-[#CB5520]">{{
                shared.shared_by_username
              }}</span>
            </p>
            <p class="text-xs text-[#7D7D7D] mt-1">
              {{ formatDate(shared.shared_at) }}
            </p>
          </div>
        </div>

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
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useRouter } from "vue-router";
import { storeToRefs } from "pinia";
import {
  useSharedPlaylistsStore,
  type SharedPlaylist,
} from "@/store/sharedPlaylists";
import { useSendPlaylist } from "@/composables/useSendPlaylist";
import LoadingSpinner from "@/components/common/LoadingSpinner.vue";

const router = useRouter();
const sharedPlaylistsStore = useSharedPlaylistsStore();
const { loading } = storeToRefs(sharedPlaylistsStore);

const {
  hasConnectedPlatforms,
  hasSinglePlatform,
  sendToDefaultPlatform,
} = useSendPlaylist();

// Use the sorted playlists from the store (unviewed first, then by date)
const sharedPlaylists = computed(() => sharedPlaylistsStore.sortedPlaylists);

onMounted(async () => {
  await sharedPlaylistsStore.fetchSharedPlaylists();
});

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
  console.log("View playlist:", shared.playlist_id);
};

const sendPlaylist = async (shared: SharedPlaylist) => {
  await openPlaylist(shared);

  // If only one platform is connected, send directly
  if (hasSinglePlatform.value) {
    const success = await sendToDefaultPlatform(shared.playlist_id);
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
      params: { playlistId: shared.playlist_id },
    });
  }
};

const formatDate = (dateString: string): string => {
  const date = new Date(dateString);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);

  if (diffMins < 60) {
    return `${diffMins} minute${diffMins !== 1 ? "s" : ""} ago`;
  } else if (diffHours < 24) {
    return `${diffHours} hour${diffHours !== 1 ? "s" : ""} ago`;
  } else if (diffDays < 7) {
    return `${diffDays} day${diffDays !== 1 ? "s" : ""} ago`;
  } else {
    return date.toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  }
};
</script>
