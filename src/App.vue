<script setup lang="ts">
import { useStore } from "./store/token";
import { watchEffect, onMounted } from "vue";
import { info } from "@tauri-apps/plugin-log";
import { useRouter } from "vue-router";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import loadingAppAnimation from "./components/LoadingAppAnimation.vue";
const router = useRouter();
const store = useStore();

const isLoading = ref(true);

watchEffect(() => {
  info(`Token: ${store.identification_token}`);
  if (store.identification_token !== "") {
    invoke("verify_token", { token: store.identification_token })
      .then((isValid: any) => {
        if (isValid) {
          router.replace("/homepage");
        }
      })
      .catch((error) => {
        console.error("Token verification failed:", error);
        store.identification_token = "";
        router.replace("/");
      });
  }
  //router.push("/spotify-auth");
});
listen("app_ready", () => {
  isLoading.value = false;
});

onMounted(async () => {
  invoke<boolean>("is_app_ready").then((isReady) => {
    isLoading.value = !isReady;
  });
  isLoading.value = false;
});
</script>

<template>
  <div class="h-screen w-screen bg-[#121212] text-white flex flex-col min-h-0">
    <loadingAppAnimation v-if="isLoading" />
    <RouterView v-else />
  </div>
</template>

<style lang="css">
html,
body {
  overscroll-behavior-x: none; /* désactive le scroll horizontal naturel */
  touch-action: pan-x pan-y; /* désactive le zoom par pincement */
  -ms-overflow-style: none;
  overflow: hidden;
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
  -ms-overflow-style: none; /* IE and Edge */
  scrollbar-width: none; /* Firefox */
  touch-action: pan-x, pan-y;
}
</style>
