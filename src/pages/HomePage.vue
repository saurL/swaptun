<template>
  <MainLayout>
    <router-view />
  </MainLayout>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import { useNotifications } from "@/composables/useNotifications";
import { usePlaylistManagement } from "@/composables/usePlaylistManagement";
import { useTour } from "@/composables/useTour";
import MainLayout from "@/layouts/MainLayout.vue";

const { hasSeenTour, awaitTourEnd } = useTour();

const { setupNotifications, cleanup: cleanupNotifications } =
  useNotifications();
const {
  setupPlaylistListeners,
  fetchAllPlaylists,
  cleanup: cleanupPlaylists,
} = usePlaylistManagement();

onMounted(async () => {
  // Setup notifications only if user has seen the tour
  console.log("hasSeenTour:", hasSeenTour.value);
  await awaitTourEnd();
  console.log("User has seen the tour, setting up notifications.");
  await setupNotifications();

  // Setup playlist listeners
  await setupPlaylistListeners();

  // Fetch all playlists
  await fetchAllPlaylists();
});

onUnmounted(() => {
  cleanupNotifications();
  cleanupPlaylists();
});
</script>
