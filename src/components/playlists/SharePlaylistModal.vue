<template>
  <div
    class="fixed inset-0 z-50 flex items-end justify-center"
    @click="handleClose"
  >
    <!-- Backdrop -->
    <div class="absolute inset-0 bg-black/50 backdrop-blur-sm"></div>

    <!-- Modal Content -->
    <div
      ref="modalRef"
      @click.stop
      class="relative w-full max-w-2xl bg-white rounded-t-3xl shadow-2xl border-t border-secondary"
      :class="{
        'animate-slide-up': isAnimating,
        'transition-none': isDragging,
        'transition-transform duration-300 ease-out': !isDragging,
      }"
      :style="modalStyle"
    >
      <!-- Drag Handle -->
      <div
        @touchstart="handleTouchStart"
        @touchmove="handleTouchMove"
        @touchend="handleTouchEnd"
        @mousedown="handleMouseDown"
        class="flex justify-center pt-3 pb-2 cursor-grab active:cursor-grabbing"
      >
        <div class="w-12 h-1.5 bg-text-secondary/30 rounded-full"></div>
      </div>

      <!-- Header -->
      <div
        class="sticky top-0 z-10 bg-white border-b border-secondary px-6 pb-4"
      >
        <div class="flex items-center justify-between mb-2">
          <h2 class="text-2xl font-bold text-text-primary">Share playlist</h2>
          <button
            @click="handleClose"
            class="p-2 rounded-full hover:bg-secondary transition-colors"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-6 w-6 text-text-secondary"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        <!-- Search Bar -->
        <SearchInput
          v-model="searchQuery"
          placeholder="Search for a friend..."
          class="mb-2"
        />
      </div>

      <!-- Friends List -->
      <div
        class="overflow-y-auto p-6 pt-4"
        style="max-height: calc(75vh - 240px)"
      >
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
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRouter, useRoute } from "vue-router";
import { useUserStore } from "@/store/user";
import { invoke } from "@tauri-apps/api/core";
import SearchInput from "@/components/common/SearchInput.vue";
import LoadingSpinner from "@/components/common/LoadingSpinner.vue";
import type Playlist from "@/models/playlist";
import type User from "@/models/user";

const router = useRouter();
const route = useRoute();
const userStore = useUserStore();

const searchQuery = ref("");
const loading = ref(false);
const sending = ref(false);

// Drag functionality
const modalRef = ref<HTMLElement | null>(null);
const isDragging = ref(false);
const isAnimating = ref(true);
const startY = ref(0);
const currentY = ref(0);
const translateY = ref(0);
const lastY = ref(0);
const lastTime = ref(0);
const velocity = ref(0);

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

const modalStyle = computed(() => {
  const height = "75vh";
  const transform = `translateY(${translateY.value}px)`;
  return {
    height,
    maxHeight: height,
    transform,
  };
});

const handleTouchStart = (e: TouchEvent) => {
  isDragging.value = true;
  isAnimating.value = false;
  startY.value = e.touches[0].clientY;
  currentY.value = translateY.value;
  lastY.value = e.touches[0].clientY;
  lastTime.value = Date.now();
  velocity.value = 0;
};

const handleTouchMove = (e: TouchEvent) => {
  if (!isDragging.value) return;

  e.preventDefault(); // Prevent scrolling while dragging

  const now = Date.now();
  const currentTouchY = e.touches[0].clientY;
  const deltaY = currentTouchY - startY.value;

  // Calculate velocity
  const timeDelta = now - lastTime.value;
  if (timeDelta > 0) {
    velocity.value = (currentTouchY - lastY.value) / timeDelta;
  }

  lastY.value = currentTouchY;
  lastTime.value = now;

  // Only allow dragging down, with slight resistance
  if (deltaY > 0) {
    translateY.value = currentY.value + deltaY;
  } else if (deltaY < 0) {
    // Small upward movement allowed with resistance
    translateY.value = currentY.value + deltaY * 0.3;
  }
};

const handleTouchEnd = () => {
  if (!isDragging.value) return;

  isDragging.value = false;

  // Get modal height directly from the element
  const modalHeight = modalRef.value?.offsetHeight || window.innerHeight * 0.75;
  const threshold = modalHeight * 0.25; // 25% of actual modal height

  // Check velocity for swipe gesture
  const isSwipeDown = velocity.value > 0.5; // Velocity threshold for swipe

  // If dragged more than 25% of modal height OR fast swipe down, close
  if (translateY.value > threshold || isSwipeDown) {
    // Animate close
    const windowHeight = window.innerHeight;
    translateY.value = windowHeight; // Move completely off screen
    setTimeout(() => {
      handleClose();
    }, 300);
  } else {
    // Reset position with spring animation
    translateY.value = 0;
  }
};

const handleMouseDown = (e: MouseEvent) => {
  isDragging.value = true;
  isAnimating.value = false;
  startY.value = e.clientY;
  currentY.value = translateY.value;
  lastY.value = e.clientY;
  lastTime.value = Date.now();
  velocity.value = 0;

  const handleMouseMove = (e: MouseEvent) => {
    if (!isDragging.value) return;

    const now = Date.now();
    const currentMouseY = e.clientY;
    const deltaY = currentMouseY - startY.value;

    // Calculate velocity
    const timeDelta = now - lastTime.value;
    if (timeDelta > 0) {
      velocity.value = (currentMouseY - lastY.value) / timeDelta;
    }

    lastY.value = currentMouseY;
    lastTime.value = now;

    // Only allow dragging down, with slight resistance
    if (deltaY > 0) {
      translateY.value = currentY.value + deltaY;
    } else if (deltaY < 0) {
      // Small upward movement allowed with resistance
      translateY.value = currentY.value + deltaY * 0.3;
    }
  };

  const handleMouseUp = () => {
    if (!isDragging.value) return;

    isDragging.value = false;

    // Get modal height directly from the element
    const modalHeight =
      modalRef.value?.offsetHeight || window.innerHeight * 0.75;
    const threshold = modalHeight * 0.25; // 25% of actual modal height

    // Check velocity for swipe gesture
    const isSwipeDown = velocity.value > 0.5;

    // If dragged more than 25% of modal height OR fast swipe, close
    if (translateY.value > threshold || isSwipeDown) {
      // Animate close
      const windowHeight = window.innerHeight;
      translateY.value = windowHeight;
      setTimeout(() => {
        handleClose();
      }, 300);
    } else {
      // Reset position
      translateY.value = 0;
    }

    document.removeEventListener("mousemove", handleMouseMove);
    document.removeEventListener("mouseup", handleMouseUp);
  };

  document.addEventListener("mousemove", handleMouseMove);
  document.addEventListener("mouseup", handleMouseUp);
};

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

    await invoke("share_playlist", {
      playlistId: playlist.value.id,
      userId: friend.id,
    });

    // Show success message (you can add a toast notification here)
    console.log(`Playlist shared with ${friend.username}`);

    // Close modal
    handleClose();
  } catch (error) {
    console.error("Error sharing playlist:", error);
    // Show error message (you can add a toast notification here)
  } finally {
    sending.value = false;
  }
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

  // Disable initial animation after mount
  setTimeout(() => {
    isAnimating.value = false;
  }, 300);
});
</script>

<style scoped>
@keyframes slide-up {
  from {
    transform: translateY(100%);
  }
  to {
    transform: translateY(0);
  }
}

.animate-slide-up {
  animation: slide-up 0.3s ease-out;
}
</style>
