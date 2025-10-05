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
const pinia = createPinia();
pinia.use(createPlugin());

let app = createApp(App).use(pinia);
const userStore = useUserStore();
await userStore.$tauri.start();
app = app.use(router);
listen<string>("routing", (event) => {
  const route = event.payload;
  console.log("Received route event with query:", route);
  router.push(route);
});

await router.isReady();

await invoke("check_opening_notification");

await invoke("check_opening_url");

app.mount("#app");

console.log("après mount");

