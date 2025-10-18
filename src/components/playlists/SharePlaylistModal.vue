<template>
  <!-- Loading Overlay -->
  <LoadingOverlay :show="sending" message="Sharing playlist..." />

  <!-- Success Dialog -->
  <ImportSuccessDialog
    ref="successDialogRef"
    platform-label="Friend"
    :platform-name="selectedFriend?.username || ''"
    @open-app="handleViewSharedPlaylist"
    @close="handleSuccessDialogClose"
  />

  <BottomSheetModal
    title="Share playlist"
    :header-height="240"
    @close="handleClose"
  >
    <template #header-content>
      <!-- Search Bar -->
      <SearchInput
        v-model="searchQuery"
        placeholder="Search for a friend..."
        class="mb-2"
      />
    </template>

    <!-- Friends List -->
    <LoadingSpinner v-if="loading" size="lg" container-class="py-12" />

        <div v-else-if="filteredFriends.length > 0" class="space-y-3">
          <button
            v-for="friend in filteredFriends"
            :key="friend.id"
            @click="handleShareWithFriend(friend)"
            :disabled="sending"
            class="w-full p-4 rounded-xl bg-background-secondary hover:bg-secondary border border-secondary hover:border-primary/30 transition-all group"
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <div
                  class="w-12 h-12 rounded-full bg-gradient-to-br from-primary to-primary-lighter flex items-center justify-center text-white text-lg font-bold"
                >
                  {{ friend.username?.charAt(0).toUpperCase() }}
                </div>
                <div class="text-left">
                  <p class="text-text-primary font-semibold">
                    {{ friend.username }}
                  </p>
                  <p class="text-sm text-text-secondary">{{ friend.email }}</p>
                </div>
              </div>
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-6 w-6 text-text-secondary group-hover:text-primary transition-colors"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M13 7l5 5m0 0l-5 5m5-5H6"
                />
              </svg>
            </div>
          </button>
        </div>

        <div
          v-else-if="!loading && userStore.friends.length === 0"
          class="py-12 text-center"
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
              d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"
            />
          </svg>
          <p class="text-text-secondary text-lg mb-2">No friends yet</p>
          <p class="text-text-secondary text-sm">
            Add friends to share playlists with them
          </p>
        </div>

        <div
          v-else-if="!loading && filteredFriends.length === 0"
          class="py-12 text-center"
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
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
          <p class="text-text-secondary">No friends match your search</p>
        </div>
  </BottomSheetModal>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRouter, useRoute } from "vue-router";
import { useUserStore } from "@/store/user";
import { invoke } from "@tauri-apps/api/core";
import BottomSheetModal from "@/components/common/BottomSheetModal.vue";
import SearchInput from "@/components/common/SearchInput.vue";
import LoadingSpinner from "@/components/common/LoadingSpinner.vue";
import LoadingOverlay from "@/components/common/LoadingOverlay.vue";
import ImportSuccessDialog from "@/components/common/ImportSuccessDialog.vue";
import type Playlist from "@/models/playlist";
import type User from "@/models/user";

const router = useRouter();
const route = useRoute();
const userStore = useUserStore();

const searchQuery = ref("");
const loading = ref(false);
const sending = ref(false);

const successDialogRef = ref<InstanceType<typeof ImportSuccessDialog> | null>(null);
const selectedFriend = ref<User | null>(null);

// Get playlist from route params
const playlist = computed<Playlist | null>(() => {
  const playlistId = route.params.playlistId as string;
  if (!playlistId) return null;

  // Search in all platform playlists
  return (
    userStore.spotifyPlaylists.find((p) => p.id == playlistId) ||
    userStore.deezerPlaylists.find((p) => p.id == playlistId) ||
    userStore.youtubePlaylists.find((p) => p.id == playlistId) ||
    userStore.applePlaylists.find((p) => p.id == playlistId) ||
    null
  );
});

// Filter friends based on search
const filteredFriends = computed(() => {
  if (!searchQuery.value) return userStore.friends;

  const query = searchQuery.value.toLowerCase();
  return userStore.friends.filter(
    (friend) =>
      friend.username?.toLowerCase().includes(query) ||
      friend.email?.toLowerCase().includes(query)
  );
});

const handleClose = () => {
  router.back();
};

const handleShareWithFriend = async (friend: User) => {
  console.log("Sharing with friend:", friend);
  console.log("Playlist:", playlist.value);
  console.log("Sending:", sending.value);
  if (!playlist.value || sending.value) return;

  try {
    sending.value = true;
    selectedFriend.value = friend;

    await invoke("share_playlist", {
      playlistId: playlist.value.id,
      userId: friend.id,
    });

    console.log(`Playlist shared with ${friend.username}`);

    // Close modal
    handleClose();

    // Show success dialog after modal is closed
    setTimeout(() => {
      successDialogRef.value?.show();
    }, 300);
  } catch (error) {
    console.error("Error sharing playlist:", error);
    selectedFriend.value = null;
  } finally {
    sending.value = false;
  }
};

const handleViewSharedPlaylist = () => {
  // Optional: Navigate to shared playlists or do nothing
  console.log("View shared playlist");
};

const handleSuccessDialogClose = () => {
  selectedFriend.value = null;
};

onMounted(() => {
  // Load friends if not already loaded
  if (userStore.friends.length === 0) {
    loading.value = true;
    // The friends should be loaded automatically by the store
    // But we set loading to false after a short delay
    setTimeout(() => {
      loading.value = false;
    }, 500);
  }
});
</script>
