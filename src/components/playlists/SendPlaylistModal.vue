<template>
  <BottomSheetModal
    title="Send to platform"
    height="auto"
    :header-height="180"
    @close="handleClose"
  >
    <template #header-content>
      <p class="text-sm text-text-secondary">
        Choose which platform to send this playlist to
      </p>
    </template>

    <!-- Platform List -->
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
  </BottomSheetModal>
</template>

<script setup lang="ts">
const sending = ref(false);
import { computed, ref } from "vue";
import { useRouter, useRoute } from "vue-router";
import BottomSheetModal from "@/components/common/BottomSheetModal.vue";
import {
  useSendPlaylist,
  type ConnectedPlatform,
} from "@/composables/useSendPlaylist";

const router = useRouter();
const route = useRoute();

const { connectedPlatforms, sendPlaylistToPlatform } = useSendPlaylist();

// Get playlist ID from route params
const playlistId = computed<number>(() => {
  const id = route.params.playlistId as string;
  return parseInt(id, 10);
});

const handleClose = () => {
  router.back();
};

const handleSendToPlatform = async (platform: ConnectedPlatform) => {
  // Capture the playlistId before closing modal (to avoid losing ref value)
  const currentPlaylistId = playlistId.value;

  // Close modal immediately after selection
  handleClose();
  sending.value = true;
  // Start sending after modal is closed
  setTimeout(async () => {
    const success = await sendPlaylistToPlatform(
      currentPlaylistId,
      platform.name,
      platform
    );

    if (success) {
      console.log(`Playlist sent to ${platform.label} successfully`);
    } else {
      console.error(`Failed to send playlist to ${platform.label}`);
    }
  }, 300);
};
</script>
