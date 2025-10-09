<template>
  <!-- Loading Overlay -->
  <LoadingOverlay :show="sending" message="Sending playlist..." />

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
      class="relative w-full max-w-2xl pb -safe bg-white rounded-t-3xl shadow-2xl border-t border-secondary"
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
          <h2 class="text-2xl font-bold text-text-primary">Send to platform</h2>
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
        <p class="text-sm text-text-secondary">
          Choose which platform to send this playlist to
        </p>
      </div>

      <!-- Platform List -->
      <div
        class="overflow-y-auto p-6 pt-4"
        style="max-height: calc(75vh - 180px)"
      >
        <div v-if="connectedPlatforms.length > 0" class="space-y-3">
          <button
            v-for="platform in connectedPlatforms"
            :key="platform.name"
            @click="handleSendToPlatform(platform)"
            :disabled="sending"
            class="w-full p-4 rounded-xl bg-background-secondary hover:bg-secondary border border-secondary hover:border-primary/30 transition-all group"
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-4">
                <div
                  class="w-14 h-14 rounded-xl bg-white flex items-center justify-center shadow-sm"
                >
                  <img
                    :src="platform.icon"
                    :alt="platform.label"
                    class="w-10 h-10 object-contain"
                  />
                </div>
                <div class="text-left">
                  <p class="text-text-primary font-semibold text-lg">
                    {{ platform.label }}
                  </p>
                  <p class="text-sm text-text-secondary">
                    Export playlist to {{ platform.label }}
                  </p>
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

        <div v-else class="py-12 text-center">
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
              d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
            />
          </svg>
          <p class="text-text-secondary text-lg mb-2">No platforms connected</p>
          <p class="text-text-secondary text-sm">
            Connect to a music platform to send playlists
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRouter, useRoute } from "vue-router";
import LoadingOverlay from "@/components/common/LoadingOverlay.vue";
import {
  useSendPlaylist,
  type ConnectedPlatform,
} from "@/composables/useSendPlaylist";

const router = useRouter();
const route = useRoute();

const { sending, connectedPlatforms, sendPlaylistToPlatform } =
  useSendPlaylist();

// Get playlist ID from route params
const playlistId = computed<number>(() => {
  const id = route.params.playlistId as string;
  return parseInt(id, 10);
});

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

const modalStyle = computed(() => {
  const height = "auto";
  const maxHeight = "75vh";
  const transform = `translateY(${translateY.value}px)`;
  return {
    height,
    maxHeight,
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

  e.preventDefault();

  const now = Date.now();
  const currentTouchY = e.touches[0].clientY;
  const deltaY = currentTouchY - startY.value;

  const timeDelta = now - lastTime.value;
  if (timeDelta > 0) {
    velocity.value = (currentTouchY - lastY.value) / timeDelta;
  }

  lastY.value = currentTouchY;
  lastTime.value = now;

  if (deltaY > 0) {
    translateY.value = currentY.value + deltaY;
  } else if (deltaY < 0) {
    translateY.value = currentY.value + deltaY * 0.3;
  }
};

const handleTouchEnd = () => {
  if (!isDragging.value) return;

  isDragging.value = false;

  const modalHeight = modalRef.value?.offsetHeight || window.innerHeight * 0.75;
  const threshold = modalHeight * 0.25;

  const isSwipeDown = velocity.value > 0.5;

  if (translateY.value > threshold || isSwipeDown) {
    const windowHeight = window.innerHeight;
    translateY.value = windowHeight;
    setTimeout(() => {
      handleClose();
    }, 300);
  } else {
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

    const timeDelta = now - lastTime.value;
    if (timeDelta > 0) {
      velocity.value = (currentMouseY - lastY.value) / timeDelta;
    }

    lastY.value = currentMouseY;
    lastTime.value = now;

    if (deltaY > 0) {
      translateY.value = currentY.value + deltaY;
    } else if (deltaY < 0) {
      translateY.value = currentY.value + deltaY * 0.3;
    }
  };

  const handleMouseUp = () => {
    if (!isDragging.value) return;

    isDragging.value = false;

    const modalHeight =
      modalRef.value?.offsetHeight || window.innerHeight * 0.75;
    const threshold = modalHeight * 0.25;

    const isSwipeDown = velocity.value > 0.5;

    if (translateY.value > threshold || isSwipeDown) {
      const windowHeight = window.innerHeight;
      translateY.value = windowHeight;
      setTimeout(() => {
        handleClose();
      }, 300);
    } else {
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

const handleSendToPlatform = async (platform: ConnectedPlatform) => {
  if (sending.value) return;

  const success = await sendPlaylistToPlatform(playlistId.value, platform.name);

  if (success) {
    console.log(`Playlist sent to ${platform.label} successfully`);
    handleClose();
  } else {
    console.error(`Failed to send playlist to ${platform.label}`);
  }
};

onMounted(() => {
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
