import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import "./assets/css/global.css";
import { createPinia } from "pinia";
import { createPlugin } from "@tauri-store/pinia";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import "tauri-plugin-safe-area-insets-css-api";
import { useUserStore } from "./store/user";
import { useSharedPlaylistsStore } from "./store/sharedPlaylists";
import { useAppStore } from "./store/app";
import { impactFeedback } from "@tauri-apps/plugin-haptics";

const pinia = createPinia();
pinia.use(createPlugin());

let app = createApp(App).use(pinia);
const userStore = useUserStore();
const sharedPlaylistsStore = useSharedPlaylistsStore();
const appStore = useAppStore();
await userStore.$tauri.start();
await sharedPlaylistsStore.$tauri.start();
await appStore.$tauri.start();

app = app.use(router);

listen<string>("routing", async (event) => {
  const route = event.payload;
  console.log("Received route event with query:", route);
  // Light haptic for routing from notifications
  try {
    await impactFeedback("light");
  } catch (error) {
    console.warn("Haptic feedback not available:", error);
  }
  router.push(route);
});

await router.isReady();

// Check if app was opened from notification or URL
await invoke("check_opening_notification");
await invoke("check_opening_url");

// Fetch shared playlists and friends at app start if user is authenticated
if (userStore.token) {
  sharedPlaylistsStore.fetchSharedPlaylists(true).catch((error) => {
    console.error("Failed to fetch shared playlists at app start:", error);
  });

  userStore.fetchFriends().catch((error) => {
    console.error("Failed to fetch friends at app start:", error);
  });
}

app.mount("#app");

console.log("après mount");

