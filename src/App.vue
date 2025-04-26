<script setup lang="ts">
import { useStore } from "./store/token";
import { watchEffect } from "vue";
import { info } from "@tauri-apps/plugin-log";
import { useRouter } from "vue-router";
import { ref, onMounted } from "vue";
import loadingAppAnimation from "./components/loadingAppAnimation.vue";
const router = useRouter();
const store = useStore();

const isLoading = ref(true);

watchEffect(() => {
  info(`Token: ${store.identification_token}`);
  if (store.identification_token !== "") {
    router.push("/homepage");
  }
});

onMounted(async () => {
  const startTime = Date.now(); // Enregistre le temps de début

  // Simulation d'un traitement long (API call, vérification de token, etc)
  await store.$tauri.start();

  // Calcule le temps écoulé et attend si nécessaire pour atteindre 2 secondes
  const elapsedTime = Date.now() - startTime;
  const remainingTime = Math.max(2000 - elapsedTime, 0);
  await new Promise((resolve) => setTimeout(resolve, remainingTime));

  // Une fois fini : on enlève l'écran de loading
  isLoading.value = false;
});
</script>

<template>
  <div
    class="min-h-screen w-screen bg-[#121212] text-white flex flex-col pb-20"
  >
    <loadingAppAnimation v-if="isLoading" />
    <RouterView v-else />
  </div>
</template>
