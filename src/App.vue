<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useSharedPlaylistsStore } from "./store/sharedPlaylists";
import { useHaptics } from "./composables/useHaptics";
import ErrorNotification from "./components/common/ErrorNotification.vue";
import ToastContainer from "./components/common/ToastContainer.vue";
import OfflineIndicator from "./components/common/OfflineIndicator.vue";
import AppTour from "./components/tour/AppTour.vue";
import { info } from "@tauri-apps/plugin-log";

interface ErrorNotificationPayload {
  type: "server_error" | "network_error";
  message: string;
}

import type { PlaylistWithMusics } from "./models/playlist";

interface SharedNotificationData {
  type: "playlist_shared";
  shared_notification: {
    playlist: PlaylistWithMusics;
    shared_by: {
      id: number;
      username: string;
    };
  };
  route: string;
}

const currentError = ref<ErrorNotificationPayload | null>(null);
const sharedPlaylistsStore = useSharedPlaylistsStore();
const haptics = useHaptics();

let unlistenError: UnlistenFn | null = null;
let unlistenPlaylistShared: UnlistenFn | null = null;

onMounted(async () => {
  // Listen for error notifications
  unlistenError = await listen<ErrorNotificationPayload>(
    "error_notification",
    async (event) => {
      currentError.value = event.payload;
      await haptics.error();
    }
  );

  // Listen for playlist shared notifications
  unlistenPlaylistShared = await listen<SharedNotificationData>(
    "playlist_shared",
    async (event) => {
      const notification = event.payload;

      if (notification && notification.shared_notification) {
        // Trigger success haptic for receiving a shared playlist
        await haptics.success();

        const { playlist, shared_by } = notification.shared_notification;

        console.log("Received shared playlist notification:", {
          playlistName: playlist.playlist.name,
          sharedBy: shared_by.username,
          musicsCount: playlist.musics?.length || 0
        });

        // Add to shared playlists store with full playlist data
        sharedPlaylistsStore
          .addSharedPlaylistFromNotification(
            playlist,
            shared_by
          )
          .then(() => {
            info(`Shared playlist added: ${playlist.playlist.name}`);
          })
          .catch((e) => {
            info(`Failed to add shared playlist: ${e}`);
          });
      }
    }
  );
});

onUnmounted(() => {
  if (unlistenError) {
    unlistenError();
  }
  if (unlistenPlaylistShared) {
    unlistenPlaylistShared();
  }
});

const dismissError = () => {
  currentError.value = null;
};
</script>
<template>
  <div
    class="h-screen flex flex-col max-h-screen w-screen max-w-full overflow-y-scroll overflow-x-hidden items-center justify-center"
  >
    <OfflineIndicator />
    <ErrorNotification :error="currentError" @dismiss="dismissError" />
    <ToastContainer />
    <AppTour />
    <RouterView />
  </div>
</template>

<style lang="css">
html,
body {
  margin: 0;
  padding: 0;
  height: 100%;
  width: 100%;
  overflow: hidden; /* Prevent scrolling */
  overscroll-behavior: none; /* Disable overscroll effects */
  touch-action: none; /* Disable all touch scrolling and zooming */
  -ms-overflow-style: none; /* IE/Edge */
  scrollbar-width: none; /* Firefox */
  position: fixed; /* Fix the body to prevent scrolling */
  top: 0;
  left: 0;
}

div,
span {
  -ms-overflow-style: auto;
}

/* Hide scrollbar for Chrome, Safari and Opera */
html::-webkit-scrollbar {
  display: none;
}

/* Hide scrollbar for IE, Edge and Firefox */
* {
  user-select: none;
  -ms-overflow-style: none;
  /* IE and Edge */
  scrollbar-width: none;
  /* Firefox */
  touch-action: pan-x, pan-y;
}
</style>
