<template>
  <div class="flex flex-col min-h-screen w-full bg-background">
    <!-- Header - auto-hides on scroll down -->

    <!-- Main content area with top padding for header + bottom padding for nav -->
    <main
      class="flex-1 min-h-screen overflow-y-auto px-6 pb-safe-plus-nav overflow-x-auto"
    >
      <AppHeader />
      <slot />
    </main>

    <!-- Bottom Navigation -->
    <BottomNav class="flex-shrink-0" />

    <!-- Global Loading Overlay for Playlist Send -->
    <LoadingOverlay :show="appStore.isSendingPlaylist" message="Sending playlist..." />

    <!-- Global Success Dialog for Playlist Send -->
    <ImportSuccessDialog
      v-if="appStore.playlistSendSuccess"
      ref="successDialogRef"
      :platform-label="appStore.playlistSendSuccess.platformLabel"
      :platform-icon="appStore.playlistSendSuccess.platformIcon"
      :platform-name="appStore.playlistSendSuccess.platformName"
      @open-app="handleOpenApp"
      @close="handleSuccessDialogClose"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import BottomNav from "@/components/nav/BottomNav.vue";
import AppHeader from "@/components/common/AppHeader.vue";
import LoadingOverlay from "@/components/common/LoadingOverlay.vue";
import ImportSuccessDialog from "@/components/common/ImportSuccessDialog.vue";
import { useAppStore } from "@/store/app";

const appStore = useAppStore();
const successDialogRef = ref<InstanceType<typeof ImportSuccessDialog> | null>(null);

// Watch for success state and show dialog
watch(() => appStore.playlistSendSuccess, (success) => {
  if (success) {
    // Small delay to ensure smooth transition
    setTimeout(() => {
      successDialogRef.value?.show();
    }, 100);
  }
});

const handleOpenApp = async (platformName: string) => {
  try {
    const playlistId = appStore.playlistSendSuccess?.playlistId;
    await invoke("open_external_app", {
      platform: platformName,
      playlistId: playlistId || null
    });
  } catch (error) {
    console.error("Error opening app:", error);
  }
};

const handleSuccessDialogClose = () => {
  appStore.setPlaylistSendSuccess(null);
};
</script>
