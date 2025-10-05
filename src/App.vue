<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useSharedPlaylistsStore } from "./store/sharedPlaylists";
import ErrorNotification from "./components/common/ErrorNotification.vue";
import { info } from "@tauri-apps/plugin-log";

interface ErrorNotificationPayload {
  type: "server_error" | "network_error";
  message: string;
}

interface SharedNotificationData {
  playlist_id: number;
  playlist_name: string;
  shared_by_id: number;
  shared_by_username: string;
  shared_by_name: string;
}

const currentError = ref<ErrorNotificationPayload | null>(null);
const sharedPlaylistsStore = useSharedPlaylistsStore();

let unlistenError: UnlistenFn | null = null;
let unlistenPlaylistShared: UnlistenFn | null = null;

onMounted(async () => {
  // Listen for error notifications
  unlistenError = await listen<ErrorNotificationPayload>(
    "error_notification",
    (event) => {
      currentError.value = event.payload;
    }
  );

  // Listen for playlist shared notifications
  unlistenPlaylistShared = await listen<SharedNotificationData>(
    "playlist_shared",
    async (event) => {
      info(
        `Playlist shared notification received: ${JSON.stringify(
          event.payload
        )}`
      );

      const notification = event.payload;
      //afficher tous les attributs de notification dans la console grâce a une boucle
      for (const [key, value] of Object.entries(notification)) {
        info(`Notification attribute: ${key} = ${value}`);
      }
      if (notification) {
        try {
          // Add to shared playlists store
          await sharedPlaylistsStore.addSharedPlaylist(
            notification.playlist_id.toString(),
            notification.playlist_name,
            notification.shared_by_id,
            notification.shared_by_username
          );

          // Show a success notification (optional - you could create a success notification component)
          info(
            `New playlist shared: ${notification.playlist_name} by ${notification.shared_by_name}`
          );
        } catch (error) {
          info(`Failed to parse shared notification: ${error}`);
        }
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
    <ErrorNotification :error="currentError" @dismiss="dismissError" />
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
